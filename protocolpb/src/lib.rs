extern crate protobuf;
//extern crate grpc;
//extern crate tls_api;

pub mod proto;

/*
#[cfg(test)]
mod tests {
    #[test]
    fn r0() {
        use proto::ClientNamenodeProtocol_grpc::ClientNamenodeProtocol;
        use grpc;
        use proto;
        
        let client = proto::ClientNamenodeProtocol_grpc::ClientNamenodeProtocolClient::new_plain("localhost", 8020, Default::default()).unwrap();
        
        //9964 2018-01-07 00:23 /tmp/cm_api.py
        let mut req/*: ClientNamenodeProtocol::GetBlockLocationsRequestProto*/ = 
            proto::ClientNamenodeProtocol::GetBlockLocationsRequestProto::new();
        req.set_src("/tmp/cm_api.py".to_owned());
        req.set_offset(0);
        req.set_length(9964);

        let resp = client.get_block_locations(grpc::RequestOptions::new(), req);
        
        println!("RESULT: {:?}", resp.wait());
    }
}
*/