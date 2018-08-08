
use cmdx::*;
use nn::*;
use dt::*;
use protobuf_api::*;
use proto_tools::*;
#[allow(unused_imports)]
use futures::prelude::*;
use *;

pub use cmdx::Mdx;


//--------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct PartialListingMessage {
    fs: Vec<HdfsFileStatusProto>,
    last: bool
}

#[derive(Debug)]
pub struct GetListingState {
    source: String,
    need_location: bool
}

impl GetListingState {
    fn new(source: String, need_location: bool) -> GetListingState {
        GetListingState { source, need_location }
    }

    #[inline]
    fn q(&self, start: Vec<u8>) -> GetListingRequestProto {
        pb_cons!(GetListingRequestProto,
                    src: self.source.clone(),
                    start_after: start,
                    need_location: self.need_location
                    )
    }

    fn s(&mut self) -> GetListingRequestProto {
        self.q(vec![])
    }

    fn n(&mut self, nr: GetListingResponseProto) -> (Vec<HdfsFileStatusProto>, Option<GetListingRequestProto>) {
        let dir_list = pb_decons!(GetListingResponseProto, nr, dir_list);
        let (fs, remaining_entries) = pb_decons!(DirectoryListingProto, dir_list,
                    partial_listing, remaining_entries);

        if remaining_entries == 0 {
            (fs, None)
        } else {
            let last_filename = Vec::from(
                fs.last().map(|o| pb_get!(HdfsFileStatusProto, o, path)).unwrap_or(&[])
            );
            (fs, Some(self.q(last_filename)))
        }
    }
}

impl ChatF for GetListingState {
    type NQ = MdxQ;
    type NR = MdxR;
    type UR = Vec<HdfsFileStatusProto>;

    fn start(&mut self) -> MdxQ {
        MdxQ::NN(0, None, NnaQ::new(NnQ::GetListing(self.s())))
    }

    fn next(&mut self, nr: MdxR) -> Result<(Vec<HdfsFileStatusProto>, Option<MdxQ>)> {
        match nr {
            MdxR::NN(_, NnaR { inner: NnR::GetListing(glrp)}) => {
                let (a, ob) = self.n(glrp);
                Ok((a, ob.map(|n| MdxQ::NN(0, None, NnaQ::new(NnQ::GetListing(n))))))
            }
            other =>
                Err(app_error!(other "Unexpected {:?} where NN(GetListing) is expected", other))
        }
    }
}


pub type GetListingStream = chat::T<Mdx, GetListingState>;

pub fn get_listing(mdx: Mdx, source: String, need_location: bool) -> GetListingStream {
    chat::new(mdx, GetListingState::new(source, need_location))
}


#[test]
fn test_get_listing() {
    init_env_logger!();

    use util::test::ptk::*;
    let host_port = "127.0.0.1:58019";
    let t = spawn_test_server(host_port, test_script! {

    expect "68:72:70:63:09:00:00:00:00:00:1e:10:08:02:10:00:18:05:22:08:01:02:03:04:04:03:02:01:\
    0c:12:0a:0a:08:63:6c:6f:75:64:65:72:61:00:00:00:58:10:08:02:10:00:18:00:22:08:01:02:03:04:04:\
    03:02:01:3e:0a:0a:67:65:74:4c:69:73:74:69:6e:67:12:2e:6f:72:67:2e:61:70:61:63:68:65:2e:68:61:\
    64:6f:6f:70:2e:68:64:66:73:2e:70:72:6f:74:6f:63:6f:6c:2e:43:6c:69:65:6e:74:50:72:6f:74:6f:63:\
    6f:6c:18:01:07:0a:01:2f:12:00:18:00",

    send "00:00:01:70:12:08:00:10:00:18:09:3a:08:01:02:03:04:04:03:02:01:40:01:db:02:0a:d8:02:0a:\
    3d:08:01:12:0a:62:65:6e:63:68:6d:61:72:6b:73:18:00:22:03:08:ff:03:2a:04:68:64:66:73:32:0a:73:\
    75:70:65:72:67:72:6f:75:70:38:e1:d7:df:d6:d5:2b:40:00:50:00:58:00:68:8e:80:01:70:00:80:01:00:\
    0a:39:08:01:12:05:68:62:61:73:65:18:00:22:03:08:ed:03:2a:05:68:62:61:73:65:32:0a:73:75:70:65:\
    72:67:72:6f:75:70:38:b4:be:e4:95:f7:2b:40:00:50:00:58:00:68:8d:80:01:70:09:80:01:00:0a:31:08:\
    01:12:04:73:6f:6c:72:18:00:22:03:08:ed:03:2a:04:73:6f:6c:72:32:04:73:6f:6c:72:38:e1:91:e8:d6:\
    d5:2b:40:00:50:00:58:00:68:f9:81:01:70:00:80:01:00:0a:36:08:01:12:03:74:6d:70:18:00:22:03:08:\
    ff:07:2a:04:68:64:66:73:32:0a:73:75:70:65:72:67:72:6f:75:70:38:eb:b2:ab:b4:97:2c:40:00:50:00:\
    58:00:68:84:80:01:70:05:80:01:00:0a:37:08:01:12:04:75:73:65:72:18:00:22:03:08:ed:03:2a:04:68:\
    64:66:73:32:0a:73:75:70:65:72:67:72:6f:75:70:38:b7:b5:e6:d6:d5:2b:40:00:50:00:58:00:68:82:80:\
    01:70:08:80:01:00:0a:36:08:01:12:03:76:61:72:18:00:22:03:08:ed:03:2a:04:68:64:66:73:32:0a:73:\
    75:70:65:72:67:72:6f:75:70:38:a4:f2:e5:d6:d5:2b:40:00:50:00:58:00:68:85:80:01:70:02:80:01:00:\
    10:00"
    });

    use std::net::ToSocketAddrs;

    let addr = host_port.to_socket_addrs().unwrap().next().unwrap();

    let session_data = SessionData {
        effective_user: "cloudera".to_owned(),
        forced_client_id: Some(vec![1, 2, 3, 4, 4, 3, 2, 1])
    };
    let mdx = Mdx::new(1, 1, session_data,  addr, vec![]);

    let gls = get_listing(mdx, "/".to_owned(), false);

    let result: Vec<HdfsFileStatusProto> =
        gls.map(|s| futures::stream::iter_ok::<_, Error>(s.into_iter()))
            .flatten()
            .collect()
            .wait()
            .expect("gls.wait()");

    let y: Vec<Cow<str>> = result.iter().map(|fs| String::from_utf8_lossy(fs.get_path())).collect();
    let z: Vec<Cow<str>> =(["benchmarks", "hbase", "solr", "tmp", "user", "var"]).iter().map(|x|Cow::from(*x)).collect();
    assert_eq!(y, z);

    //-----------------------------------
    let _ = t.join().unwrap();

}

//==================================================================================================
use bytes::Bytes;
use std::collections::VecDeque;
use dt::ReadBlock;

enum GetFsmEvent {
    Init,
    /// Acknowledge reception of the packet. Arg is `data_len`.
    PacketAck(u64),
    /// Signal that there was an error receiving the packet (CRC or whatever else)
    PacketError,
    /// End of stream has been received when reading a block
    BlockComplete,
    /// Block locations have been received
    BlockLocations(Vec<LocatedBlock>)
}

enum GetFsmAction {
    NOP,
    RequestBlockLocations { offset: u64, len: u64 },
    ReadBlock(DatanodeInfo, ReadBlock),
    Err(Error),
    Success
}

/*
#[derive(Debug)]
pub struct DatanodeID {
    datanode_uuid: String,
    ip_addr: String,
    xfer_port: u32
}

#[derive(Debug)]
pub struct DatanodeInfo {
    id: DatanodeID
}
*/
#[derive(Debug, Clone)]
pub struct DatanodeInfo {
    inner: DatanodeInfoProto
}

impl DatanodeInfo {
    pub fn into_xfer_info(self) -> (String, String, u32) {
        let id = pb_decons!(DatanodeInfoProto, self.inner, id);
        pb_decons!(DatanodeIDProto, id, datanode_uuid, ip_addr, xfer_port)
    }
}

impl Into<DatanodeInfoProto> for DatanodeInfo {
    fn into(self) -> DatanodeInfoProto {
        self.inner
    }
}

impl From<DatanodeInfoProto> for DatanodeInfo {
    fn from(inner: DatanodeInfoProto) -> Self {
        DatanodeInfo { inner }
    }
}


#[derive(Debug)]
struct LocatedBlock {
    /// Offset of the block within the file
    o: u64,
    b: ExtendedBlock,
    t: Token,
    /// remaining datanodes
    locs: VecDeque<DatanodeInfo>
}

impl LocatedBlock {
    /// Builds ReadBlock action given `[read_offset, max_read_offset)`.
    /// Returns `GetFsmAction::ReadBlock` on success, `GetFsmAction::NOP` if `read_offset` is
    /// ahead of the block's range, various `GetFsmAction::Err` otherwise.
    fn build_read_block(&mut self, read_offset: u64, max_read_offset: u64) -> GetFsmAction {
        let lb = self.o;
        let ub = lb + self.b.get_num_bytes();
        let lb_ok = lb <= read_offset;
        let ub_ok = read_offset < ub;
        if lb_ok && ub_ok {
            //fetch next DNI
            if let Some(dni)  = self.locs.pop_front() {
                GetFsmAction::ReadBlock(dni, ReadBlock {
                    b: self.b.clone(),
                    t: self.t.clone(),
                    offset: read_offset - lb,
                    len: max_read_offset.min(ub) - read_offset
                })
            } else {
                //no valid replica
                GetFsmAction::Err(app_error!(other "All replicas of block {:?} are corrupt", self.b))
            }
        } else if !ub_ok {
            //Skip to the next block
            GetFsmAction::NOP
        } else {
            //error -- gap between blocks. NN corruption.
            GetFsmAction::Err(app_error!(other
            "Read pointer falls behind the current block in chain (gap between blocks) on {:?}: \
            current block's range: [{}, {}), requested range [{}, {})",
                self.b, lb, ub, read_offset, max_read_offset
            ))
        }
    }
}

struct GetFsm {
    q: VecDeque<LocatedBlock>,
    /// Upper bound of `read_offset` (size of the file)
    max_read_offset: u64,
    /// file position read successfully so far. This is at a packet boundary or at the end.
    read_offset: u64
}

impl GetFsm {
    fn new(read_offset: u64, max_read_offset: u64) -> GetFsm {
        GetFsm { q: VecDeque::new(), read_offset, max_read_offset }
    }

    fn adjust_max_read_offset(&mut self, max_read_offset: u64) {
        self.max_read_offset = max_read_offset.min(self.max_read_offset);
    }

    fn next_block(&mut self) -> GetFsmAction {
        if self.read_offset < self.max_read_offset {
            loop {
                let a = match self.q.front_mut() {
                    Some(located_block) =>
                        located_block.build_read_block(self.read_offset, self.max_read_offset),
                    None =>
                        GetFsmAction::RequestBlockLocations {
                            offset: self.read_offset,
                            len: self.max_read_offset - self.read_offset
                        }
                };
                if let GetFsmAction::NOP = a {
                    //skip the block and start over
                    self.q.pop_front();
                } else {
                    break a
                }
            }
        } else {
            GetFsmAction::Success
        }
    }

    fn handle(&mut self, evt: GetFsmEvent) -> GetFsmAction {
        use self::GetFsmEvent as E;
        use self::GetFsmAction as A;

        match evt {
            E::Init | E::BlockComplete | E::PacketError =>
                self.next_block(),
            E::BlockLocations(blocks) =>
                if blocks.is_empty() {
                    A::Err(app_error!(other "invalid empty BlockLocations"))
                } else {
                    self.q.extend(blocks.into_iter());
                    self.next_block()
                }
            E::PacketAck(len) => {
                self.read_offset += len;
                A::NOP
            }
        }
    }
}

pub struct Get {
    /// Source file path
    src: String,
    fsm: GetFsm
}

impl Get {
    fn new(src: String, read_offset: u64, max_read_offset: u64) -> Get {
        Get { src, fsm: GetFsm::new(read_offset, max_read_offset) }
    }

    fn translate_a(&self, a: GetFsmAction) -> SourceAction<MdxQ, Bytes> {
        fn get_block_locations_request(src: String, offset: u64, length: u64) -> MdxQ {
            let q = pb_cons!{GetBlockLocationsRequestProto,
                src: src,
                offset: offset,
                length: length
            };
            MdxQ::NN(0, None, NnaQ::new(nn::NnQ::GetBlockLocations(q)))
        }

        fn read_block_request(datanode_info: DatanodeInfo, read_block: ReadBlock) -> Result<MdxQ> {
            use std::net::{SocketAddr, IpAddr};
            use std::str::FromStr;

            let (uuid, ip, port) = datanode_info.into_xfer_info();
            let ip = IpAddr::from_str(&ip).map_err(|e| app_error!(other "Could not parse DN IP `{}`: `{}`", ip, e))?;
            let addr = SocketAddr::new(ip, port as u16);
            Ok(MdxQ::DT(0, Some(addr), dt::DtaQ::ReadBlock(read_block)))
        }

        match a {
            GetFsmAction::NOP =>
                SourceAction::z(),
            GetFsmAction::RequestBlockLocations { offset, len } =>
                SourceAction::z().send(get_block_locations_request(self.src.clone(), offset, len)),
            GetFsmAction::ReadBlock(datanode_info, read_block) =>
                match read_block_request(datanode_info, read_block) {
                    Ok(r) => SourceAction::z().send(r),
                    Err(e) => SourceAction::z().err(e)
                }
            GetFsmAction::Err(e) =>
                SourceAction::z().err(e),
            GetFsmAction::Success =>
                SourceAction::z().eos()
        }
    }

    #[inline]
    fn handle_t(&mut self, evt: GetFsmEvent) -> SourceAction<MdxQ, Bytes> {
        let a = self.fsm.handle(evt);
        self.translate_a(a)
    }

    fn translate_block_locations(&mut self, gblrp: GetBlockLocationsResponseProto) -> Result<Vec<LocatedBlock>> {
        let lbp = pb_decons!(GetBlockLocationsResponseProto, gblrp, locations);
        let (
            file_length, blocks
            //, _under_construction, _last_block, _is_last_block_complete, _file_encryption_info
        ) = pb_decons!(LocatedBlocksProto, lbp,
                    file_length, blocks
                    //, under_construction, last_block, is_last_block_complete, file_encryption_info
                    );

        self.fsm.adjust_max_read_offset(file_length);

        blocks.into_iter().fold(Ok(Vec::new()), |mut r, block| {
            if let Ok(r) = r {
                let (
                    b, offset, locs, corrupt, block_token //, _is_cached, _storage_types, _storage_ids
                ) = pb_decons!(LocatedBlockProto, block,
                        b, offset, locs, corrupt, block_token //, is_cached, storage_types, storage_ids
                        );
                if corrupt {
                    Err(app_error!(other "All instances of block {:?} are corrupt", b).into())
                } else {
                    Ok(vec_cons(r, LocatedBlock {
                        o: offset,
                        b: b.into(),
                        t: block_token.into(),
                        locs: {
                            let o: Vec<DatanodeInfo> = locs.into_iter().map(|w| w.into()).collect();
                            o.into()
                        }
                    }))
                }
            } else {
                r
            }
        })
    }
}

impl ProtoFsmSource for Get {
    type NQ = MdxQ;
    type NR = MdxR;
    type UR = Bytes;

    fn handle_n(&mut self, ne: NetEvent<MdxR>) -> SourceAction<MdxQ, Bytes> {
        match ne {
            NetEvent::Init =>
                self.handle_t(GetFsmEvent::Init).recv(true),
            NetEvent::Incoming(MdxR::NN(_, NnaR { inner: NnR::GetBlockLocations(gblrp) })) =>
                match self.translate_block_locations(gblrp) {
                    Ok(bloc) => self.handle_t(GetFsmEvent::BlockLocations(bloc)),
                    Err(e) => SourceAction::z().err(e)
                }
            NetEvent::Incoming(MdxR::DT(_, DtaR::Data(bytes))) =>
                if !bytes.is_empty() {
                    SourceAction::z().deliver(bytes)
                } else {
                    self.handle_t(GetFsmEvent::BlockComplete)
                }
            NetEvent::Incoming(other) =>
                SourceAction::z().err(app_error!(other "Unexpected response: {:?}", other)),
            NetEvent::Idle =>
                SourceAction::z(),
            NetEvent::Err(e) =>
                SourceAction::z().err(e),
            NetEvent::EndOfStream =>
                SourceAction::z().err(app_error!(premature eof))
        }
    }
}

pub type GetStream = source_layer::T<Mdx, Get>;

pub fn get_part_stream(mdx: Mdx, src: String, start_offset: u64, end_offset: u64) -> GetStream {
    source_layer::new(mdx, Get::new(src, start_offset, end_offset))
}

pub fn get_stream(mdx: Mdx, src: String) -> GetStream {
    get_part_stream(mdx, src, 0, u64::max_value())
}

pub type GetAsyncRead = async_io::AsyncReadStream<GetStream>;

pub fn get(mdx: Mdx, src: String) -> GetAsyncRead {
    async_io::AsyncReadStream::new(get_stream(mdx, src))
}



/*

macro_rules! expect_response {
    { $v:expr, $p:pat, $e:expr } => {
        if let $p = $v {
            Ok($e)
        } else {
            Err(app_error!(other "unexpected response, expected `{}`, got `{:?}`", stringify!($p), $v).into())
        }
    };
}




//--------------------------------------------------------------------------------------------------
// Get command
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
    expect_nn_response!(r, NnR::GetBlockLocations(gbl), pb_decons!(GetBlockLocationsResponseProto, gbl, locations))
}

pub fn get_block_locations(rc: &ReactorClient, src: String, offset: u64, length: u64) -> BFI<LocatedBlocksProto> {
    let q = pb_cons!{GetBlockLocationsRequestProto,
        src: src,
        offset: offset,
        length: length
    };
    Box::new(rc.call_nn(nn::NnQ::GetBlockLocations(q)).and_then(move |r|
        if let NnR::GetBlockLocations(gbl) = r {
            let locs = pb_decons!(GetBlockLocationsResponseProto, gbl, locations);
            Ok(locs)
        } else {
            Err(app_error!(other "Unexpected response type").into())
        }
    ))
}


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
    /// file position read successfully so far. This is at a block boundary or at the end.
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
    finished: bool
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
        finished: false
    } }

    //fn with_error(self, e: Error) -> Self { Get { err: vec_cons(self.err, e.into()), ..self } }

    fn idle(mut self) -> (Option<IoResult<GetOperation>>, Self) {
        match self.s {
            (GBLS::Idle, GBS::Idle(w)) =>
                if self.read_offset < self.max_read_offset {
                    //there are remaining blocks to read
                    let remainder = self.max_read_offset - self.read_offset;
                    let (req, s1) = match self.q.pop_front() {
                        Some(mut lb) => {
                            let len = pb_get!(ExtendedBlockProto, lb.b, num_bytes);
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
                    file_length, blocks
                    //, _under_construction, _last_block, _is_last_block_complete, _file_encryption_info
                ) = pb_decons!(LocatedBlocksProto, lbp,
                    file_length, blocks
                    //, under_construction, last_block, is_last_block_complete, file_encryption_info
                    );

                self.max_read_offset = file_length.min(self.max_read_offset);
                for blk in blocks.into_iter() {
                    let (
                        b, offset, locs, corrupt, block_token //, _is_cached, _storage_types, _storage_ids
                    ) = pb_decons!(LocatedBlockProto, blk,
                        b, offset, locs, corrupt, block_token //, is_cached, storage_types, storage_ids
                        );
                    if corrupt { return (
                        Some(Err(app_error!(other "All instances of block {:?} are corrupt", b).into())),
                        Get { s: (GBLS::GetBlockLocations, gbs), ..self }
                    )}
                    //TODO these should be moved rather than cloned. Consider `take_b()` etc.
                    self.q.push_back(LocatedBlock {
                        o: offset,
                        b,
                        t: block_token,
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
        let dt::BlockReadState { w, read_position: _ } = rb.state();

        let s = match self.s {
            (gbls, GBS::GetBlock { b:_, offset:_, len }) => {
                self.read_offset += len;
                self.write_offset += len as usize;
                (gbls, GBS::Idle(w))
            },
            s => return (Some(Err(app_error!(other "Invalid /rbr").into())), Self { s, ..self })
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
    fn c_result(a: (Option<IoResult<GetOperation>>, Self)) -> (Result<Option<GetOperation>>, Self) {
        match a {
            (Some(Ok(op)), sf) => (Ok(Some(op)), sf),
            (Some(Err(e)), sf) => (Err(e.into()), sf),
            (None, sf) => (Ok(None), sf)
        }
    }
}

impl ReactorProtocolFsm for Get {
    type FN = nn::CallW;
    type FD = dt::ReadBlock<File>;

    fn start(self) -> (Result<Option<GetOperation>>, Self) {
        Self::c_result(self.idle())
    }

    fn complete(self, op: GetOperation) -> (Result<Option<GetOperation>>, Self) {
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
                (Err(app_error!(other "Invalid RO result {:?}", o)), self)
        }
    }

    fn error(self, e: Error, op: GetOperation) -> (Result<Option<GetOperation>>, Self) {
        error!("Get: error on: {:?}", op);
        match op {
            //ReactorOperation { key: _, addr: _, p: CProtocolFsm::NN(_ /*nn::CallW::Null*/) } => {
            //    (Err(e), self)
            //}
            ReactorOperation { key: _, addr: _, p: CProtocolFsm::DT(rb) } => {
                let dt::BlockReadState { w, read_position: _ } = rb.state();
                //TODO: switch to next datanode, resume
                // (gbls, GBS::GetBlock { b, offset, len }) =>
                //w.position(self.write_offset)
                //(gbls, GBS::GetBlock { b, offset, len })
                (Err(e), Self { s: (GBLS::Idle, GBS::Idle(w)), finished: true, ..self })
            }
            _ =>
                (Err(e), self)
        }
    }
}

pub fn get(rc: ReactorClient, src: String, dst: File) -> BFTET<(ReactorClient, Get)> {
     rc.run(Get::new(src, dst))
}



//--------------------------------------------------------------------------------------------------
// Put command
//--------------------------------------------------------------------------------------------------
*/