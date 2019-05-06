use hdfs_comm::rpc::Protocol;
use hdfs_protos::hadoop::hdfs::datanode::{RegisterDatanodeResponseProto, RegisterDatanodeRequestProto};
use prost::Message;

//use std::sync::{Arc, RwLock};

pub struct DatanodeProtocol {
}

impl DatanodeProtocol {
    pub fn new() -> DatanodeProtocol {
        DatanodeProtocol {
        }
    }

    fn register_datanode(&self, req_buf: &[u8], resp_buf: &mut Vec<u8>) {
        let request = RegisterDatanodeRequestProto
            ::decode_length_delimited(req_buf).unwrap();
        let mut response = RegisterDatanodeResponseProto::default();

        // register datanode
        debug!("registerDatanode({:?})", request);
        // TODO - register datanode

        response.encode_length_delimited(resp_buf).unwrap();
    }
}

impl Protocol for DatanodeProtocol {
    fn process(&self, method: &str, req_buf: &[u8],
            resp_buf: &mut Vec<u8>) {
        match method {
            "registerDatanode" => self.register_datanode(req_buf, resp_buf),
            _ => error!("unimplemented method '{}'", method),
        }
    }
}
