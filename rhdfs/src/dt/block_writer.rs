/*
use std::io::Read;
use std::fmt;
use std::fmt::Debug;

use super::codec::{DtReq, DtRsp, OpBlockWriteMessage};
use super::packet::BlockDataPacket;
use super::checksum::*;
use super::*;
use protobuf_api::*;
use *;

/// State of block writer
#[derive(Debug)]
pub struct BlockWriteState<R> {
    pub r: R,
    ///Write position inside the block: the position written so far, not including any unacked data
    pub write_position: i64
}

impl<R> BlockWriteState<R> {
    fn new(r: R, write_position: i64) -> BlockWriteState<R> {
        BlockWriteState { r, write_position }
    }
}

pub struct BlockWriteTracker<R> {
    r: R,
    c: Box<ChecksumValidator>,
    seqno: i64,
    offset: i64
}

impl<R> Debug for BlockWriteTracker<R> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("BlockWriteTracker")
            .field("c", &self.c)
            .field("seqno", &self.seqno)
            .field("offset", &self.offset)
            .finish()
    }
}
/// The return value type for `next_packet`
enum PacketResult {
    /// There is a packet to be sent out
    Packet(BlockDataPacket),
    /// There is no packet to send (right now or forever). Some asks aren't received
    Acking,
    /// The work is done
    Complete,
    /// An error occurred
    Err(Error)

}

impl<R: Read> BlockWriteTracker<R> {
    fn new(BlockWriteState { r, write_position }: BlockWriteState<R>, c: Box<ChecksumValidator>) -> BlockWriteTracker<R> {
        BlockWriteTracker {
            r, c,
            seqno: 0,
            offset: write_position
        }
    }

    fn decons(self) -> BlockWriteState<R> {
        BlockWriteState::new(self.r, self.offset)
    }

    fn next_packet(self) -> (PacketResult, Self) {
        unimplemented!()
    }

    fn ack(self, ack: PipelineAckProto) -> (PacketResult, Self) {
        unimplemented!()
    }
}

fn build_block_write_tracker<R: Read>(csp: ChecksumProto, bws: BlockWriteState<R>) -> BlockWriteTracker<R> {
    BlockWriteTracker::new(bws, new_checksum(csp))
}


#[derive(Debug)]
pub enum WriteBlock<R> {
    Init(OpWriteBlockProto, BlockWriteState<R>),
    ResponseWait(BlockWriteState<R>, ChecksumProto),
    Packet(BlockWriteTracker<R>),
    End(BlockWriteState<R>)
}


impl<R> WriteBlock<R> where
    R: Read + Send + Debug + 'static {
    pub fn new(h: BaseHeaderProto,
               targets: Vec<DatanodeInfoProto>,
               storage_types: Vec<StorageTypeProto>,
               r: R) -> Self {
        let tlen = targets.len() as u32;
        let st = storage_types[0];

        //DFS_BYTES_PER_CHECKSUM_KEY = "dfs.bytes-per-checksum"
        //DFS_BYTES_PER_CHECKSUM_DEFAULT = 512
        //DFS_CHECKSUM_TYPE_KEY = "dfs.checksum.type"
        //DFS_CHECKSUM_TYPE_DEFAULT = "CRC32C"

        let owbp = pb_cons!(OpWriteBlockProto,
            header: pb_cons!(ClientOperationHeaderProto,
                //client_name: client_name.to_owned(),
                base_header: h
            ),
            targets: targets,
            stage: OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_CREATE,
            pipeline_size: tlen,
            min_bytes_rcvd: 0,
            max_bytes_rcvd: 0,
            latest_generation_stamp: 0,
            requested_checksum: pb_cons!(ChecksumProto,
                type: ChecksumTypeProto::CHECKSUM_CRC32C,
                bytes_per_checksum: 512),

            storage_type: st,
            target_storage_types: storage_types

            //caching_strategy,
            //allow_lazy_persist
        );
        WriteBlock::Init(owbp, BlockWriteState::new(r, 0 as i64))
    }
    pub fn state(self) -> BlockWriteState<R> {
        match self {
            WriteBlock::Init(_, bws) |
            WriteBlock::ResponseWait(bws, _) |
            WriteBlock::End(bws) =>
                bws,
            WriteBlock::Packet(bwt) =>
                bwt.decons()
        }
    }

    fn packet_result((r, bwt): (PacketResult, BlockWriteTracker<R>)) -> (ProtocolFsmResult, Self) {
        match r {
            PacketResult::Packet(packet) =>
                pfsm!(send(DtReq::Packet(packet)) waiting / goto WriteBlock::Packet(bwt)),
            PacketResult::Acking =>
                pfsm!(wait / goto WriteBlock::Packet(bwt)),
            PacketResult::Complete =>
                pfsm!(return success / goto WriteBlock::End(bwt.decons())),
            PacketResult::Err(e) =>
                pfsm!(return error(e) / goto WriteBlock::End(bwt.decons())),
        }
    }
}

impl<R> ProtocolFsm for WriteBlock<R> where
    R: Read + Send + Debug + 'static {
    fn idle(self) -> (ProtocolFsmResult, Self) {
        use self::WriteBlock as S;
        match self {
            S::Init(owbp, w) => {
                let csp = pb_get!(OpWriteBlockProto, owbp, requested_checksum).clone();
                pfsm!(send(DtReq::WriteBlock(owbp)) / goto WriteBlock::ResponseWait(w, csp))
            },
            S::Packet(bwt) =>
                S::packet_result(bwt.next_packet()),
            s @ S::ResponseWait(..) |
            s @ S::End(..) =>
                pfsm!(wait / goto s)
            //pfsm!(return error(app_error!(other "Invalid s/e {:?}/idle", s))/ goto s)
        }
    }

    fn incoming(self, rsp: DtRsp) -> (ProtocolFsmResult, Self) {
        use self::WriteBlock as S;
        match (self, rsp) {
            (S::ResponseWait(bws, csp), DtRsp::WriteBlock(OpBlockWriteMessage::Initial(has_data, borp))) =>
                if has_data {
                    S::packet_result(build_block_write_tracker(csp, bws).next_packet())
                } else {
                    let (s, m) = pb_decons!(BlockOpResponseProto, borp, status, message);
                    pfsm!(return error(app_error!(dt s, m)) / goto S::End(bws))
                },
            (S::Packet(bwt), DtRsp::WriteBlock(OpBlockWriteMessage::Ack(pkt))) =>
                Self::packet_result(bwt.ack(pkt)),
            //abnormal conditions
            (S::Init(_, bws), ev) =>
                pfsm!(return error(app_error!(other "Unexpected s/e Init/{:?}", ev)) / goto S::End(bws)),
            (S::Packet(bwt), ev) =>
                pfsm!(return error(app_error!(other "Unexpected s/e Packet/{:?}", ev)) / goto S::End(bwt.decons())),
            (S::ResponseWait(bws, _), ev) =>
                pfsm!(return error(app_error!(other "Unexpected s/e ResponseWait/{:?}", ev)) / goto S::End(bws)),
            (S::End(bws), ev) =>
                pfsm!(return error(app_error!(other "Unexpected s/e End/{:?}", ev)) / goto S::End(bws))
        }
    }
}
*/