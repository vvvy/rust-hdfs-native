
//use std;
//use std::borrow::Cow;
use std::net::ToSocketAddrs;
use tokio_core::reactor::Core;
use futures::{Future};
use futures::future::ok;

use protobuf_api::*;
use nn::*;
use *;


use protobuf_api::*;

pub trait ListingSink {
    fn files(&mut self, fs: &[HdfsFileStatusProto], src_pos: usize, last_in_src: bool, last: bool);
}

pub struct GetListingState {
    src: std::collections::LinkedList<String>,
    pos: usize,
    sk: Box<ListingSink>,
    need_location: bool
}

impl GetListingState {
    pub fn new(sk: Box<ListingSink>, args: config::GetListing) -> GetListingState {
        GetListingState { src: args.src.into_iter().collect(), pos: 0, sk, need_location: args.need_location }
    }
}

impl ProtocolFsm for GetListingState {

    fn start(self) -> (ProtocolFsmResult, Self) {
        let next = if self.src.is_empty() {
            ProtocolFsmResult::Success
        } else {
            let cur_src =
                self.src.front().map(|v|v.clone()).unwrap_or(String::new());
            ProtocolFsmResult::Send(NnQ::GetListing(
                pb_cons!(GetListingRequestProto,
                    src: cur_src,
                    start_after: vec![],
                    need_location: self.need_location)
            ))
        };
        (next, self)
    }

    fn incoming(mut self, nr: NnR) -> (ProtocolFsmResult, Self) {
        let r = match nr {
            NnR::GetListing(glr) => glr,
            o => return (
                ProtocolFsmResult::Err(app_error!(other "unexpected response: expected `NnR::GetListing` but got {:?}", o).into()),
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
            ProtocolFsmResult::Success
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
            ProtocolFsmResult::Send(NnQ::GetListing(
                pb_cons!(GetListingRequestProto,
                    src: cur_src,
                    start_after: cur_file,
                    need_location: self.need_location)
            ))
        };
        (next, self)
    }
}




//path: String, start_from: String, max_count: u32
/*
pub fn get_listing(sink: Box<ListingSink>, args: config::GetListing, cfg: &config::Common) -> Result<Box<ListingSink>> {

    let f =
        c.and_then(|conn| conn.run(GetListingState::new(sink, args)));

    let (c, s) = core.run(f)?;
    Ok(s.sk)

    Ok(sink)
}
*/







//path: String, start_from: String, max_count: u32
pub fn ___get_listing(sink: Box<ListingSink>, args: config::GetListing, cfg: &config::Common) -> Result<Box<ListingSink>> {

    let mut core = Core::new()?;

    let addr = "127.0.0.1:8020".to_socket_addrs()?.next().ok_or(app_error!(other "NN host not found"))?;

    let mut c = nn::Connection::connect(
        &core.handle(),
        &addr,
        "cloudera".to_owned()
    );

    struct State {
        src: std::collections::LinkedList<String>,
        pos: usize,
        sk: Box<ListingSink>,
        need_location: bool
    }

    impl State {
        fn new(sk: Box<ListingSink>, src: Vec<String>, need_location: bool) -> State {
            State { src: src.into_iter().collect(), pos: 0, sk, need_location }
        }
    }

    impl ProtocolFsm for State {

        fn start(self) -> (ProtocolFsmResult, Self) {
            let next = if self.src.is_empty() {
                ProtocolFsmResult::Success
            } else {
                let cur_src =
                    self.src.front().map(|v|v.clone()).unwrap_or(String::new());
                ProtocolFsmResult::Send(NnQ::GetListing(
                    pb_cons!(GetListingRequestProto,
                    src: cur_src,
                    start_after: vec![],
                    need_location: self.need_location)
                ))
            };
            (next, self)
        }

        fn incoming(mut self, nr: NnR) -> (ProtocolFsmResult, Self) {
            let r = match nr {
                NnR::GetListing(glr) => glr,
                o => return (
                    ProtocolFsmResult::Err(app_error!(other "unexpected response: expected `NnR::GetListing` but got {:?}", o).into()),
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
                ProtocolFsmResult::Success
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
                ProtocolFsmResult::Send(NnQ::GetListing(
                    pb_cons!(GetListingRequestProto,
                    src: cur_src,
                    start_after: cur_file,
                    need_location: self.need_location)
                ))
            };
            (next, self)
        }
    }

    let f =
        c.and_then(|conn| conn.run(State::new(sink, args.src, args.need_location)));

    let (c, s) = core.run(f)?;
    Ok(s.sk)

    /*
    for src in args.src {
        let mut q = pb_cons!(GetListingRequestProto,
        src: src,
        start_after: vec![],
        need_location: false
        );

        let f =
            c.and_then(|conn| conn.call(NnQ::GetListing(q)));

        let (nr, c1) = core.run(f)?;
        c = Box::new(ok(c1));

        let r = match nr {
            NnR::GetListing(glr) => Ok(glr),
            o => Err(app_error!(other "unexpected response: expected `NnR::GetListing` but got {:?}", o))
        }?;

        let x: &[HdfsFileStatusProto] = pb_decons!(DirectoryListingProto,
            pb_decons!(GetListingResponseProto, r, dir_list),
            partial_listing);

        for fs in x {
            let sz = match fs.get_fileType() {
                HdfsFileStatusProto_FileType::IS_DIR => format!("<dir, {} entries>", fs.get_childrenNum()),
                HdfsFileStatusProto_FileType::IS_SYMLINK => format!("->{}", String::from_utf8_lossy(fs.get_symlink())),
                _ => format!("{}", fs.get_length())
            };
            println!("{}\t{}", String::from_utf8_lossy(fs.get_path()), sz);
        }
        trace!("RESULT: {:?}", r);
    }

    Ok(sink)
    */
}

pub fn read_block() -> Result<()> { unimplemented!() }
