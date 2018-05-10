
use std::fs::File;
use futures::Future;

use protobuf_api::*;
use reactor::*;
use nn::{NnR, NnQ};
use *;


macro_rules! expect_nn_response {
    { $v:expr, $p:pat, $e:expr } => {
        if let $p = $v {
            Ok($e)
        } else {
            Err(app_error!(other "unexpected response, expected `{}`, got `{:?}`", stringify!($p), $v).into())
        }
    };
}


pub trait ListingSink : Send {
    fn files(&mut self, fs: &[HdfsFileStatusProto], src_pos: usize, last_in_src: bool, last: bool);
}

struct GetListingState<LS> {
    src: std::collections::LinkedList<String>,
    pos: usize,
    ls: LS,
    need_location: bool,
    err: Vec<IoError>
}

impl<LS> GetListingState<LS> {
    fn new(ls: LS, args: config::GetListing) -> GetListingState<LS> {
        GetListingState { src: args.src.into_iter().collect(), pos: 0, ls, need_location: args.need_location, err: vec![] }
    }
}

impl<LS> nn::ProtocolFsm for GetListingState<LS> where LS: ListingSink {

    fn start(self) -> (nn::ProtocolFsmResult, Self) {
        let next = if self.src.is_empty() {
            nn::ProtocolFsmResult::Exit
        } else {
            let cur_src =
                self.src.front().map(|v|v.clone()).unwrap_or(String::new());
            nn::ProtocolFsmResult::Send(NnQ::GetListing(
                pb_cons!(GetListingRequestProto,
                    src: cur_src,
                    start_after: vec![],
                    need_location: self.need_location)
            ))
        };
        (next, self)
    }

    fn incoming(mut self, nr: NnR) -> (nn::ProtocolFsmResult, Self) {
        let r = match nr {
            NnR::GetListing(glr) => glr,
            o => return (
                nn::ProtocolFsmResult::Exit,
                Self { err: vec_cons(self.err,app_error!(other "unexpected response: expected `NnR::GetListing` but got {:?}", o).into()), ..self }
            )
        };

        let dir_list = pb_decons!(GetListingResponseProto, r, dir_list);

        let (fs, rmdr): (&[HdfsFileStatusProto], u32) =
            pb_decons!(DirectoryListingProto, dir_list, partial_listing, remaining_entries);
        let last_in_src = rmdr == 0;
        let pos = self.pos;
        //if no entries remain to read for this src, switch to the next
        if last_in_src && !self.src.is_empty() {
            self.src.pop_front();
            self.pos += 1;
        }
        let last = self.src.is_empty();
        self.ls.files(fs, pos, last_in_src, last);

        let next = if last {
            nn::ProtocolFsmResult::Exit
        } else {
            //current src (must be Some())
            let cur_src =
                self.src.front().map(|v|v.clone()).unwrap_or_else(||String::new());
            //current file position (an entry where to start listing, i.e. end of last result)
            let cur_file = if last_in_src {
                vec![]
            } else {
                Vec::from(fs.last().map(|o| pb_decons!(HdfsFileStatusProto, o, path)).unwrap_or(&[]))
            };
            nn::ProtocolFsmResult::Send(NnQ::GetListing(
                pb_cons!(GetListingRequestProto,
                    src: cur_src,
                    start_after: cur_file,
                    need_location: self.need_location)
            ))
        };
        (next, self)
    }
}

impl<LS> ErrorAccumulator for GetListingState<LS> {
    fn error(self, e: IoError) -> Self { GetListingState { err: vec_cons(self.err, e), ..self } }
}

pub fn get_listing<LS: ListingSink + 'static>(rc: &ReactorClient, args: config::GetListing, ls: LS) -> BFTT<LS> {
    bimap(rc.run_nn(GetListingState::new(ls, args)), |gls| gls.ls)
}

//--------------------------------------------------------------------------------------------------

const MAX_FILE_LENGTH: u64 = std::i64::MAX as u64;

fn get_block_locations_request(src: String, offset: u64, length: u64) -> NnQ {
    let q = pb_cons!{GetBlockLocationsRequestProto,
        src: src,
        offset: offset,
        length: length
    };
    nn::NnQ::GetBlockLocations(q)
}

fn  get_block_locations_result(r: nn::NnR) -> IoResult<LocatedBlocksProto> {
    expect_nn_response!(r, NnR::GetBlockLocations(mut gbl), pb_decons!(GetBlockLocationsResponseProto, gbl, locations))
}

pub fn get_block_locations(rc: &ReactorClient, src: String, offset: u64, length: u64) -> BFI<LocatedBlocksProto> {
    let q = pb_cons!{GetBlockLocationsRequestProto,
        src: src,
        offset: offset,
        length: length
    };
    Box::new(rc.call_nn(nn::NnQ::GetBlockLocations(q)).and_then(move |r|
        if let NnR::GetBlockLocations(mut gbl) = r {
            let locs = pb_decons!(GetBlockLocationsResponseProto, gbl, locations);
            Ok(locs)
        } else {
            Err(app_error!(other "Unexpected response type").into())
        }
    ))
}


//--------------------------------------------------------------------------------------------------
use std::collections::VecDeque;


#[derive(Debug)]
struct LocatedBlock {
    /// Offset of the block within the file
    o: u64,
    b: ExtendedBlockProto,
    t: TokenProto,
    /// remaining datanodes
    locs: VecDeque<DatanodeInfoProto>
}

#[derive(Debug)]
enum GBLS {
    Idle,
    GetBlockLocations
}

#[derive(Debug)]
enum GBS {
    Idle(File),
    GetBlock { b: LocatedBlock, offset: u64, len: u64 }
}

#[derive(Debug)]
pub struct Get {
    /// Source file path
    src: String,
    /// file position read successfully so far. This is at a block boundary, unless at the end.
    read_offset: u64,
    /// Upper bound of `read_offset`.
    max_read_offset: u64,
    /// committed output file position. This is at a block boundary.
    /// Seek here to restart the current block
    write_offset: usize,
    /// queue of file blocks.
    q: VecDeque<LocatedBlock>,
    /// state
    s: (GBLS, GBS),
    /// done indicator
    finished: bool,
    /// errors accumulated so far
    err: Vec<IoError>
}

type GetOperation = ReactorOperation<nn::CallW, dt::ReadBlock<File>>;

impl Get {
    pub fn new(src: String, dst: File) -> Get { Get {
        src,
        read_offset: 0,
        max_read_offset: MAX_FILE_LENGTH,
        write_offset: 0,
        q: VecDeque::new(),
        s: (GBLS::Idle, GBS::Idle(dst)),
        finished: false,
        err: vec![]
    } }

    fn with_error(self, e: Error) -> Self { Get { err: vec_cons(self.err, e.into()), ..self } }

    fn idle(mut self) -> (Option<IoResult<GetOperation>>, Self) {
        match self.s {
            (GBLS::Idle, GBS::Idle(w)) =>
                if self.read_offset < self.max_read_offset {
                    //there are remaining blocks to read
                    let remainder = self.max_read_offset - self.read_offset;
                    let (req, s1) = match self.q.pop_front() {
                        Some(mut lb) => {
                            let len = pb_decons!(ExtendedBlockProto, lb.b, num_bytes);
                            let len = len.min(remainder);
                            //TODO fix 0
                            let offset = 0;
                            let req = Self::send_read_block(&mut lb, offset, len, w);
                            (req, (GBLS::Idle, GBS::GetBlock { b: lb, offset, len }))
                        }
                        None => {
                            //queue is empty, so refill
                            let req = Self::send_get_block_locations(self.src.clone(), self.read_offset, remainder);
                            (req, (GBLS::GetBlockLocations, GBS::Idle(w)))
                        }
                    };
                    (Some(req), Self { s: s1, ..self  })
                } else {
                    //Game over
                    (None, Self { s: (GBLS::Idle, GBS::Idle(w)), finished: true, ..self })
                }
            s =>
                (None, Self { s, ..self })
        }
    }

    fn gblr(mut self, rlbp: IoResult<LocatedBlocksProto>) -> (Option<IoResult<GetOperation>>, Self) {
       let s = match self.s {
            (GBLS::GetBlockLocations, gbs) => {
                let lbp = match rlbp {
                    Ok(lbp) => lbp,
                    Err(e) => return (Some(Err(e)), Get { s:(GBLS::GetBlockLocations, gbs), ..self} )
                };

                let (
                    file_length, blocks, _under_construction, _last_block,
                    _is_last_block_complete, _file_encryption_info
                ) = pb_decons!(LocatedBlocksProto, lbp,
                    file_length, blocks, under_construction, last_block,
                    is_last_block_complete, file_encryption_info
                    );

                self.max_read_offset = file_length.min(self.max_read_offset);
                for blk in blocks {
                    let (
                        b, offset, locs, corrupt, block_token, _is_cached, _storage_types, _storage_ids
                    ) = pb_decons!(LocatedBlockProto, blk,
                        b, offset, locs, corrupt, block_token, is_cached, storage_types, storage_ids
                        );
                    if corrupt { return (
                        Some(Err(app_error!(other "All instances of block {:?} are corrupt", b).into())),
                        Get { s: (GBLS::GetBlockLocations, gbs), ..self }
                    )}
                    //TODO these should be moved rather than cloned. Consider `take_b()` etc.
                    self.q.push_back(LocatedBlock {
                        o: offset,
                        b: b.clone(),
                        t: block_token.clone(),
                        locs: {
                            let o: Vec<DatanodeInfoProto> = locs.into_iter().map(|w| w.clone()).collect();
                            o.into()
                        }
                    });
                }
                (GBLS::Idle, gbs)
            }
            s => {
                return (Some(Err(app_error!(other "Unexpected GBLS::Idle/gblr").into())), Self { s, ..self })
            }
        };
        Self { s, ..self }.idle()
    }

    fn rbr(mut self, rb: dt::ReadBlock<File>) -> (Option<IoResult<GetOperation>>, Self) {
        let (w, mut err) = rb.result();

        let s = match self.s {
            (gbls, GBS::GetBlock { b, offset, len }) =>
                if let Some(e) = err.pop() {
                    //TODO
                    //w.position(self.write_offset)
                    (gbls, GBS::GetBlock { b, offset, len })

                } else {
                    self.read_offset += len;
                    self.write_offset += len as usize;
                    (gbls, GBS::Idle(w))
                },
            s => return (Some(Err(app_error!(other "Invalid /rbr").into())), Self { s, ..self})
        };
        Self { s, ..self }.idle()
    }

    fn send_get_block_locations(src: String, offset: u64, len: u64) -> IoResult<GetOperation> {
        trace!("Get: sending get_block_locations_request({}, {}, {})", src, offset, len);
        Ok(GetOperation::for_nn(nn::CallW::new(
            get_block_locations_request(src, offset, len)
        )))
    }

    fn send_read_block(lb: &mut LocatedBlock, offset: u64, len: u64, w: File) -> IoResult<GetOperation> {
        if let Some(dni) = lb.locs.pop_front() {
            Self::send_read_block_int(
                lb.b.clone(),
                lb.t.clone(),
                dni,
                offset,
                len,
                w
            )
        } else {
            Err(app_error!(other "Unable to retrieve block {:?}, no locations left", lb.b).into())
        }
    }

    fn send_read_block_int(b: ExtendedBlockProto, t: TokenProto, dni: DatanodeInfoProto, offset: u64, len: u64, w: File) -> IoResult<GetOperation> {
        use std::net::{SocketAddr, IpAddr};
        use std::str::FromStr;
        let dnid = pb_decons!(DatanodeInfoProto, dni, id);
        let (uuid, ip, port) = pb_decons!(DatanodeIDProto, dnid, datanode_uuid, ip_addr, xfer_port);
        let ip = IpAddr::from_str(&ip).map_err(|e| app_error!(other "Could not parse DN IP `{}`: `{}`", ip, e) as Error)?;
        let addr = SocketAddr::new(ip, port as u16);
        let bhp = pb_cons!(BaseHeaderProto, block: b, token: t);
        trace!("Get: sending read_block_request(uuid={}, addr={}, b={:?}, offset={}, num_bytes={})", uuid, addr, bhp, offset, len);
        Ok(GetOperation::for_dt(
            uuid.to_owned(), addr,
            dt::ReadBlock::new(bhp, offset, len, w)
        ))
    }

    /// Converts result into the generic form
    fn c_result(a: (Option<IoResult<GetOperation>>, Self)) -> (Vec<GetOperation>, Self) {
        match a {
            (Some(Ok(op)), sf) => (vec![op], sf),
            (Some(Err(e)), sf) => (vec![], sf.with_error(e.into())),
            (None, sf) => (vec![], sf)
        }
    }
}

impl ReactorProtocolFsm for Get {
    type FN = nn::CallW;
    type FD = dt::ReadBlock<File>;

    fn start(self) -> (Vec<ReactorOperation<Self::FN, Self::FD>>, Self) {
        Self::c_result(self.idle())
    }

    fn complete(self, op: GetOperation) -> (Vec<GetOperation>, Self) {
        match op {
            ReactorOperation { key: _, addr: _, p: CProtocolFsm::NN(nn::CallW::R(r)) } => {
                let r = get_block_locations_result(r);
                trace!("Get: received get_block_locations_result: {:?}", r);
                Self::c_result(self.gblr(r))
            }
            ReactorOperation { key: _, addr: _, p: CProtocolFsm::DT(rb) } => {
                trace!("Get: received resulting ReadBlock: {:?}", rb);
                Self::c_result(self.rbr(rb))
            }
            o =>
                (vec![], self.with_error(app_error!(other "Invalid RO result {:?}", o)))
        }
    }

    fn error(self, op: GetOperation) -> (Vec<GetOperation>, Self) {
        error!("Get: error on: {:?}", op);
        match op {
            ReactorOperation { key: _, addr: _, p: CProtocolFsm::NN(nn::CallW::Err(e)) } => {
                (vec![], self.with_error(e.into()))
            }
            ReactorOperation { key: _, addr: _, p: CProtocolFsm::DT(rb) } => {
                let (w, mut err) = rb.result();
                let err = err.pop().unwrap_or_else(|| app_error!(other "Unknown error"));
                (vec![], Self { s: (GBLS::Idle, GBS::Idle(w)), finished: true, ..self }.with_error(err.into()))
            }
            o =>
                (vec![], self.with_error(app_error!(other "generic RO error: {:?}", o)))
        }
    }
}

impl ErrorAccumulator for Get {
    fn error(self, e: IoError) -> Self { self.with_error(e.into())  }
}

impl ErrorExtractor for Get {
    fn extract_errors(mut self) -> (Vec<IoError>, Self) {
        (std::mem::replace(&mut self.err, vec![]), self)
    }
}

pub fn get(rc: &ReactorClient, src: String, dst: File) -> BFTT<Get> {
     rc.clone().run(Get::new(src, dst))
}


pub fn read_block() -> Result<()> { unimplemented!() }
