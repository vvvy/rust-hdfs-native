use proto_tools::*;
use super::*;
use *;
use futures::Stream;


#[derive(Debug)]
pub struct GetListingOps {
    source: String,
    need_location: bool,
}

impl GetListingOps {
    fn new(source: String, need_location: bool) -> GetListingOps {
        GetListingOps { source, need_location }
    }

    #[inline]
    fn q(&self, start: Vec<u8>) -> GetListingRequestProto {
        pb_cons!(GetListingRequestProto,
                    src: self.source.clone(),
                    start_after: start,
                    need_location: self.need_location
                    )
    }

    fn s(&self) -> GetListingRequestProto {
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


#[derive(Debug)]
pub struct GetListingState {
    ops: GetListingOps,
    s: Option<GetListingRequestProto>
}

impl GetListingState {
    fn new(source: String, need_location: bool) -> GetListingState {
        let ops = GetListingOps::new(source, need_location);
        let s = Some(ops.s());
        GetListingState { ops, s }
    }

}

impl StreamProtocolFsm for GetListingState {
    type R = (Vec<HdfsFileStatusProto>, bool);
    type LQ = nn::NnQ;
    type LR = nn::NnR;

    fn get_downstream(&mut self) -> Result<SendReq<nn::NnQ>> {
        match self.s.take() {
            Some(glrp) => Ok(SendReq::EnqueueAndFlush(nn::NnQ::GetListing(glrp))),
            None => Ok(SendReq::NOP)
        }
    }

    fn handle_upstream(&mut self, lr: Option<nn::NnR>) -> Result<Async<Option<(Vec<HdfsFileStatusProto>, bool)>>> {
        match lr {
            Some(nn::NnR::GetListing(glrp)) => {
                let (r, s) = self.ops.n(glrp);
                self.s = s;
                Ok(Async::Ready(Some((r, self.s.is_some()))))
            }
            None =>
                Err(app_error!(other "GetListingState: Premature EOS")),
            other =>
                Err(app_error!(other "GetListingState: invalid response {:?}", other))
        }
    }
}

pub struct GetListing {
    s: StreamProtocol<NNChannel, GetListingState>,
    eos: bool
}

impl GetListing {
    pub fn new(c: NNChannel, source: String, need_location: bool) -> GetListing {
        GetListing {
            s: StreamProtocol::new(c, GetListingState::new(source, need_location)),
            eos: false
        }
    }
    pub fn into_inner(self) -> NNChannel {
        self.s.into_parts().0
    }
}

impl Stream for GetListing {
    type Item = Vec<HdfsFileStatusProto>;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Option<Vec<HdfsFileStatusProto>>>> {
        if self.eos {
            Ok(Async::Ready(None))
        } else {
            match self.s.poll()? {
                Async::Ready(Some((v, more))) => {
                    self.eos = !more;
                    Ok(Async::Ready(Some(v)))
                }
                Async::Ready(None) =>
                    Err(app_error!(other "GetListing: premature EOS")),
                Async::NotReady =>
                    Ok(Async::NotReady)
            }
        }
    }
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

    let nncd = NNCD {
        addr,
        eff_user: "cloudera".to_owned(),
        client_id: Some(vec![1, 2, 3, 4, 4, 3, 2, 1])
    };
    let c = NNChannel::new(nncd, Duration::new(30, 0));

    let gls = get_listing(c, "/".to_owned(), false);

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
