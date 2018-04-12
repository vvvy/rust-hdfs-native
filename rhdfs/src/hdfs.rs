
use protobuf_api::*;
use cpool::*;
use reactor::*;
use nn::{NnR, NnQ};
use *;



pub type HdfsReactorContext = ReactorContext<
    ConnectionPoolClient<CKey, CAddr, nn::Connection>,
    ConnectionPoolClient<CKey, CAddr, dt::Connection>,
>;

//impl HdfsReactorContext {
//    fn new()
//}



pub trait ListingSink {
    fn files(&mut self, fs: &[HdfsFileStatusProto], src_pos: usize, last_in_src: bool, last: bool);
}

struct GetListingState {
    src: std::collections::LinkedList<String>,
    pos: usize,
    sk: Box<ListingSink>,
    need_location: bool
}

impl GetListingState {
    fn new(sk: Box<ListingSink>, args: config::GetListing) -> GetListingState {
        GetListingState { src: args.src.into_iter().collect(), pos: 0, sk, need_location: args.need_location }
    }
}

impl nn::ProtocolFsm for GetListingState {

    fn start(self) -> (nn::ProtocolFsmResult, Self) {
        let next = if self.src.is_empty() {
            nn::ProtocolFsmResult::Success
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
                nn::ProtocolFsmResult::Err(app_error!(other "unexpected response: expected `NnR::GetListing` but got {:?}", o).into()),
                self
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
        self.sk.files(fs, pos, last_in_src, last);

        let next = if last {
            nn::ProtocolFsmResult::Success
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

pub fn get_listing(cp: &mut SyncConnectionPoolST, args: config::GetListing, sk: Box<ListingSink>) -> Result<Box<ListingSink>> {
    let p = GetListingState::new(sk, args);
    let r = cp.run_nn(p)?;
    Ok(r.sk)
}


pub fn get_block_locations(cp: &mut SyncConnectionPoolST, src: String) -> Result<LocatedBlocksProto> {
    let q = pb_cons!{GetBlockLocationsRequestProto,
        src: src,
        offset: 0,
        length: std::i64::MAX as u64
    };
    let r = cp.call_nn(nn::NnQ::GetBlockLocations(q))?;
    if let NnR::GetBlockLocations(mut gbl) = r {
        let locs = pb_decons!(GetBlockLocationsResponseProto, gbl, locations);
        Ok(locs)
    } else {
        Err(app_error!(other "Unexpected response type")/*.into()*/)
    }
}


pub fn get(cp: &mut SyncConnectionPoolST, args: config::Get) -> Result<()> {
    for f in args.src {
        let r = hdfs::get_block_locations(cp, f)?;
        println!("!!!! {:?}", r);
    }
    Ok(())
}


pub fn read_block() -> Result<()> { unimplemented!() }
