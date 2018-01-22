// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct RPCTraceInfoProto {
    // message fields
    traceId: ::std::option::Option<i64>,
    parentId: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RPCTraceInfoProto {}

impl RPCTraceInfoProto {
    pub fn new() -> RPCTraceInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RPCTraceInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<RPCTraceInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RPCTraceInfoProto,
        };
        unsafe {
            instance.get(RPCTraceInfoProto::new)
        }
    }

    // optional int64 traceId = 1;

    pub fn clear_traceId(&mut self) {
        self.traceId = ::std::option::Option::None;
    }

    pub fn has_traceId(&self) -> bool {
        self.traceId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_traceId(&mut self, v: i64) {
        self.traceId = ::std::option::Option::Some(v);
    }

    pub fn get_traceId(&self) -> i64 {
        self.traceId.unwrap_or(0)
    }

    fn get_traceId_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.traceId
    }

    fn mut_traceId_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.traceId
    }

    // optional int64 parentId = 2;

    pub fn clear_parentId(&mut self) {
        self.parentId = ::std::option::Option::None;
    }

    pub fn has_parentId(&self) -> bool {
        self.parentId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parentId(&mut self, v: i64) {
        self.parentId = ::std::option::Option::Some(v);
    }

    pub fn get_parentId(&self) -> i64 {
        self.parentId.unwrap_or(0)
    }

    fn get_parentId_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.parentId
    }

    fn mut_parentId_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.parentId
    }
}

impl ::protobuf::Message for RPCTraceInfoProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.traceId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.parentId = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.traceId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.parentId {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.traceId {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.parentId {
            os.write_int64(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RPCTraceInfoProto {
    fn new() -> RPCTraceInfoProto {
        RPCTraceInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RPCTraceInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "traceId",
                    RPCTraceInfoProto::get_traceId_for_reflect,
                    RPCTraceInfoProto::mut_traceId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "parentId",
                    RPCTraceInfoProto::get_parentId_for_reflect,
                    RPCTraceInfoProto::mut_parentId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RPCTraceInfoProto>(
                    "RPCTraceInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RPCTraceInfoProto {
    fn clear(&mut self) {
        self.clear_traceId();
        self.clear_parentId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RPCTraceInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RPCTraceInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpcRequestHeaderProto {
    // message fields
    rpcKind: ::std::option::Option<RpcKindProto>,
    rpcOp: ::std::option::Option<RpcRequestHeaderProto_OperationProto>,
    callId: ::std::option::Option<i32>,
    clientId: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    retryCount: ::std::option::Option<i32>,
    traceInfo: ::protobuf::SingularPtrField<RPCTraceInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpcRequestHeaderProto {}

impl RpcRequestHeaderProto {
    pub fn new() -> RpcRequestHeaderProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpcRequestHeaderProto {
        static mut instance: ::protobuf::lazy::Lazy<RpcRequestHeaderProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpcRequestHeaderProto,
        };
        unsafe {
            instance.get(RpcRequestHeaderProto::new)
        }
    }

    // optional .hadoop.common.RpcKindProto rpcKind = 1;

    pub fn clear_rpcKind(&mut self) {
        self.rpcKind = ::std::option::Option::None;
    }

    pub fn has_rpcKind(&self) -> bool {
        self.rpcKind.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rpcKind(&mut self, v: RpcKindProto) {
        self.rpcKind = ::std::option::Option::Some(v);
    }

    pub fn get_rpcKind(&self) -> RpcKindProto {
        self.rpcKind.unwrap_or(RpcKindProto::RPC_BUILTIN)
    }

    fn get_rpcKind_for_reflect(&self) -> &::std::option::Option<RpcKindProto> {
        &self.rpcKind
    }

    fn mut_rpcKind_for_reflect(&mut self) -> &mut ::std::option::Option<RpcKindProto> {
        &mut self.rpcKind
    }

    // optional .hadoop.common.RpcRequestHeaderProto.OperationProto rpcOp = 2;

    pub fn clear_rpcOp(&mut self) {
        self.rpcOp = ::std::option::Option::None;
    }

    pub fn has_rpcOp(&self) -> bool {
        self.rpcOp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rpcOp(&mut self, v: RpcRequestHeaderProto_OperationProto) {
        self.rpcOp = ::std::option::Option::Some(v);
    }

    pub fn get_rpcOp(&self) -> RpcRequestHeaderProto_OperationProto {
        self.rpcOp.unwrap_or(RpcRequestHeaderProto_OperationProto::RPC_FINAL_PACKET)
    }

    fn get_rpcOp_for_reflect(&self) -> &::std::option::Option<RpcRequestHeaderProto_OperationProto> {
        &self.rpcOp
    }

    fn mut_rpcOp_for_reflect(&mut self) -> &mut ::std::option::Option<RpcRequestHeaderProto_OperationProto> {
        &mut self.rpcOp
    }

    // required sint32 callId = 3;

    pub fn clear_callId(&mut self) {
        self.callId = ::std::option::Option::None;
    }

    pub fn has_callId(&self) -> bool {
        self.callId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_callId(&mut self, v: i32) {
        self.callId = ::std::option::Option::Some(v);
    }

    pub fn get_callId(&self) -> i32 {
        self.callId.unwrap_or(0)
    }

    fn get_callId_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.callId
    }

    fn mut_callId_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.callId
    }

    // required bytes clientId = 4;

    pub fn clear_clientId(&mut self) {
        self.clientId.clear();
    }

    pub fn has_clientId(&self) -> bool {
        self.clientId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientId(&mut self, v: ::std::vec::Vec<u8>) {
        self.clientId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientId(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.clientId.is_none() {
            self.clientId.set_default();
        }
        self.clientId.as_mut().unwrap()
    }

    // Take field
    pub fn take_clientId(&mut self) -> ::std::vec::Vec<u8> {
        self.clientId.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_clientId(&self) -> &[u8] {
        match self.clientId.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_clientId_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.clientId
    }

    fn mut_clientId_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.clientId
    }

    // optional sint32 retryCount = 5;

    pub fn clear_retryCount(&mut self) {
        self.retryCount = ::std::option::Option::None;
    }

    pub fn has_retryCount(&self) -> bool {
        self.retryCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_retryCount(&mut self, v: i32) {
        self.retryCount = ::std::option::Option::Some(v);
    }

    pub fn get_retryCount(&self) -> i32 {
        self.retryCount.unwrap_or(-1i32)
    }

    fn get_retryCount_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.retryCount
    }

    fn mut_retryCount_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.retryCount
    }

    // optional .hadoop.common.RPCTraceInfoProto traceInfo = 6;

    pub fn clear_traceInfo(&mut self) {
        self.traceInfo.clear();
    }

    pub fn has_traceInfo(&self) -> bool {
        self.traceInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_traceInfo(&mut self, v: RPCTraceInfoProto) {
        self.traceInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_traceInfo(&mut self) -> &mut RPCTraceInfoProto {
        if self.traceInfo.is_none() {
            self.traceInfo.set_default();
        }
        self.traceInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_traceInfo(&mut self) -> RPCTraceInfoProto {
        self.traceInfo.take().unwrap_or_else(|| RPCTraceInfoProto::new())
    }

    pub fn get_traceInfo(&self) -> &RPCTraceInfoProto {
        self.traceInfo.as_ref().unwrap_or_else(|| RPCTraceInfoProto::default_instance())
    }

    fn get_traceInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<RPCTraceInfoProto> {
        &self.traceInfo
    }

    fn mut_traceInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RPCTraceInfoProto> {
        &mut self.traceInfo
    }
}

impl ::protobuf::Message for RpcRequestHeaderProto {
    fn is_initialized(&self) -> bool {
        if self.callId.is_none() {
            return false;
        }
        if self.clientId.is_none() {
            return false;
        }
        for v in &self.traceInfo {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.rpcKind = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.rpcOp = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.callId = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.clientId)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.retryCount = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.traceInfo)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.rpcKind {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.rpcOp {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(v) = self.callId {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, v);
        }
        if let Some(ref v) = self.clientId.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        if let Some(v) = self.retryCount {
            my_size += ::protobuf::rt::value_varint_zigzag_size(5, v);
        }
        if let Some(ref v) = self.traceInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.rpcKind {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.rpcOp {
            os.write_enum(2, v.value())?;
        }
        if let Some(v) = self.callId {
            os.write_sint32(3, v)?;
        }
        if let Some(ref v) = self.clientId.as_ref() {
            os.write_bytes(4, &v)?;
        }
        if let Some(v) = self.retryCount {
            os.write_sint32(5, v)?;
        }
        if let Some(ref v) = self.traceInfo.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpcRequestHeaderProto {
    fn new() -> RpcRequestHeaderProto {
        RpcRequestHeaderProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpcRequestHeaderProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RpcKindProto>>(
                    "rpcKind",
                    RpcRequestHeaderProto::get_rpcKind_for_reflect,
                    RpcRequestHeaderProto::mut_rpcKind_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RpcRequestHeaderProto_OperationProto>>(
                    "rpcOp",
                    RpcRequestHeaderProto::get_rpcOp_for_reflect,
                    RpcRequestHeaderProto::mut_rpcOp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "callId",
                    RpcRequestHeaderProto::get_callId_for_reflect,
                    RpcRequestHeaderProto::mut_callId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "clientId",
                    RpcRequestHeaderProto::get_clientId_for_reflect,
                    RpcRequestHeaderProto::mut_clientId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "retryCount",
                    RpcRequestHeaderProto::get_retryCount_for_reflect,
                    RpcRequestHeaderProto::mut_retryCount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RPCTraceInfoProto>>(
                    "traceInfo",
                    RpcRequestHeaderProto::get_traceInfo_for_reflect,
                    RpcRequestHeaderProto::mut_traceInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpcRequestHeaderProto>(
                    "RpcRequestHeaderProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpcRequestHeaderProto {
    fn clear(&mut self) {
        self.clear_rpcKind();
        self.clear_rpcOp();
        self.clear_callId();
        self.clear_clientId();
        self.clear_retryCount();
        self.clear_traceInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpcRequestHeaderProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpcRequestHeaderProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RpcRequestHeaderProto_OperationProto {
    RPC_FINAL_PACKET = 0,
    RPC_CONTINUATION_PACKET = 1,
    RPC_CLOSE_CONNECTION = 2,
}

impl ::protobuf::ProtobufEnum for RpcRequestHeaderProto_OperationProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RpcRequestHeaderProto_OperationProto> {
        match value {
            0 => ::std::option::Option::Some(RpcRequestHeaderProto_OperationProto::RPC_FINAL_PACKET),
            1 => ::std::option::Option::Some(RpcRequestHeaderProto_OperationProto::RPC_CONTINUATION_PACKET),
            2 => ::std::option::Option::Some(RpcRequestHeaderProto_OperationProto::RPC_CLOSE_CONNECTION),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RpcRequestHeaderProto_OperationProto] = &[
            RpcRequestHeaderProto_OperationProto::RPC_FINAL_PACKET,
            RpcRequestHeaderProto_OperationProto::RPC_CONTINUATION_PACKET,
            RpcRequestHeaderProto_OperationProto::RPC_CLOSE_CONNECTION,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RpcRequestHeaderProto_OperationProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RpcRequestHeaderProto_OperationProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RpcRequestHeaderProto_OperationProto {
}

impl ::protobuf::reflect::ProtobufValue for RpcRequestHeaderProto_OperationProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpcResponseHeaderProto {
    // message fields
    callId: ::std::option::Option<u32>,
    status: ::std::option::Option<RpcResponseHeaderProto_RpcStatusProto>,
    serverIpcVersionNum: ::std::option::Option<u32>,
    exceptionClassName: ::protobuf::SingularField<::std::string::String>,
    errorMsg: ::protobuf::SingularField<::std::string::String>,
    errorDetail: ::std::option::Option<RpcResponseHeaderProto_RpcErrorCodeProto>,
    clientId: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    retryCount: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpcResponseHeaderProto {}

impl RpcResponseHeaderProto {
    pub fn new() -> RpcResponseHeaderProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpcResponseHeaderProto {
        static mut instance: ::protobuf::lazy::Lazy<RpcResponseHeaderProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpcResponseHeaderProto,
        };
        unsafe {
            instance.get(RpcResponseHeaderProto::new)
        }
    }

    // required uint32 callId = 1;

    pub fn clear_callId(&mut self) {
        self.callId = ::std::option::Option::None;
    }

    pub fn has_callId(&self) -> bool {
        self.callId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_callId(&mut self, v: u32) {
        self.callId = ::std::option::Option::Some(v);
    }

    pub fn get_callId(&self) -> u32 {
        self.callId.unwrap_or(0)
    }

    fn get_callId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.callId
    }

    fn mut_callId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.callId
    }

    // required .hadoop.common.RpcResponseHeaderProto.RpcStatusProto status = 2;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: RpcResponseHeaderProto_RpcStatusProto) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> RpcResponseHeaderProto_RpcStatusProto {
        self.status.unwrap_or(RpcResponseHeaderProto_RpcStatusProto::SUCCESS)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<RpcResponseHeaderProto_RpcStatusProto> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<RpcResponseHeaderProto_RpcStatusProto> {
        &mut self.status
    }

    // optional uint32 serverIpcVersionNum = 3;

    pub fn clear_serverIpcVersionNum(&mut self) {
        self.serverIpcVersionNum = ::std::option::Option::None;
    }

    pub fn has_serverIpcVersionNum(&self) -> bool {
        self.serverIpcVersionNum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_serverIpcVersionNum(&mut self, v: u32) {
        self.serverIpcVersionNum = ::std::option::Option::Some(v);
    }

    pub fn get_serverIpcVersionNum(&self) -> u32 {
        self.serverIpcVersionNum.unwrap_or(0)
    }

    fn get_serverIpcVersionNum_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.serverIpcVersionNum
    }

    fn mut_serverIpcVersionNum_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.serverIpcVersionNum
    }

    // optional string exceptionClassName = 4;

    pub fn clear_exceptionClassName(&mut self) {
        self.exceptionClassName.clear();
    }

    pub fn has_exceptionClassName(&self) -> bool {
        self.exceptionClassName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_exceptionClassName(&mut self, v: ::std::string::String) {
        self.exceptionClassName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_exceptionClassName(&mut self) -> &mut ::std::string::String {
        if self.exceptionClassName.is_none() {
            self.exceptionClassName.set_default();
        }
        self.exceptionClassName.as_mut().unwrap()
    }

    // Take field
    pub fn take_exceptionClassName(&mut self) -> ::std::string::String {
        self.exceptionClassName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_exceptionClassName(&self) -> &str {
        match self.exceptionClassName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_exceptionClassName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.exceptionClassName
    }

    fn mut_exceptionClassName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.exceptionClassName
    }

    // optional string errorMsg = 5;

    pub fn clear_errorMsg(&mut self) {
        self.errorMsg.clear();
    }

    pub fn has_errorMsg(&self) -> bool {
        self.errorMsg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_errorMsg(&mut self, v: ::std::string::String) {
        self.errorMsg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_errorMsg(&mut self) -> &mut ::std::string::String {
        if self.errorMsg.is_none() {
            self.errorMsg.set_default();
        }
        self.errorMsg.as_mut().unwrap()
    }

    // Take field
    pub fn take_errorMsg(&mut self) -> ::std::string::String {
        self.errorMsg.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_errorMsg(&self) -> &str {
        match self.errorMsg.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_errorMsg_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.errorMsg
    }

    fn mut_errorMsg_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.errorMsg
    }

    // optional .hadoop.common.RpcResponseHeaderProto.RpcErrorCodeProto errorDetail = 6;

    pub fn clear_errorDetail(&mut self) {
        self.errorDetail = ::std::option::Option::None;
    }

    pub fn has_errorDetail(&self) -> bool {
        self.errorDetail.is_some()
    }

    // Param is passed by value, moved
    pub fn set_errorDetail(&mut self, v: RpcResponseHeaderProto_RpcErrorCodeProto) {
        self.errorDetail = ::std::option::Option::Some(v);
    }

    pub fn get_errorDetail(&self) -> RpcResponseHeaderProto_RpcErrorCodeProto {
        self.errorDetail.unwrap_or(RpcResponseHeaderProto_RpcErrorCodeProto::ERROR_APPLICATION)
    }

    fn get_errorDetail_for_reflect(&self) -> &::std::option::Option<RpcResponseHeaderProto_RpcErrorCodeProto> {
        &self.errorDetail
    }

    fn mut_errorDetail_for_reflect(&mut self) -> &mut ::std::option::Option<RpcResponseHeaderProto_RpcErrorCodeProto> {
        &mut self.errorDetail
    }

    // optional bytes clientId = 7;

    pub fn clear_clientId(&mut self) {
        self.clientId.clear();
    }

    pub fn has_clientId(&self) -> bool {
        self.clientId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientId(&mut self, v: ::std::vec::Vec<u8>) {
        self.clientId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientId(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.clientId.is_none() {
            self.clientId.set_default();
        }
        self.clientId.as_mut().unwrap()
    }

    // Take field
    pub fn take_clientId(&mut self) -> ::std::vec::Vec<u8> {
        self.clientId.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_clientId(&self) -> &[u8] {
        match self.clientId.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_clientId_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.clientId
    }

    fn mut_clientId_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.clientId
    }

    // optional sint32 retryCount = 8;

    pub fn clear_retryCount(&mut self) {
        self.retryCount = ::std::option::Option::None;
    }

    pub fn has_retryCount(&self) -> bool {
        self.retryCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_retryCount(&mut self, v: i32) {
        self.retryCount = ::std::option::Option::Some(v);
    }

    pub fn get_retryCount(&self) -> i32 {
        self.retryCount.unwrap_or(-1i32)
    }

    fn get_retryCount_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.retryCount
    }

    fn mut_retryCount_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.retryCount
    }
}

impl ::protobuf::Message for RpcResponseHeaderProto {
    fn is_initialized(&self) -> bool {
        if self.callId.is_none() {
            return false;
        }
        if self.status.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.callId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.serverIpcVersionNum = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.exceptionClassName)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.errorMsg)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.errorDetail = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.clientId)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.retryCount = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.callId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(v) = self.serverIpcVersionNum {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.exceptionClassName.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.errorMsg.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(v) = self.errorDetail {
            my_size += ::protobuf::rt::enum_size(6, v);
        }
        if let Some(ref v) = self.clientId.as_ref() {
            my_size += ::protobuf::rt::bytes_size(7, &v);
        }
        if let Some(v) = self.retryCount {
            my_size += ::protobuf::rt::value_varint_zigzag_size(8, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.callId {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.status {
            os.write_enum(2, v.value())?;
        }
        if let Some(v) = self.serverIpcVersionNum {
            os.write_uint32(3, v)?;
        }
        if let Some(ref v) = self.exceptionClassName.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.errorMsg.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(v) = self.errorDetail {
            os.write_enum(6, v.value())?;
        }
        if let Some(ref v) = self.clientId.as_ref() {
            os.write_bytes(7, &v)?;
        }
        if let Some(v) = self.retryCount {
            os.write_sint32(8, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpcResponseHeaderProto {
    fn new() -> RpcResponseHeaderProto {
        RpcResponseHeaderProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpcResponseHeaderProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "callId",
                    RpcResponseHeaderProto::get_callId_for_reflect,
                    RpcResponseHeaderProto::mut_callId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RpcResponseHeaderProto_RpcStatusProto>>(
                    "status",
                    RpcResponseHeaderProto::get_status_for_reflect,
                    RpcResponseHeaderProto::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "serverIpcVersionNum",
                    RpcResponseHeaderProto::get_serverIpcVersionNum_for_reflect,
                    RpcResponseHeaderProto::mut_serverIpcVersionNum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "exceptionClassName",
                    RpcResponseHeaderProto::get_exceptionClassName_for_reflect,
                    RpcResponseHeaderProto::mut_exceptionClassName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "errorMsg",
                    RpcResponseHeaderProto::get_errorMsg_for_reflect,
                    RpcResponseHeaderProto::mut_errorMsg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RpcResponseHeaderProto_RpcErrorCodeProto>>(
                    "errorDetail",
                    RpcResponseHeaderProto::get_errorDetail_for_reflect,
                    RpcResponseHeaderProto::mut_errorDetail_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "clientId",
                    RpcResponseHeaderProto::get_clientId_for_reflect,
                    RpcResponseHeaderProto::mut_clientId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "retryCount",
                    RpcResponseHeaderProto::get_retryCount_for_reflect,
                    RpcResponseHeaderProto::mut_retryCount_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpcResponseHeaderProto>(
                    "RpcResponseHeaderProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpcResponseHeaderProto {
    fn clear(&mut self) {
        self.clear_callId();
        self.clear_status();
        self.clear_serverIpcVersionNum();
        self.clear_exceptionClassName();
        self.clear_errorMsg();
        self.clear_errorDetail();
        self.clear_clientId();
        self.clear_retryCount();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpcResponseHeaderProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpcResponseHeaderProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RpcResponseHeaderProto_RpcStatusProto {
    SUCCESS = 0,
    ERROR = 1,
    FATAL = 2,
}

impl ::protobuf::ProtobufEnum for RpcResponseHeaderProto_RpcStatusProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RpcResponseHeaderProto_RpcStatusProto> {
        match value {
            0 => ::std::option::Option::Some(RpcResponseHeaderProto_RpcStatusProto::SUCCESS),
            1 => ::std::option::Option::Some(RpcResponseHeaderProto_RpcStatusProto::ERROR),
            2 => ::std::option::Option::Some(RpcResponseHeaderProto_RpcStatusProto::FATAL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RpcResponseHeaderProto_RpcStatusProto] = &[
            RpcResponseHeaderProto_RpcStatusProto::SUCCESS,
            RpcResponseHeaderProto_RpcStatusProto::ERROR,
            RpcResponseHeaderProto_RpcStatusProto::FATAL,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RpcResponseHeaderProto_RpcStatusProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RpcResponseHeaderProto_RpcStatusProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RpcResponseHeaderProto_RpcStatusProto {
}

impl ::protobuf::reflect::ProtobufValue for RpcResponseHeaderProto_RpcStatusProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RpcResponseHeaderProto_RpcErrorCodeProto {
    ERROR_APPLICATION = 1,
    ERROR_NO_SUCH_METHOD = 2,
    ERROR_NO_SUCH_PROTOCOL = 3,
    ERROR_RPC_SERVER = 4,
    ERROR_SERIALIZING_RESPONSE = 5,
    ERROR_RPC_VERSION_MISMATCH = 6,
    FATAL_UNKNOWN = 10,
    FATAL_UNSUPPORTED_SERIALIZATION = 11,
    FATAL_INVALID_RPC_HEADER = 12,
    FATAL_DESERIALIZING_REQUEST = 13,
    FATAL_VERSION_MISMATCH = 14,
    FATAL_UNAUTHORIZED = 15,
}

impl ::protobuf::ProtobufEnum for RpcResponseHeaderProto_RpcErrorCodeProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RpcResponseHeaderProto_RpcErrorCodeProto> {
        match value {
            1 => ::std::option::Option::Some(RpcResponseHeaderProto_RpcErrorCodeProto::ERROR_APPLICATION),
            2 => ::std::option::Option::Some(RpcResponseHeaderProto_RpcErrorCodeProto::ERROR_NO_SUCH_METHOD),
            3 => ::std::option::Option::Some(RpcResponseHeaderProto_RpcErrorCodeProto::ERROR_NO_SUCH_PROTOCOL),
            4 => ::std::option::Option::Some(RpcResponseHeaderProto_RpcErrorCodeProto::ERROR_RPC_SERVER),
            5 => ::std::option::Option::Some(RpcResponseHeaderProto_RpcErrorCodeProto::ERROR_SERIALIZING_RESPONSE),
            6 => ::std::option::Option::Some(RpcResponseHeaderProto_RpcErrorCodeProto::ERROR_RPC_VERSION_MISMATCH),
            10 => ::std::option::Option::Some(RpcResponseHeaderProto_RpcErrorCodeProto::FATAL_UNKNOWN),
            11 => ::std::option::Option::Some(RpcResponseHeaderProto_RpcErrorCodeProto::FATAL_UNSUPPORTED_SERIALIZATION),
            12 => ::std::option::Option::Some(RpcResponseHeaderProto_RpcErrorCodeProto::FATAL_INVALID_RPC_HEADER),
            13 => ::std::option::Option::Some(RpcResponseHeaderProto_RpcErrorCodeProto::FATAL_DESERIALIZING_REQUEST),
            14 => ::std::option::Option::Some(RpcResponseHeaderProto_RpcErrorCodeProto::FATAL_VERSION_MISMATCH),
            15 => ::std::option::Option::Some(RpcResponseHeaderProto_RpcErrorCodeProto::FATAL_UNAUTHORIZED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RpcResponseHeaderProto_RpcErrorCodeProto] = &[
            RpcResponseHeaderProto_RpcErrorCodeProto::ERROR_APPLICATION,
            RpcResponseHeaderProto_RpcErrorCodeProto::ERROR_NO_SUCH_METHOD,
            RpcResponseHeaderProto_RpcErrorCodeProto::ERROR_NO_SUCH_PROTOCOL,
            RpcResponseHeaderProto_RpcErrorCodeProto::ERROR_RPC_SERVER,
            RpcResponseHeaderProto_RpcErrorCodeProto::ERROR_SERIALIZING_RESPONSE,
            RpcResponseHeaderProto_RpcErrorCodeProto::ERROR_RPC_VERSION_MISMATCH,
            RpcResponseHeaderProto_RpcErrorCodeProto::FATAL_UNKNOWN,
            RpcResponseHeaderProto_RpcErrorCodeProto::FATAL_UNSUPPORTED_SERIALIZATION,
            RpcResponseHeaderProto_RpcErrorCodeProto::FATAL_INVALID_RPC_HEADER,
            RpcResponseHeaderProto_RpcErrorCodeProto::FATAL_DESERIALIZING_REQUEST,
            RpcResponseHeaderProto_RpcErrorCodeProto::FATAL_VERSION_MISMATCH,
            RpcResponseHeaderProto_RpcErrorCodeProto::FATAL_UNAUTHORIZED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RpcResponseHeaderProto_RpcErrorCodeProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RpcResponseHeaderProto_RpcErrorCodeProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RpcResponseHeaderProto_RpcErrorCodeProto {
}

impl ::protobuf::reflect::ProtobufValue for RpcResponseHeaderProto_RpcErrorCodeProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpcSaslProto {
    // message fields
    version: ::std::option::Option<u32>,
    state: ::std::option::Option<RpcSaslProto_SaslState>,
    token: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    auths: ::protobuf::RepeatedField<RpcSaslProto_SaslAuth>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpcSaslProto {}

impl RpcSaslProto {
    pub fn new() -> RpcSaslProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpcSaslProto {
        static mut instance: ::protobuf::lazy::Lazy<RpcSaslProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpcSaslProto,
        };
        unsafe {
            instance.get(RpcSaslProto::new)
        }
    }

    // optional uint32 version = 1;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u32 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.version
    }

    // required .hadoop.common.RpcSaslProto.SaslState state = 2;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: RpcSaslProto_SaslState) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> RpcSaslProto_SaslState {
        self.state.unwrap_or(RpcSaslProto_SaslState::SUCCESS)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<RpcSaslProto_SaslState> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<RpcSaslProto_SaslState> {
        &mut self.state
    }

    // optional bytes token = 3;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::vec::Vec<u8>) {
        self.token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.token.is_none() {
            self.token.set_default();
        }
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::vec::Vec<u8> {
        self.token.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_token(&self) -> &[u8] {
        match self.token.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_token_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.token
    }

    // repeated .hadoop.common.RpcSaslProto.SaslAuth auths = 4;

    pub fn clear_auths(&mut self) {
        self.auths.clear();
    }

    // Param is passed by value, moved
    pub fn set_auths(&mut self, v: ::protobuf::RepeatedField<RpcSaslProto_SaslAuth>) {
        self.auths = v;
    }

    // Mutable pointer to the field.
    pub fn mut_auths(&mut self) -> &mut ::protobuf::RepeatedField<RpcSaslProto_SaslAuth> {
        &mut self.auths
    }

    // Take field
    pub fn take_auths(&mut self) -> ::protobuf::RepeatedField<RpcSaslProto_SaslAuth> {
        ::std::mem::replace(&mut self.auths, ::protobuf::RepeatedField::new())
    }

    pub fn get_auths(&self) -> &[RpcSaslProto_SaslAuth] {
        &self.auths
    }

    fn get_auths_for_reflect(&self) -> &::protobuf::RepeatedField<RpcSaslProto_SaslAuth> {
        &self.auths
    }

    fn mut_auths_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RpcSaslProto_SaslAuth> {
        &mut self.auths
    }
}

impl ::protobuf::Message for RpcSaslProto {
    fn is_initialized(&self) -> bool {
        if self.state.is_none() {
            return false;
        }
        for v in &self.auths {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.state = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.token)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.auths)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(ref v) = self.token.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        for value in &self.auths {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.state {
            os.write_enum(2, v.value())?;
        }
        if let Some(ref v) = self.token.as_ref() {
            os.write_bytes(3, &v)?;
        }
        for v in &self.auths {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpcSaslProto {
    fn new() -> RpcSaslProto {
        RpcSaslProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpcSaslProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version",
                    RpcSaslProto::get_version_for_reflect,
                    RpcSaslProto::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<RpcSaslProto_SaslState>>(
                    "state",
                    RpcSaslProto::get_state_for_reflect,
                    RpcSaslProto::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "token",
                    RpcSaslProto::get_token_for_reflect,
                    RpcSaslProto::mut_token_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RpcSaslProto_SaslAuth>>(
                    "auths",
                    RpcSaslProto::get_auths_for_reflect,
                    RpcSaslProto::mut_auths_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpcSaslProto>(
                    "RpcSaslProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpcSaslProto {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_state();
        self.clear_token();
        self.clear_auths();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpcSaslProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpcSaslProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RpcSaslProto_SaslAuth {
    // message fields
    method: ::protobuf::SingularField<::std::string::String>,
    mechanism: ::protobuf::SingularField<::std::string::String>,
    protocol: ::protobuf::SingularField<::std::string::String>,
    serverId: ::protobuf::SingularField<::std::string::String>,
    challenge: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RpcSaslProto_SaslAuth {}

impl RpcSaslProto_SaslAuth {
    pub fn new() -> RpcSaslProto_SaslAuth {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RpcSaslProto_SaslAuth {
        static mut instance: ::protobuf::lazy::Lazy<RpcSaslProto_SaslAuth> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RpcSaslProto_SaslAuth,
        };
        unsafe {
            instance.get(RpcSaslProto_SaslAuth::new)
        }
    }

    // required string method = 1;

    pub fn clear_method(&mut self) {
        self.method.clear();
    }

    pub fn has_method(&self) -> bool {
        self.method.is_some()
    }

    // Param is passed by value, moved
    pub fn set_method(&mut self, v: ::std::string::String) {
        self.method = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_method(&mut self) -> &mut ::std::string::String {
        if self.method.is_none() {
            self.method.set_default();
        }
        self.method.as_mut().unwrap()
    }

    // Take field
    pub fn take_method(&mut self) -> ::std::string::String {
        self.method.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_method(&self) -> &str {
        match self.method.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_method_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.method
    }

    fn mut_method_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.method
    }

    // required string mechanism = 2;

    pub fn clear_mechanism(&mut self) {
        self.mechanism.clear();
    }

    pub fn has_mechanism(&self) -> bool {
        self.mechanism.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mechanism(&mut self, v: ::std::string::String) {
        self.mechanism = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mechanism(&mut self) -> &mut ::std::string::String {
        if self.mechanism.is_none() {
            self.mechanism.set_default();
        }
        self.mechanism.as_mut().unwrap()
    }

    // Take field
    pub fn take_mechanism(&mut self) -> ::std::string::String {
        self.mechanism.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_mechanism(&self) -> &str {
        match self.mechanism.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_mechanism_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.mechanism
    }

    fn mut_mechanism_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.mechanism
    }

    // optional string protocol = 3;

    pub fn clear_protocol(&mut self) {
        self.protocol.clear();
    }

    pub fn has_protocol(&self) -> bool {
        self.protocol.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol(&mut self, v: ::std::string::String) {
        self.protocol = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_protocol(&mut self) -> &mut ::std::string::String {
        if self.protocol.is_none() {
            self.protocol.set_default();
        }
        self.protocol.as_mut().unwrap()
    }

    // Take field
    pub fn take_protocol(&mut self) -> ::std::string::String {
        self.protocol.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_protocol(&self) -> &str {
        match self.protocol.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_protocol_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.protocol
    }

    fn mut_protocol_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.protocol
    }

    // optional string serverId = 4;

    pub fn clear_serverId(&mut self) {
        self.serverId.clear();
    }

    pub fn has_serverId(&self) -> bool {
        self.serverId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_serverId(&mut self, v: ::std::string::String) {
        self.serverId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_serverId(&mut self) -> &mut ::std::string::String {
        if self.serverId.is_none() {
            self.serverId.set_default();
        }
        self.serverId.as_mut().unwrap()
    }

    // Take field
    pub fn take_serverId(&mut self) -> ::std::string::String {
        self.serverId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_serverId(&self) -> &str {
        match self.serverId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_serverId_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.serverId
    }

    fn mut_serverId_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.serverId
    }

    // optional bytes challenge = 5;

    pub fn clear_challenge(&mut self) {
        self.challenge.clear();
    }

    pub fn has_challenge(&self) -> bool {
        self.challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenge(&mut self, v: ::std::vec::Vec<u8>) {
        self.challenge = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_challenge(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.challenge.is_none() {
            self.challenge.set_default();
        }
        self.challenge.as_mut().unwrap()
    }

    // Take field
    pub fn take_challenge(&mut self) -> ::std::vec::Vec<u8> {
        self.challenge.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_challenge(&self) -> &[u8] {
        match self.challenge.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_challenge_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.challenge
    }

    fn mut_challenge_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.challenge
    }
}

impl ::protobuf::Message for RpcSaslProto_SaslAuth {
    fn is_initialized(&self) -> bool {
        if self.method.is_none() {
            return false;
        }
        if self.mechanism.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.method)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.mechanism)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.protocol)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.serverId)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.challenge)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.method.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.mechanism.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.protocol.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.serverId.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.challenge.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.method.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.mechanism.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.protocol.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.serverId.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.challenge.as_ref() {
            os.write_bytes(5, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RpcSaslProto_SaslAuth {
    fn new() -> RpcSaslProto_SaslAuth {
        RpcSaslProto_SaslAuth::new()
    }

    fn descriptor_static(_: ::std::option::Option<RpcSaslProto_SaslAuth>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "method",
                    RpcSaslProto_SaslAuth::get_method_for_reflect,
                    RpcSaslProto_SaslAuth::mut_method_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "mechanism",
                    RpcSaslProto_SaslAuth::get_mechanism_for_reflect,
                    RpcSaslProto_SaslAuth::mut_mechanism_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "protocol",
                    RpcSaslProto_SaslAuth::get_protocol_for_reflect,
                    RpcSaslProto_SaslAuth::mut_protocol_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "serverId",
                    RpcSaslProto_SaslAuth::get_serverId_for_reflect,
                    RpcSaslProto_SaslAuth::mut_serverId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "challenge",
                    RpcSaslProto_SaslAuth::get_challenge_for_reflect,
                    RpcSaslProto_SaslAuth::mut_challenge_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RpcSaslProto_SaslAuth>(
                    "RpcSaslProto_SaslAuth",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RpcSaslProto_SaslAuth {
    fn clear(&mut self) {
        self.clear_method();
        self.clear_mechanism();
        self.clear_protocol();
        self.clear_serverId();
        self.clear_challenge();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RpcSaslProto_SaslAuth {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RpcSaslProto_SaslAuth {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RpcSaslProto_SaslState {
    SUCCESS = 0,
    NEGOTIATE = 1,
    INITIATE = 2,
    CHALLENGE = 3,
    RESPONSE = 4,
    WRAP = 5,
}

impl ::protobuf::ProtobufEnum for RpcSaslProto_SaslState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RpcSaslProto_SaslState> {
        match value {
            0 => ::std::option::Option::Some(RpcSaslProto_SaslState::SUCCESS),
            1 => ::std::option::Option::Some(RpcSaslProto_SaslState::NEGOTIATE),
            2 => ::std::option::Option::Some(RpcSaslProto_SaslState::INITIATE),
            3 => ::std::option::Option::Some(RpcSaslProto_SaslState::CHALLENGE),
            4 => ::std::option::Option::Some(RpcSaslProto_SaslState::RESPONSE),
            5 => ::std::option::Option::Some(RpcSaslProto_SaslState::WRAP),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RpcSaslProto_SaslState] = &[
            RpcSaslProto_SaslState::SUCCESS,
            RpcSaslProto_SaslState::NEGOTIATE,
            RpcSaslProto_SaslState::INITIATE,
            RpcSaslProto_SaslState::CHALLENGE,
            RpcSaslProto_SaslState::RESPONSE,
            RpcSaslProto_SaslState::WRAP,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RpcSaslProto_SaslState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RpcSaslProto_SaslState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RpcSaslProto_SaslState {
}

impl ::protobuf::reflect::ProtobufValue for RpcSaslProto_SaslState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum RpcKindProto {
    RPC_BUILTIN = 0,
    RPC_WRITABLE = 1,
    RPC_PROTOCOL_BUFFER = 2,
}

impl ::protobuf::ProtobufEnum for RpcKindProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<RpcKindProto> {
        match value {
            0 => ::std::option::Option::Some(RpcKindProto::RPC_BUILTIN),
            1 => ::std::option::Option::Some(RpcKindProto::RPC_WRITABLE),
            2 => ::std::option::Option::Some(RpcKindProto::RPC_PROTOCOL_BUFFER),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [RpcKindProto] = &[
            RpcKindProto::RPC_BUILTIN,
            RpcKindProto::RPC_WRITABLE,
            RpcKindProto::RPC_PROTOCOL_BUFFER,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<RpcKindProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("RpcKindProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for RpcKindProto {
}

impl ::protobuf::reflect::ProtobufValue for RpcKindProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fRpcHeader.proto\x12\rhadoop.common\"6\n\x11RPCTraceInfoProto\x12\
    \x0f\n\x07traceId\x18\x01\x20\x01(\x03\x12\x10\n\x08parentId\x18\x02\x20\
    \x01(\x03\"\xd7\x02\n\x15RpcRequestHeaderProto\x12,\n\x07rpcKind\x18\x01\
    \x20\x01(\x0e2\x1b.hadoop.common.RpcKindProto\x12B\n\x05rpcOp\x18\x02\
    \x20\x01(\x0e23.hadoop.common.RpcRequestHeaderProto.OperationProto\x12\
    \x0e\n\x06callId\x18\x03\x20\x02(\x11\x12\x10\n\x08clientId\x18\x04\x20\
    \x02(\x0c\x12\x16\n\nretryCount\x18\x05\x20\x01(\x11:\x02-1\x123\n\ttrac\
    eInfo\x18\x06\x20\x01(\x0b2\x20.hadoop.common.RPCTraceInfoProto\"]\n\x0e\
    OperationProto\x12\x14\n\x10RPC_FINAL_PACKET\x10\0\x12\x1b\n\x17RPC_CONT\
    INUATION_PACKET\x10\x01\x12\x18\n\x14RPC_CLOSE_CONNECTION\x10\x02\"\xca\
    \x05\n\x16RpcResponseHeaderProto\x12\x0e\n\x06callId\x18\x01\x20\x02(\r\
    \x12D\n\x06status\x18\x02\x20\x02(\x0e24.hadoop.common.RpcResponseHeader\
    Proto.RpcStatusProto\x12\x1b\n\x13serverIpcVersionNum\x18\x03\x20\x01(\r\
    \x12\x1a\n\x12exceptionClassName\x18\x04\x20\x01(\t\x12\x10\n\x08errorMs\
    g\x18\x05\x20\x01(\t\x12L\n\x0berrorDetail\x18\x06\x20\x01(\x0e27.hadoop\
    .common.RpcResponseHeaderProto.RpcErrorCodeProto\x12\x10\n\x08clientId\
    \x18\x07\x20\x01(\x0c\x12\x16\n\nretryCount\x18\x08\x20\x01(\x11:\x02-1\
    \"3\n\x0eRpcStatusProto\x12\x0b\n\x07SUCCESS\x10\0\x12\t\n\x05ERROR\x10\
    \x01\x12\t\n\x05FATAL\x10\x02\"\xe1\x02\n\x11RpcErrorCodeProto\x12\x15\n\
    \x11ERROR_APPLICATION\x10\x01\x12\x18\n\x14ERROR_NO_SUCH_METHOD\x10\x02\
    \x12\x1a\n\x16ERROR_NO_SUCH_PROTOCOL\x10\x03\x12\x14\n\x10ERROR_RPC_SERV\
    ER\x10\x04\x12\x1e\n\x1aERROR_SERIALIZING_RESPONSE\x10\x05\x12\x1e\n\x1a\
    ERROR_RPC_VERSION_MISMATCH\x10\x06\x12\x11\n\rFATAL_UNKNOWN\x10\n\x12#\n\
    \x1fFATAL_UNSUPPORTED_SERIALIZATION\x10\x0b\x12\x1c\n\x18FATAL_INVALID_R\
    PC_HEADER\x10\x0c\x12\x1f\n\x1bFATAL_DESERIALIZING_REQUEST\x10\r\x12\x1a\
    \n\x16FATAL_VERSION_MISMATCH\x10\x0e\x12\x16\n\x12FATAL_UNAUTHORIZED\x10\
    \x0f\"\xdd\x02\n\x0cRpcSaslProto\x12\x0f\n\x07version\x18\x01\x20\x01(\r\
    \x124\n\x05state\x18\x02\x20\x02(\x0e2%.hadoop.common.RpcSaslProto.SaslS\
    tate\x12\r\n\x05token\x18\x03\x20\x01(\x0c\x123\n\x05auths\x18\x04\x20\
    \x03(\x0b2$.hadoop.common.RpcSaslProto.SaslAuth\x1ad\n\x08SaslAuth\x12\
    \x0e\n\x06method\x18\x01\x20\x02(\t\x12\x11\n\tmechanism\x18\x02\x20\x02\
    (\t\x12\x10\n\x08protocol\x18\x03\x20\x01(\t\x12\x10\n\x08serverId\x18\
    \x04\x20\x01(\t\x12\x11\n\tchallenge\x18\x05\x20\x01(\x0c\"\\\n\tSaslSta\
    te\x12\x0b\n\x07SUCCESS\x10\0\x12\r\n\tNEGOTIATE\x10\x01\x12\x0c\n\x08IN\
    ITIATE\x10\x02\x12\r\n\tCHALLENGE\x10\x03\x12\x0c\n\x08RESPONSE\x10\x04\
    \x12\x08\n\x04WRAP\x10\x05*J\n\x0cRpcKindProto\x12\x0f\n\x0bRPC_BUILTIN\
    \x10\0\x12\x10\n\x0cRPC_WRITABLE\x10\x01\x12\x17\n\x13RPC_PROTOCOL_BUFFE\
    R\x10\x02B4\n\x1eorg.apache.hadoop.ipc.protobufB\x0fRpcHeaderProtos\xa0\
    \x01\x01\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
