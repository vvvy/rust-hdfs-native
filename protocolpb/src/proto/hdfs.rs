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
pub struct ExtendedBlockProto {
    // message fields
    poolId: ::protobuf::SingularField<::std::string::String>,
    blockId: ::std::option::Option<u64>,
    generationStamp: ::std::option::Option<u64>,
    numBytes: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExtendedBlockProto {}

impl ExtendedBlockProto {
    pub fn new() -> ExtendedBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExtendedBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<ExtendedBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExtendedBlockProto,
        };
        unsafe {
            instance.get(ExtendedBlockProto::new)
        }
    }

    // required string poolId = 1;

    pub fn clear_poolId(&mut self) {
        self.poolId.clear();
    }

    pub fn has_poolId(&self) -> bool {
        self.poolId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_poolId(&mut self, v: ::std::string::String) {
        self.poolId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_poolId(&mut self) -> &mut ::std::string::String {
        if self.poolId.is_none() {
            self.poolId.set_default();
        }
        self.poolId.as_mut().unwrap()
    }

    // Take field
    pub fn take_poolId(&mut self) -> ::std::string::String {
        self.poolId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_poolId(&self) -> &str {
        match self.poolId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_poolId_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.poolId
    }

    fn mut_poolId_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.poolId
    }

    // required uint64 blockId = 2;

    pub fn clear_blockId(&mut self) {
        self.blockId = ::std::option::Option::None;
    }

    pub fn has_blockId(&self) -> bool {
        self.blockId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockId(&mut self, v: u64) {
        self.blockId = ::std::option::Option::Some(v);
    }

    pub fn get_blockId(&self) -> u64 {
        self.blockId.unwrap_or(0)
    }

    fn get_blockId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.blockId
    }

    fn mut_blockId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.blockId
    }

    // required uint64 generationStamp = 3;

    pub fn clear_generationStamp(&mut self) {
        self.generationStamp = ::std::option::Option::None;
    }

    pub fn has_generationStamp(&self) -> bool {
        self.generationStamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_generationStamp(&mut self, v: u64) {
        self.generationStamp = ::std::option::Option::Some(v);
    }

    pub fn get_generationStamp(&self) -> u64 {
        self.generationStamp.unwrap_or(0)
    }

    fn get_generationStamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.generationStamp
    }

    fn mut_generationStamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.generationStamp
    }

    // optional uint64 numBytes = 4;

    pub fn clear_numBytes(&mut self) {
        self.numBytes = ::std::option::Option::None;
    }

    pub fn has_numBytes(&self) -> bool {
        self.numBytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numBytes(&mut self, v: u64) {
        self.numBytes = ::std::option::Option::Some(v);
    }

    pub fn get_numBytes(&self) -> u64 {
        self.numBytes.unwrap_or(0u64)
    }

    fn get_numBytes_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.numBytes
    }

    fn mut_numBytes_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.numBytes
    }
}

impl ::protobuf::Message for ExtendedBlockProto {
    fn is_initialized(&self) -> bool {
        if self.poolId.is_none() {
            return false;
        }
        if self.blockId.is_none() {
            return false;
        }
        if self.generationStamp.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.poolId)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.blockId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.generationStamp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.numBytes = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.poolId.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.blockId {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.generationStamp {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numBytes {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.poolId.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.blockId {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.generationStamp {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.numBytes {
            os.write_uint64(4, v)?;
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

impl ::protobuf::MessageStatic for ExtendedBlockProto {
    fn new() -> ExtendedBlockProto {
        ExtendedBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExtendedBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "poolId",
                    ExtendedBlockProto::get_poolId_for_reflect,
                    ExtendedBlockProto::mut_poolId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "blockId",
                    ExtendedBlockProto::get_blockId_for_reflect,
                    ExtendedBlockProto::mut_blockId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "generationStamp",
                    ExtendedBlockProto::get_generationStamp_for_reflect,
                    ExtendedBlockProto::mut_generationStamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "numBytes",
                    ExtendedBlockProto::get_numBytes_for_reflect,
                    ExtendedBlockProto::mut_numBytes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExtendedBlockProto>(
                    "ExtendedBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExtendedBlockProto {
    fn clear(&mut self) {
        self.clear_poolId();
        self.clear_blockId();
        self.clear_generationStamp();
        self.clear_numBytes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExtendedBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExtendedBlockProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DatanodeIDProto {
    // message fields
    ipAddr: ::protobuf::SingularField<::std::string::String>,
    hostName: ::protobuf::SingularField<::std::string::String>,
    datanodeUuid: ::protobuf::SingularField<::std::string::String>,
    xferPort: ::std::option::Option<u32>,
    infoPort: ::std::option::Option<u32>,
    ipcPort: ::std::option::Option<u32>,
    infoSecurePort: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DatanodeIDProto {}

impl DatanodeIDProto {
    pub fn new() -> DatanodeIDProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DatanodeIDProto {
        static mut instance: ::protobuf::lazy::Lazy<DatanodeIDProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DatanodeIDProto,
        };
        unsafe {
            instance.get(DatanodeIDProto::new)
        }
    }

    // required string ipAddr = 1;

    pub fn clear_ipAddr(&mut self) {
        self.ipAddr.clear();
    }

    pub fn has_ipAddr(&self) -> bool {
        self.ipAddr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ipAddr(&mut self, v: ::std::string::String) {
        self.ipAddr = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ipAddr(&mut self) -> &mut ::std::string::String {
        if self.ipAddr.is_none() {
            self.ipAddr.set_default();
        }
        self.ipAddr.as_mut().unwrap()
    }

    // Take field
    pub fn take_ipAddr(&mut self) -> ::std::string::String {
        self.ipAddr.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ipAddr(&self) -> &str {
        match self.ipAddr.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ipAddr_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ipAddr
    }

    fn mut_ipAddr_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ipAddr
    }

    // required string hostName = 2;

    pub fn clear_hostName(&mut self) {
        self.hostName.clear();
    }

    pub fn has_hostName(&self) -> bool {
        self.hostName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostName(&mut self, v: ::std::string::String) {
        self.hostName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostName(&mut self) -> &mut ::std::string::String {
        if self.hostName.is_none() {
            self.hostName.set_default();
        }
        self.hostName.as_mut().unwrap()
    }

    // Take field
    pub fn take_hostName(&mut self) -> ::std::string::String {
        self.hostName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hostName(&self) -> &str {
        match self.hostName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_hostName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.hostName
    }

    fn mut_hostName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.hostName
    }

    // required string datanodeUuid = 3;

    pub fn clear_datanodeUuid(&mut self) {
        self.datanodeUuid.clear();
    }

    pub fn has_datanodeUuid(&self) -> bool {
        self.datanodeUuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_datanodeUuid(&mut self, v: ::std::string::String) {
        self.datanodeUuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_datanodeUuid(&mut self) -> &mut ::std::string::String {
        if self.datanodeUuid.is_none() {
            self.datanodeUuid.set_default();
        }
        self.datanodeUuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_datanodeUuid(&mut self) -> ::std::string::String {
        self.datanodeUuid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_datanodeUuid(&self) -> &str {
        match self.datanodeUuid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_datanodeUuid_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.datanodeUuid
    }

    fn mut_datanodeUuid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.datanodeUuid
    }

    // required uint32 xferPort = 4;

    pub fn clear_xferPort(&mut self) {
        self.xferPort = ::std::option::Option::None;
    }

    pub fn has_xferPort(&self) -> bool {
        self.xferPort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xferPort(&mut self, v: u32) {
        self.xferPort = ::std::option::Option::Some(v);
    }

    pub fn get_xferPort(&self) -> u32 {
        self.xferPort.unwrap_or(0)
    }

    fn get_xferPort_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.xferPort
    }

    fn mut_xferPort_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.xferPort
    }

    // required uint32 infoPort = 5;

    pub fn clear_infoPort(&mut self) {
        self.infoPort = ::std::option::Option::None;
    }

    pub fn has_infoPort(&self) -> bool {
        self.infoPort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_infoPort(&mut self, v: u32) {
        self.infoPort = ::std::option::Option::Some(v);
    }

    pub fn get_infoPort(&self) -> u32 {
        self.infoPort.unwrap_or(0)
    }

    fn get_infoPort_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.infoPort
    }

    fn mut_infoPort_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.infoPort
    }

    // required uint32 ipcPort = 6;

    pub fn clear_ipcPort(&mut self) {
        self.ipcPort = ::std::option::Option::None;
    }

    pub fn has_ipcPort(&self) -> bool {
        self.ipcPort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ipcPort(&mut self, v: u32) {
        self.ipcPort = ::std::option::Option::Some(v);
    }

    pub fn get_ipcPort(&self) -> u32 {
        self.ipcPort.unwrap_or(0)
    }

    fn get_ipcPort_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ipcPort
    }

    fn mut_ipcPort_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ipcPort
    }

    // optional uint32 infoSecurePort = 7;

    pub fn clear_infoSecurePort(&mut self) {
        self.infoSecurePort = ::std::option::Option::None;
    }

    pub fn has_infoSecurePort(&self) -> bool {
        self.infoSecurePort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_infoSecurePort(&mut self, v: u32) {
        self.infoSecurePort = ::std::option::Option::Some(v);
    }

    pub fn get_infoSecurePort(&self) -> u32 {
        self.infoSecurePort.unwrap_or(0u32)
    }

    fn get_infoSecurePort_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.infoSecurePort
    }

    fn mut_infoSecurePort_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.infoSecurePort
    }
}

impl ::protobuf::Message for DatanodeIDProto {
    fn is_initialized(&self) -> bool {
        if self.ipAddr.is_none() {
            return false;
        }
        if self.hostName.is_none() {
            return false;
        }
        if self.datanodeUuid.is_none() {
            return false;
        }
        if self.xferPort.is_none() {
            return false;
        }
        if self.infoPort.is_none() {
            return false;
        }
        if self.ipcPort.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ipAddr)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hostName)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.datanodeUuid)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.xferPort = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.infoPort = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ipcPort = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.infoSecurePort = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.ipAddr.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.hostName.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.datanodeUuid.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.xferPort {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.infoPort {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ipcPort {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.infoSecurePort {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.ipAddr.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.hostName.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.datanodeUuid.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.xferPort {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.infoPort {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.ipcPort {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.infoSecurePort {
            os.write_uint32(7, v)?;
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

impl ::protobuf::MessageStatic for DatanodeIDProto {
    fn new() -> DatanodeIDProto {
        DatanodeIDProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DatanodeIDProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ipAddr",
                    DatanodeIDProto::get_ipAddr_for_reflect,
                    DatanodeIDProto::mut_ipAddr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hostName",
                    DatanodeIDProto::get_hostName_for_reflect,
                    DatanodeIDProto::mut_hostName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "datanodeUuid",
                    DatanodeIDProto::get_datanodeUuid_for_reflect,
                    DatanodeIDProto::mut_datanodeUuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "xferPort",
                    DatanodeIDProto::get_xferPort_for_reflect,
                    DatanodeIDProto::mut_xferPort_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "infoPort",
                    DatanodeIDProto::get_infoPort_for_reflect,
                    DatanodeIDProto::mut_infoPort_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ipcPort",
                    DatanodeIDProto::get_ipcPort_for_reflect,
                    DatanodeIDProto::mut_ipcPort_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "infoSecurePort",
                    DatanodeIDProto::get_infoSecurePort_for_reflect,
                    DatanodeIDProto::mut_infoSecurePort_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DatanodeIDProto>(
                    "DatanodeIDProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DatanodeIDProto {
    fn clear(&mut self) {
        self.clear_ipAddr();
        self.clear_hostName();
        self.clear_datanodeUuid();
        self.clear_xferPort();
        self.clear_infoPort();
        self.clear_ipcPort();
        self.clear_infoSecurePort();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DatanodeIDProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DatanodeIDProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DatanodeLocalInfoProto {
    // message fields
    softwareVersion: ::protobuf::SingularField<::std::string::String>,
    configVersion: ::protobuf::SingularField<::std::string::String>,
    uptime: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DatanodeLocalInfoProto {}

impl DatanodeLocalInfoProto {
    pub fn new() -> DatanodeLocalInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DatanodeLocalInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<DatanodeLocalInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DatanodeLocalInfoProto,
        };
        unsafe {
            instance.get(DatanodeLocalInfoProto::new)
        }
    }

    // required string softwareVersion = 1;

    pub fn clear_softwareVersion(&mut self) {
        self.softwareVersion.clear();
    }

    pub fn has_softwareVersion(&self) -> bool {
        self.softwareVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_softwareVersion(&mut self, v: ::std::string::String) {
        self.softwareVersion = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_softwareVersion(&mut self) -> &mut ::std::string::String {
        if self.softwareVersion.is_none() {
            self.softwareVersion.set_default();
        }
        self.softwareVersion.as_mut().unwrap()
    }

    // Take field
    pub fn take_softwareVersion(&mut self) -> ::std::string::String {
        self.softwareVersion.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_softwareVersion(&self) -> &str {
        match self.softwareVersion.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_softwareVersion_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.softwareVersion
    }

    fn mut_softwareVersion_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.softwareVersion
    }

    // required string configVersion = 2;

    pub fn clear_configVersion(&mut self) {
        self.configVersion.clear();
    }

    pub fn has_configVersion(&self) -> bool {
        self.configVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_configVersion(&mut self, v: ::std::string::String) {
        self.configVersion = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_configVersion(&mut self) -> &mut ::std::string::String {
        if self.configVersion.is_none() {
            self.configVersion.set_default();
        }
        self.configVersion.as_mut().unwrap()
    }

    // Take field
    pub fn take_configVersion(&mut self) -> ::std::string::String {
        self.configVersion.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_configVersion(&self) -> &str {
        match self.configVersion.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_configVersion_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.configVersion
    }

    fn mut_configVersion_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.configVersion
    }

    // required uint64 uptime = 3;

    pub fn clear_uptime(&mut self) {
        self.uptime = ::std::option::Option::None;
    }

    pub fn has_uptime(&self) -> bool {
        self.uptime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uptime(&mut self, v: u64) {
        self.uptime = ::std::option::Option::Some(v);
    }

    pub fn get_uptime(&self) -> u64 {
        self.uptime.unwrap_or(0)
    }

    fn get_uptime_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.uptime
    }

    fn mut_uptime_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.uptime
    }
}

impl ::protobuf::Message for DatanodeLocalInfoProto {
    fn is_initialized(&self) -> bool {
        if self.softwareVersion.is_none() {
            return false;
        }
        if self.configVersion.is_none() {
            return false;
        }
        if self.uptime.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.softwareVersion)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.configVersion)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.uptime = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.softwareVersion.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.configVersion.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.uptime {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.softwareVersion.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.configVersion.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.uptime {
            os.write_uint64(3, v)?;
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

impl ::protobuf::MessageStatic for DatanodeLocalInfoProto {
    fn new() -> DatanodeLocalInfoProto {
        DatanodeLocalInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DatanodeLocalInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "softwareVersion",
                    DatanodeLocalInfoProto::get_softwareVersion_for_reflect,
                    DatanodeLocalInfoProto::mut_softwareVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "configVersion",
                    DatanodeLocalInfoProto::get_configVersion_for_reflect,
                    DatanodeLocalInfoProto::mut_configVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "uptime",
                    DatanodeLocalInfoProto::get_uptime_for_reflect,
                    DatanodeLocalInfoProto::mut_uptime_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DatanodeLocalInfoProto>(
                    "DatanodeLocalInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DatanodeLocalInfoProto {
    fn clear(&mut self) {
        self.clear_softwareVersion();
        self.clear_configVersion();
        self.clear_uptime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DatanodeLocalInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DatanodeLocalInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DatanodeInfosProto {
    // message fields
    datanodes: ::protobuf::RepeatedField<DatanodeInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DatanodeInfosProto {}

impl DatanodeInfosProto {
    pub fn new() -> DatanodeInfosProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DatanodeInfosProto {
        static mut instance: ::protobuf::lazy::Lazy<DatanodeInfosProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DatanodeInfosProto,
        };
        unsafe {
            instance.get(DatanodeInfosProto::new)
        }
    }

    // repeated .hadoop.hdfs.DatanodeInfoProto datanodes = 1;

    pub fn clear_datanodes(&mut self) {
        self.datanodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_datanodes(&mut self, v: ::protobuf::RepeatedField<DatanodeInfoProto>) {
        self.datanodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_datanodes(&mut self) -> &mut ::protobuf::RepeatedField<DatanodeInfoProto> {
        &mut self.datanodes
    }

    // Take field
    pub fn take_datanodes(&mut self) -> ::protobuf::RepeatedField<DatanodeInfoProto> {
        ::std::mem::replace(&mut self.datanodes, ::protobuf::RepeatedField::new())
    }

    pub fn get_datanodes(&self) -> &[DatanodeInfoProto] {
        &self.datanodes
    }

    fn get_datanodes_for_reflect(&self) -> &::protobuf::RepeatedField<DatanodeInfoProto> {
        &self.datanodes
    }

    fn mut_datanodes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DatanodeInfoProto> {
        &mut self.datanodes
    }
}

impl ::protobuf::Message for DatanodeInfosProto {
    fn is_initialized(&self) -> bool {
        for v in &self.datanodes {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.datanodes)?;
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
        for value in &self.datanodes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.datanodes {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for DatanodeInfosProto {
    fn new() -> DatanodeInfosProto {
        DatanodeInfosProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DatanodeInfosProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DatanodeInfoProto>>(
                    "datanodes",
                    DatanodeInfosProto::get_datanodes_for_reflect,
                    DatanodeInfosProto::mut_datanodes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DatanodeInfosProto>(
                    "DatanodeInfosProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DatanodeInfosProto {
    fn clear(&mut self) {
        self.clear_datanodes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DatanodeInfosProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DatanodeInfosProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DatanodeInfoProto {
    // message fields
    id: ::protobuf::SingularPtrField<DatanodeIDProto>,
    capacity: ::std::option::Option<u64>,
    dfsUsed: ::std::option::Option<u64>,
    remaining: ::std::option::Option<u64>,
    blockPoolUsed: ::std::option::Option<u64>,
    lastUpdate: ::std::option::Option<u64>,
    xceiverCount: ::std::option::Option<u32>,
    location: ::protobuf::SingularField<::std::string::String>,
    adminState: ::std::option::Option<DatanodeInfoProto_AdminState>,
    cacheCapacity: ::std::option::Option<u64>,
    cacheUsed: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DatanodeInfoProto {}

impl DatanodeInfoProto {
    pub fn new() -> DatanodeInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DatanodeInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<DatanodeInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DatanodeInfoProto,
        };
        unsafe {
            instance.get(DatanodeInfoProto::new)
        }
    }

    // required .hadoop.hdfs.DatanodeIDProto id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: DatanodeIDProto) {
        self.id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut DatanodeIDProto {
        if self.id.is_none() {
            self.id.set_default();
        }
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> DatanodeIDProto {
        self.id.take().unwrap_or_else(|| DatanodeIDProto::new())
    }

    pub fn get_id(&self) -> &DatanodeIDProto {
        self.id.as_ref().unwrap_or_else(|| DatanodeIDProto::default_instance())
    }

    fn get_id_for_reflect(&self) -> &::protobuf::SingularPtrField<DatanodeIDProto> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DatanodeIDProto> {
        &mut self.id
    }

    // optional uint64 capacity = 2;

    pub fn clear_capacity(&mut self) {
        self.capacity = ::std::option::Option::None;
    }

    pub fn has_capacity(&self) -> bool {
        self.capacity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_capacity(&mut self, v: u64) {
        self.capacity = ::std::option::Option::Some(v);
    }

    pub fn get_capacity(&self) -> u64 {
        self.capacity.unwrap_or(0u64)
    }

    fn get_capacity_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.capacity
    }

    fn mut_capacity_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.capacity
    }

    // optional uint64 dfsUsed = 3;

    pub fn clear_dfsUsed(&mut self) {
        self.dfsUsed = ::std::option::Option::None;
    }

    pub fn has_dfsUsed(&self) -> bool {
        self.dfsUsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dfsUsed(&mut self, v: u64) {
        self.dfsUsed = ::std::option::Option::Some(v);
    }

    pub fn get_dfsUsed(&self) -> u64 {
        self.dfsUsed.unwrap_or(0u64)
    }

    fn get_dfsUsed_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.dfsUsed
    }

    fn mut_dfsUsed_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.dfsUsed
    }

    // optional uint64 remaining = 4;

    pub fn clear_remaining(&mut self) {
        self.remaining = ::std::option::Option::None;
    }

    pub fn has_remaining(&self) -> bool {
        self.remaining.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remaining(&mut self, v: u64) {
        self.remaining = ::std::option::Option::Some(v);
    }

    pub fn get_remaining(&self) -> u64 {
        self.remaining.unwrap_or(0u64)
    }

    fn get_remaining_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.remaining
    }

    fn mut_remaining_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.remaining
    }

    // optional uint64 blockPoolUsed = 5;

    pub fn clear_blockPoolUsed(&mut self) {
        self.blockPoolUsed = ::std::option::Option::None;
    }

    pub fn has_blockPoolUsed(&self) -> bool {
        self.blockPoolUsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPoolUsed(&mut self, v: u64) {
        self.blockPoolUsed = ::std::option::Option::Some(v);
    }

    pub fn get_blockPoolUsed(&self) -> u64 {
        self.blockPoolUsed.unwrap_or(0u64)
    }

    fn get_blockPoolUsed_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.blockPoolUsed
    }

    fn mut_blockPoolUsed_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.blockPoolUsed
    }

    // optional uint64 lastUpdate = 6;

    pub fn clear_lastUpdate(&mut self) {
        self.lastUpdate = ::std::option::Option::None;
    }

    pub fn has_lastUpdate(&self) -> bool {
        self.lastUpdate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastUpdate(&mut self, v: u64) {
        self.lastUpdate = ::std::option::Option::Some(v);
    }

    pub fn get_lastUpdate(&self) -> u64 {
        self.lastUpdate.unwrap_or(0u64)
    }

    fn get_lastUpdate_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lastUpdate
    }

    fn mut_lastUpdate_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lastUpdate
    }

    // optional uint32 xceiverCount = 7;

    pub fn clear_xceiverCount(&mut self) {
        self.xceiverCount = ::std::option::Option::None;
    }

    pub fn has_xceiverCount(&self) -> bool {
        self.xceiverCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xceiverCount(&mut self, v: u32) {
        self.xceiverCount = ::std::option::Option::Some(v);
    }

    pub fn get_xceiverCount(&self) -> u32 {
        self.xceiverCount.unwrap_or(0u32)
    }

    fn get_xceiverCount_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.xceiverCount
    }

    fn mut_xceiverCount_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.xceiverCount
    }

    // optional string location = 8;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: ::std::string::String) {
        self.location = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut ::std::string::String {
        if self.location.is_none() {
            self.location.set_default();
        }
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> ::std::string::String {
        self.location.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_location(&self) -> &str {
        match self.location.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_location_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.location
    }

    // optional .hadoop.hdfs.DatanodeInfoProto.AdminState adminState = 10;

    pub fn clear_adminState(&mut self) {
        self.adminState = ::std::option::Option::None;
    }

    pub fn has_adminState(&self) -> bool {
        self.adminState.is_some()
    }

    // Param is passed by value, moved
    pub fn set_adminState(&mut self, v: DatanodeInfoProto_AdminState) {
        self.adminState = ::std::option::Option::Some(v);
    }

    pub fn get_adminState(&self) -> DatanodeInfoProto_AdminState {
        self.adminState.unwrap_or(DatanodeInfoProto_AdminState::NORMAL)
    }

    fn get_adminState_for_reflect(&self) -> &::std::option::Option<DatanodeInfoProto_AdminState> {
        &self.adminState
    }

    fn mut_adminState_for_reflect(&mut self) -> &mut ::std::option::Option<DatanodeInfoProto_AdminState> {
        &mut self.adminState
    }

    // optional uint64 cacheCapacity = 11;

    pub fn clear_cacheCapacity(&mut self) {
        self.cacheCapacity = ::std::option::Option::None;
    }

    pub fn has_cacheCapacity(&self) -> bool {
        self.cacheCapacity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cacheCapacity(&mut self, v: u64) {
        self.cacheCapacity = ::std::option::Option::Some(v);
    }

    pub fn get_cacheCapacity(&self) -> u64 {
        self.cacheCapacity.unwrap_or(0u64)
    }

    fn get_cacheCapacity_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.cacheCapacity
    }

    fn mut_cacheCapacity_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.cacheCapacity
    }

    // optional uint64 cacheUsed = 12;

    pub fn clear_cacheUsed(&mut self) {
        self.cacheUsed = ::std::option::Option::None;
    }

    pub fn has_cacheUsed(&self) -> bool {
        self.cacheUsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cacheUsed(&mut self, v: u64) {
        self.cacheUsed = ::std::option::Option::Some(v);
    }

    pub fn get_cacheUsed(&self) -> u64 {
        self.cacheUsed.unwrap_or(0u64)
    }

    fn get_cacheUsed_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.cacheUsed
    }

    fn mut_cacheUsed_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.cacheUsed
    }
}

impl ::protobuf::Message for DatanodeInfoProto {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        }
        for v in &self.id {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.capacity = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.dfsUsed = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.remaining = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.blockPoolUsed = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.lastUpdate = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.xceiverCount = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.location)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.adminState = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.cacheCapacity = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.cacheUsed = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.capacity {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.dfsUsed {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.remaining {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.blockPoolUsed {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.lastUpdate {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.xceiverCount {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.location.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        if let Some(v) = self.adminState {
            my_size += ::protobuf::rt::enum_size(10, v);
        }
        if let Some(v) = self.cacheCapacity {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.cacheUsed {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.id.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.capacity {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.dfsUsed {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.remaining {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.blockPoolUsed {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.lastUpdate {
            os.write_uint64(6, v)?;
        }
        if let Some(v) = self.xceiverCount {
            os.write_uint32(7, v)?;
        }
        if let Some(ref v) = self.location.as_ref() {
            os.write_string(8, &v)?;
        }
        if let Some(v) = self.adminState {
            os.write_enum(10, v.value())?;
        }
        if let Some(v) = self.cacheCapacity {
            os.write_uint64(11, v)?;
        }
        if let Some(v) = self.cacheUsed {
            os.write_uint64(12, v)?;
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

impl ::protobuf::MessageStatic for DatanodeInfoProto {
    fn new() -> DatanodeInfoProto {
        DatanodeInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DatanodeInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DatanodeIDProto>>(
                    "id",
                    DatanodeInfoProto::get_id_for_reflect,
                    DatanodeInfoProto::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "capacity",
                    DatanodeInfoProto::get_capacity_for_reflect,
                    DatanodeInfoProto::mut_capacity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "dfsUsed",
                    DatanodeInfoProto::get_dfsUsed_for_reflect,
                    DatanodeInfoProto::mut_dfsUsed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "remaining",
                    DatanodeInfoProto::get_remaining_for_reflect,
                    DatanodeInfoProto::mut_remaining_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "blockPoolUsed",
                    DatanodeInfoProto::get_blockPoolUsed_for_reflect,
                    DatanodeInfoProto::mut_blockPoolUsed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lastUpdate",
                    DatanodeInfoProto::get_lastUpdate_for_reflect,
                    DatanodeInfoProto::mut_lastUpdate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "xceiverCount",
                    DatanodeInfoProto::get_xceiverCount_for_reflect,
                    DatanodeInfoProto::mut_xceiverCount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "location",
                    DatanodeInfoProto::get_location_for_reflect,
                    DatanodeInfoProto::mut_location_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DatanodeInfoProto_AdminState>>(
                    "adminState",
                    DatanodeInfoProto::get_adminState_for_reflect,
                    DatanodeInfoProto::mut_adminState_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "cacheCapacity",
                    DatanodeInfoProto::get_cacheCapacity_for_reflect,
                    DatanodeInfoProto::mut_cacheCapacity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "cacheUsed",
                    DatanodeInfoProto::get_cacheUsed_for_reflect,
                    DatanodeInfoProto::mut_cacheUsed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DatanodeInfoProto>(
                    "DatanodeInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DatanodeInfoProto {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_capacity();
        self.clear_dfsUsed();
        self.clear_remaining();
        self.clear_blockPoolUsed();
        self.clear_lastUpdate();
        self.clear_xceiverCount();
        self.clear_location();
        self.clear_adminState();
        self.clear_cacheCapacity();
        self.clear_cacheUsed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DatanodeInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DatanodeInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DatanodeInfoProto_AdminState {
    NORMAL = 0,
    DECOMMISSION_INPROGRESS = 1,
    DECOMMISSIONED = 2,
}

impl ::protobuf::ProtobufEnum for DatanodeInfoProto_AdminState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DatanodeInfoProto_AdminState> {
        match value {
            0 => ::std::option::Option::Some(DatanodeInfoProto_AdminState::NORMAL),
            1 => ::std::option::Option::Some(DatanodeInfoProto_AdminState::DECOMMISSION_INPROGRESS),
            2 => ::std::option::Option::Some(DatanodeInfoProto_AdminState::DECOMMISSIONED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DatanodeInfoProto_AdminState] = &[
            DatanodeInfoProto_AdminState::NORMAL,
            DatanodeInfoProto_AdminState::DECOMMISSION_INPROGRESS,
            DatanodeInfoProto_AdminState::DECOMMISSIONED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DatanodeInfoProto_AdminState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DatanodeInfoProto_AdminState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DatanodeInfoProto_AdminState {
}

impl ::protobuf::reflect::ProtobufValue for DatanodeInfoProto_AdminState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DatanodeStorageProto {
    // message fields
    storageUuid: ::protobuf::SingularField<::std::string::String>,
    state: ::std::option::Option<DatanodeStorageProto_StorageState>,
    storageType: ::std::option::Option<StorageTypeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DatanodeStorageProto {}

impl DatanodeStorageProto {
    pub fn new() -> DatanodeStorageProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DatanodeStorageProto {
        static mut instance: ::protobuf::lazy::Lazy<DatanodeStorageProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DatanodeStorageProto,
        };
        unsafe {
            instance.get(DatanodeStorageProto::new)
        }
    }

    // required string storageUuid = 1;

    pub fn clear_storageUuid(&mut self) {
        self.storageUuid.clear();
    }

    pub fn has_storageUuid(&self) -> bool {
        self.storageUuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageUuid(&mut self, v: ::std::string::String) {
        self.storageUuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storageUuid(&mut self) -> &mut ::std::string::String {
        if self.storageUuid.is_none() {
            self.storageUuid.set_default();
        }
        self.storageUuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_storageUuid(&mut self) -> ::std::string::String {
        self.storageUuid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_storageUuid(&self) -> &str {
        match self.storageUuid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_storageUuid_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.storageUuid
    }

    fn mut_storageUuid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.storageUuid
    }

    // optional .hadoop.hdfs.DatanodeStorageProto.StorageState state = 2;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: DatanodeStorageProto_StorageState) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> DatanodeStorageProto_StorageState {
        self.state.unwrap_or(DatanodeStorageProto_StorageState::NORMAL)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<DatanodeStorageProto_StorageState> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<DatanodeStorageProto_StorageState> {
        &mut self.state
    }

    // optional .hadoop.hdfs.StorageTypeProto storageType = 3;

    pub fn clear_storageType(&mut self) {
        self.storageType = ::std::option::Option::None;
    }

    pub fn has_storageType(&self) -> bool {
        self.storageType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageType(&mut self, v: StorageTypeProto) {
        self.storageType = ::std::option::Option::Some(v);
    }

    pub fn get_storageType(&self) -> StorageTypeProto {
        self.storageType.unwrap_or(StorageTypeProto::DISK)
    }

    fn get_storageType_for_reflect(&self) -> &::std::option::Option<StorageTypeProto> {
        &self.storageType
    }

    fn mut_storageType_for_reflect(&mut self) -> &mut ::std::option::Option<StorageTypeProto> {
        &mut self.storageType
    }
}

impl ::protobuf::Message for DatanodeStorageProto {
    fn is_initialized(&self) -> bool {
        if self.storageUuid.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.storageUuid)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.state = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.storageType = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.storageUuid.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(v) = self.storageType {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.storageUuid.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.state {
            os.write_enum(2, v.value())?;
        }
        if let Some(v) = self.storageType {
            os.write_enum(3, v.value())?;
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

impl ::protobuf::MessageStatic for DatanodeStorageProto {
    fn new() -> DatanodeStorageProto {
        DatanodeStorageProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DatanodeStorageProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "storageUuid",
                    DatanodeStorageProto::get_storageUuid_for_reflect,
                    DatanodeStorageProto::mut_storageUuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DatanodeStorageProto_StorageState>>(
                    "state",
                    DatanodeStorageProto::get_state_for_reflect,
                    DatanodeStorageProto::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<StorageTypeProto>>(
                    "storageType",
                    DatanodeStorageProto::get_storageType_for_reflect,
                    DatanodeStorageProto::mut_storageType_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DatanodeStorageProto>(
                    "DatanodeStorageProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DatanodeStorageProto {
    fn clear(&mut self) {
        self.clear_storageUuid();
        self.clear_state();
        self.clear_storageType();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DatanodeStorageProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DatanodeStorageProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DatanodeStorageProto_StorageState {
    NORMAL = 0,
    READ_ONLY_SHARED = 1,
}

impl ::protobuf::ProtobufEnum for DatanodeStorageProto_StorageState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DatanodeStorageProto_StorageState> {
        match value {
            0 => ::std::option::Option::Some(DatanodeStorageProto_StorageState::NORMAL),
            1 => ::std::option::Option::Some(DatanodeStorageProto_StorageState::READ_ONLY_SHARED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DatanodeStorageProto_StorageState] = &[
            DatanodeStorageProto_StorageState::NORMAL,
            DatanodeStorageProto_StorageState::READ_ONLY_SHARED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DatanodeStorageProto_StorageState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DatanodeStorageProto_StorageState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DatanodeStorageProto_StorageState {
}

impl ::protobuf::reflect::ProtobufValue for DatanodeStorageProto_StorageState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StorageReportProto {
    // message fields
    storageUuid: ::protobuf::SingularField<::std::string::String>,
    failed: ::std::option::Option<bool>,
    capacity: ::std::option::Option<u64>,
    dfsUsed: ::std::option::Option<u64>,
    remaining: ::std::option::Option<u64>,
    blockPoolUsed: ::std::option::Option<u64>,
    storage: ::protobuf::SingularPtrField<DatanodeStorageProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StorageReportProto {}

impl StorageReportProto {
    pub fn new() -> StorageReportProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StorageReportProto {
        static mut instance: ::protobuf::lazy::Lazy<StorageReportProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StorageReportProto,
        };
        unsafe {
            instance.get(StorageReportProto::new)
        }
    }

    // required string storageUuid = 1;

    pub fn clear_storageUuid(&mut self) {
        self.storageUuid.clear();
    }

    pub fn has_storageUuid(&self) -> bool {
        self.storageUuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageUuid(&mut self, v: ::std::string::String) {
        self.storageUuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storageUuid(&mut self) -> &mut ::std::string::String {
        if self.storageUuid.is_none() {
            self.storageUuid.set_default();
        }
        self.storageUuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_storageUuid(&mut self) -> ::std::string::String {
        self.storageUuid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_storageUuid(&self) -> &str {
        match self.storageUuid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_storageUuid_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.storageUuid
    }

    fn mut_storageUuid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.storageUuid
    }

    // optional bool failed = 2;

    pub fn clear_failed(&mut self) {
        self.failed = ::std::option::Option::None;
    }

    pub fn has_failed(&self) -> bool {
        self.failed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_failed(&mut self, v: bool) {
        self.failed = ::std::option::Option::Some(v);
    }

    pub fn get_failed(&self) -> bool {
        self.failed.unwrap_or(false)
    }

    fn get_failed_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.failed
    }

    fn mut_failed_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.failed
    }

    // optional uint64 capacity = 3;

    pub fn clear_capacity(&mut self) {
        self.capacity = ::std::option::Option::None;
    }

    pub fn has_capacity(&self) -> bool {
        self.capacity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_capacity(&mut self, v: u64) {
        self.capacity = ::std::option::Option::Some(v);
    }

    pub fn get_capacity(&self) -> u64 {
        self.capacity.unwrap_or(0u64)
    }

    fn get_capacity_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.capacity
    }

    fn mut_capacity_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.capacity
    }

    // optional uint64 dfsUsed = 4;

    pub fn clear_dfsUsed(&mut self) {
        self.dfsUsed = ::std::option::Option::None;
    }

    pub fn has_dfsUsed(&self) -> bool {
        self.dfsUsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dfsUsed(&mut self, v: u64) {
        self.dfsUsed = ::std::option::Option::Some(v);
    }

    pub fn get_dfsUsed(&self) -> u64 {
        self.dfsUsed.unwrap_or(0u64)
    }

    fn get_dfsUsed_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.dfsUsed
    }

    fn mut_dfsUsed_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.dfsUsed
    }

    // optional uint64 remaining = 5;

    pub fn clear_remaining(&mut self) {
        self.remaining = ::std::option::Option::None;
    }

    pub fn has_remaining(&self) -> bool {
        self.remaining.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remaining(&mut self, v: u64) {
        self.remaining = ::std::option::Option::Some(v);
    }

    pub fn get_remaining(&self) -> u64 {
        self.remaining.unwrap_or(0u64)
    }

    fn get_remaining_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.remaining
    }

    fn mut_remaining_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.remaining
    }

    // optional uint64 blockPoolUsed = 6;

    pub fn clear_blockPoolUsed(&mut self) {
        self.blockPoolUsed = ::std::option::Option::None;
    }

    pub fn has_blockPoolUsed(&self) -> bool {
        self.blockPoolUsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPoolUsed(&mut self, v: u64) {
        self.blockPoolUsed = ::std::option::Option::Some(v);
    }

    pub fn get_blockPoolUsed(&self) -> u64 {
        self.blockPoolUsed.unwrap_or(0u64)
    }

    fn get_blockPoolUsed_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.blockPoolUsed
    }

    fn mut_blockPoolUsed_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.blockPoolUsed
    }

    // optional .hadoop.hdfs.DatanodeStorageProto storage = 7;

    pub fn clear_storage(&mut self) {
        self.storage.clear();
    }

    pub fn has_storage(&self) -> bool {
        self.storage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storage(&mut self, v: DatanodeStorageProto) {
        self.storage = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storage(&mut self) -> &mut DatanodeStorageProto {
        if self.storage.is_none() {
            self.storage.set_default();
        }
        self.storage.as_mut().unwrap()
    }

    // Take field
    pub fn take_storage(&mut self) -> DatanodeStorageProto {
        self.storage.take().unwrap_or_else(|| DatanodeStorageProto::new())
    }

    pub fn get_storage(&self) -> &DatanodeStorageProto {
        self.storage.as_ref().unwrap_or_else(|| DatanodeStorageProto::default_instance())
    }

    fn get_storage_for_reflect(&self) -> &::protobuf::SingularPtrField<DatanodeStorageProto> {
        &self.storage
    }

    fn mut_storage_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DatanodeStorageProto> {
        &mut self.storage
    }
}

impl ::protobuf::Message for StorageReportProto {
    fn is_initialized(&self) -> bool {
        if self.storageUuid.is_none() {
            return false;
        }
        for v in &self.storage {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.storageUuid)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.failed = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.capacity = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.dfsUsed = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.remaining = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.blockPoolUsed = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.storage)?;
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
        if let Some(ref v) = self.storageUuid.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.failed {
            my_size += 2;
        }
        if let Some(v) = self.capacity {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.dfsUsed {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.remaining {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.blockPoolUsed {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.storage.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.storageUuid.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.failed {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.capacity {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.dfsUsed {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.remaining {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.blockPoolUsed {
            os.write_uint64(6, v)?;
        }
        if let Some(ref v) = self.storage.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for StorageReportProto {
    fn new() -> StorageReportProto {
        StorageReportProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StorageReportProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "storageUuid",
                    StorageReportProto::get_storageUuid_for_reflect,
                    StorageReportProto::mut_storageUuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "failed",
                    StorageReportProto::get_failed_for_reflect,
                    StorageReportProto::mut_failed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "capacity",
                    StorageReportProto::get_capacity_for_reflect,
                    StorageReportProto::mut_capacity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "dfsUsed",
                    StorageReportProto::get_dfsUsed_for_reflect,
                    StorageReportProto::mut_dfsUsed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "remaining",
                    StorageReportProto::get_remaining_for_reflect,
                    StorageReportProto::mut_remaining_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "blockPoolUsed",
                    StorageReportProto::get_blockPoolUsed_for_reflect,
                    StorageReportProto::mut_blockPoolUsed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DatanodeStorageProto>>(
                    "storage",
                    StorageReportProto::get_storage_for_reflect,
                    StorageReportProto::mut_storage_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StorageReportProto>(
                    "StorageReportProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StorageReportProto {
    fn clear(&mut self) {
        self.clear_storageUuid();
        self.clear_failed();
        self.clear_capacity();
        self.clear_dfsUsed();
        self.clear_remaining();
        self.clear_blockPoolUsed();
        self.clear_storage();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StorageReportProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StorageReportProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ContentSummaryProto {
    // message fields
    length: ::std::option::Option<u64>,
    fileCount: ::std::option::Option<u64>,
    directoryCount: ::std::option::Option<u64>,
    quota: ::std::option::Option<u64>,
    spaceConsumed: ::std::option::Option<u64>,
    spaceQuota: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ContentSummaryProto {}

impl ContentSummaryProto {
    pub fn new() -> ContentSummaryProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ContentSummaryProto {
        static mut instance: ::protobuf::lazy::Lazy<ContentSummaryProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ContentSummaryProto,
        };
        unsafe {
            instance.get(ContentSummaryProto::new)
        }
    }

    // required uint64 length = 1;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u64) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length(&self) -> u64 {
        self.length.unwrap_or(0)
    }

    fn get_length_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.length
    }

    // required uint64 fileCount = 2;

    pub fn clear_fileCount(&mut self) {
        self.fileCount = ::std::option::Option::None;
    }

    pub fn has_fileCount(&self) -> bool {
        self.fileCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileCount(&mut self, v: u64) {
        self.fileCount = ::std::option::Option::Some(v);
    }

    pub fn get_fileCount(&self) -> u64 {
        self.fileCount.unwrap_or(0)
    }

    fn get_fileCount_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.fileCount
    }

    fn mut_fileCount_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.fileCount
    }

    // required uint64 directoryCount = 3;

    pub fn clear_directoryCount(&mut self) {
        self.directoryCount = ::std::option::Option::None;
    }

    pub fn has_directoryCount(&self) -> bool {
        self.directoryCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_directoryCount(&mut self, v: u64) {
        self.directoryCount = ::std::option::Option::Some(v);
    }

    pub fn get_directoryCount(&self) -> u64 {
        self.directoryCount.unwrap_or(0)
    }

    fn get_directoryCount_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.directoryCount
    }

    fn mut_directoryCount_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.directoryCount
    }

    // required uint64 quota = 4;

    pub fn clear_quota(&mut self) {
        self.quota = ::std::option::Option::None;
    }

    pub fn has_quota(&self) -> bool {
        self.quota.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quota(&mut self, v: u64) {
        self.quota = ::std::option::Option::Some(v);
    }

    pub fn get_quota(&self) -> u64 {
        self.quota.unwrap_or(0)
    }

    fn get_quota_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.quota
    }

    fn mut_quota_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.quota
    }

    // required uint64 spaceConsumed = 5;

    pub fn clear_spaceConsumed(&mut self) {
        self.spaceConsumed = ::std::option::Option::None;
    }

    pub fn has_spaceConsumed(&self) -> bool {
        self.spaceConsumed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spaceConsumed(&mut self, v: u64) {
        self.spaceConsumed = ::std::option::Option::Some(v);
    }

    pub fn get_spaceConsumed(&self) -> u64 {
        self.spaceConsumed.unwrap_or(0)
    }

    fn get_spaceConsumed_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.spaceConsumed
    }

    fn mut_spaceConsumed_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.spaceConsumed
    }

    // required uint64 spaceQuota = 6;

    pub fn clear_spaceQuota(&mut self) {
        self.spaceQuota = ::std::option::Option::None;
    }

    pub fn has_spaceQuota(&self) -> bool {
        self.spaceQuota.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spaceQuota(&mut self, v: u64) {
        self.spaceQuota = ::std::option::Option::Some(v);
    }

    pub fn get_spaceQuota(&self) -> u64 {
        self.spaceQuota.unwrap_or(0)
    }

    fn get_spaceQuota_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.spaceQuota
    }

    fn mut_spaceQuota_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.spaceQuota
    }
}

impl ::protobuf::Message for ContentSummaryProto {
    fn is_initialized(&self) -> bool {
        if self.length.is_none() {
            return false;
        }
        if self.fileCount.is_none() {
            return false;
        }
        if self.directoryCount.is_none() {
            return false;
        }
        if self.quota.is_none() {
            return false;
        }
        if self.spaceConsumed.is_none() {
            return false;
        }
        if self.spaceQuota.is_none() {
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
                    let tmp = is.read_uint64()?;
                    self.length = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.fileCount = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.directoryCount = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.quota = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.spaceConsumed = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.spaceQuota = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.fileCount {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.directoryCount {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.quota {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.spaceConsumed {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.spaceQuota {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.length {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.fileCount {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.directoryCount {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.quota {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.spaceConsumed {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.spaceQuota {
            os.write_uint64(6, v)?;
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

impl ::protobuf::MessageStatic for ContentSummaryProto {
    fn new() -> ContentSummaryProto {
        ContentSummaryProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ContentSummaryProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "length",
                    ContentSummaryProto::get_length_for_reflect,
                    ContentSummaryProto::mut_length_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "fileCount",
                    ContentSummaryProto::get_fileCount_for_reflect,
                    ContentSummaryProto::mut_fileCount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "directoryCount",
                    ContentSummaryProto::get_directoryCount_for_reflect,
                    ContentSummaryProto::mut_directoryCount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "quota",
                    ContentSummaryProto::get_quota_for_reflect,
                    ContentSummaryProto::mut_quota_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "spaceConsumed",
                    ContentSummaryProto::get_spaceConsumed_for_reflect,
                    ContentSummaryProto::mut_spaceConsumed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "spaceQuota",
                    ContentSummaryProto::get_spaceQuota_for_reflect,
                    ContentSummaryProto::mut_spaceQuota_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ContentSummaryProto>(
                    "ContentSummaryProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ContentSummaryProto {
    fn clear(&mut self) {
        self.clear_length();
        self.clear_fileCount();
        self.clear_directoryCount();
        self.clear_quota();
        self.clear_spaceConsumed();
        self.clear_spaceQuota();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ContentSummaryProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ContentSummaryProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CorruptFileBlocksProto {
    // message fields
    files: ::protobuf::RepeatedField<::std::string::String>,
    cookie: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CorruptFileBlocksProto {}

impl CorruptFileBlocksProto {
    pub fn new() -> CorruptFileBlocksProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CorruptFileBlocksProto {
        static mut instance: ::protobuf::lazy::Lazy<CorruptFileBlocksProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CorruptFileBlocksProto,
        };
        unsafe {
            instance.get(CorruptFileBlocksProto::new)
        }
    }

    // repeated string files = 1;

    pub fn clear_files(&mut self) {
        self.files.clear();
    }

    // Param is passed by value, moved
    pub fn set_files(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.files = v;
    }

    // Mutable pointer to the field.
    pub fn mut_files(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.files
    }

    // Take field
    pub fn take_files(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.files, ::protobuf::RepeatedField::new())
    }

    pub fn get_files(&self) -> &[::std::string::String] {
        &self.files
    }

    fn get_files_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.files
    }

    fn mut_files_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.files
    }

    // required string cookie = 2;

    pub fn clear_cookie(&mut self) {
        self.cookie.clear();
    }

    pub fn has_cookie(&self) -> bool {
        self.cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cookie(&mut self, v: ::std::string::String) {
        self.cookie = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cookie(&mut self) -> &mut ::std::string::String {
        if self.cookie.is_none() {
            self.cookie.set_default();
        }
        self.cookie.as_mut().unwrap()
    }

    // Take field
    pub fn take_cookie(&mut self) -> ::std::string::String {
        self.cookie.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cookie(&self) -> &str {
        match self.cookie.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_cookie_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.cookie
    }

    fn mut_cookie_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.cookie
    }
}

impl ::protobuf::Message for CorruptFileBlocksProto {
    fn is_initialized(&self) -> bool {
        if self.cookie.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.files)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.cookie)?;
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
        for value in &self.files {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if let Some(ref v) = self.cookie.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.files {
            os.write_string(1, &v)?;
        };
        if let Some(ref v) = self.cookie.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for CorruptFileBlocksProto {
    fn new() -> CorruptFileBlocksProto {
        CorruptFileBlocksProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CorruptFileBlocksProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "files",
                    CorruptFileBlocksProto::get_files_for_reflect,
                    CorruptFileBlocksProto::mut_files_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cookie",
                    CorruptFileBlocksProto::get_cookie_for_reflect,
                    CorruptFileBlocksProto::mut_cookie_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CorruptFileBlocksProto>(
                    "CorruptFileBlocksProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CorruptFileBlocksProto {
    fn clear(&mut self) {
        self.clear_files();
        self.clear_cookie();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CorruptFileBlocksProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CorruptFileBlocksProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FsPermissionProto {
    // message fields
    perm: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FsPermissionProto {}

impl FsPermissionProto {
    pub fn new() -> FsPermissionProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FsPermissionProto {
        static mut instance: ::protobuf::lazy::Lazy<FsPermissionProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FsPermissionProto,
        };
        unsafe {
            instance.get(FsPermissionProto::new)
        }
    }

    // required uint32 perm = 1;

    pub fn clear_perm(&mut self) {
        self.perm = ::std::option::Option::None;
    }

    pub fn has_perm(&self) -> bool {
        self.perm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_perm(&mut self, v: u32) {
        self.perm = ::std::option::Option::Some(v);
    }

    pub fn get_perm(&self) -> u32 {
        self.perm.unwrap_or(0)
    }

    fn get_perm_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.perm
    }

    fn mut_perm_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.perm
    }
}

impl ::protobuf::Message for FsPermissionProto {
    fn is_initialized(&self) -> bool {
        if self.perm.is_none() {
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
                    self.perm = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.perm {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.perm {
            os.write_uint32(1, v)?;
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

impl ::protobuf::MessageStatic for FsPermissionProto {
    fn new() -> FsPermissionProto {
        FsPermissionProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FsPermissionProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "perm",
                    FsPermissionProto::get_perm_for_reflect,
                    FsPermissionProto::mut_perm_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FsPermissionProto>(
                    "FsPermissionProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FsPermissionProto {
    fn clear(&mut self) {
        self.clear_perm();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FsPermissionProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FsPermissionProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StorageTypesProto {
    // message fields
    storageTypes: ::std::vec::Vec<StorageTypeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StorageTypesProto {}

impl StorageTypesProto {
    pub fn new() -> StorageTypesProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StorageTypesProto {
        static mut instance: ::protobuf::lazy::Lazy<StorageTypesProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StorageTypesProto,
        };
        unsafe {
            instance.get(StorageTypesProto::new)
        }
    }

    // repeated .hadoop.hdfs.StorageTypeProto storageTypes = 1;

    pub fn clear_storageTypes(&mut self) {
        self.storageTypes.clear();
    }

    // Param is passed by value, moved
    pub fn set_storageTypes(&mut self, v: ::std::vec::Vec<StorageTypeProto>) {
        self.storageTypes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_storageTypes(&mut self) -> &mut ::std::vec::Vec<StorageTypeProto> {
        &mut self.storageTypes
    }

    // Take field
    pub fn take_storageTypes(&mut self) -> ::std::vec::Vec<StorageTypeProto> {
        ::std::mem::replace(&mut self.storageTypes, ::std::vec::Vec::new())
    }

    pub fn get_storageTypes(&self) -> &[StorageTypeProto] {
        &self.storageTypes
    }

    fn get_storageTypes_for_reflect(&self) -> &::std::vec::Vec<StorageTypeProto> {
        &self.storageTypes
    }

    fn mut_storageTypes_for_reflect(&mut self) -> &mut ::std::vec::Vec<StorageTypeProto> {
        &mut self.storageTypes
    }
}

impl ::protobuf::Message for StorageTypesProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.storageTypes)?;
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
        for value in &self.storageTypes {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.storageTypes {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for StorageTypesProto {
    fn new() -> StorageTypesProto {
        StorageTypesProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StorageTypesProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<StorageTypeProto>>(
                    "storageTypes",
                    StorageTypesProto::get_storageTypes_for_reflect,
                    StorageTypesProto::mut_storageTypes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StorageTypesProto>(
                    "StorageTypesProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StorageTypesProto {
    fn clear(&mut self) {
        self.clear_storageTypes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StorageTypesProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StorageTypesProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockStoragePolicyProto {
    // message fields
    policyId: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    creationPolicy: ::protobuf::SingularPtrField<StorageTypesProto>,
    creationFallbackPolicy: ::protobuf::SingularPtrField<StorageTypesProto>,
    replicationFallbackPolicy: ::protobuf::SingularPtrField<StorageTypesProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockStoragePolicyProto {}

impl BlockStoragePolicyProto {
    pub fn new() -> BlockStoragePolicyProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockStoragePolicyProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockStoragePolicyProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockStoragePolicyProto,
        };
        unsafe {
            instance.get(BlockStoragePolicyProto::new)
        }
    }

    // required uint32 policyId = 1;

    pub fn clear_policyId(&mut self) {
        self.policyId = ::std::option::Option::None;
    }

    pub fn has_policyId(&self) -> bool {
        self.policyId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_policyId(&mut self, v: u32) {
        self.policyId = ::std::option::Option::Some(v);
    }

    pub fn get_policyId(&self) -> u32 {
        self.policyId.unwrap_or(0)
    }

    fn get_policyId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.policyId
    }

    fn mut_policyId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.policyId
    }

    // required string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // required .hadoop.hdfs.StorageTypesProto creationPolicy = 3;

    pub fn clear_creationPolicy(&mut self) {
        self.creationPolicy.clear();
    }

    pub fn has_creationPolicy(&self) -> bool {
        self.creationPolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_creationPolicy(&mut self, v: StorageTypesProto) {
        self.creationPolicy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_creationPolicy(&mut self) -> &mut StorageTypesProto {
        if self.creationPolicy.is_none() {
            self.creationPolicy.set_default();
        }
        self.creationPolicy.as_mut().unwrap()
    }

    // Take field
    pub fn take_creationPolicy(&mut self) -> StorageTypesProto {
        self.creationPolicy.take().unwrap_or_else(|| StorageTypesProto::new())
    }

    pub fn get_creationPolicy(&self) -> &StorageTypesProto {
        self.creationPolicy.as_ref().unwrap_or_else(|| StorageTypesProto::default_instance())
    }

    fn get_creationPolicy_for_reflect(&self) -> &::protobuf::SingularPtrField<StorageTypesProto> {
        &self.creationPolicy
    }

    fn mut_creationPolicy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<StorageTypesProto> {
        &mut self.creationPolicy
    }

    // optional .hadoop.hdfs.StorageTypesProto creationFallbackPolicy = 4;

    pub fn clear_creationFallbackPolicy(&mut self) {
        self.creationFallbackPolicy.clear();
    }

    pub fn has_creationFallbackPolicy(&self) -> bool {
        self.creationFallbackPolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_creationFallbackPolicy(&mut self, v: StorageTypesProto) {
        self.creationFallbackPolicy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_creationFallbackPolicy(&mut self) -> &mut StorageTypesProto {
        if self.creationFallbackPolicy.is_none() {
            self.creationFallbackPolicy.set_default();
        }
        self.creationFallbackPolicy.as_mut().unwrap()
    }

    // Take field
    pub fn take_creationFallbackPolicy(&mut self) -> StorageTypesProto {
        self.creationFallbackPolicy.take().unwrap_or_else(|| StorageTypesProto::new())
    }

    pub fn get_creationFallbackPolicy(&self) -> &StorageTypesProto {
        self.creationFallbackPolicy.as_ref().unwrap_or_else(|| StorageTypesProto::default_instance())
    }

    fn get_creationFallbackPolicy_for_reflect(&self) -> &::protobuf::SingularPtrField<StorageTypesProto> {
        &self.creationFallbackPolicy
    }

    fn mut_creationFallbackPolicy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<StorageTypesProto> {
        &mut self.creationFallbackPolicy
    }

    // optional .hadoop.hdfs.StorageTypesProto replicationFallbackPolicy = 5;

    pub fn clear_replicationFallbackPolicy(&mut self) {
        self.replicationFallbackPolicy.clear();
    }

    pub fn has_replicationFallbackPolicy(&self) -> bool {
        self.replicationFallbackPolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replicationFallbackPolicy(&mut self, v: StorageTypesProto) {
        self.replicationFallbackPolicy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_replicationFallbackPolicy(&mut self) -> &mut StorageTypesProto {
        if self.replicationFallbackPolicy.is_none() {
            self.replicationFallbackPolicy.set_default();
        }
        self.replicationFallbackPolicy.as_mut().unwrap()
    }

    // Take field
    pub fn take_replicationFallbackPolicy(&mut self) -> StorageTypesProto {
        self.replicationFallbackPolicy.take().unwrap_or_else(|| StorageTypesProto::new())
    }

    pub fn get_replicationFallbackPolicy(&self) -> &StorageTypesProto {
        self.replicationFallbackPolicy.as_ref().unwrap_or_else(|| StorageTypesProto::default_instance())
    }

    fn get_replicationFallbackPolicy_for_reflect(&self) -> &::protobuf::SingularPtrField<StorageTypesProto> {
        &self.replicationFallbackPolicy
    }

    fn mut_replicationFallbackPolicy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<StorageTypesProto> {
        &mut self.replicationFallbackPolicy
    }
}

impl ::protobuf::Message for BlockStoragePolicyProto {
    fn is_initialized(&self) -> bool {
        if self.policyId.is_none() {
            return false;
        }
        if self.name.is_none() {
            return false;
        }
        if self.creationPolicy.is_none() {
            return false;
        }
        for v in &self.creationPolicy {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.creationFallbackPolicy {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.replicationFallbackPolicy {
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
                    self.policyId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.creationPolicy)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.creationFallbackPolicy)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.replicationFallbackPolicy)?;
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
        if let Some(v) = self.policyId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.creationPolicy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.creationFallbackPolicy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.replicationFallbackPolicy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.policyId {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.creationPolicy.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.creationFallbackPolicy.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.replicationFallbackPolicy.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for BlockStoragePolicyProto {
    fn new() -> BlockStoragePolicyProto {
        BlockStoragePolicyProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockStoragePolicyProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "policyId",
                    BlockStoragePolicyProto::get_policyId_for_reflect,
                    BlockStoragePolicyProto::mut_policyId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    BlockStoragePolicyProto::get_name_for_reflect,
                    BlockStoragePolicyProto::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StorageTypesProto>>(
                    "creationPolicy",
                    BlockStoragePolicyProto::get_creationPolicy_for_reflect,
                    BlockStoragePolicyProto::mut_creationPolicy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StorageTypesProto>>(
                    "creationFallbackPolicy",
                    BlockStoragePolicyProto::get_creationFallbackPolicy_for_reflect,
                    BlockStoragePolicyProto::mut_creationFallbackPolicy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StorageTypesProto>>(
                    "replicationFallbackPolicy",
                    BlockStoragePolicyProto::get_replicationFallbackPolicy_for_reflect,
                    BlockStoragePolicyProto::mut_replicationFallbackPolicy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockStoragePolicyProto>(
                    "BlockStoragePolicyProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockStoragePolicyProto {
    fn clear(&mut self) {
        self.clear_policyId();
        self.clear_name();
        self.clear_creationPolicy();
        self.clear_creationFallbackPolicy();
        self.clear_replicationFallbackPolicy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockStoragePolicyProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockStoragePolicyProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StorageUuidsProto {
    // message fields
    storageUuids: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StorageUuidsProto {}

impl StorageUuidsProto {
    pub fn new() -> StorageUuidsProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StorageUuidsProto {
        static mut instance: ::protobuf::lazy::Lazy<StorageUuidsProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StorageUuidsProto,
        };
        unsafe {
            instance.get(StorageUuidsProto::new)
        }
    }

    // repeated string storageUuids = 1;

    pub fn clear_storageUuids(&mut self) {
        self.storageUuids.clear();
    }

    // Param is passed by value, moved
    pub fn set_storageUuids(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.storageUuids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_storageUuids(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.storageUuids
    }

    // Take field
    pub fn take_storageUuids(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.storageUuids, ::protobuf::RepeatedField::new())
    }

    pub fn get_storageUuids(&self) -> &[::std::string::String] {
        &self.storageUuids
    }

    fn get_storageUuids_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.storageUuids
    }

    fn mut_storageUuids_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.storageUuids
    }
}

impl ::protobuf::Message for StorageUuidsProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.storageUuids)?;
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
        for value in &self.storageUuids {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.storageUuids {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for StorageUuidsProto {
    fn new() -> StorageUuidsProto {
        StorageUuidsProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StorageUuidsProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "storageUuids",
                    StorageUuidsProto::get_storageUuids_for_reflect,
                    StorageUuidsProto::mut_storageUuids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StorageUuidsProto>(
                    "StorageUuidsProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StorageUuidsProto {
    fn clear(&mut self) {
        self.clear_storageUuids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StorageUuidsProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StorageUuidsProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LocatedBlockProto {
    // message fields
    b: ::protobuf::SingularPtrField<ExtendedBlockProto>,
    offset: ::std::option::Option<u64>,
    locs: ::protobuf::RepeatedField<DatanodeInfoProto>,
    corrupt: ::std::option::Option<bool>,
    blockToken: ::protobuf::SingularPtrField<super::Security::TokenProto>,
    isCached: ::std::vec::Vec<bool>,
    storageTypes: ::std::vec::Vec<StorageTypeProto>,
    storageIDs: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LocatedBlockProto {}

impl LocatedBlockProto {
    pub fn new() -> LocatedBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LocatedBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<LocatedBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LocatedBlockProto,
        };
        unsafe {
            instance.get(LocatedBlockProto::new)
        }
    }

    // required .hadoop.hdfs.ExtendedBlockProto b = 1;

    pub fn clear_b(&mut self) {
        self.b.clear();
    }

    pub fn has_b(&self) -> bool {
        self.b.is_some()
    }

    // Param is passed by value, moved
    pub fn set_b(&mut self, v: ExtendedBlockProto) {
        self.b = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_b(&mut self) -> &mut ExtendedBlockProto {
        if self.b.is_none() {
            self.b.set_default();
        }
        self.b.as_mut().unwrap()
    }

    // Take field
    pub fn take_b(&mut self) -> ExtendedBlockProto {
        self.b.take().unwrap_or_else(|| ExtendedBlockProto::new())
    }

    pub fn get_b(&self) -> &ExtendedBlockProto {
        self.b.as_ref().unwrap_or_else(|| ExtendedBlockProto::default_instance())
    }

    fn get_b_for_reflect(&self) -> &::protobuf::SingularPtrField<ExtendedBlockProto> {
        &self.b
    }

    fn mut_b_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ExtendedBlockProto> {
        &mut self.b
    }

    // required uint64 offset = 2;

    pub fn clear_offset(&mut self) {
        self.offset = ::std::option::Option::None;
    }

    pub fn has_offset(&self) -> bool {
        self.offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_offset(&mut self, v: u64) {
        self.offset = ::std::option::Option::Some(v);
    }

    pub fn get_offset(&self) -> u64 {
        self.offset.unwrap_or(0)
    }

    fn get_offset_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.offset
    }

    fn mut_offset_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.offset
    }

    // repeated .hadoop.hdfs.DatanodeInfoProto locs = 3;

    pub fn clear_locs(&mut self) {
        self.locs.clear();
    }

    // Param is passed by value, moved
    pub fn set_locs(&mut self, v: ::protobuf::RepeatedField<DatanodeInfoProto>) {
        self.locs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_locs(&mut self) -> &mut ::protobuf::RepeatedField<DatanodeInfoProto> {
        &mut self.locs
    }

    // Take field
    pub fn take_locs(&mut self) -> ::protobuf::RepeatedField<DatanodeInfoProto> {
        ::std::mem::replace(&mut self.locs, ::protobuf::RepeatedField::new())
    }

    pub fn get_locs(&self) -> &[DatanodeInfoProto] {
        &self.locs
    }

    fn get_locs_for_reflect(&self) -> &::protobuf::RepeatedField<DatanodeInfoProto> {
        &self.locs
    }

    fn mut_locs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DatanodeInfoProto> {
        &mut self.locs
    }

    // required bool corrupt = 4;

    pub fn clear_corrupt(&mut self) {
        self.corrupt = ::std::option::Option::None;
    }

    pub fn has_corrupt(&self) -> bool {
        self.corrupt.is_some()
    }

    // Param is passed by value, moved
    pub fn set_corrupt(&mut self, v: bool) {
        self.corrupt = ::std::option::Option::Some(v);
    }

    pub fn get_corrupt(&self) -> bool {
        self.corrupt.unwrap_or(false)
    }

    fn get_corrupt_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.corrupt
    }

    fn mut_corrupt_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.corrupt
    }

    // required .hadoop.common.TokenProto blockToken = 5;

    pub fn clear_blockToken(&mut self) {
        self.blockToken.clear();
    }

    pub fn has_blockToken(&self) -> bool {
        self.blockToken.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockToken(&mut self, v: super::Security::TokenProto) {
        self.blockToken = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockToken(&mut self) -> &mut super::Security::TokenProto {
        if self.blockToken.is_none() {
            self.blockToken.set_default();
        }
        self.blockToken.as_mut().unwrap()
    }

    // Take field
    pub fn take_blockToken(&mut self) -> super::Security::TokenProto {
        self.blockToken.take().unwrap_or_else(|| super::Security::TokenProto::new())
    }

    pub fn get_blockToken(&self) -> &super::Security::TokenProto {
        self.blockToken.as_ref().unwrap_or_else(|| super::Security::TokenProto::default_instance())
    }

    fn get_blockToken_for_reflect(&self) -> &::protobuf::SingularPtrField<super::Security::TokenProto> {
        &self.blockToken
    }

    fn mut_blockToken_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::Security::TokenProto> {
        &mut self.blockToken
    }

    // repeated bool isCached = 6;

    pub fn clear_isCached(&mut self) {
        self.isCached.clear();
    }

    // Param is passed by value, moved
    pub fn set_isCached(&mut self, v: ::std::vec::Vec<bool>) {
        self.isCached = v;
    }

    // Mutable pointer to the field.
    pub fn mut_isCached(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.isCached
    }

    // Take field
    pub fn take_isCached(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.isCached, ::std::vec::Vec::new())
    }

    pub fn get_isCached(&self) -> &[bool] {
        &self.isCached
    }

    fn get_isCached_for_reflect(&self) -> &::std::vec::Vec<bool> {
        &self.isCached
    }

    fn mut_isCached_for_reflect(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.isCached
    }

    // repeated .hadoop.hdfs.StorageTypeProto storageTypes = 7;

    pub fn clear_storageTypes(&mut self) {
        self.storageTypes.clear();
    }

    // Param is passed by value, moved
    pub fn set_storageTypes(&mut self, v: ::std::vec::Vec<StorageTypeProto>) {
        self.storageTypes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_storageTypes(&mut self) -> &mut ::std::vec::Vec<StorageTypeProto> {
        &mut self.storageTypes
    }

    // Take field
    pub fn take_storageTypes(&mut self) -> ::std::vec::Vec<StorageTypeProto> {
        ::std::mem::replace(&mut self.storageTypes, ::std::vec::Vec::new())
    }

    pub fn get_storageTypes(&self) -> &[StorageTypeProto] {
        &self.storageTypes
    }

    fn get_storageTypes_for_reflect(&self) -> &::std::vec::Vec<StorageTypeProto> {
        &self.storageTypes
    }

    fn mut_storageTypes_for_reflect(&mut self) -> &mut ::std::vec::Vec<StorageTypeProto> {
        &mut self.storageTypes
    }

    // repeated string storageIDs = 8;

    pub fn clear_storageIDs(&mut self) {
        self.storageIDs.clear();
    }

    // Param is passed by value, moved
    pub fn set_storageIDs(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.storageIDs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_storageIDs(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.storageIDs
    }

    // Take field
    pub fn take_storageIDs(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.storageIDs, ::protobuf::RepeatedField::new())
    }

    pub fn get_storageIDs(&self) -> &[::std::string::String] {
        &self.storageIDs
    }

    fn get_storageIDs_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.storageIDs
    }

    fn mut_storageIDs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.storageIDs
    }
}

impl ::protobuf::Message for LocatedBlockProto {
    fn is_initialized(&self) -> bool {
        if self.b.is_none() {
            return false;
        }
        if self.offset.is_none() {
            return false;
        }
        if self.corrupt.is_none() {
            return false;
        }
        if self.blockToken.is_none() {
            return false;
        }
        for v in &self.b {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.locs {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.blockToken {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.b)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.offset = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.locs)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.corrupt = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.blockToken)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.isCached)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.storageTypes)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.storageIDs)?;
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
        if let Some(ref v) = self.b.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.offset {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.locs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.corrupt {
            my_size += 2;
        }
        if let Some(ref v) = self.blockToken.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.isCached.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.isCached.len() as u32) + (self.isCached.len() * 1) as u32;
        }
        for value in &self.storageTypes {
            my_size += ::protobuf::rt::enum_size(7, *value);
        };
        for value in &self.storageIDs {
            my_size += ::protobuf::rt::string_size(8, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.b.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.offset {
            os.write_uint64(2, v)?;
        }
        for v in &self.locs {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.corrupt {
            os.write_bool(4, v)?;
        }
        if let Some(ref v) = self.blockToken.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.isCached.is_empty() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.isCached.len() * 1) as u32)?;
            for v in &self.isCached {
                os.write_bool_no_tag(*v)?;
            };
        }
        for v in &self.storageTypes {
            os.write_enum(7, v.value())?;
        };
        for v in &self.storageIDs {
            os.write_string(8, &v)?;
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

impl ::protobuf::MessageStatic for LocatedBlockProto {
    fn new() -> LocatedBlockProto {
        LocatedBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<LocatedBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ExtendedBlockProto>>(
                    "b",
                    LocatedBlockProto::get_b_for_reflect,
                    LocatedBlockProto::mut_b_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "offset",
                    LocatedBlockProto::get_offset_for_reflect,
                    LocatedBlockProto::mut_offset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DatanodeInfoProto>>(
                    "locs",
                    LocatedBlockProto::get_locs_for_reflect,
                    LocatedBlockProto::mut_locs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "corrupt",
                    LocatedBlockProto::get_corrupt_for_reflect,
                    LocatedBlockProto::mut_corrupt_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::Security::TokenProto>>(
                    "blockToken",
                    LocatedBlockProto::get_blockToken_for_reflect,
                    LocatedBlockProto::mut_blockToken_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "isCached",
                    LocatedBlockProto::get_isCached_for_reflect,
                    LocatedBlockProto::mut_isCached_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<StorageTypeProto>>(
                    "storageTypes",
                    LocatedBlockProto::get_storageTypes_for_reflect,
                    LocatedBlockProto::mut_storageTypes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "storageIDs",
                    LocatedBlockProto::get_storageIDs_for_reflect,
                    LocatedBlockProto::mut_storageIDs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LocatedBlockProto>(
                    "LocatedBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LocatedBlockProto {
    fn clear(&mut self) {
        self.clear_b();
        self.clear_offset();
        self.clear_locs();
        self.clear_corrupt();
        self.clear_blockToken();
        self.clear_isCached();
        self.clear_storageTypes();
        self.clear_storageIDs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LocatedBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LocatedBlockProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataEncryptionKeyProto {
    // message fields
    keyId: ::std::option::Option<u32>,
    blockPoolId: ::protobuf::SingularField<::std::string::String>,
    nonce: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    encryptionKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    expiryDate: ::std::option::Option<u64>,
    encryptionAlgorithm: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataEncryptionKeyProto {}

impl DataEncryptionKeyProto {
    pub fn new() -> DataEncryptionKeyProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataEncryptionKeyProto {
        static mut instance: ::protobuf::lazy::Lazy<DataEncryptionKeyProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataEncryptionKeyProto,
        };
        unsafe {
            instance.get(DataEncryptionKeyProto::new)
        }
    }

    // required uint32 keyId = 1;

    pub fn clear_keyId(&mut self) {
        self.keyId = ::std::option::Option::None;
    }

    pub fn has_keyId(&self) -> bool {
        self.keyId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyId(&mut self, v: u32) {
        self.keyId = ::std::option::Option::Some(v);
    }

    pub fn get_keyId(&self) -> u32 {
        self.keyId.unwrap_or(0)
    }

    fn get_keyId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.keyId
    }

    fn mut_keyId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.keyId
    }

    // required string blockPoolId = 2;

    pub fn clear_blockPoolId(&mut self) {
        self.blockPoolId.clear();
    }

    pub fn has_blockPoolId(&self) -> bool {
        self.blockPoolId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPoolId(&mut self, v: ::std::string::String) {
        self.blockPoolId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockPoolId(&mut self) -> &mut ::std::string::String {
        if self.blockPoolId.is_none() {
            self.blockPoolId.set_default();
        }
        self.blockPoolId.as_mut().unwrap()
    }

    // Take field
    pub fn take_blockPoolId(&mut self) -> ::std::string::String {
        self.blockPoolId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_blockPoolId(&self) -> &str {
        match self.blockPoolId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_blockPoolId_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.blockPoolId
    }

    fn mut_blockPoolId_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.blockPoolId
    }

    // required bytes nonce = 3;

    pub fn clear_nonce(&mut self) {
        self.nonce.clear();
    }

    pub fn has_nonce(&self) -> bool {
        self.nonce.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nonce(&mut self, v: ::std::vec::Vec<u8>) {
        self.nonce = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nonce(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.nonce.is_none() {
            self.nonce.set_default();
        }
        self.nonce.as_mut().unwrap()
    }

    // Take field
    pub fn take_nonce(&mut self) -> ::std::vec::Vec<u8> {
        self.nonce.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_nonce(&self) -> &[u8] {
        match self.nonce.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_nonce_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.nonce
    }

    fn mut_nonce_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.nonce
    }

    // required bytes encryptionKey = 4;

    pub fn clear_encryptionKey(&mut self) {
        self.encryptionKey.clear();
    }

    pub fn has_encryptionKey(&self) -> bool {
        self.encryptionKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encryptionKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.encryptionKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encryptionKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.encryptionKey.is_none() {
            self.encryptionKey.set_default();
        }
        self.encryptionKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_encryptionKey(&mut self) -> ::std::vec::Vec<u8> {
        self.encryptionKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_encryptionKey(&self) -> &[u8] {
        match self.encryptionKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_encryptionKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.encryptionKey
    }

    fn mut_encryptionKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.encryptionKey
    }

    // required uint64 expiryDate = 5;

    pub fn clear_expiryDate(&mut self) {
        self.expiryDate = ::std::option::Option::None;
    }

    pub fn has_expiryDate(&self) -> bool {
        self.expiryDate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expiryDate(&mut self, v: u64) {
        self.expiryDate = ::std::option::Option::Some(v);
    }

    pub fn get_expiryDate(&self) -> u64 {
        self.expiryDate.unwrap_or(0)
    }

    fn get_expiryDate_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.expiryDate
    }

    fn mut_expiryDate_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.expiryDate
    }

    // optional string encryptionAlgorithm = 6;

    pub fn clear_encryptionAlgorithm(&mut self) {
        self.encryptionAlgorithm.clear();
    }

    pub fn has_encryptionAlgorithm(&self) -> bool {
        self.encryptionAlgorithm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encryptionAlgorithm(&mut self, v: ::std::string::String) {
        self.encryptionAlgorithm = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encryptionAlgorithm(&mut self) -> &mut ::std::string::String {
        if self.encryptionAlgorithm.is_none() {
            self.encryptionAlgorithm.set_default();
        }
        self.encryptionAlgorithm.as_mut().unwrap()
    }

    // Take field
    pub fn take_encryptionAlgorithm(&mut self) -> ::std::string::String {
        self.encryptionAlgorithm.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_encryptionAlgorithm(&self) -> &str {
        match self.encryptionAlgorithm.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_encryptionAlgorithm_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.encryptionAlgorithm
    }

    fn mut_encryptionAlgorithm_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.encryptionAlgorithm
    }
}

impl ::protobuf::Message for DataEncryptionKeyProto {
    fn is_initialized(&self) -> bool {
        if self.keyId.is_none() {
            return false;
        }
        if self.blockPoolId.is_none() {
            return false;
        }
        if self.nonce.is_none() {
            return false;
        }
        if self.encryptionKey.is_none() {
            return false;
        }
        if self.expiryDate.is_none() {
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
                    self.keyId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.blockPoolId)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.nonce)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.encryptionKey)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.expiryDate = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.encryptionAlgorithm)?;
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
        if let Some(v) = self.keyId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.blockPoolId.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.nonce.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        if let Some(ref v) = self.encryptionKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        if let Some(v) = self.expiryDate {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.encryptionAlgorithm.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.keyId {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.blockPoolId.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.nonce.as_ref() {
            os.write_bytes(3, &v)?;
        }
        if let Some(ref v) = self.encryptionKey.as_ref() {
            os.write_bytes(4, &v)?;
        }
        if let Some(v) = self.expiryDate {
            os.write_uint64(5, v)?;
        }
        if let Some(ref v) = self.encryptionAlgorithm.as_ref() {
            os.write_string(6, &v)?;
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

impl ::protobuf::MessageStatic for DataEncryptionKeyProto {
    fn new() -> DataEncryptionKeyProto {
        DataEncryptionKeyProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataEncryptionKeyProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "keyId",
                    DataEncryptionKeyProto::get_keyId_for_reflect,
                    DataEncryptionKeyProto::mut_keyId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "blockPoolId",
                    DataEncryptionKeyProto::get_blockPoolId_for_reflect,
                    DataEncryptionKeyProto::mut_blockPoolId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "nonce",
                    DataEncryptionKeyProto::get_nonce_for_reflect,
                    DataEncryptionKeyProto::mut_nonce_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "encryptionKey",
                    DataEncryptionKeyProto::get_encryptionKey_for_reflect,
                    DataEncryptionKeyProto::mut_encryptionKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "expiryDate",
                    DataEncryptionKeyProto::get_expiryDate_for_reflect,
                    DataEncryptionKeyProto::mut_expiryDate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "encryptionAlgorithm",
                    DataEncryptionKeyProto::get_encryptionAlgorithm_for_reflect,
                    DataEncryptionKeyProto::mut_encryptionAlgorithm_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataEncryptionKeyProto>(
                    "DataEncryptionKeyProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataEncryptionKeyProto {
    fn clear(&mut self) {
        self.clear_keyId();
        self.clear_blockPoolId();
        self.clear_nonce();
        self.clear_encryptionKey();
        self.clear_expiryDate();
        self.clear_encryptionAlgorithm();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataEncryptionKeyProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataEncryptionKeyProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FileEncryptionInfoProto {
    // message fields
    suite: ::std::option::Option<CipherSuiteProto>,
    cryptoProtocolVersion: ::std::option::Option<CryptoProtocolVersionProto>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    iv: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    keyName: ::protobuf::SingularField<::std::string::String>,
    ezKeyVersionName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FileEncryptionInfoProto {}

impl FileEncryptionInfoProto {
    pub fn new() -> FileEncryptionInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FileEncryptionInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<FileEncryptionInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FileEncryptionInfoProto,
        };
        unsafe {
            instance.get(FileEncryptionInfoProto::new)
        }
    }

    // required .hadoop.hdfs.CipherSuiteProto suite = 1;

    pub fn clear_suite(&mut self) {
        self.suite = ::std::option::Option::None;
    }

    pub fn has_suite(&self) -> bool {
        self.suite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suite(&mut self, v: CipherSuiteProto) {
        self.suite = ::std::option::Option::Some(v);
    }

    pub fn get_suite(&self) -> CipherSuiteProto {
        self.suite.unwrap_or(CipherSuiteProto::UNKNOWN)
    }

    fn get_suite_for_reflect(&self) -> &::std::option::Option<CipherSuiteProto> {
        &self.suite
    }

    fn mut_suite_for_reflect(&mut self) -> &mut ::std::option::Option<CipherSuiteProto> {
        &mut self.suite
    }

    // required .hadoop.hdfs.CryptoProtocolVersionProto cryptoProtocolVersion = 2;

    pub fn clear_cryptoProtocolVersion(&mut self) {
        self.cryptoProtocolVersion = ::std::option::Option::None;
    }

    pub fn has_cryptoProtocolVersion(&self) -> bool {
        self.cryptoProtocolVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cryptoProtocolVersion(&mut self, v: CryptoProtocolVersionProto) {
        self.cryptoProtocolVersion = ::std::option::Option::Some(v);
    }

    pub fn get_cryptoProtocolVersion(&self) -> CryptoProtocolVersionProto {
        self.cryptoProtocolVersion.unwrap_or(CryptoProtocolVersionProto::UNKNOWN_PROTOCOL_VERSION)
    }

    fn get_cryptoProtocolVersion_for_reflect(&self) -> &::std::option::Option<CryptoProtocolVersionProto> {
        &self.cryptoProtocolVersion
    }

    fn mut_cryptoProtocolVersion_for_reflect(&mut self) -> &mut ::std::option::Option<CryptoProtocolVersionProto> {
        &mut self.cryptoProtocolVersion
    }

    // required bytes key = 3;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        }
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // required bytes iv = 4;

    pub fn clear_iv(&mut self) {
        self.iv.clear();
    }

    pub fn has_iv(&self) -> bool {
        self.iv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iv(&mut self, v: ::std::vec::Vec<u8>) {
        self.iv = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_iv(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.iv.is_none() {
            self.iv.set_default();
        }
        self.iv.as_mut().unwrap()
    }

    // Take field
    pub fn take_iv(&mut self) -> ::std::vec::Vec<u8> {
        self.iv.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_iv(&self) -> &[u8] {
        match self.iv.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_iv_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.iv
    }

    fn mut_iv_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.iv
    }

    // required string keyName = 5;

    pub fn clear_keyName(&mut self) {
        self.keyName.clear();
    }

    pub fn has_keyName(&self) -> bool {
        self.keyName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyName(&mut self, v: ::std::string::String) {
        self.keyName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_keyName(&mut self) -> &mut ::std::string::String {
        if self.keyName.is_none() {
            self.keyName.set_default();
        }
        self.keyName.as_mut().unwrap()
    }

    // Take field
    pub fn take_keyName(&mut self) -> ::std::string::String {
        self.keyName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_keyName(&self) -> &str {
        match self.keyName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_keyName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.keyName
    }

    fn mut_keyName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.keyName
    }

    // required string ezKeyVersionName = 6;

    pub fn clear_ezKeyVersionName(&mut self) {
        self.ezKeyVersionName.clear();
    }

    pub fn has_ezKeyVersionName(&self) -> bool {
        self.ezKeyVersionName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ezKeyVersionName(&mut self, v: ::std::string::String) {
        self.ezKeyVersionName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ezKeyVersionName(&mut self) -> &mut ::std::string::String {
        if self.ezKeyVersionName.is_none() {
            self.ezKeyVersionName.set_default();
        }
        self.ezKeyVersionName.as_mut().unwrap()
    }

    // Take field
    pub fn take_ezKeyVersionName(&mut self) -> ::std::string::String {
        self.ezKeyVersionName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ezKeyVersionName(&self) -> &str {
        match self.ezKeyVersionName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ezKeyVersionName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ezKeyVersionName
    }

    fn mut_ezKeyVersionName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ezKeyVersionName
    }
}

impl ::protobuf::Message for FileEncryptionInfoProto {
    fn is_initialized(&self) -> bool {
        if self.suite.is_none() {
            return false;
        }
        if self.cryptoProtocolVersion.is_none() {
            return false;
        }
        if self.key.is_none() {
            return false;
        }
        if self.iv.is_none() {
            return false;
        }
        if self.keyName.is_none() {
            return false;
        }
        if self.ezKeyVersionName.is_none() {
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
                    let tmp = is.read_enum()?;
                    self.suite = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.cryptoProtocolVersion = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.iv)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.keyName)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ezKeyVersionName)?;
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
        if let Some(v) = self.suite {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.cryptoProtocolVersion {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(ref v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        if let Some(ref v) = self.iv.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        if let Some(ref v) = self.keyName.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.ezKeyVersionName.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.suite {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.cryptoProtocolVersion {
            os.write_enum(2, v.value())?;
        }
        if let Some(ref v) = self.key.as_ref() {
            os.write_bytes(3, &v)?;
        }
        if let Some(ref v) = self.iv.as_ref() {
            os.write_bytes(4, &v)?;
        }
        if let Some(ref v) = self.keyName.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.ezKeyVersionName.as_ref() {
            os.write_string(6, &v)?;
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

impl ::protobuf::MessageStatic for FileEncryptionInfoProto {
    fn new() -> FileEncryptionInfoProto {
        FileEncryptionInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FileEncryptionInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CipherSuiteProto>>(
                    "suite",
                    FileEncryptionInfoProto::get_suite_for_reflect,
                    FileEncryptionInfoProto::mut_suite_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CryptoProtocolVersionProto>>(
                    "cryptoProtocolVersion",
                    FileEncryptionInfoProto::get_cryptoProtocolVersion_for_reflect,
                    FileEncryptionInfoProto::mut_cryptoProtocolVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    FileEncryptionInfoProto::get_key_for_reflect,
                    FileEncryptionInfoProto::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "iv",
                    FileEncryptionInfoProto::get_iv_for_reflect,
                    FileEncryptionInfoProto::mut_iv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "keyName",
                    FileEncryptionInfoProto::get_keyName_for_reflect,
                    FileEncryptionInfoProto::mut_keyName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ezKeyVersionName",
                    FileEncryptionInfoProto::get_ezKeyVersionName_for_reflect,
                    FileEncryptionInfoProto::mut_ezKeyVersionName_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FileEncryptionInfoProto>(
                    "FileEncryptionInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FileEncryptionInfoProto {
    fn clear(&mut self) {
        self.clear_suite();
        self.clear_cryptoProtocolVersion();
        self.clear_key();
        self.clear_iv();
        self.clear_keyName();
        self.clear_ezKeyVersionName();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FileEncryptionInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FileEncryptionInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PerFileEncryptionInfoProto {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    iv: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    ezKeyVersionName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PerFileEncryptionInfoProto {}

impl PerFileEncryptionInfoProto {
    pub fn new() -> PerFileEncryptionInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PerFileEncryptionInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<PerFileEncryptionInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PerFileEncryptionInfoProto,
        };
        unsafe {
            instance.get(PerFileEncryptionInfoProto::new)
        }
    }

    // required bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        }
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // required bytes iv = 2;

    pub fn clear_iv(&mut self) {
        self.iv.clear();
    }

    pub fn has_iv(&self) -> bool {
        self.iv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iv(&mut self, v: ::std::vec::Vec<u8>) {
        self.iv = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_iv(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.iv.is_none() {
            self.iv.set_default();
        }
        self.iv.as_mut().unwrap()
    }

    // Take field
    pub fn take_iv(&mut self) -> ::std::vec::Vec<u8> {
        self.iv.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_iv(&self) -> &[u8] {
        match self.iv.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_iv_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.iv
    }

    fn mut_iv_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.iv
    }

    // required string ezKeyVersionName = 3;

    pub fn clear_ezKeyVersionName(&mut self) {
        self.ezKeyVersionName.clear();
    }

    pub fn has_ezKeyVersionName(&self) -> bool {
        self.ezKeyVersionName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ezKeyVersionName(&mut self, v: ::std::string::String) {
        self.ezKeyVersionName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ezKeyVersionName(&mut self) -> &mut ::std::string::String {
        if self.ezKeyVersionName.is_none() {
            self.ezKeyVersionName.set_default();
        }
        self.ezKeyVersionName.as_mut().unwrap()
    }

    // Take field
    pub fn take_ezKeyVersionName(&mut self) -> ::std::string::String {
        self.ezKeyVersionName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ezKeyVersionName(&self) -> &str {
        match self.ezKeyVersionName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ezKeyVersionName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ezKeyVersionName
    }

    fn mut_ezKeyVersionName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ezKeyVersionName
    }
}

impl ::protobuf::Message for PerFileEncryptionInfoProto {
    fn is_initialized(&self) -> bool {
        if self.key.is_none() {
            return false;
        }
        if self.iv.is_none() {
            return false;
        }
        if self.ezKeyVersionName.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.iv)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ezKeyVersionName)?;
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
        if let Some(ref v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.iv.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(ref v) = self.ezKeyVersionName.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.key.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.iv.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(ref v) = self.ezKeyVersionName.as_ref() {
            os.write_string(3, &v)?;
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

impl ::protobuf::MessageStatic for PerFileEncryptionInfoProto {
    fn new() -> PerFileEncryptionInfoProto {
        PerFileEncryptionInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<PerFileEncryptionInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    PerFileEncryptionInfoProto::get_key_for_reflect,
                    PerFileEncryptionInfoProto::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "iv",
                    PerFileEncryptionInfoProto::get_iv_for_reflect,
                    PerFileEncryptionInfoProto::mut_iv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ezKeyVersionName",
                    PerFileEncryptionInfoProto::get_ezKeyVersionName_for_reflect,
                    PerFileEncryptionInfoProto::mut_ezKeyVersionName_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PerFileEncryptionInfoProto>(
                    "PerFileEncryptionInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PerFileEncryptionInfoProto {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_iv();
        self.clear_ezKeyVersionName();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PerFileEncryptionInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PerFileEncryptionInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ZoneEncryptionInfoProto {
    // message fields
    suite: ::std::option::Option<CipherSuiteProto>,
    cryptoProtocolVersion: ::std::option::Option<CryptoProtocolVersionProto>,
    keyName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ZoneEncryptionInfoProto {}

impl ZoneEncryptionInfoProto {
    pub fn new() -> ZoneEncryptionInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ZoneEncryptionInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<ZoneEncryptionInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ZoneEncryptionInfoProto,
        };
        unsafe {
            instance.get(ZoneEncryptionInfoProto::new)
        }
    }

    // required .hadoop.hdfs.CipherSuiteProto suite = 1;

    pub fn clear_suite(&mut self) {
        self.suite = ::std::option::Option::None;
    }

    pub fn has_suite(&self) -> bool {
        self.suite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suite(&mut self, v: CipherSuiteProto) {
        self.suite = ::std::option::Option::Some(v);
    }

    pub fn get_suite(&self) -> CipherSuiteProto {
        self.suite.unwrap_or(CipherSuiteProto::UNKNOWN)
    }

    fn get_suite_for_reflect(&self) -> &::std::option::Option<CipherSuiteProto> {
        &self.suite
    }

    fn mut_suite_for_reflect(&mut self) -> &mut ::std::option::Option<CipherSuiteProto> {
        &mut self.suite
    }

    // required .hadoop.hdfs.CryptoProtocolVersionProto cryptoProtocolVersion = 2;

    pub fn clear_cryptoProtocolVersion(&mut self) {
        self.cryptoProtocolVersion = ::std::option::Option::None;
    }

    pub fn has_cryptoProtocolVersion(&self) -> bool {
        self.cryptoProtocolVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cryptoProtocolVersion(&mut self, v: CryptoProtocolVersionProto) {
        self.cryptoProtocolVersion = ::std::option::Option::Some(v);
    }

    pub fn get_cryptoProtocolVersion(&self) -> CryptoProtocolVersionProto {
        self.cryptoProtocolVersion.unwrap_or(CryptoProtocolVersionProto::UNKNOWN_PROTOCOL_VERSION)
    }

    fn get_cryptoProtocolVersion_for_reflect(&self) -> &::std::option::Option<CryptoProtocolVersionProto> {
        &self.cryptoProtocolVersion
    }

    fn mut_cryptoProtocolVersion_for_reflect(&mut self) -> &mut ::std::option::Option<CryptoProtocolVersionProto> {
        &mut self.cryptoProtocolVersion
    }

    // required string keyName = 3;

    pub fn clear_keyName(&mut self) {
        self.keyName.clear();
    }

    pub fn has_keyName(&self) -> bool {
        self.keyName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyName(&mut self, v: ::std::string::String) {
        self.keyName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_keyName(&mut self) -> &mut ::std::string::String {
        if self.keyName.is_none() {
            self.keyName.set_default();
        }
        self.keyName.as_mut().unwrap()
    }

    // Take field
    pub fn take_keyName(&mut self) -> ::std::string::String {
        self.keyName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_keyName(&self) -> &str {
        match self.keyName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_keyName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.keyName
    }

    fn mut_keyName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.keyName
    }
}

impl ::protobuf::Message for ZoneEncryptionInfoProto {
    fn is_initialized(&self) -> bool {
        if self.suite.is_none() {
            return false;
        }
        if self.cryptoProtocolVersion.is_none() {
            return false;
        }
        if self.keyName.is_none() {
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
                    let tmp = is.read_enum()?;
                    self.suite = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.cryptoProtocolVersion = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.keyName)?;
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
        if let Some(v) = self.suite {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.cryptoProtocolVersion {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(ref v) = self.keyName.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.suite {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.cryptoProtocolVersion {
            os.write_enum(2, v.value())?;
        }
        if let Some(ref v) = self.keyName.as_ref() {
            os.write_string(3, &v)?;
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

impl ::protobuf::MessageStatic for ZoneEncryptionInfoProto {
    fn new() -> ZoneEncryptionInfoProto {
        ZoneEncryptionInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ZoneEncryptionInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CipherSuiteProto>>(
                    "suite",
                    ZoneEncryptionInfoProto::get_suite_for_reflect,
                    ZoneEncryptionInfoProto::mut_suite_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CryptoProtocolVersionProto>>(
                    "cryptoProtocolVersion",
                    ZoneEncryptionInfoProto::get_cryptoProtocolVersion_for_reflect,
                    ZoneEncryptionInfoProto::mut_cryptoProtocolVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "keyName",
                    ZoneEncryptionInfoProto::get_keyName_for_reflect,
                    ZoneEncryptionInfoProto::mut_keyName_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ZoneEncryptionInfoProto>(
                    "ZoneEncryptionInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ZoneEncryptionInfoProto {
    fn clear(&mut self) {
        self.clear_suite();
        self.clear_cryptoProtocolVersion();
        self.clear_keyName();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ZoneEncryptionInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ZoneEncryptionInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CipherOptionProto {
    // message fields
    suite: ::std::option::Option<CipherSuiteProto>,
    inKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    inIv: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    outKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    outIv: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CipherOptionProto {}

impl CipherOptionProto {
    pub fn new() -> CipherOptionProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CipherOptionProto {
        static mut instance: ::protobuf::lazy::Lazy<CipherOptionProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CipherOptionProto,
        };
        unsafe {
            instance.get(CipherOptionProto::new)
        }
    }

    // required .hadoop.hdfs.CipherSuiteProto suite = 1;

    pub fn clear_suite(&mut self) {
        self.suite = ::std::option::Option::None;
    }

    pub fn has_suite(&self) -> bool {
        self.suite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suite(&mut self, v: CipherSuiteProto) {
        self.suite = ::std::option::Option::Some(v);
    }

    pub fn get_suite(&self) -> CipherSuiteProto {
        self.suite.unwrap_or(CipherSuiteProto::UNKNOWN)
    }

    fn get_suite_for_reflect(&self) -> &::std::option::Option<CipherSuiteProto> {
        &self.suite
    }

    fn mut_suite_for_reflect(&mut self) -> &mut ::std::option::Option<CipherSuiteProto> {
        &mut self.suite
    }

    // optional bytes inKey = 2;

    pub fn clear_inKey(&mut self) {
        self.inKey.clear();
    }

    pub fn has_inKey(&self) -> bool {
        self.inKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.inKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.inKey.is_none() {
            self.inKey.set_default();
        }
        self.inKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_inKey(&mut self) -> ::std::vec::Vec<u8> {
        self.inKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_inKey(&self) -> &[u8] {
        match self.inKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_inKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.inKey
    }

    fn mut_inKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.inKey
    }

    // optional bytes inIv = 3;

    pub fn clear_inIv(&mut self) {
        self.inIv.clear();
    }

    pub fn has_inIv(&self) -> bool {
        self.inIv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inIv(&mut self, v: ::std::vec::Vec<u8>) {
        self.inIv = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inIv(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.inIv.is_none() {
            self.inIv.set_default();
        }
        self.inIv.as_mut().unwrap()
    }

    // Take field
    pub fn take_inIv(&mut self) -> ::std::vec::Vec<u8> {
        self.inIv.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_inIv(&self) -> &[u8] {
        match self.inIv.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_inIv_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.inIv
    }

    fn mut_inIv_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.inIv
    }

    // optional bytes outKey = 4;

    pub fn clear_outKey(&mut self) {
        self.outKey.clear();
    }

    pub fn has_outKey(&self) -> bool {
        self.outKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_outKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.outKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_outKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.outKey.is_none() {
            self.outKey.set_default();
        }
        self.outKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_outKey(&mut self) -> ::std::vec::Vec<u8> {
        self.outKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_outKey(&self) -> &[u8] {
        match self.outKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_outKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.outKey
    }

    fn mut_outKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.outKey
    }

    // optional bytes outIv = 5;

    pub fn clear_outIv(&mut self) {
        self.outIv.clear();
    }

    pub fn has_outIv(&self) -> bool {
        self.outIv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_outIv(&mut self, v: ::std::vec::Vec<u8>) {
        self.outIv = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_outIv(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.outIv.is_none() {
            self.outIv.set_default();
        }
        self.outIv.as_mut().unwrap()
    }

    // Take field
    pub fn take_outIv(&mut self) -> ::std::vec::Vec<u8> {
        self.outIv.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_outIv(&self) -> &[u8] {
        match self.outIv.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_outIv_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.outIv
    }

    fn mut_outIv_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.outIv
    }
}

impl ::protobuf::Message for CipherOptionProto {
    fn is_initialized(&self) -> bool {
        if self.suite.is_none() {
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
                    let tmp = is.read_enum()?;
                    self.suite = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.inKey)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.inIv)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.outKey)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.outIv)?;
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
        if let Some(v) = self.suite {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.inKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(ref v) = self.inIv.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        if let Some(ref v) = self.outKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        if let Some(ref v) = self.outIv.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.suite {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.inKey.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(ref v) = self.inIv.as_ref() {
            os.write_bytes(3, &v)?;
        }
        if let Some(ref v) = self.outKey.as_ref() {
            os.write_bytes(4, &v)?;
        }
        if let Some(ref v) = self.outIv.as_ref() {
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

impl ::protobuf::MessageStatic for CipherOptionProto {
    fn new() -> CipherOptionProto {
        CipherOptionProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CipherOptionProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CipherSuiteProto>>(
                    "suite",
                    CipherOptionProto::get_suite_for_reflect,
                    CipherOptionProto::mut_suite_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "inKey",
                    CipherOptionProto::get_inKey_for_reflect,
                    CipherOptionProto::mut_inKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "inIv",
                    CipherOptionProto::get_inIv_for_reflect,
                    CipherOptionProto::mut_inIv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "outKey",
                    CipherOptionProto::get_outKey_for_reflect,
                    CipherOptionProto::mut_outKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "outIv",
                    CipherOptionProto::get_outIv_for_reflect,
                    CipherOptionProto::mut_outIv_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CipherOptionProto>(
                    "CipherOptionProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CipherOptionProto {
    fn clear(&mut self) {
        self.clear_suite();
        self.clear_inKey();
        self.clear_inIv();
        self.clear_outKey();
        self.clear_outIv();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CipherOptionProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CipherOptionProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LocatedBlocksProto {
    // message fields
    fileLength: ::std::option::Option<u64>,
    blocks: ::protobuf::RepeatedField<LocatedBlockProto>,
    underConstruction: ::std::option::Option<bool>,
    lastBlock: ::protobuf::SingularPtrField<LocatedBlockProto>,
    isLastBlockComplete: ::std::option::Option<bool>,
    fileEncryptionInfo: ::protobuf::SingularPtrField<FileEncryptionInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LocatedBlocksProto {}

impl LocatedBlocksProto {
    pub fn new() -> LocatedBlocksProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LocatedBlocksProto {
        static mut instance: ::protobuf::lazy::Lazy<LocatedBlocksProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LocatedBlocksProto,
        };
        unsafe {
            instance.get(LocatedBlocksProto::new)
        }
    }

    // required uint64 fileLength = 1;

    pub fn clear_fileLength(&mut self) {
        self.fileLength = ::std::option::Option::None;
    }

    pub fn has_fileLength(&self) -> bool {
        self.fileLength.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileLength(&mut self, v: u64) {
        self.fileLength = ::std::option::Option::Some(v);
    }

    pub fn get_fileLength(&self) -> u64 {
        self.fileLength.unwrap_or(0)
    }

    fn get_fileLength_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.fileLength
    }

    fn mut_fileLength_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.fileLength
    }

    // repeated .hadoop.hdfs.LocatedBlockProto blocks = 2;

    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    // Param is passed by value, moved
    pub fn set_blocks(&mut self, v: ::protobuf::RepeatedField<LocatedBlockProto>) {
        self.blocks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blocks(&mut self) -> &mut ::protobuf::RepeatedField<LocatedBlockProto> {
        &mut self.blocks
    }

    // Take field
    pub fn take_blocks(&mut self) -> ::protobuf::RepeatedField<LocatedBlockProto> {
        ::std::mem::replace(&mut self.blocks, ::protobuf::RepeatedField::new())
    }

    pub fn get_blocks(&self) -> &[LocatedBlockProto] {
        &self.blocks
    }

    fn get_blocks_for_reflect(&self) -> &::protobuf::RepeatedField<LocatedBlockProto> {
        &self.blocks
    }

    fn mut_blocks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<LocatedBlockProto> {
        &mut self.blocks
    }

    // required bool underConstruction = 3;

    pub fn clear_underConstruction(&mut self) {
        self.underConstruction = ::std::option::Option::None;
    }

    pub fn has_underConstruction(&self) -> bool {
        self.underConstruction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_underConstruction(&mut self, v: bool) {
        self.underConstruction = ::std::option::Option::Some(v);
    }

    pub fn get_underConstruction(&self) -> bool {
        self.underConstruction.unwrap_or(false)
    }

    fn get_underConstruction_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.underConstruction
    }

    fn mut_underConstruction_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.underConstruction
    }

    // optional .hadoop.hdfs.LocatedBlockProto lastBlock = 4;

    pub fn clear_lastBlock(&mut self) {
        self.lastBlock.clear();
    }

    pub fn has_lastBlock(&self) -> bool {
        self.lastBlock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastBlock(&mut self, v: LocatedBlockProto) {
        self.lastBlock = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lastBlock(&mut self) -> &mut LocatedBlockProto {
        if self.lastBlock.is_none() {
            self.lastBlock.set_default();
        }
        self.lastBlock.as_mut().unwrap()
    }

    // Take field
    pub fn take_lastBlock(&mut self) -> LocatedBlockProto {
        self.lastBlock.take().unwrap_or_else(|| LocatedBlockProto::new())
    }

    pub fn get_lastBlock(&self) -> &LocatedBlockProto {
        self.lastBlock.as_ref().unwrap_or_else(|| LocatedBlockProto::default_instance())
    }

    fn get_lastBlock_for_reflect(&self) -> &::protobuf::SingularPtrField<LocatedBlockProto> {
        &self.lastBlock
    }

    fn mut_lastBlock_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LocatedBlockProto> {
        &mut self.lastBlock
    }

    // required bool isLastBlockComplete = 5;

    pub fn clear_isLastBlockComplete(&mut self) {
        self.isLastBlockComplete = ::std::option::Option::None;
    }

    pub fn has_isLastBlockComplete(&self) -> bool {
        self.isLastBlockComplete.is_some()
    }

    // Param is passed by value, moved
    pub fn set_isLastBlockComplete(&mut self, v: bool) {
        self.isLastBlockComplete = ::std::option::Option::Some(v);
    }

    pub fn get_isLastBlockComplete(&self) -> bool {
        self.isLastBlockComplete.unwrap_or(false)
    }

    fn get_isLastBlockComplete_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.isLastBlockComplete
    }

    fn mut_isLastBlockComplete_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.isLastBlockComplete
    }

    // optional .hadoop.hdfs.FileEncryptionInfoProto fileEncryptionInfo = 6;

    pub fn clear_fileEncryptionInfo(&mut self) {
        self.fileEncryptionInfo.clear();
    }

    pub fn has_fileEncryptionInfo(&self) -> bool {
        self.fileEncryptionInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileEncryptionInfo(&mut self, v: FileEncryptionInfoProto) {
        self.fileEncryptionInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fileEncryptionInfo(&mut self) -> &mut FileEncryptionInfoProto {
        if self.fileEncryptionInfo.is_none() {
            self.fileEncryptionInfo.set_default();
        }
        self.fileEncryptionInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_fileEncryptionInfo(&mut self) -> FileEncryptionInfoProto {
        self.fileEncryptionInfo.take().unwrap_or_else(|| FileEncryptionInfoProto::new())
    }

    pub fn get_fileEncryptionInfo(&self) -> &FileEncryptionInfoProto {
        self.fileEncryptionInfo.as_ref().unwrap_or_else(|| FileEncryptionInfoProto::default_instance())
    }

    fn get_fileEncryptionInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<FileEncryptionInfoProto> {
        &self.fileEncryptionInfo
    }

    fn mut_fileEncryptionInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FileEncryptionInfoProto> {
        &mut self.fileEncryptionInfo
    }
}

impl ::protobuf::Message for LocatedBlocksProto {
    fn is_initialized(&self) -> bool {
        if self.fileLength.is_none() {
            return false;
        }
        if self.underConstruction.is_none() {
            return false;
        }
        if self.isLastBlockComplete.is_none() {
            return false;
        }
        for v in &self.blocks {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.lastBlock {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.fileEncryptionInfo {
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
                    let tmp = is.read_uint64()?;
                    self.fileLength = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.blocks)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.underConstruction = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lastBlock)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.isLastBlockComplete = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fileEncryptionInfo)?;
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
        if let Some(v) = self.fileLength {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.blocks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.underConstruction {
            my_size += 2;
        }
        if let Some(ref v) = self.lastBlock.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.isLastBlockComplete {
            my_size += 2;
        }
        if let Some(ref v) = self.fileEncryptionInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fileLength {
            os.write_uint64(1, v)?;
        }
        for v in &self.blocks {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.underConstruction {
            os.write_bool(3, v)?;
        }
        if let Some(ref v) = self.lastBlock.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.isLastBlockComplete {
            os.write_bool(5, v)?;
        }
        if let Some(ref v) = self.fileEncryptionInfo.as_ref() {
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

impl ::protobuf::MessageStatic for LocatedBlocksProto {
    fn new() -> LocatedBlocksProto {
        LocatedBlocksProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<LocatedBlocksProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "fileLength",
                    LocatedBlocksProto::get_fileLength_for_reflect,
                    LocatedBlocksProto::mut_fileLength_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LocatedBlockProto>>(
                    "blocks",
                    LocatedBlocksProto::get_blocks_for_reflect,
                    LocatedBlocksProto::mut_blocks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "underConstruction",
                    LocatedBlocksProto::get_underConstruction_for_reflect,
                    LocatedBlocksProto::mut_underConstruction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LocatedBlockProto>>(
                    "lastBlock",
                    LocatedBlocksProto::get_lastBlock_for_reflect,
                    LocatedBlocksProto::mut_lastBlock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "isLastBlockComplete",
                    LocatedBlocksProto::get_isLastBlockComplete_for_reflect,
                    LocatedBlocksProto::mut_isLastBlockComplete_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FileEncryptionInfoProto>>(
                    "fileEncryptionInfo",
                    LocatedBlocksProto::get_fileEncryptionInfo_for_reflect,
                    LocatedBlocksProto::mut_fileEncryptionInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LocatedBlocksProto>(
                    "LocatedBlocksProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LocatedBlocksProto {
    fn clear(&mut self) {
        self.clear_fileLength();
        self.clear_blocks();
        self.clear_underConstruction();
        self.clear_lastBlock();
        self.clear_isLastBlockComplete();
        self.clear_fileEncryptionInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LocatedBlocksProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LocatedBlocksProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HdfsFileStatusProto {
    // message fields
    fileType: ::std::option::Option<HdfsFileStatusProto_FileType>,
    path: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    length: ::std::option::Option<u64>,
    permission: ::protobuf::SingularPtrField<FsPermissionProto>,
    owner: ::protobuf::SingularField<::std::string::String>,
    group: ::protobuf::SingularField<::std::string::String>,
    modification_time: ::std::option::Option<u64>,
    access_time: ::std::option::Option<u64>,
    symlink: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    block_replication: ::std::option::Option<u32>,
    blocksize: ::std::option::Option<u64>,
    locations: ::protobuf::SingularPtrField<LocatedBlocksProto>,
    fileId: ::std::option::Option<u64>,
    childrenNum: ::std::option::Option<i32>,
    fileEncryptionInfo: ::protobuf::SingularPtrField<FileEncryptionInfoProto>,
    storagePolicy: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HdfsFileStatusProto {}

impl HdfsFileStatusProto {
    pub fn new() -> HdfsFileStatusProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HdfsFileStatusProto {
        static mut instance: ::protobuf::lazy::Lazy<HdfsFileStatusProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HdfsFileStatusProto,
        };
        unsafe {
            instance.get(HdfsFileStatusProto::new)
        }
    }

    // required .hadoop.hdfs.HdfsFileStatusProto.FileType fileType = 1;

    pub fn clear_fileType(&mut self) {
        self.fileType = ::std::option::Option::None;
    }

    pub fn has_fileType(&self) -> bool {
        self.fileType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileType(&mut self, v: HdfsFileStatusProto_FileType) {
        self.fileType = ::std::option::Option::Some(v);
    }

    pub fn get_fileType(&self) -> HdfsFileStatusProto_FileType {
        self.fileType.unwrap_or(HdfsFileStatusProto_FileType::IS_DIR)
    }

    fn get_fileType_for_reflect(&self) -> &::std::option::Option<HdfsFileStatusProto_FileType> {
        &self.fileType
    }

    fn mut_fileType_for_reflect(&mut self) -> &mut ::std::option::Option<HdfsFileStatusProto_FileType> {
        &mut self.fileType
    }

    // required bytes path = 2;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::vec::Vec<u8>) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.path.is_none() {
            self.path.set_default();
        }
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::vec::Vec<u8> {
        self.path.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_path(&self) -> &[u8] {
        match self.path.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.path
    }

    // required uint64 length = 3;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u64) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length(&self) -> u64 {
        self.length.unwrap_or(0)
    }

    fn get_length_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.length
    }

    // required .hadoop.hdfs.FsPermissionProto permission = 4;

    pub fn clear_permission(&mut self) {
        self.permission.clear();
    }

    pub fn has_permission(&self) -> bool {
        self.permission.is_some()
    }

    // Param is passed by value, moved
    pub fn set_permission(&mut self, v: FsPermissionProto) {
        self.permission = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_permission(&mut self) -> &mut FsPermissionProto {
        if self.permission.is_none() {
            self.permission.set_default();
        }
        self.permission.as_mut().unwrap()
    }

    // Take field
    pub fn take_permission(&mut self) -> FsPermissionProto {
        self.permission.take().unwrap_or_else(|| FsPermissionProto::new())
    }

    pub fn get_permission(&self) -> &FsPermissionProto {
        self.permission.as_ref().unwrap_or_else(|| FsPermissionProto::default_instance())
    }

    fn get_permission_for_reflect(&self) -> &::protobuf::SingularPtrField<FsPermissionProto> {
        &self.permission
    }

    fn mut_permission_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FsPermissionProto> {
        &mut self.permission
    }

    // required string owner = 5;

    pub fn clear_owner(&mut self) {
        self.owner.clear();
    }

    pub fn has_owner(&self) -> bool {
        self.owner.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner(&mut self, v: ::std::string::String) {
        self.owner = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_owner(&mut self) -> &mut ::std::string::String {
        if self.owner.is_none() {
            self.owner.set_default();
        }
        self.owner.as_mut().unwrap()
    }

    // Take field
    pub fn take_owner(&mut self) -> ::std::string::String {
        self.owner.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_owner(&self) -> &str {
        match self.owner.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_owner_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.owner
    }

    fn mut_owner_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.owner
    }

    // required string group = 6;

    pub fn clear_group(&mut self) {
        self.group.clear();
    }

    pub fn has_group(&self) -> bool {
        self.group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group(&mut self, v: ::std::string::String) {
        self.group = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group(&mut self) -> &mut ::std::string::String {
        if self.group.is_none() {
            self.group.set_default();
        }
        self.group.as_mut().unwrap()
    }

    // Take field
    pub fn take_group(&mut self) -> ::std::string::String {
        self.group.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_group(&self) -> &str {
        match self.group.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_group_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.group
    }

    fn mut_group_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.group
    }

    // required uint64 modification_time = 7;

    pub fn clear_modification_time(&mut self) {
        self.modification_time = ::std::option::Option::None;
    }

    pub fn has_modification_time(&self) -> bool {
        self.modification_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modification_time(&mut self, v: u64) {
        self.modification_time = ::std::option::Option::Some(v);
    }

    pub fn get_modification_time(&self) -> u64 {
        self.modification_time.unwrap_or(0)
    }

    fn get_modification_time_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.modification_time
    }

    fn mut_modification_time_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.modification_time
    }

    // required uint64 access_time = 8;

    pub fn clear_access_time(&mut self) {
        self.access_time = ::std::option::Option::None;
    }

    pub fn has_access_time(&self) -> bool {
        self.access_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_access_time(&mut self, v: u64) {
        self.access_time = ::std::option::Option::Some(v);
    }

    pub fn get_access_time(&self) -> u64 {
        self.access_time.unwrap_or(0)
    }

    fn get_access_time_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.access_time
    }

    fn mut_access_time_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.access_time
    }

    // optional bytes symlink = 9;

    pub fn clear_symlink(&mut self) {
        self.symlink.clear();
    }

    pub fn has_symlink(&self) -> bool {
        self.symlink.is_some()
    }

    // Param is passed by value, moved
    pub fn set_symlink(&mut self, v: ::std::vec::Vec<u8>) {
        self.symlink = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_symlink(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.symlink.is_none() {
            self.symlink.set_default();
        }
        self.symlink.as_mut().unwrap()
    }

    // Take field
    pub fn take_symlink(&mut self) -> ::std::vec::Vec<u8> {
        self.symlink.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_symlink(&self) -> &[u8] {
        match self.symlink.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_symlink_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.symlink
    }

    fn mut_symlink_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.symlink
    }

    // optional uint32 block_replication = 10;

    pub fn clear_block_replication(&mut self) {
        self.block_replication = ::std::option::Option::None;
    }

    pub fn has_block_replication(&self) -> bool {
        self.block_replication.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block_replication(&mut self, v: u32) {
        self.block_replication = ::std::option::Option::Some(v);
    }

    pub fn get_block_replication(&self) -> u32 {
        self.block_replication.unwrap_or(0u32)
    }

    fn get_block_replication_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.block_replication
    }

    fn mut_block_replication_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.block_replication
    }

    // optional uint64 blocksize = 11;

    pub fn clear_blocksize(&mut self) {
        self.blocksize = ::std::option::Option::None;
    }

    pub fn has_blocksize(&self) -> bool {
        self.blocksize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blocksize(&mut self, v: u64) {
        self.blocksize = ::std::option::Option::Some(v);
    }

    pub fn get_blocksize(&self) -> u64 {
        self.blocksize.unwrap_or(0u64)
    }

    fn get_blocksize_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.blocksize
    }

    fn mut_blocksize_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.blocksize
    }

    // optional .hadoop.hdfs.LocatedBlocksProto locations = 12;

    pub fn clear_locations(&mut self) {
        self.locations.clear();
    }

    pub fn has_locations(&self) -> bool {
        self.locations.is_some()
    }

    // Param is passed by value, moved
    pub fn set_locations(&mut self, v: LocatedBlocksProto) {
        self.locations = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_locations(&mut self) -> &mut LocatedBlocksProto {
        if self.locations.is_none() {
            self.locations.set_default();
        }
        self.locations.as_mut().unwrap()
    }

    // Take field
    pub fn take_locations(&mut self) -> LocatedBlocksProto {
        self.locations.take().unwrap_or_else(|| LocatedBlocksProto::new())
    }

    pub fn get_locations(&self) -> &LocatedBlocksProto {
        self.locations.as_ref().unwrap_or_else(|| LocatedBlocksProto::default_instance())
    }

    fn get_locations_for_reflect(&self) -> &::protobuf::SingularPtrField<LocatedBlocksProto> {
        &self.locations
    }

    fn mut_locations_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LocatedBlocksProto> {
        &mut self.locations
    }

    // optional uint64 fileId = 13;

    pub fn clear_fileId(&mut self) {
        self.fileId = ::std::option::Option::None;
    }

    pub fn has_fileId(&self) -> bool {
        self.fileId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileId(&mut self, v: u64) {
        self.fileId = ::std::option::Option::Some(v);
    }

    pub fn get_fileId(&self) -> u64 {
        self.fileId.unwrap_or(0u64)
    }

    fn get_fileId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.fileId
    }

    fn mut_fileId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.fileId
    }

    // optional int32 childrenNum = 14;

    pub fn clear_childrenNum(&mut self) {
        self.childrenNum = ::std::option::Option::None;
    }

    pub fn has_childrenNum(&self) -> bool {
        self.childrenNum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_childrenNum(&mut self, v: i32) {
        self.childrenNum = ::std::option::Option::Some(v);
    }

    pub fn get_childrenNum(&self) -> i32 {
        self.childrenNum.unwrap_or(-1i32)
    }

    fn get_childrenNum_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.childrenNum
    }

    fn mut_childrenNum_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.childrenNum
    }

    // optional .hadoop.hdfs.FileEncryptionInfoProto fileEncryptionInfo = 15;

    pub fn clear_fileEncryptionInfo(&mut self) {
        self.fileEncryptionInfo.clear();
    }

    pub fn has_fileEncryptionInfo(&self) -> bool {
        self.fileEncryptionInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileEncryptionInfo(&mut self, v: FileEncryptionInfoProto) {
        self.fileEncryptionInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fileEncryptionInfo(&mut self) -> &mut FileEncryptionInfoProto {
        if self.fileEncryptionInfo.is_none() {
            self.fileEncryptionInfo.set_default();
        }
        self.fileEncryptionInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_fileEncryptionInfo(&mut self) -> FileEncryptionInfoProto {
        self.fileEncryptionInfo.take().unwrap_or_else(|| FileEncryptionInfoProto::new())
    }

    pub fn get_fileEncryptionInfo(&self) -> &FileEncryptionInfoProto {
        self.fileEncryptionInfo.as_ref().unwrap_or_else(|| FileEncryptionInfoProto::default_instance())
    }

    fn get_fileEncryptionInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<FileEncryptionInfoProto> {
        &self.fileEncryptionInfo
    }

    fn mut_fileEncryptionInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FileEncryptionInfoProto> {
        &mut self.fileEncryptionInfo
    }

    // optional uint32 storagePolicy = 16;

    pub fn clear_storagePolicy(&mut self) {
        self.storagePolicy = ::std::option::Option::None;
    }

    pub fn has_storagePolicy(&self) -> bool {
        self.storagePolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storagePolicy(&mut self, v: u32) {
        self.storagePolicy = ::std::option::Option::Some(v);
    }

    pub fn get_storagePolicy(&self) -> u32 {
        self.storagePolicy.unwrap_or(0u32)
    }

    fn get_storagePolicy_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.storagePolicy
    }

    fn mut_storagePolicy_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.storagePolicy
    }
}

impl ::protobuf::Message for HdfsFileStatusProto {
    fn is_initialized(&self) -> bool {
        if self.fileType.is_none() {
            return false;
        }
        if self.path.is_none() {
            return false;
        }
        if self.length.is_none() {
            return false;
        }
        if self.permission.is_none() {
            return false;
        }
        if self.owner.is_none() {
            return false;
        }
        if self.group.is_none() {
            return false;
        }
        if self.modification_time.is_none() {
            return false;
        }
        if self.access_time.is_none() {
            return false;
        }
        for v in &self.permission {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.locations {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.fileEncryptionInfo {
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
                    self.fileType = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.path)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.length = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.permission)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.owner)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.group)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.modification_time = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.access_time = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.symlink)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.block_replication = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.blocksize = ::std::option::Option::Some(tmp);
                },
                12 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.locations)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.fileId = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.childrenNum = ::std::option::Option::Some(tmp);
                },
                15 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fileEncryptionInfo)?;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.storagePolicy = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.fileType {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.path.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.permission.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.owner.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.group.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(v) = self.modification_time {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.access_time {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.symlink.as_ref() {
            my_size += ::protobuf::rt::bytes_size(9, &v);
        }
        if let Some(v) = self.block_replication {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.blocksize {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.locations.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.fileId {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.childrenNum {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.fileEncryptionInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.storagePolicy {
            my_size += ::protobuf::rt::value_size(16, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fileType {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.path.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(v) = self.length {
            os.write_uint64(3, v)?;
        }
        if let Some(ref v) = self.permission.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.owner.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.group.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(v) = self.modification_time {
            os.write_uint64(7, v)?;
        }
        if let Some(v) = self.access_time {
            os.write_uint64(8, v)?;
        }
        if let Some(ref v) = self.symlink.as_ref() {
            os.write_bytes(9, &v)?;
        }
        if let Some(v) = self.block_replication {
            os.write_uint32(10, v)?;
        }
        if let Some(v) = self.blocksize {
            os.write_uint64(11, v)?;
        }
        if let Some(ref v) = self.locations.as_ref() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.fileId {
            os.write_uint64(13, v)?;
        }
        if let Some(v) = self.childrenNum {
            os.write_int32(14, v)?;
        }
        if let Some(ref v) = self.fileEncryptionInfo.as_ref() {
            os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.storagePolicy {
            os.write_uint32(16, v)?;
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

impl ::protobuf::MessageStatic for HdfsFileStatusProto {
    fn new() -> HdfsFileStatusProto {
        HdfsFileStatusProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<HdfsFileStatusProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<HdfsFileStatusProto_FileType>>(
                    "fileType",
                    HdfsFileStatusProto::get_fileType_for_reflect,
                    HdfsFileStatusProto::mut_fileType_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "path",
                    HdfsFileStatusProto::get_path_for_reflect,
                    HdfsFileStatusProto::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "length",
                    HdfsFileStatusProto::get_length_for_reflect,
                    HdfsFileStatusProto::mut_length_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FsPermissionProto>>(
                    "permission",
                    HdfsFileStatusProto::get_permission_for_reflect,
                    HdfsFileStatusProto::mut_permission_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "owner",
                    HdfsFileStatusProto::get_owner_for_reflect,
                    HdfsFileStatusProto::mut_owner_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "group",
                    HdfsFileStatusProto::get_group_for_reflect,
                    HdfsFileStatusProto::mut_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "modification_time",
                    HdfsFileStatusProto::get_modification_time_for_reflect,
                    HdfsFileStatusProto::mut_modification_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "access_time",
                    HdfsFileStatusProto::get_access_time_for_reflect,
                    HdfsFileStatusProto::mut_access_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "symlink",
                    HdfsFileStatusProto::get_symlink_for_reflect,
                    HdfsFileStatusProto::mut_symlink_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "block_replication",
                    HdfsFileStatusProto::get_block_replication_for_reflect,
                    HdfsFileStatusProto::mut_block_replication_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "blocksize",
                    HdfsFileStatusProto::get_blocksize_for_reflect,
                    HdfsFileStatusProto::mut_blocksize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LocatedBlocksProto>>(
                    "locations",
                    HdfsFileStatusProto::get_locations_for_reflect,
                    HdfsFileStatusProto::mut_locations_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "fileId",
                    HdfsFileStatusProto::get_fileId_for_reflect,
                    HdfsFileStatusProto::mut_fileId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "childrenNum",
                    HdfsFileStatusProto::get_childrenNum_for_reflect,
                    HdfsFileStatusProto::mut_childrenNum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FileEncryptionInfoProto>>(
                    "fileEncryptionInfo",
                    HdfsFileStatusProto::get_fileEncryptionInfo_for_reflect,
                    HdfsFileStatusProto::mut_fileEncryptionInfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "storagePolicy",
                    HdfsFileStatusProto::get_storagePolicy_for_reflect,
                    HdfsFileStatusProto::mut_storagePolicy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HdfsFileStatusProto>(
                    "HdfsFileStatusProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HdfsFileStatusProto {
    fn clear(&mut self) {
        self.clear_fileType();
        self.clear_path();
        self.clear_length();
        self.clear_permission();
        self.clear_owner();
        self.clear_group();
        self.clear_modification_time();
        self.clear_access_time();
        self.clear_symlink();
        self.clear_block_replication();
        self.clear_blocksize();
        self.clear_locations();
        self.clear_fileId();
        self.clear_childrenNum();
        self.clear_fileEncryptionInfo();
        self.clear_storagePolicy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HdfsFileStatusProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HdfsFileStatusProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum HdfsFileStatusProto_FileType {
    IS_DIR = 1,
    IS_FILE = 2,
    IS_SYMLINK = 3,
}

impl ::protobuf::ProtobufEnum for HdfsFileStatusProto_FileType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<HdfsFileStatusProto_FileType> {
        match value {
            1 => ::std::option::Option::Some(HdfsFileStatusProto_FileType::IS_DIR),
            2 => ::std::option::Option::Some(HdfsFileStatusProto_FileType::IS_FILE),
            3 => ::std::option::Option::Some(HdfsFileStatusProto_FileType::IS_SYMLINK),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [HdfsFileStatusProto_FileType] = &[
            HdfsFileStatusProto_FileType::IS_DIR,
            HdfsFileStatusProto_FileType::IS_FILE,
            HdfsFileStatusProto_FileType::IS_SYMLINK,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<HdfsFileStatusProto_FileType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("HdfsFileStatusProto_FileType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for HdfsFileStatusProto_FileType {
}

impl ::protobuf::reflect::ProtobufValue for HdfsFileStatusProto_FileType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FsServerDefaultsProto {
    // message fields
    blockSize: ::std::option::Option<u64>,
    bytesPerChecksum: ::std::option::Option<u32>,
    writePacketSize: ::std::option::Option<u32>,
    replication: ::std::option::Option<u32>,
    fileBufferSize: ::std::option::Option<u32>,
    encryptDataTransfer: ::std::option::Option<bool>,
    trashInterval: ::std::option::Option<u64>,
    checksumType: ::std::option::Option<ChecksumTypeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FsServerDefaultsProto {}

impl FsServerDefaultsProto {
    pub fn new() -> FsServerDefaultsProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FsServerDefaultsProto {
        static mut instance: ::protobuf::lazy::Lazy<FsServerDefaultsProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FsServerDefaultsProto,
        };
        unsafe {
            instance.get(FsServerDefaultsProto::new)
        }
    }

    // required uint64 blockSize = 1;

    pub fn clear_blockSize(&mut self) {
        self.blockSize = ::std::option::Option::None;
    }

    pub fn has_blockSize(&self) -> bool {
        self.blockSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockSize(&mut self, v: u64) {
        self.blockSize = ::std::option::Option::Some(v);
    }

    pub fn get_blockSize(&self) -> u64 {
        self.blockSize.unwrap_or(0)
    }

    fn get_blockSize_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.blockSize
    }

    fn mut_blockSize_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.blockSize
    }

    // required uint32 bytesPerChecksum = 2;

    pub fn clear_bytesPerChecksum(&mut self) {
        self.bytesPerChecksum = ::std::option::Option::None;
    }

    pub fn has_bytesPerChecksum(&self) -> bool {
        self.bytesPerChecksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytesPerChecksum(&mut self, v: u32) {
        self.bytesPerChecksum = ::std::option::Option::Some(v);
    }

    pub fn get_bytesPerChecksum(&self) -> u32 {
        self.bytesPerChecksum.unwrap_or(0)
    }

    fn get_bytesPerChecksum_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.bytesPerChecksum
    }

    fn mut_bytesPerChecksum_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.bytesPerChecksum
    }

    // required uint32 writePacketSize = 3;

    pub fn clear_writePacketSize(&mut self) {
        self.writePacketSize = ::std::option::Option::None;
    }

    pub fn has_writePacketSize(&self) -> bool {
        self.writePacketSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_writePacketSize(&mut self, v: u32) {
        self.writePacketSize = ::std::option::Option::Some(v);
    }

    pub fn get_writePacketSize(&self) -> u32 {
        self.writePacketSize.unwrap_or(0)
    }

    fn get_writePacketSize_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.writePacketSize
    }

    fn mut_writePacketSize_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.writePacketSize
    }

    // required uint32 replication = 4;

    pub fn clear_replication(&mut self) {
        self.replication = ::std::option::Option::None;
    }

    pub fn has_replication(&self) -> bool {
        self.replication.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replication(&mut self, v: u32) {
        self.replication = ::std::option::Option::Some(v);
    }

    pub fn get_replication(&self) -> u32 {
        self.replication.unwrap_or(0)
    }

    fn get_replication_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.replication
    }

    fn mut_replication_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.replication
    }

    // required uint32 fileBufferSize = 5;

    pub fn clear_fileBufferSize(&mut self) {
        self.fileBufferSize = ::std::option::Option::None;
    }

    pub fn has_fileBufferSize(&self) -> bool {
        self.fileBufferSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileBufferSize(&mut self, v: u32) {
        self.fileBufferSize = ::std::option::Option::Some(v);
    }

    pub fn get_fileBufferSize(&self) -> u32 {
        self.fileBufferSize.unwrap_or(0)
    }

    fn get_fileBufferSize_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.fileBufferSize
    }

    fn mut_fileBufferSize_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.fileBufferSize
    }

    // optional bool encryptDataTransfer = 6;

    pub fn clear_encryptDataTransfer(&mut self) {
        self.encryptDataTransfer = ::std::option::Option::None;
    }

    pub fn has_encryptDataTransfer(&self) -> bool {
        self.encryptDataTransfer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encryptDataTransfer(&mut self, v: bool) {
        self.encryptDataTransfer = ::std::option::Option::Some(v);
    }

    pub fn get_encryptDataTransfer(&self) -> bool {
        self.encryptDataTransfer.unwrap_or(false)
    }

    fn get_encryptDataTransfer_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.encryptDataTransfer
    }

    fn mut_encryptDataTransfer_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.encryptDataTransfer
    }

    // optional uint64 trashInterval = 7;

    pub fn clear_trashInterval(&mut self) {
        self.trashInterval = ::std::option::Option::None;
    }

    pub fn has_trashInterval(&self) -> bool {
        self.trashInterval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trashInterval(&mut self, v: u64) {
        self.trashInterval = ::std::option::Option::Some(v);
    }

    pub fn get_trashInterval(&self) -> u64 {
        self.trashInterval.unwrap_or(0u64)
    }

    fn get_trashInterval_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.trashInterval
    }

    fn mut_trashInterval_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.trashInterval
    }

    // optional .hadoop.hdfs.ChecksumTypeProto checksumType = 8;

    pub fn clear_checksumType(&mut self) {
        self.checksumType = ::std::option::Option::None;
    }

    pub fn has_checksumType(&self) -> bool {
        self.checksumType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checksumType(&mut self, v: ChecksumTypeProto) {
        self.checksumType = ::std::option::Option::Some(v);
    }

    pub fn get_checksumType(&self) -> ChecksumTypeProto {
        self.checksumType.unwrap_or(ChecksumTypeProto::CHECKSUM_CRC32)
    }

    fn get_checksumType_for_reflect(&self) -> &::std::option::Option<ChecksumTypeProto> {
        &self.checksumType
    }

    fn mut_checksumType_for_reflect(&mut self) -> &mut ::std::option::Option<ChecksumTypeProto> {
        &mut self.checksumType
    }
}

impl ::protobuf::Message for FsServerDefaultsProto {
    fn is_initialized(&self) -> bool {
        if self.blockSize.is_none() {
            return false;
        }
        if self.bytesPerChecksum.is_none() {
            return false;
        }
        if self.writePacketSize.is_none() {
            return false;
        }
        if self.replication.is_none() {
            return false;
        }
        if self.fileBufferSize.is_none() {
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
                    let tmp = is.read_uint64()?;
                    self.blockSize = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.bytesPerChecksum = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.writePacketSize = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.replication = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.fileBufferSize = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.encryptDataTransfer = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.trashInterval = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.checksumType = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.blockSize {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.bytesPerChecksum {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.writePacketSize {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.replication {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.fileBufferSize {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.encryptDataTransfer {
            my_size += 2;
        }
        if let Some(v) = self.trashInterval {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.checksumType {
            my_size += ::protobuf::rt::enum_size(8, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.blockSize {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.bytesPerChecksum {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.writePacketSize {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.replication {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.fileBufferSize {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.encryptDataTransfer {
            os.write_bool(6, v)?;
        }
        if let Some(v) = self.trashInterval {
            os.write_uint64(7, v)?;
        }
        if let Some(v) = self.checksumType {
            os.write_enum(8, v.value())?;
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

impl ::protobuf::MessageStatic for FsServerDefaultsProto {
    fn new() -> FsServerDefaultsProto {
        FsServerDefaultsProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FsServerDefaultsProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "blockSize",
                    FsServerDefaultsProto::get_blockSize_for_reflect,
                    FsServerDefaultsProto::mut_blockSize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "bytesPerChecksum",
                    FsServerDefaultsProto::get_bytesPerChecksum_for_reflect,
                    FsServerDefaultsProto::mut_bytesPerChecksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "writePacketSize",
                    FsServerDefaultsProto::get_writePacketSize_for_reflect,
                    FsServerDefaultsProto::mut_writePacketSize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "replication",
                    FsServerDefaultsProto::get_replication_for_reflect,
                    FsServerDefaultsProto::mut_replication_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "fileBufferSize",
                    FsServerDefaultsProto::get_fileBufferSize_for_reflect,
                    FsServerDefaultsProto::mut_fileBufferSize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "encryptDataTransfer",
                    FsServerDefaultsProto::get_encryptDataTransfer_for_reflect,
                    FsServerDefaultsProto::mut_encryptDataTransfer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "trashInterval",
                    FsServerDefaultsProto::get_trashInterval_for_reflect,
                    FsServerDefaultsProto::mut_trashInterval_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ChecksumTypeProto>>(
                    "checksumType",
                    FsServerDefaultsProto::get_checksumType_for_reflect,
                    FsServerDefaultsProto::mut_checksumType_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FsServerDefaultsProto>(
                    "FsServerDefaultsProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FsServerDefaultsProto {
    fn clear(&mut self) {
        self.clear_blockSize();
        self.clear_bytesPerChecksum();
        self.clear_writePacketSize();
        self.clear_replication();
        self.clear_fileBufferSize();
        self.clear_encryptDataTransfer();
        self.clear_trashInterval();
        self.clear_checksumType();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FsServerDefaultsProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FsServerDefaultsProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DirectoryListingProto {
    // message fields
    partialListing: ::protobuf::RepeatedField<HdfsFileStatusProto>,
    remainingEntries: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DirectoryListingProto {}

impl DirectoryListingProto {
    pub fn new() -> DirectoryListingProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DirectoryListingProto {
        static mut instance: ::protobuf::lazy::Lazy<DirectoryListingProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DirectoryListingProto,
        };
        unsafe {
            instance.get(DirectoryListingProto::new)
        }
    }

    // repeated .hadoop.hdfs.HdfsFileStatusProto partialListing = 1;

    pub fn clear_partialListing(&mut self) {
        self.partialListing.clear();
    }

    // Param is passed by value, moved
    pub fn set_partialListing(&mut self, v: ::protobuf::RepeatedField<HdfsFileStatusProto>) {
        self.partialListing = v;
    }

    // Mutable pointer to the field.
    pub fn mut_partialListing(&mut self) -> &mut ::protobuf::RepeatedField<HdfsFileStatusProto> {
        &mut self.partialListing
    }

    // Take field
    pub fn take_partialListing(&mut self) -> ::protobuf::RepeatedField<HdfsFileStatusProto> {
        ::std::mem::replace(&mut self.partialListing, ::protobuf::RepeatedField::new())
    }

    pub fn get_partialListing(&self) -> &[HdfsFileStatusProto] {
        &self.partialListing
    }

    fn get_partialListing_for_reflect(&self) -> &::protobuf::RepeatedField<HdfsFileStatusProto> {
        &self.partialListing
    }

    fn mut_partialListing_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<HdfsFileStatusProto> {
        &mut self.partialListing
    }

    // required uint32 remainingEntries = 2;

    pub fn clear_remainingEntries(&mut self) {
        self.remainingEntries = ::std::option::Option::None;
    }

    pub fn has_remainingEntries(&self) -> bool {
        self.remainingEntries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remainingEntries(&mut self, v: u32) {
        self.remainingEntries = ::std::option::Option::Some(v);
    }

    pub fn get_remainingEntries(&self) -> u32 {
        self.remainingEntries.unwrap_or(0)
    }

    fn get_remainingEntries_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.remainingEntries
    }

    fn mut_remainingEntries_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.remainingEntries
    }
}

impl ::protobuf::Message for DirectoryListingProto {
    fn is_initialized(&self) -> bool {
        if self.remainingEntries.is_none() {
            return false;
        }
        for v in &self.partialListing {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.partialListing)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.remainingEntries = ::std::option::Option::Some(tmp);
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
        for value in &self.partialListing {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.remainingEntries {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.partialListing {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.remainingEntries {
            os.write_uint32(2, v)?;
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

impl ::protobuf::MessageStatic for DirectoryListingProto {
    fn new() -> DirectoryListingProto {
        DirectoryListingProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DirectoryListingProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HdfsFileStatusProto>>(
                    "partialListing",
                    DirectoryListingProto::get_partialListing_for_reflect,
                    DirectoryListingProto::mut_partialListing_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "remainingEntries",
                    DirectoryListingProto::get_remainingEntries_for_reflect,
                    DirectoryListingProto::mut_remainingEntries_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DirectoryListingProto>(
                    "DirectoryListingProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DirectoryListingProto {
    fn clear(&mut self) {
        self.clear_partialListing();
        self.clear_remainingEntries();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DirectoryListingProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DirectoryListingProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshottableDirectoryStatusProto {
    // message fields
    dirStatus: ::protobuf::SingularPtrField<HdfsFileStatusProto>,
    snapshot_quota: ::std::option::Option<u32>,
    snapshot_number: ::std::option::Option<u32>,
    parent_fullpath: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshottableDirectoryStatusProto {}

impl SnapshottableDirectoryStatusProto {
    pub fn new() -> SnapshottableDirectoryStatusProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshottableDirectoryStatusProto {
        static mut instance: ::protobuf::lazy::Lazy<SnapshottableDirectoryStatusProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshottableDirectoryStatusProto,
        };
        unsafe {
            instance.get(SnapshottableDirectoryStatusProto::new)
        }
    }

    // required .hadoop.hdfs.HdfsFileStatusProto dirStatus = 1;

    pub fn clear_dirStatus(&mut self) {
        self.dirStatus.clear();
    }

    pub fn has_dirStatus(&self) -> bool {
        self.dirStatus.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dirStatus(&mut self, v: HdfsFileStatusProto) {
        self.dirStatus = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dirStatus(&mut self) -> &mut HdfsFileStatusProto {
        if self.dirStatus.is_none() {
            self.dirStatus.set_default();
        }
        self.dirStatus.as_mut().unwrap()
    }

    // Take field
    pub fn take_dirStatus(&mut self) -> HdfsFileStatusProto {
        self.dirStatus.take().unwrap_or_else(|| HdfsFileStatusProto::new())
    }

    pub fn get_dirStatus(&self) -> &HdfsFileStatusProto {
        self.dirStatus.as_ref().unwrap_or_else(|| HdfsFileStatusProto::default_instance())
    }

    fn get_dirStatus_for_reflect(&self) -> &::protobuf::SingularPtrField<HdfsFileStatusProto> {
        &self.dirStatus
    }

    fn mut_dirStatus_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<HdfsFileStatusProto> {
        &mut self.dirStatus
    }

    // required uint32 snapshot_quota = 2;

    pub fn clear_snapshot_quota(&mut self) {
        self.snapshot_quota = ::std::option::Option::None;
    }

    pub fn has_snapshot_quota(&self) -> bool {
        self.snapshot_quota.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshot_quota(&mut self, v: u32) {
        self.snapshot_quota = ::std::option::Option::Some(v);
    }

    pub fn get_snapshot_quota(&self) -> u32 {
        self.snapshot_quota.unwrap_or(0)
    }

    fn get_snapshot_quota_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.snapshot_quota
    }

    fn mut_snapshot_quota_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.snapshot_quota
    }

    // required uint32 snapshot_number = 3;

    pub fn clear_snapshot_number(&mut self) {
        self.snapshot_number = ::std::option::Option::None;
    }

    pub fn has_snapshot_number(&self) -> bool {
        self.snapshot_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshot_number(&mut self, v: u32) {
        self.snapshot_number = ::std::option::Option::Some(v);
    }

    pub fn get_snapshot_number(&self) -> u32 {
        self.snapshot_number.unwrap_or(0)
    }

    fn get_snapshot_number_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.snapshot_number
    }

    fn mut_snapshot_number_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.snapshot_number
    }

    // required bytes parent_fullpath = 4;

    pub fn clear_parent_fullpath(&mut self) {
        self.parent_fullpath.clear();
    }

    pub fn has_parent_fullpath(&self) -> bool {
        self.parent_fullpath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parent_fullpath(&mut self, v: ::std::vec::Vec<u8>) {
        self.parent_fullpath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_parent_fullpath(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.parent_fullpath.is_none() {
            self.parent_fullpath.set_default();
        }
        self.parent_fullpath.as_mut().unwrap()
    }

    // Take field
    pub fn take_parent_fullpath(&mut self) -> ::std::vec::Vec<u8> {
        self.parent_fullpath.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_parent_fullpath(&self) -> &[u8] {
        match self.parent_fullpath.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_parent_fullpath_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.parent_fullpath
    }

    fn mut_parent_fullpath_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.parent_fullpath
    }
}

impl ::protobuf::Message for SnapshottableDirectoryStatusProto {
    fn is_initialized(&self) -> bool {
        if self.dirStatus.is_none() {
            return false;
        }
        if self.snapshot_quota.is_none() {
            return false;
        }
        if self.snapshot_number.is_none() {
            return false;
        }
        if self.parent_fullpath.is_none() {
            return false;
        }
        for v in &self.dirStatus {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.dirStatus)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.snapshot_quota = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.snapshot_number = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.parent_fullpath)?;
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
        if let Some(ref v) = self.dirStatus.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.snapshot_quota {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.snapshot_number {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.parent_fullpath.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.dirStatus.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.snapshot_quota {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.snapshot_number {
            os.write_uint32(3, v)?;
        }
        if let Some(ref v) = self.parent_fullpath.as_ref() {
            os.write_bytes(4, &v)?;
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

impl ::protobuf::MessageStatic for SnapshottableDirectoryStatusProto {
    fn new() -> SnapshottableDirectoryStatusProto {
        SnapshottableDirectoryStatusProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshottableDirectoryStatusProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HdfsFileStatusProto>>(
                    "dirStatus",
                    SnapshottableDirectoryStatusProto::get_dirStatus_for_reflect,
                    SnapshottableDirectoryStatusProto::mut_dirStatus_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "snapshot_quota",
                    SnapshottableDirectoryStatusProto::get_snapshot_quota_for_reflect,
                    SnapshottableDirectoryStatusProto::mut_snapshot_quota_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "snapshot_number",
                    SnapshottableDirectoryStatusProto::get_snapshot_number_for_reflect,
                    SnapshottableDirectoryStatusProto::mut_snapshot_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "parent_fullpath",
                    SnapshottableDirectoryStatusProto::get_parent_fullpath_for_reflect,
                    SnapshottableDirectoryStatusProto::mut_parent_fullpath_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshottableDirectoryStatusProto>(
                    "SnapshottableDirectoryStatusProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshottableDirectoryStatusProto {
    fn clear(&mut self) {
        self.clear_dirStatus();
        self.clear_snapshot_quota();
        self.clear_snapshot_number();
        self.clear_parent_fullpath();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshottableDirectoryStatusProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshottableDirectoryStatusProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshottableDirectoryListingProto {
    // message fields
    snapshottableDirListing: ::protobuf::RepeatedField<SnapshottableDirectoryStatusProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshottableDirectoryListingProto {}

impl SnapshottableDirectoryListingProto {
    pub fn new() -> SnapshottableDirectoryListingProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshottableDirectoryListingProto {
        static mut instance: ::protobuf::lazy::Lazy<SnapshottableDirectoryListingProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshottableDirectoryListingProto,
        };
        unsafe {
            instance.get(SnapshottableDirectoryListingProto::new)
        }
    }

    // repeated .hadoop.hdfs.SnapshottableDirectoryStatusProto snapshottableDirListing = 1;

    pub fn clear_snapshottableDirListing(&mut self) {
        self.snapshottableDirListing.clear();
    }

    // Param is passed by value, moved
    pub fn set_snapshottableDirListing(&mut self, v: ::protobuf::RepeatedField<SnapshottableDirectoryStatusProto>) {
        self.snapshottableDirListing = v;
    }

    // Mutable pointer to the field.
    pub fn mut_snapshottableDirListing(&mut self) -> &mut ::protobuf::RepeatedField<SnapshottableDirectoryStatusProto> {
        &mut self.snapshottableDirListing
    }

    // Take field
    pub fn take_snapshottableDirListing(&mut self) -> ::protobuf::RepeatedField<SnapshottableDirectoryStatusProto> {
        ::std::mem::replace(&mut self.snapshottableDirListing, ::protobuf::RepeatedField::new())
    }

    pub fn get_snapshottableDirListing(&self) -> &[SnapshottableDirectoryStatusProto] {
        &self.snapshottableDirListing
    }

    fn get_snapshottableDirListing_for_reflect(&self) -> &::protobuf::RepeatedField<SnapshottableDirectoryStatusProto> {
        &self.snapshottableDirListing
    }

    fn mut_snapshottableDirListing_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SnapshottableDirectoryStatusProto> {
        &mut self.snapshottableDirListing
    }
}

impl ::protobuf::Message for SnapshottableDirectoryListingProto {
    fn is_initialized(&self) -> bool {
        for v in &self.snapshottableDirListing {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.snapshottableDirListing)?;
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
        for value in &self.snapshottableDirListing {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.snapshottableDirListing {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for SnapshottableDirectoryListingProto {
    fn new() -> SnapshottableDirectoryListingProto {
        SnapshottableDirectoryListingProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshottableDirectoryListingProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SnapshottableDirectoryStatusProto>>(
                    "snapshottableDirListing",
                    SnapshottableDirectoryListingProto::get_snapshottableDirListing_for_reflect,
                    SnapshottableDirectoryListingProto::mut_snapshottableDirListing_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshottableDirectoryListingProto>(
                    "SnapshottableDirectoryListingProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshottableDirectoryListingProto {
    fn clear(&mut self) {
        self.clear_snapshottableDirListing();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshottableDirectoryListingProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshottableDirectoryListingProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotDiffReportEntryProto {
    // message fields
    fullpath: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    modificationLabel: ::protobuf::SingularField<::std::string::String>,
    targetPath: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotDiffReportEntryProto {}

impl SnapshotDiffReportEntryProto {
    pub fn new() -> SnapshotDiffReportEntryProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotDiffReportEntryProto {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotDiffReportEntryProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotDiffReportEntryProto,
        };
        unsafe {
            instance.get(SnapshotDiffReportEntryProto::new)
        }
    }

    // required bytes fullpath = 1;

    pub fn clear_fullpath(&mut self) {
        self.fullpath.clear();
    }

    pub fn has_fullpath(&self) -> bool {
        self.fullpath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fullpath(&mut self, v: ::std::vec::Vec<u8>) {
        self.fullpath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fullpath(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.fullpath.is_none() {
            self.fullpath.set_default();
        }
        self.fullpath.as_mut().unwrap()
    }

    // Take field
    pub fn take_fullpath(&mut self) -> ::std::vec::Vec<u8> {
        self.fullpath.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_fullpath(&self) -> &[u8] {
        match self.fullpath.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_fullpath_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.fullpath
    }

    fn mut_fullpath_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.fullpath
    }

    // required string modificationLabel = 2;

    pub fn clear_modificationLabel(&mut self) {
        self.modificationLabel.clear();
    }

    pub fn has_modificationLabel(&self) -> bool {
        self.modificationLabel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modificationLabel(&mut self, v: ::std::string::String) {
        self.modificationLabel = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_modificationLabel(&mut self) -> &mut ::std::string::String {
        if self.modificationLabel.is_none() {
            self.modificationLabel.set_default();
        }
        self.modificationLabel.as_mut().unwrap()
    }

    // Take field
    pub fn take_modificationLabel(&mut self) -> ::std::string::String {
        self.modificationLabel.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_modificationLabel(&self) -> &str {
        match self.modificationLabel.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_modificationLabel_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.modificationLabel
    }

    fn mut_modificationLabel_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.modificationLabel
    }

    // optional bytes targetPath = 3;

    pub fn clear_targetPath(&mut self) {
        self.targetPath.clear();
    }

    pub fn has_targetPath(&self) -> bool {
        self.targetPath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_targetPath(&mut self, v: ::std::vec::Vec<u8>) {
        self.targetPath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_targetPath(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.targetPath.is_none() {
            self.targetPath.set_default();
        }
        self.targetPath.as_mut().unwrap()
    }

    // Take field
    pub fn take_targetPath(&mut self) -> ::std::vec::Vec<u8> {
        self.targetPath.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_targetPath(&self) -> &[u8] {
        match self.targetPath.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_targetPath_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.targetPath
    }

    fn mut_targetPath_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.targetPath
    }
}

impl ::protobuf::Message for SnapshotDiffReportEntryProto {
    fn is_initialized(&self) -> bool {
        if self.fullpath.is_none() {
            return false;
        }
        if self.modificationLabel.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.fullpath)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.modificationLabel)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.targetPath)?;
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
        if let Some(ref v) = self.fullpath.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.modificationLabel.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.targetPath.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.fullpath.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.modificationLabel.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.targetPath.as_ref() {
            os.write_bytes(3, &v)?;
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

impl ::protobuf::MessageStatic for SnapshotDiffReportEntryProto {
    fn new() -> SnapshotDiffReportEntryProto {
        SnapshotDiffReportEntryProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotDiffReportEntryProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "fullpath",
                    SnapshotDiffReportEntryProto::get_fullpath_for_reflect,
                    SnapshotDiffReportEntryProto::mut_fullpath_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "modificationLabel",
                    SnapshotDiffReportEntryProto::get_modificationLabel_for_reflect,
                    SnapshotDiffReportEntryProto::mut_modificationLabel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "targetPath",
                    SnapshotDiffReportEntryProto::get_targetPath_for_reflect,
                    SnapshotDiffReportEntryProto::mut_targetPath_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotDiffReportEntryProto>(
                    "SnapshotDiffReportEntryProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotDiffReportEntryProto {
    fn clear(&mut self) {
        self.clear_fullpath();
        self.clear_modificationLabel();
        self.clear_targetPath();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotDiffReportEntryProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotDiffReportEntryProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotDiffReportProto {
    // message fields
    snapshotRoot: ::protobuf::SingularField<::std::string::String>,
    fromSnapshot: ::protobuf::SingularField<::std::string::String>,
    toSnapshot: ::protobuf::SingularField<::std::string::String>,
    diffReportEntries: ::protobuf::RepeatedField<SnapshotDiffReportEntryProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotDiffReportProto {}

impl SnapshotDiffReportProto {
    pub fn new() -> SnapshotDiffReportProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotDiffReportProto {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotDiffReportProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotDiffReportProto,
        };
        unsafe {
            instance.get(SnapshotDiffReportProto::new)
        }
    }

    // required string snapshotRoot = 1;

    pub fn clear_snapshotRoot(&mut self) {
        self.snapshotRoot.clear();
    }

    pub fn has_snapshotRoot(&self) -> bool {
        self.snapshotRoot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshotRoot(&mut self, v: ::std::string::String) {
        self.snapshotRoot = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_snapshotRoot(&mut self) -> &mut ::std::string::String {
        if self.snapshotRoot.is_none() {
            self.snapshotRoot.set_default();
        }
        self.snapshotRoot.as_mut().unwrap()
    }

    // Take field
    pub fn take_snapshotRoot(&mut self) -> ::std::string::String {
        self.snapshotRoot.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_snapshotRoot(&self) -> &str {
        match self.snapshotRoot.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_snapshotRoot_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.snapshotRoot
    }

    fn mut_snapshotRoot_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.snapshotRoot
    }

    // required string fromSnapshot = 2;

    pub fn clear_fromSnapshot(&mut self) {
        self.fromSnapshot.clear();
    }

    pub fn has_fromSnapshot(&self) -> bool {
        self.fromSnapshot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fromSnapshot(&mut self, v: ::std::string::String) {
        self.fromSnapshot = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fromSnapshot(&mut self) -> &mut ::std::string::String {
        if self.fromSnapshot.is_none() {
            self.fromSnapshot.set_default();
        }
        self.fromSnapshot.as_mut().unwrap()
    }

    // Take field
    pub fn take_fromSnapshot(&mut self) -> ::std::string::String {
        self.fromSnapshot.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_fromSnapshot(&self) -> &str {
        match self.fromSnapshot.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_fromSnapshot_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.fromSnapshot
    }

    fn mut_fromSnapshot_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.fromSnapshot
    }

    // required string toSnapshot = 3;

    pub fn clear_toSnapshot(&mut self) {
        self.toSnapshot.clear();
    }

    pub fn has_toSnapshot(&self) -> bool {
        self.toSnapshot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_toSnapshot(&mut self, v: ::std::string::String) {
        self.toSnapshot = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_toSnapshot(&mut self) -> &mut ::std::string::String {
        if self.toSnapshot.is_none() {
            self.toSnapshot.set_default();
        }
        self.toSnapshot.as_mut().unwrap()
    }

    // Take field
    pub fn take_toSnapshot(&mut self) -> ::std::string::String {
        self.toSnapshot.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_toSnapshot(&self) -> &str {
        match self.toSnapshot.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_toSnapshot_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.toSnapshot
    }

    fn mut_toSnapshot_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.toSnapshot
    }

    // repeated .hadoop.hdfs.SnapshotDiffReportEntryProto diffReportEntries = 4;

    pub fn clear_diffReportEntries(&mut self) {
        self.diffReportEntries.clear();
    }

    // Param is passed by value, moved
    pub fn set_diffReportEntries(&mut self, v: ::protobuf::RepeatedField<SnapshotDiffReportEntryProto>) {
        self.diffReportEntries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_diffReportEntries(&mut self) -> &mut ::protobuf::RepeatedField<SnapshotDiffReportEntryProto> {
        &mut self.diffReportEntries
    }

    // Take field
    pub fn take_diffReportEntries(&mut self) -> ::protobuf::RepeatedField<SnapshotDiffReportEntryProto> {
        ::std::mem::replace(&mut self.diffReportEntries, ::protobuf::RepeatedField::new())
    }

    pub fn get_diffReportEntries(&self) -> &[SnapshotDiffReportEntryProto] {
        &self.diffReportEntries
    }

    fn get_diffReportEntries_for_reflect(&self) -> &::protobuf::RepeatedField<SnapshotDiffReportEntryProto> {
        &self.diffReportEntries
    }

    fn mut_diffReportEntries_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SnapshotDiffReportEntryProto> {
        &mut self.diffReportEntries
    }
}

impl ::protobuf::Message for SnapshotDiffReportProto {
    fn is_initialized(&self) -> bool {
        if self.snapshotRoot.is_none() {
            return false;
        }
        if self.fromSnapshot.is_none() {
            return false;
        }
        if self.toSnapshot.is_none() {
            return false;
        }
        for v in &self.diffReportEntries {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.snapshotRoot)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.fromSnapshot)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.toSnapshot)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.diffReportEntries)?;
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
        if let Some(ref v) = self.snapshotRoot.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.fromSnapshot.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.toSnapshot.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        for value in &self.diffReportEntries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.snapshotRoot.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.fromSnapshot.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.toSnapshot.as_ref() {
            os.write_string(3, &v)?;
        }
        for v in &self.diffReportEntries {
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

impl ::protobuf::MessageStatic for SnapshotDiffReportProto {
    fn new() -> SnapshotDiffReportProto {
        SnapshotDiffReportProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotDiffReportProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "snapshotRoot",
                    SnapshotDiffReportProto::get_snapshotRoot_for_reflect,
                    SnapshotDiffReportProto::mut_snapshotRoot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "fromSnapshot",
                    SnapshotDiffReportProto::get_fromSnapshot_for_reflect,
                    SnapshotDiffReportProto::mut_fromSnapshot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "toSnapshot",
                    SnapshotDiffReportProto::get_toSnapshot_for_reflect,
                    SnapshotDiffReportProto::mut_toSnapshot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SnapshotDiffReportEntryProto>>(
                    "diffReportEntries",
                    SnapshotDiffReportProto::get_diffReportEntries_for_reflect,
                    SnapshotDiffReportProto::mut_diffReportEntries_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotDiffReportProto>(
                    "SnapshotDiffReportProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotDiffReportProto {
    fn clear(&mut self) {
        self.clear_snapshotRoot();
        self.clear_fromSnapshot();
        self.clear_toSnapshot();
        self.clear_diffReportEntries();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotDiffReportProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotDiffReportProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StorageInfoProto {
    // message fields
    layoutVersion: ::std::option::Option<u32>,
    namespceID: ::std::option::Option<u32>,
    clusterID: ::protobuf::SingularField<::std::string::String>,
    cTime: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StorageInfoProto {}

impl StorageInfoProto {
    pub fn new() -> StorageInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StorageInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<StorageInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StorageInfoProto,
        };
        unsafe {
            instance.get(StorageInfoProto::new)
        }
    }

    // required uint32 layoutVersion = 1;

    pub fn clear_layoutVersion(&mut self) {
        self.layoutVersion = ::std::option::Option::None;
    }

    pub fn has_layoutVersion(&self) -> bool {
        self.layoutVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_layoutVersion(&mut self, v: u32) {
        self.layoutVersion = ::std::option::Option::Some(v);
    }

    pub fn get_layoutVersion(&self) -> u32 {
        self.layoutVersion.unwrap_or(0)
    }

    fn get_layoutVersion_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.layoutVersion
    }

    fn mut_layoutVersion_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.layoutVersion
    }

    // required uint32 namespceID = 2;

    pub fn clear_namespceID(&mut self) {
        self.namespceID = ::std::option::Option::None;
    }

    pub fn has_namespceID(&self) -> bool {
        self.namespceID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_namespceID(&mut self, v: u32) {
        self.namespceID = ::std::option::Option::Some(v);
    }

    pub fn get_namespceID(&self) -> u32 {
        self.namespceID.unwrap_or(0)
    }

    fn get_namespceID_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.namespceID
    }

    fn mut_namespceID_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.namespceID
    }

    // required string clusterID = 3;

    pub fn clear_clusterID(&mut self) {
        self.clusterID.clear();
    }

    pub fn has_clusterID(&self) -> bool {
        self.clusterID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clusterID(&mut self, v: ::std::string::String) {
        self.clusterID = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clusterID(&mut self) -> &mut ::std::string::String {
        if self.clusterID.is_none() {
            self.clusterID.set_default();
        }
        self.clusterID.as_mut().unwrap()
    }

    // Take field
    pub fn take_clusterID(&mut self) -> ::std::string::String {
        self.clusterID.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_clusterID(&self) -> &str {
        match self.clusterID.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_clusterID_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.clusterID
    }

    fn mut_clusterID_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.clusterID
    }

    // required uint64 cTime = 4;

    pub fn clear_cTime(&mut self) {
        self.cTime = ::std::option::Option::None;
    }

    pub fn has_cTime(&self) -> bool {
        self.cTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cTime(&mut self, v: u64) {
        self.cTime = ::std::option::Option::Some(v);
    }

    pub fn get_cTime(&self) -> u64 {
        self.cTime.unwrap_or(0)
    }

    fn get_cTime_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.cTime
    }

    fn mut_cTime_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.cTime
    }
}

impl ::protobuf::Message for StorageInfoProto {
    fn is_initialized(&self) -> bool {
        if self.layoutVersion.is_none() {
            return false;
        }
        if self.namespceID.is_none() {
            return false;
        }
        if self.clusterID.is_none() {
            return false;
        }
        if self.cTime.is_none() {
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
                    self.layoutVersion = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.namespceID = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.clusterID)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.cTime = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.layoutVersion {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.namespceID {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.clusterID.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.cTime {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.layoutVersion {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.namespceID {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.clusterID.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.cTime {
            os.write_uint64(4, v)?;
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

impl ::protobuf::MessageStatic for StorageInfoProto {
    fn new() -> StorageInfoProto {
        StorageInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StorageInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "layoutVersion",
                    StorageInfoProto::get_layoutVersion_for_reflect,
                    StorageInfoProto::mut_layoutVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "namespceID",
                    StorageInfoProto::get_namespceID_for_reflect,
                    StorageInfoProto::mut_namespceID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "clusterID",
                    StorageInfoProto::get_clusterID_for_reflect,
                    StorageInfoProto::mut_clusterID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "cTime",
                    StorageInfoProto::get_cTime_for_reflect,
                    StorageInfoProto::mut_cTime_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StorageInfoProto>(
                    "StorageInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StorageInfoProto {
    fn clear(&mut self) {
        self.clear_layoutVersion();
        self.clear_namespceID();
        self.clear_clusterID();
        self.clear_cTime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StorageInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StorageInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NamenodeRegistrationProto {
    // message fields
    rpcAddress: ::protobuf::SingularField<::std::string::String>,
    httpAddress: ::protobuf::SingularField<::std::string::String>,
    storageInfo: ::protobuf::SingularPtrField<StorageInfoProto>,
    role: ::std::option::Option<NamenodeRegistrationProto_NamenodeRoleProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NamenodeRegistrationProto {}

impl NamenodeRegistrationProto {
    pub fn new() -> NamenodeRegistrationProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NamenodeRegistrationProto {
        static mut instance: ::protobuf::lazy::Lazy<NamenodeRegistrationProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NamenodeRegistrationProto,
        };
        unsafe {
            instance.get(NamenodeRegistrationProto::new)
        }
    }

    // required string rpcAddress = 1;

    pub fn clear_rpcAddress(&mut self) {
        self.rpcAddress.clear();
    }

    pub fn has_rpcAddress(&self) -> bool {
        self.rpcAddress.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rpcAddress(&mut self, v: ::std::string::String) {
        self.rpcAddress = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rpcAddress(&mut self) -> &mut ::std::string::String {
        if self.rpcAddress.is_none() {
            self.rpcAddress.set_default();
        }
        self.rpcAddress.as_mut().unwrap()
    }

    // Take field
    pub fn take_rpcAddress(&mut self) -> ::std::string::String {
        self.rpcAddress.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_rpcAddress(&self) -> &str {
        match self.rpcAddress.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_rpcAddress_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.rpcAddress
    }

    fn mut_rpcAddress_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.rpcAddress
    }

    // required string httpAddress = 2;

    pub fn clear_httpAddress(&mut self) {
        self.httpAddress.clear();
    }

    pub fn has_httpAddress(&self) -> bool {
        self.httpAddress.is_some()
    }

    // Param is passed by value, moved
    pub fn set_httpAddress(&mut self, v: ::std::string::String) {
        self.httpAddress = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_httpAddress(&mut self) -> &mut ::std::string::String {
        if self.httpAddress.is_none() {
            self.httpAddress.set_default();
        }
        self.httpAddress.as_mut().unwrap()
    }

    // Take field
    pub fn take_httpAddress(&mut self) -> ::std::string::String {
        self.httpAddress.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_httpAddress(&self) -> &str {
        match self.httpAddress.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_httpAddress_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.httpAddress
    }

    fn mut_httpAddress_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.httpAddress
    }

    // required .hadoop.hdfs.StorageInfoProto storageInfo = 3;

    pub fn clear_storageInfo(&mut self) {
        self.storageInfo.clear();
    }

    pub fn has_storageInfo(&self) -> bool {
        self.storageInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageInfo(&mut self, v: StorageInfoProto) {
        self.storageInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storageInfo(&mut self) -> &mut StorageInfoProto {
        if self.storageInfo.is_none() {
            self.storageInfo.set_default();
        }
        self.storageInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_storageInfo(&mut self) -> StorageInfoProto {
        self.storageInfo.take().unwrap_or_else(|| StorageInfoProto::new())
    }

    pub fn get_storageInfo(&self) -> &StorageInfoProto {
        self.storageInfo.as_ref().unwrap_or_else(|| StorageInfoProto::default_instance())
    }

    fn get_storageInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<StorageInfoProto> {
        &self.storageInfo
    }

    fn mut_storageInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<StorageInfoProto> {
        &mut self.storageInfo
    }

    // optional .hadoop.hdfs.NamenodeRegistrationProto.NamenodeRoleProto role = 4;

    pub fn clear_role(&mut self) {
        self.role = ::std::option::Option::None;
    }

    pub fn has_role(&self) -> bool {
        self.role.is_some()
    }

    // Param is passed by value, moved
    pub fn set_role(&mut self, v: NamenodeRegistrationProto_NamenodeRoleProto) {
        self.role = ::std::option::Option::Some(v);
    }

    pub fn get_role(&self) -> NamenodeRegistrationProto_NamenodeRoleProto {
        self.role.unwrap_or(NamenodeRegistrationProto_NamenodeRoleProto::NAMENODE)
    }

    fn get_role_for_reflect(&self) -> &::std::option::Option<NamenodeRegistrationProto_NamenodeRoleProto> {
        &self.role
    }

    fn mut_role_for_reflect(&mut self) -> &mut ::std::option::Option<NamenodeRegistrationProto_NamenodeRoleProto> {
        &mut self.role
    }
}

impl ::protobuf::Message for NamenodeRegistrationProto {
    fn is_initialized(&self) -> bool {
        if self.rpcAddress.is_none() {
            return false;
        }
        if self.httpAddress.is_none() {
            return false;
        }
        if self.storageInfo.is_none() {
            return false;
        }
        for v in &self.storageInfo {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.rpcAddress)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.httpAddress)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.storageInfo)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.role = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.rpcAddress.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.httpAddress.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.storageInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.role {
            my_size += ::protobuf::rt::enum_size(4, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.rpcAddress.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.httpAddress.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.storageInfo.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.role {
            os.write_enum(4, v.value())?;
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

impl ::protobuf::MessageStatic for NamenodeRegistrationProto {
    fn new() -> NamenodeRegistrationProto {
        NamenodeRegistrationProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<NamenodeRegistrationProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "rpcAddress",
                    NamenodeRegistrationProto::get_rpcAddress_for_reflect,
                    NamenodeRegistrationProto::mut_rpcAddress_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "httpAddress",
                    NamenodeRegistrationProto::get_httpAddress_for_reflect,
                    NamenodeRegistrationProto::mut_httpAddress_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StorageInfoProto>>(
                    "storageInfo",
                    NamenodeRegistrationProto::get_storageInfo_for_reflect,
                    NamenodeRegistrationProto::mut_storageInfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<NamenodeRegistrationProto_NamenodeRoleProto>>(
                    "role",
                    NamenodeRegistrationProto::get_role_for_reflect,
                    NamenodeRegistrationProto::mut_role_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NamenodeRegistrationProto>(
                    "NamenodeRegistrationProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NamenodeRegistrationProto {
    fn clear(&mut self) {
        self.clear_rpcAddress();
        self.clear_httpAddress();
        self.clear_storageInfo();
        self.clear_role();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NamenodeRegistrationProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NamenodeRegistrationProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum NamenodeRegistrationProto_NamenodeRoleProto {
    NAMENODE = 1,
    BACKUP = 2,
    CHECKPOINT = 3,
}

impl ::protobuf::ProtobufEnum for NamenodeRegistrationProto_NamenodeRoleProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NamenodeRegistrationProto_NamenodeRoleProto> {
        match value {
            1 => ::std::option::Option::Some(NamenodeRegistrationProto_NamenodeRoleProto::NAMENODE),
            2 => ::std::option::Option::Some(NamenodeRegistrationProto_NamenodeRoleProto::BACKUP),
            3 => ::std::option::Option::Some(NamenodeRegistrationProto_NamenodeRoleProto::CHECKPOINT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [NamenodeRegistrationProto_NamenodeRoleProto] = &[
            NamenodeRegistrationProto_NamenodeRoleProto::NAMENODE,
            NamenodeRegistrationProto_NamenodeRoleProto::BACKUP,
            NamenodeRegistrationProto_NamenodeRoleProto::CHECKPOINT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<NamenodeRegistrationProto_NamenodeRoleProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("NamenodeRegistrationProto_NamenodeRoleProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for NamenodeRegistrationProto_NamenodeRoleProto {
}

impl ::protobuf::reflect::ProtobufValue for NamenodeRegistrationProto_NamenodeRoleProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CheckpointSignatureProto {
    // message fields
    blockPoolId: ::protobuf::SingularField<::std::string::String>,
    mostRecentCheckpointTxId: ::std::option::Option<u64>,
    curSegmentTxId: ::std::option::Option<u64>,
    storageInfo: ::protobuf::SingularPtrField<StorageInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CheckpointSignatureProto {}

impl CheckpointSignatureProto {
    pub fn new() -> CheckpointSignatureProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CheckpointSignatureProto {
        static mut instance: ::protobuf::lazy::Lazy<CheckpointSignatureProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CheckpointSignatureProto,
        };
        unsafe {
            instance.get(CheckpointSignatureProto::new)
        }
    }

    // required string blockPoolId = 1;

    pub fn clear_blockPoolId(&mut self) {
        self.blockPoolId.clear();
    }

    pub fn has_blockPoolId(&self) -> bool {
        self.blockPoolId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPoolId(&mut self, v: ::std::string::String) {
        self.blockPoolId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockPoolId(&mut self) -> &mut ::std::string::String {
        if self.blockPoolId.is_none() {
            self.blockPoolId.set_default();
        }
        self.blockPoolId.as_mut().unwrap()
    }

    // Take field
    pub fn take_blockPoolId(&mut self) -> ::std::string::String {
        self.blockPoolId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_blockPoolId(&self) -> &str {
        match self.blockPoolId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_blockPoolId_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.blockPoolId
    }

    fn mut_blockPoolId_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.blockPoolId
    }

    // required uint64 mostRecentCheckpointTxId = 2;

    pub fn clear_mostRecentCheckpointTxId(&mut self) {
        self.mostRecentCheckpointTxId = ::std::option::Option::None;
    }

    pub fn has_mostRecentCheckpointTxId(&self) -> bool {
        self.mostRecentCheckpointTxId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mostRecentCheckpointTxId(&mut self, v: u64) {
        self.mostRecentCheckpointTxId = ::std::option::Option::Some(v);
    }

    pub fn get_mostRecentCheckpointTxId(&self) -> u64 {
        self.mostRecentCheckpointTxId.unwrap_or(0)
    }

    fn get_mostRecentCheckpointTxId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.mostRecentCheckpointTxId
    }

    fn mut_mostRecentCheckpointTxId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.mostRecentCheckpointTxId
    }

    // required uint64 curSegmentTxId = 3;

    pub fn clear_curSegmentTxId(&mut self) {
        self.curSegmentTxId = ::std::option::Option::None;
    }

    pub fn has_curSegmentTxId(&self) -> bool {
        self.curSegmentTxId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_curSegmentTxId(&mut self, v: u64) {
        self.curSegmentTxId = ::std::option::Option::Some(v);
    }

    pub fn get_curSegmentTxId(&self) -> u64 {
        self.curSegmentTxId.unwrap_or(0)
    }

    fn get_curSegmentTxId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.curSegmentTxId
    }

    fn mut_curSegmentTxId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.curSegmentTxId
    }

    // required .hadoop.hdfs.StorageInfoProto storageInfo = 4;

    pub fn clear_storageInfo(&mut self) {
        self.storageInfo.clear();
    }

    pub fn has_storageInfo(&self) -> bool {
        self.storageInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageInfo(&mut self, v: StorageInfoProto) {
        self.storageInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storageInfo(&mut self) -> &mut StorageInfoProto {
        if self.storageInfo.is_none() {
            self.storageInfo.set_default();
        }
        self.storageInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_storageInfo(&mut self) -> StorageInfoProto {
        self.storageInfo.take().unwrap_or_else(|| StorageInfoProto::new())
    }

    pub fn get_storageInfo(&self) -> &StorageInfoProto {
        self.storageInfo.as_ref().unwrap_or_else(|| StorageInfoProto::default_instance())
    }

    fn get_storageInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<StorageInfoProto> {
        &self.storageInfo
    }

    fn mut_storageInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<StorageInfoProto> {
        &mut self.storageInfo
    }
}

impl ::protobuf::Message for CheckpointSignatureProto {
    fn is_initialized(&self) -> bool {
        if self.blockPoolId.is_none() {
            return false;
        }
        if self.mostRecentCheckpointTxId.is_none() {
            return false;
        }
        if self.curSegmentTxId.is_none() {
            return false;
        }
        if self.storageInfo.is_none() {
            return false;
        }
        for v in &self.storageInfo {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.blockPoolId)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.mostRecentCheckpointTxId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.curSegmentTxId = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.storageInfo)?;
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
        if let Some(ref v) = self.blockPoolId.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.mostRecentCheckpointTxId {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.curSegmentTxId {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.storageInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.blockPoolId.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.mostRecentCheckpointTxId {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.curSegmentTxId {
            os.write_uint64(3, v)?;
        }
        if let Some(ref v) = self.storageInfo.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CheckpointSignatureProto {
    fn new() -> CheckpointSignatureProto {
        CheckpointSignatureProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CheckpointSignatureProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "blockPoolId",
                    CheckpointSignatureProto::get_blockPoolId_for_reflect,
                    CheckpointSignatureProto::mut_blockPoolId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "mostRecentCheckpointTxId",
                    CheckpointSignatureProto::get_mostRecentCheckpointTxId_for_reflect,
                    CheckpointSignatureProto::mut_mostRecentCheckpointTxId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "curSegmentTxId",
                    CheckpointSignatureProto::get_curSegmentTxId_for_reflect,
                    CheckpointSignatureProto::mut_curSegmentTxId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StorageInfoProto>>(
                    "storageInfo",
                    CheckpointSignatureProto::get_storageInfo_for_reflect,
                    CheckpointSignatureProto::mut_storageInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CheckpointSignatureProto>(
                    "CheckpointSignatureProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CheckpointSignatureProto {
    fn clear(&mut self) {
        self.clear_blockPoolId();
        self.clear_mostRecentCheckpointTxId();
        self.clear_curSegmentTxId();
        self.clear_storageInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CheckpointSignatureProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CheckpointSignatureProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NamenodeCommandProto {
    // message fields
    action: ::std::option::Option<u32>,
    field_type: ::std::option::Option<NamenodeCommandProto_Type>,
    checkpointCmd: ::protobuf::SingularPtrField<CheckpointCommandProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NamenodeCommandProto {}

impl NamenodeCommandProto {
    pub fn new() -> NamenodeCommandProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NamenodeCommandProto {
        static mut instance: ::protobuf::lazy::Lazy<NamenodeCommandProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NamenodeCommandProto,
        };
        unsafe {
            instance.get(NamenodeCommandProto::new)
        }
    }

    // required uint32 action = 1;

    pub fn clear_action(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_action(&self) -> bool {
        self.action.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action(&mut self, v: u32) {
        self.action = ::std::option::Option::Some(v);
    }

    pub fn get_action(&self) -> u32 {
        self.action.unwrap_or(0)
    }

    fn get_action_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.action
    }

    fn mut_action_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.action
    }

    // required .hadoop.hdfs.NamenodeCommandProto.Type type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: NamenodeCommandProto_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> NamenodeCommandProto_Type {
        self.field_type.unwrap_or(NamenodeCommandProto_Type::NamenodeCommand)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<NamenodeCommandProto_Type> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<NamenodeCommandProto_Type> {
        &mut self.field_type
    }

    // optional .hadoop.hdfs.CheckpointCommandProto checkpointCmd = 3;

    pub fn clear_checkpointCmd(&mut self) {
        self.checkpointCmd.clear();
    }

    pub fn has_checkpointCmd(&self) -> bool {
        self.checkpointCmd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checkpointCmd(&mut self, v: CheckpointCommandProto) {
        self.checkpointCmd = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_checkpointCmd(&mut self) -> &mut CheckpointCommandProto {
        if self.checkpointCmd.is_none() {
            self.checkpointCmd.set_default();
        }
        self.checkpointCmd.as_mut().unwrap()
    }

    // Take field
    pub fn take_checkpointCmd(&mut self) -> CheckpointCommandProto {
        self.checkpointCmd.take().unwrap_or_else(|| CheckpointCommandProto::new())
    }

    pub fn get_checkpointCmd(&self) -> &CheckpointCommandProto {
        self.checkpointCmd.as_ref().unwrap_or_else(|| CheckpointCommandProto::default_instance())
    }

    fn get_checkpointCmd_for_reflect(&self) -> &::protobuf::SingularPtrField<CheckpointCommandProto> {
        &self.checkpointCmd
    }

    fn mut_checkpointCmd_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CheckpointCommandProto> {
        &mut self.checkpointCmd
    }
}

impl ::protobuf::Message for NamenodeCommandProto {
    fn is_initialized(&self) -> bool {
        if self.action.is_none() {
            return false;
        }
        if self.field_type.is_none() {
            return false;
        }
        for v in &self.checkpointCmd {
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
                    self.action = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.checkpointCmd)?;
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
        if let Some(v) = self.action {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(ref v) = self.checkpointCmd.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.action {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.field_type {
            os.write_enum(2, v.value())?;
        }
        if let Some(ref v) = self.checkpointCmd.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for NamenodeCommandProto {
    fn new() -> NamenodeCommandProto {
        NamenodeCommandProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<NamenodeCommandProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "action",
                    NamenodeCommandProto::get_action_for_reflect,
                    NamenodeCommandProto::mut_action_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<NamenodeCommandProto_Type>>(
                    "type",
                    NamenodeCommandProto::get_field_type_for_reflect,
                    NamenodeCommandProto::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CheckpointCommandProto>>(
                    "checkpointCmd",
                    NamenodeCommandProto::get_checkpointCmd_for_reflect,
                    NamenodeCommandProto::mut_checkpointCmd_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NamenodeCommandProto>(
                    "NamenodeCommandProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NamenodeCommandProto {
    fn clear(&mut self) {
        self.clear_action();
        self.clear_field_type();
        self.clear_checkpointCmd();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NamenodeCommandProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NamenodeCommandProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum NamenodeCommandProto_Type {
    NamenodeCommand = 0,
    CheckPointCommand = 1,
}

impl ::protobuf::ProtobufEnum for NamenodeCommandProto_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NamenodeCommandProto_Type> {
        match value {
            0 => ::std::option::Option::Some(NamenodeCommandProto_Type::NamenodeCommand),
            1 => ::std::option::Option::Some(NamenodeCommandProto_Type::CheckPointCommand),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [NamenodeCommandProto_Type] = &[
            NamenodeCommandProto_Type::NamenodeCommand,
            NamenodeCommandProto_Type::CheckPointCommand,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<NamenodeCommandProto_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("NamenodeCommandProto_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for NamenodeCommandProto_Type {
}

impl ::protobuf::reflect::ProtobufValue for NamenodeCommandProto_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CheckpointCommandProto {
    // message fields
    signature: ::protobuf::SingularPtrField<CheckpointSignatureProto>,
    needToReturnImage: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CheckpointCommandProto {}

impl CheckpointCommandProto {
    pub fn new() -> CheckpointCommandProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CheckpointCommandProto {
        static mut instance: ::protobuf::lazy::Lazy<CheckpointCommandProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CheckpointCommandProto,
        };
        unsafe {
            instance.get(CheckpointCommandProto::new)
        }
    }

    // required .hadoop.hdfs.CheckpointSignatureProto signature = 1;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: CheckpointSignatureProto) {
        self.signature = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut CheckpointSignatureProto {
        if self.signature.is_none() {
            self.signature.set_default();
        }
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> CheckpointSignatureProto {
        self.signature.take().unwrap_or_else(|| CheckpointSignatureProto::new())
    }

    pub fn get_signature(&self) -> &CheckpointSignatureProto {
        self.signature.as_ref().unwrap_or_else(|| CheckpointSignatureProto::default_instance())
    }

    fn get_signature_for_reflect(&self) -> &::protobuf::SingularPtrField<CheckpointSignatureProto> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CheckpointSignatureProto> {
        &mut self.signature
    }

    // required bool needToReturnImage = 2;

    pub fn clear_needToReturnImage(&mut self) {
        self.needToReturnImage = ::std::option::Option::None;
    }

    pub fn has_needToReturnImage(&self) -> bool {
        self.needToReturnImage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_needToReturnImage(&mut self, v: bool) {
        self.needToReturnImage = ::std::option::Option::Some(v);
    }

    pub fn get_needToReturnImage(&self) -> bool {
        self.needToReturnImage.unwrap_or(false)
    }

    fn get_needToReturnImage_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.needToReturnImage
    }

    fn mut_needToReturnImage_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.needToReturnImage
    }
}

impl ::protobuf::Message for CheckpointCommandProto {
    fn is_initialized(&self) -> bool {
        if self.signature.is_none() {
            return false;
        }
        if self.needToReturnImage.is_none() {
            return false;
        }
        for v in &self.signature {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.signature)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.needToReturnImage = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.signature.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.needToReturnImage {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.signature.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.needToReturnImage {
            os.write_bool(2, v)?;
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

impl ::protobuf::MessageStatic for CheckpointCommandProto {
    fn new() -> CheckpointCommandProto {
        CheckpointCommandProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CheckpointCommandProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CheckpointSignatureProto>>(
                    "signature",
                    CheckpointCommandProto::get_signature_for_reflect,
                    CheckpointCommandProto::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "needToReturnImage",
                    CheckpointCommandProto::get_needToReturnImage_for_reflect,
                    CheckpointCommandProto::mut_needToReturnImage_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CheckpointCommandProto>(
                    "CheckpointCommandProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CheckpointCommandProto {
    fn clear(&mut self) {
        self.clear_signature();
        self.clear_needToReturnImage();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CheckpointCommandProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CheckpointCommandProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockProto {
    // message fields
    blockId: ::std::option::Option<u64>,
    genStamp: ::std::option::Option<u64>,
    numBytes: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockProto {}

impl BlockProto {
    pub fn new() -> BlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockProto,
        };
        unsafe {
            instance.get(BlockProto::new)
        }
    }

    // required uint64 blockId = 1;

    pub fn clear_blockId(&mut self) {
        self.blockId = ::std::option::Option::None;
    }

    pub fn has_blockId(&self) -> bool {
        self.blockId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockId(&mut self, v: u64) {
        self.blockId = ::std::option::Option::Some(v);
    }

    pub fn get_blockId(&self) -> u64 {
        self.blockId.unwrap_or(0)
    }

    fn get_blockId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.blockId
    }

    fn mut_blockId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.blockId
    }

    // required uint64 genStamp = 2;

    pub fn clear_genStamp(&mut self) {
        self.genStamp = ::std::option::Option::None;
    }

    pub fn has_genStamp(&self) -> bool {
        self.genStamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_genStamp(&mut self, v: u64) {
        self.genStamp = ::std::option::Option::Some(v);
    }

    pub fn get_genStamp(&self) -> u64 {
        self.genStamp.unwrap_or(0)
    }

    fn get_genStamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.genStamp
    }

    fn mut_genStamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.genStamp
    }

    // optional uint64 numBytes = 3;

    pub fn clear_numBytes(&mut self) {
        self.numBytes = ::std::option::Option::None;
    }

    pub fn has_numBytes(&self) -> bool {
        self.numBytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numBytes(&mut self, v: u64) {
        self.numBytes = ::std::option::Option::Some(v);
    }

    pub fn get_numBytes(&self) -> u64 {
        self.numBytes.unwrap_or(0u64)
    }

    fn get_numBytes_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.numBytes
    }

    fn mut_numBytes_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.numBytes
    }
}

impl ::protobuf::Message for BlockProto {
    fn is_initialized(&self) -> bool {
        if self.blockId.is_none() {
            return false;
        }
        if self.genStamp.is_none() {
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
                    let tmp = is.read_uint64()?;
                    self.blockId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.genStamp = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.numBytes = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.blockId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.genStamp {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numBytes {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.blockId {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.genStamp {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.numBytes {
            os.write_uint64(3, v)?;
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

impl ::protobuf::MessageStatic for BlockProto {
    fn new() -> BlockProto {
        BlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "blockId",
                    BlockProto::get_blockId_for_reflect,
                    BlockProto::mut_blockId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "genStamp",
                    BlockProto::get_genStamp_for_reflect,
                    BlockProto::mut_genStamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "numBytes",
                    BlockProto::get_numBytes_for_reflect,
                    BlockProto::mut_numBytes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockProto>(
                    "BlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockProto {
    fn clear(&mut self) {
        self.clear_blockId();
        self.clear_genStamp();
        self.clear_numBytes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockWithLocationsProto {
    // message fields
    block: ::protobuf::SingularPtrField<BlockProto>,
    datanodeUuids: ::protobuf::RepeatedField<::std::string::String>,
    storageUuids: ::protobuf::RepeatedField<::std::string::String>,
    storageTypes: ::std::vec::Vec<StorageTypeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockWithLocationsProto {}

impl BlockWithLocationsProto {
    pub fn new() -> BlockWithLocationsProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockWithLocationsProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockWithLocationsProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockWithLocationsProto,
        };
        unsafe {
            instance.get(BlockWithLocationsProto::new)
        }
    }

    // required .hadoop.hdfs.BlockProto block = 1;

    pub fn clear_block(&mut self) {
        self.block.clear();
    }

    pub fn has_block(&self) -> bool {
        self.block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block(&mut self, v: BlockProto) {
        self.block = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block(&mut self) -> &mut BlockProto {
        if self.block.is_none() {
            self.block.set_default();
        }
        self.block.as_mut().unwrap()
    }

    // Take field
    pub fn take_block(&mut self) -> BlockProto {
        self.block.take().unwrap_or_else(|| BlockProto::new())
    }

    pub fn get_block(&self) -> &BlockProto {
        self.block.as_ref().unwrap_or_else(|| BlockProto::default_instance())
    }

    fn get_block_for_reflect(&self) -> &::protobuf::SingularPtrField<BlockProto> {
        &self.block
    }

    fn mut_block_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BlockProto> {
        &mut self.block
    }

    // repeated string datanodeUuids = 2;

    pub fn clear_datanodeUuids(&mut self) {
        self.datanodeUuids.clear();
    }

    // Param is passed by value, moved
    pub fn set_datanodeUuids(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.datanodeUuids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_datanodeUuids(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.datanodeUuids
    }

    // Take field
    pub fn take_datanodeUuids(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.datanodeUuids, ::protobuf::RepeatedField::new())
    }

    pub fn get_datanodeUuids(&self) -> &[::std::string::String] {
        &self.datanodeUuids
    }

    fn get_datanodeUuids_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.datanodeUuids
    }

    fn mut_datanodeUuids_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.datanodeUuids
    }

    // repeated string storageUuids = 3;

    pub fn clear_storageUuids(&mut self) {
        self.storageUuids.clear();
    }

    // Param is passed by value, moved
    pub fn set_storageUuids(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.storageUuids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_storageUuids(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.storageUuids
    }

    // Take field
    pub fn take_storageUuids(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.storageUuids, ::protobuf::RepeatedField::new())
    }

    pub fn get_storageUuids(&self) -> &[::std::string::String] {
        &self.storageUuids
    }

    fn get_storageUuids_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.storageUuids
    }

    fn mut_storageUuids_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.storageUuids
    }

    // repeated .hadoop.hdfs.StorageTypeProto storageTypes = 4;

    pub fn clear_storageTypes(&mut self) {
        self.storageTypes.clear();
    }

    // Param is passed by value, moved
    pub fn set_storageTypes(&mut self, v: ::std::vec::Vec<StorageTypeProto>) {
        self.storageTypes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_storageTypes(&mut self) -> &mut ::std::vec::Vec<StorageTypeProto> {
        &mut self.storageTypes
    }

    // Take field
    pub fn take_storageTypes(&mut self) -> ::std::vec::Vec<StorageTypeProto> {
        ::std::mem::replace(&mut self.storageTypes, ::std::vec::Vec::new())
    }

    pub fn get_storageTypes(&self) -> &[StorageTypeProto] {
        &self.storageTypes
    }

    fn get_storageTypes_for_reflect(&self) -> &::std::vec::Vec<StorageTypeProto> {
        &self.storageTypes
    }

    fn mut_storageTypes_for_reflect(&mut self) -> &mut ::std::vec::Vec<StorageTypeProto> {
        &mut self.storageTypes
    }
}

impl ::protobuf::Message for BlockWithLocationsProto {
    fn is_initialized(&self) -> bool {
        if self.block.is_none() {
            return false;
        }
        for v in &self.block {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.block)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.datanodeUuids)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.storageUuids)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.storageTypes)?;
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
        if let Some(ref v) = self.block.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.datanodeUuids {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.storageUuids {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.storageTypes {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.block.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.datanodeUuids {
            os.write_string(2, &v)?;
        };
        for v in &self.storageUuids {
            os.write_string(3, &v)?;
        };
        for v in &self.storageTypes {
            os.write_enum(4, v.value())?;
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

impl ::protobuf::MessageStatic for BlockWithLocationsProto {
    fn new() -> BlockWithLocationsProto {
        BlockWithLocationsProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockWithLocationsProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockProto>>(
                    "block",
                    BlockWithLocationsProto::get_block_for_reflect,
                    BlockWithLocationsProto::mut_block_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "datanodeUuids",
                    BlockWithLocationsProto::get_datanodeUuids_for_reflect,
                    BlockWithLocationsProto::mut_datanodeUuids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "storageUuids",
                    BlockWithLocationsProto::get_storageUuids_for_reflect,
                    BlockWithLocationsProto::mut_storageUuids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<StorageTypeProto>>(
                    "storageTypes",
                    BlockWithLocationsProto::get_storageTypes_for_reflect,
                    BlockWithLocationsProto::mut_storageTypes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockWithLocationsProto>(
                    "BlockWithLocationsProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockWithLocationsProto {
    fn clear(&mut self) {
        self.clear_block();
        self.clear_datanodeUuids();
        self.clear_storageUuids();
        self.clear_storageTypes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockWithLocationsProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockWithLocationsProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlocksWithLocationsProto {
    // message fields
    blocks: ::protobuf::RepeatedField<BlockWithLocationsProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlocksWithLocationsProto {}

impl BlocksWithLocationsProto {
    pub fn new() -> BlocksWithLocationsProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlocksWithLocationsProto {
        static mut instance: ::protobuf::lazy::Lazy<BlocksWithLocationsProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlocksWithLocationsProto,
        };
        unsafe {
            instance.get(BlocksWithLocationsProto::new)
        }
    }

    // repeated .hadoop.hdfs.BlockWithLocationsProto blocks = 1;

    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    // Param is passed by value, moved
    pub fn set_blocks(&mut self, v: ::protobuf::RepeatedField<BlockWithLocationsProto>) {
        self.blocks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blocks(&mut self) -> &mut ::protobuf::RepeatedField<BlockWithLocationsProto> {
        &mut self.blocks
    }

    // Take field
    pub fn take_blocks(&mut self) -> ::protobuf::RepeatedField<BlockWithLocationsProto> {
        ::std::mem::replace(&mut self.blocks, ::protobuf::RepeatedField::new())
    }

    pub fn get_blocks(&self) -> &[BlockWithLocationsProto] {
        &self.blocks
    }

    fn get_blocks_for_reflect(&self) -> &::protobuf::RepeatedField<BlockWithLocationsProto> {
        &self.blocks
    }

    fn mut_blocks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<BlockWithLocationsProto> {
        &mut self.blocks
    }
}

impl ::protobuf::Message for BlocksWithLocationsProto {
    fn is_initialized(&self) -> bool {
        for v in &self.blocks {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.blocks)?;
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
        for value in &self.blocks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.blocks {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for BlocksWithLocationsProto {
    fn new() -> BlocksWithLocationsProto {
        BlocksWithLocationsProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlocksWithLocationsProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockWithLocationsProto>>(
                    "blocks",
                    BlocksWithLocationsProto::get_blocks_for_reflect,
                    BlocksWithLocationsProto::mut_blocks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlocksWithLocationsProto>(
                    "BlocksWithLocationsProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlocksWithLocationsProto {
    fn clear(&mut self) {
        self.clear_blocks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlocksWithLocationsProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlocksWithLocationsProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RemoteEditLogProto {
    // message fields
    startTxId: ::std::option::Option<u64>,
    endTxId: ::std::option::Option<u64>,
    isInProgress: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemoteEditLogProto {}

impl RemoteEditLogProto {
    pub fn new() -> RemoteEditLogProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoteEditLogProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoteEditLogProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoteEditLogProto,
        };
        unsafe {
            instance.get(RemoteEditLogProto::new)
        }
    }

    // required uint64 startTxId = 1;

    pub fn clear_startTxId(&mut self) {
        self.startTxId = ::std::option::Option::None;
    }

    pub fn has_startTxId(&self) -> bool {
        self.startTxId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_startTxId(&mut self, v: u64) {
        self.startTxId = ::std::option::Option::Some(v);
    }

    pub fn get_startTxId(&self) -> u64 {
        self.startTxId.unwrap_or(0)
    }

    fn get_startTxId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.startTxId
    }

    fn mut_startTxId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.startTxId
    }

    // required uint64 endTxId = 2;

    pub fn clear_endTxId(&mut self) {
        self.endTxId = ::std::option::Option::None;
    }

    pub fn has_endTxId(&self) -> bool {
        self.endTxId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_endTxId(&mut self, v: u64) {
        self.endTxId = ::std::option::Option::Some(v);
    }

    pub fn get_endTxId(&self) -> u64 {
        self.endTxId.unwrap_or(0)
    }

    fn get_endTxId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.endTxId
    }

    fn mut_endTxId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.endTxId
    }

    // optional bool isInProgress = 3;

    pub fn clear_isInProgress(&mut self) {
        self.isInProgress = ::std::option::Option::None;
    }

    pub fn has_isInProgress(&self) -> bool {
        self.isInProgress.is_some()
    }

    // Param is passed by value, moved
    pub fn set_isInProgress(&mut self, v: bool) {
        self.isInProgress = ::std::option::Option::Some(v);
    }

    pub fn get_isInProgress(&self) -> bool {
        self.isInProgress.unwrap_or(false)
    }

    fn get_isInProgress_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.isInProgress
    }

    fn mut_isInProgress_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.isInProgress
    }
}

impl ::protobuf::Message for RemoteEditLogProto {
    fn is_initialized(&self) -> bool {
        if self.startTxId.is_none() {
            return false;
        }
        if self.endTxId.is_none() {
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
                    let tmp = is.read_uint64()?;
                    self.startTxId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.endTxId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.isInProgress = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.startTxId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.endTxId {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.isInProgress {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.startTxId {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.endTxId {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.isInProgress {
            os.write_bool(3, v)?;
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

impl ::protobuf::MessageStatic for RemoteEditLogProto {
    fn new() -> RemoteEditLogProto {
        RemoteEditLogProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoteEditLogProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "startTxId",
                    RemoteEditLogProto::get_startTxId_for_reflect,
                    RemoteEditLogProto::mut_startTxId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "endTxId",
                    RemoteEditLogProto::get_endTxId_for_reflect,
                    RemoteEditLogProto::mut_endTxId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "isInProgress",
                    RemoteEditLogProto::get_isInProgress_for_reflect,
                    RemoteEditLogProto::mut_isInProgress_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RemoteEditLogProto>(
                    "RemoteEditLogProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoteEditLogProto {
    fn clear(&mut self) {
        self.clear_startTxId();
        self.clear_endTxId();
        self.clear_isInProgress();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RemoteEditLogProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemoteEditLogProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RemoteEditLogManifestProto {
    // message fields
    logs: ::protobuf::RepeatedField<RemoteEditLogProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemoteEditLogManifestProto {}

impl RemoteEditLogManifestProto {
    pub fn new() -> RemoteEditLogManifestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoteEditLogManifestProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoteEditLogManifestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoteEditLogManifestProto,
        };
        unsafe {
            instance.get(RemoteEditLogManifestProto::new)
        }
    }

    // repeated .hadoop.hdfs.RemoteEditLogProto logs = 1;

    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }

    // Param is passed by value, moved
    pub fn set_logs(&mut self, v: ::protobuf::RepeatedField<RemoteEditLogProto>) {
        self.logs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_logs(&mut self) -> &mut ::protobuf::RepeatedField<RemoteEditLogProto> {
        &mut self.logs
    }

    // Take field
    pub fn take_logs(&mut self) -> ::protobuf::RepeatedField<RemoteEditLogProto> {
        ::std::mem::replace(&mut self.logs, ::protobuf::RepeatedField::new())
    }

    pub fn get_logs(&self) -> &[RemoteEditLogProto] {
        &self.logs
    }

    fn get_logs_for_reflect(&self) -> &::protobuf::RepeatedField<RemoteEditLogProto> {
        &self.logs
    }

    fn mut_logs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RemoteEditLogProto> {
        &mut self.logs
    }
}

impl ::protobuf::Message for RemoteEditLogManifestProto {
    fn is_initialized(&self) -> bool {
        for v in &self.logs {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.logs)?;
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
        for value in &self.logs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.logs {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for RemoteEditLogManifestProto {
    fn new() -> RemoteEditLogManifestProto {
        RemoteEditLogManifestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoteEditLogManifestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RemoteEditLogProto>>(
                    "logs",
                    RemoteEditLogManifestProto::get_logs_for_reflect,
                    RemoteEditLogManifestProto::mut_logs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RemoteEditLogManifestProto>(
                    "RemoteEditLogManifestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoteEditLogManifestProto {
    fn clear(&mut self) {
        self.clear_logs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RemoteEditLogManifestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemoteEditLogManifestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NamespaceInfoProto {
    // message fields
    buildVersion: ::protobuf::SingularField<::std::string::String>,
    unused: ::std::option::Option<u32>,
    blockPoolID: ::protobuf::SingularField<::std::string::String>,
    storageInfo: ::protobuf::SingularPtrField<StorageInfoProto>,
    softwareVersion: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NamespaceInfoProto {}

impl NamespaceInfoProto {
    pub fn new() -> NamespaceInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NamespaceInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<NamespaceInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NamespaceInfoProto,
        };
        unsafe {
            instance.get(NamespaceInfoProto::new)
        }
    }

    // required string buildVersion = 1;

    pub fn clear_buildVersion(&mut self) {
        self.buildVersion.clear();
    }

    pub fn has_buildVersion(&self) -> bool {
        self.buildVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buildVersion(&mut self, v: ::std::string::String) {
        self.buildVersion = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_buildVersion(&mut self) -> &mut ::std::string::String {
        if self.buildVersion.is_none() {
            self.buildVersion.set_default();
        }
        self.buildVersion.as_mut().unwrap()
    }

    // Take field
    pub fn take_buildVersion(&mut self) -> ::std::string::String {
        self.buildVersion.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_buildVersion(&self) -> &str {
        match self.buildVersion.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_buildVersion_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.buildVersion
    }

    fn mut_buildVersion_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.buildVersion
    }

    // required uint32 unused = 2;

    pub fn clear_unused(&mut self) {
        self.unused = ::std::option::Option::None;
    }

    pub fn has_unused(&self) -> bool {
        self.unused.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unused(&mut self, v: u32) {
        self.unused = ::std::option::Option::Some(v);
    }

    pub fn get_unused(&self) -> u32 {
        self.unused.unwrap_or(0)
    }

    fn get_unused_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.unused
    }

    fn mut_unused_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.unused
    }

    // required string blockPoolID = 3;

    pub fn clear_blockPoolID(&mut self) {
        self.blockPoolID.clear();
    }

    pub fn has_blockPoolID(&self) -> bool {
        self.blockPoolID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPoolID(&mut self, v: ::std::string::String) {
        self.blockPoolID = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockPoolID(&mut self) -> &mut ::std::string::String {
        if self.blockPoolID.is_none() {
            self.blockPoolID.set_default();
        }
        self.blockPoolID.as_mut().unwrap()
    }

    // Take field
    pub fn take_blockPoolID(&mut self) -> ::std::string::String {
        self.blockPoolID.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_blockPoolID(&self) -> &str {
        match self.blockPoolID.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_blockPoolID_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.blockPoolID
    }

    fn mut_blockPoolID_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.blockPoolID
    }

    // required .hadoop.hdfs.StorageInfoProto storageInfo = 4;

    pub fn clear_storageInfo(&mut self) {
        self.storageInfo.clear();
    }

    pub fn has_storageInfo(&self) -> bool {
        self.storageInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageInfo(&mut self, v: StorageInfoProto) {
        self.storageInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storageInfo(&mut self) -> &mut StorageInfoProto {
        if self.storageInfo.is_none() {
            self.storageInfo.set_default();
        }
        self.storageInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_storageInfo(&mut self) -> StorageInfoProto {
        self.storageInfo.take().unwrap_or_else(|| StorageInfoProto::new())
    }

    pub fn get_storageInfo(&self) -> &StorageInfoProto {
        self.storageInfo.as_ref().unwrap_or_else(|| StorageInfoProto::default_instance())
    }

    fn get_storageInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<StorageInfoProto> {
        &self.storageInfo
    }

    fn mut_storageInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<StorageInfoProto> {
        &mut self.storageInfo
    }

    // required string softwareVersion = 5;

    pub fn clear_softwareVersion(&mut self) {
        self.softwareVersion.clear();
    }

    pub fn has_softwareVersion(&self) -> bool {
        self.softwareVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_softwareVersion(&mut self, v: ::std::string::String) {
        self.softwareVersion = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_softwareVersion(&mut self) -> &mut ::std::string::String {
        if self.softwareVersion.is_none() {
            self.softwareVersion.set_default();
        }
        self.softwareVersion.as_mut().unwrap()
    }

    // Take field
    pub fn take_softwareVersion(&mut self) -> ::std::string::String {
        self.softwareVersion.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_softwareVersion(&self) -> &str {
        match self.softwareVersion.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_softwareVersion_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.softwareVersion
    }

    fn mut_softwareVersion_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.softwareVersion
    }
}

impl ::protobuf::Message for NamespaceInfoProto {
    fn is_initialized(&self) -> bool {
        if self.buildVersion.is_none() {
            return false;
        }
        if self.unused.is_none() {
            return false;
        }
        if self.blockPoolID.is_none() {
            return false;
        }
        if self.storageInfo.is_none() {
            return false;
        }
        if self.softwareVersion.is_none() {
            return false;
        }
        for v in &self.storageInfo {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.buildVersion)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.unused = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.blockPoolID)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.storageInfo)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.softwareVersion)?;
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
        if let Some(ref v) = self.buildVersion.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.unused {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.blockPoolID.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.storageInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.softwareVersion.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.buildVersion.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.unused {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.blockPoolID.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.storageInfo.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.softwareVersion.as_ref() {
            os.write_string(5, &v)?;
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

impl ::protobuf::MessageStatic for NamespaceInfoProto {
    fn new() -> NamespaceInfoProto {
        NamespaceInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<NamespaceInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "buildVersion",
                    NamespaceInfoProto::get_buildVersion_for_reflect,
                    NamespaceInfoProto::mut_buildVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "unused",
                    NamespaceInfoProto::get_unused_for_reflect,
                    NamespaceInfoProto::mut_unused_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "blockPoolID",
                    NamespaceInfoProto::get_blockPoolID_for_reflect,
                    NamespaceInfoProto::mut_blockPoolID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StorageInfoProto>>(
                    "storageInfo",
                    NamespaceInfoProto::get_storageInfo_for_reflect,
                    NamespaceInfoProto::mut_storageInfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "softwareVersion",
                    NamespaceInfoProto::get_softwareVersion_for_reflect,
                    NamespaceInfoProto::mut_softwareVersion_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NamespaceInfoProto>(
                    "NamespaceInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NamespaceInfoProto {
    fn clear(&mut self) {
        self.clear_buildVersion();
        self.clear_unused();
        self.clear_blockPoolID();
        self.clear_storageInfo();
        self.clear_softwareVersion();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NamespaceInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NamespaceInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockKeyProto {
    // message fields
    keyId: ::std::option::Option<u32>,
    expiryDate: ::std::option::Option<u64>,
    keyBytes: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockKeyProto {}

impl BlockKeyProto {
    pub fn new() -> BlockKeyProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockKeyProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockKeyProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockKeyProto,
        };
        unsafe {
            instance.get(BlockKeyProto::new)
        }
    }

    // required uint32 keyId = 1;

    pub fn clear_keyId(&mut self) {
        self.keyId = ::std::option::Option::None;
    }

    pub fn has_keyId(&self) -> bool {
        self.keyId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyId(&mut self, v: u32) {
        self.keyId = ::std::option::Option::Some(v);
    }

    pub fn get_keyId(&self) -> u32 {
        self.keyId.unwrap_or(0)
    }

    fn get_keyId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.keyId
    }

    fn mut_keyId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.keyId
    }

    // required uint64 expiryDate = 2;

    pub fn clear_expiryDate(&mut self) {
        self.expiryDate = ::std::option::Option::None;
    }

    pub fn has_expiryDate(&self) -> bool {
        self.expiryDate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expiryDate(&mut self, v: u64) {
        self.expiryDate = ::std::option::Option::Some(v);
    }

    pub fn get_expiryDate(&self) -> u64 {
        self.expiryDate.unwrap_or(0)
    }

    fn get_expiryDate_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.expiryDate
    }

    fn mut_expiryDate_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.expiryDate
    }

    // optional bytes keyBytes = 3;

    pub fn clear_keyBytes(&mut self) {
        self.keyBytes.clear();
    }

    pub fn has_keyBytes(&self) -> bool {
        self.keyBytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyBytes(&mut self, v: ::std::vec::Vec<u8>) {
        self.keyBytes = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_keyBytes(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.keyBytes.is_none() {
            self.keyBytes.set_default();
        }
        self.keyBytes.as_mut().unwrap()
    }

    // Take field
    pub fn take_keyBytes(&mut self) -> ::std::vec::Vec<u8> {
        self.keyBytes.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_keyBytes(&self) -> &[u8] {
        match self.keyBytes.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_keyBytes_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.keyBytes
    }

    fn mut_keyBytes_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.keyBytes
    }
}

impl ::protobuf::Message for BlockKeyProto {
    fn is_initialized(&self) -> bool {
        if self.keyId.is_none() {
            return false;
        }
        if self.expiryDate.is_none() {
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
                    self.keyId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.expiryDate = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.keyBytes)?;
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
        if let Some(v) = self.keyId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.expiryDate {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.keyBytes.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.keyId {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.expiryDate {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.keyBytes.as_ref() {
            os.write_bytes(3, &v)?;
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

impl ::protobuf::MessageStatic for BlockKeyProto {
    fn new() -> BlockKeyProto {
        BlockKeyProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockKeyProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "keyId",
                    BlockKeyProto::get_keyId_for_reflect,
                    BlockKeyProto::mut_keyId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "expiryDate",
                    BlockKeyProto::get_expiryDate_for_reflect,
                    BlockKeyProto::mut_expiryDate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "keyBytes",
                    BlockKeyProto::get_keyBytes_for_reflect,
                    BlockKeyProto::mut_keyBytes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockKeyProto>(
                    "BlockKeyProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockKeyProto {
    fn clear(&mut self) {
        self.clear_keyId();
        self.clear_expiryDate();
        self.clear_keyBytes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockKeyProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockKeyProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ExportedBlockKeysProto {
    // message fields
    isBlockTokenEnabled: ::std::option::Option<bool>,
    keyUpdateInterval: ::std::option::Option<u64>,
    tokenLifeTime: ::std::option::Option<u64>,
    currentKey: ::protobuf::SingularPtrField<BlockKeyProto>,
    allKeys: ::protobuf::RepeatedField<BlockKeyProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExportedBlockKeysProto {}

impl ExportedBlockKeysProto {
    pub fn new() -> ExportedBlockKeysProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExportedBlockKeysProto {
        static mut instance: ::protobuf::lazy::Lazy<ExportedBlockKeysProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExportedBlockKeysProto,
        };
        unsafe {
            instance.get(ExportedBlockKeysProto::new)
        }
    }

    // required bool isBlockTokenEnabled = 1;

    pub fn clear_isBlockTokenEnabled(&mut self) {
        self.isBlockTokenEnabled = ::std::option::Option::None;
    }

    pub fn has_isBlockTokenEnabled(&self) -> bool {
        self.isBlockTokenEnabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_isBlockTokenEnabled(&mut self, v: bool) {
        self.isBlockTokenEnabled = ::std::option::Option::Some(v);
    }

    pub fn get_isBlockTokenEnabled(&self) -> bool {
        self.isBlockTokenEnabled.unwrap_or(false)
    }

    fn get_isBlockTokenEnabled_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.isBlockTokenEnabled
    }

    fn mut_isBlockTokenEnabled_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.isBlockTokenEnabled
    }

    // required uint64 keyUpdateInterval = 2;

    pub fn clear_keyUpdateInterval(&mut self) {
        self.keyUpdateInterval = ::std::option::Option::None;
    }

    pub fn has_keyUpdateInterval(&self) -> bool {
        self.keyUpdateInterval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyUpdateInterval(&mut self, v: u64) {
        self.keyUpdateInterval = ::std::option::Option::Some(v);
    }

    pub fn get_keyUpdateInterval(&self) -> u64 {
        self.keyUpdateInterval.unwrap_or(0)
    }

    fn get_keyUpdateInterval_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.keyUpdateInterval
    }

    fn mut_keyUpdateInterval_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.keyUpdateInterval
    }

    // required uint64 tokenLifeTime = 3;

    pub fn clear_tokenLifeTime(&mut self) {
        self.tokenLifeTime = ::std::option::Option::None;
    }

    pub fn has_tokenLifeTime(&self) -> bool {
        self.tokenLifeTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tokenLifeTime(&mut self, v: u64) {
        self.tokenLifeTime = ::std::option::Option::Some(v);
    }

    pub fn get_tokenLifeTime(&self) -> u64 {
        self.tokenLifeTime.unwrap_or(0)
    }

    fn get_tokenLifeTime_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.tokenLifeTime
    }

    fn mut_tokenLifeTime_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.tokenLifeTime
    }

    // required .hadoop.hdfs.BlockKeyProto currentKey = 4;

    pub fn clear_currentKey(&mut self) {
        self.currentKey.clear();
    }

    pub fn has_currentKey(&self) -> bool {
        self.currentKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentKey(&mut self, v: BlockKeyProto) {
        self.currentKey = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currentKey(&mut self) -> &mut BlockKeyProto {
        if self.currentKey.is_none() {
            self.currentKey.set_default();
        }
        self.currentKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_currentKey(&mut self) -> BlockKeyProto {
        self.currentKey.take().unwrap_or_else(|| BlockKeyProto::new())
    }

    pub fn get_currentKey(&self) -> &BlockKeyProto {
        self.currentKey.as_ref().unwrap_or_else(|| BlockKeyProto::default_instance())
    }

    fn get_currentKey_for_reflect(&self) -> &::protobuf::SingularPtrField<BlockKeyProto> {
        &self.currentKey
    }

    fn mut_currentKey_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BlockKeyProto> {
        &mut self.currentKey
    }

    // repeated .hadoop.hdfs.BlockKeyProto allKeys = 5;

    pub fn clear_allKeys(&mut self) {
        self.allKeys.clear();
    }

    // Param is passed by value, moved
    pub fn set_allKeys(&mut self, v: ::protobuf::RepeatedField<BlockKeyProto>) {
        self.allKeys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_allKeys(&mut self) -> &mut ::protobuf::RepeatedField<BlockKeyProto> {
        &mut self.allKeys
    }

    // Take field
    pub fn take_allKeys(&mut self) -> ::protobuf::RepeatedField<BlockKeyProto> {
        ::std::mem::replace(&mut self.allKeys, ::protobuf::RepeatedField::new())
    }

    pub fn get_allKeys(&self) -> &[BlockKeyProto] {
        &self.allKeys
    }

    fn get_allKeys_for_reflect(&self) -> &::protobuf::RepeatedField<BlockKeyProto> {
        &self.allKeys
    }

    fn mut_allKeys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<BlockKeyProto> {
        &mut self.allKeys
    }
}

impl ::protobuf::Message for ExportedBlockKeysProto {
    fn is_initialized(&self) -> bool {
        if self.isBlockTokenEnabled.is_none() {
            return false;
        }
        if self.keyUpdateInterval.is_none() {
            return false;
        }
        if self.tokenLifeTime.is_none() {
            return false;
        }
        if self.currentKey.is_none() {
            return false;
        }
        for v in &self.currentKey {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.allKeys {
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
                    let tmp = is.read_bool()?;
                    self.isBlockTokenEnabled = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.keyUpdateInterval = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.tokenLifeTime = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.currentKey)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.allKeys)?;
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
        if let Some(v) = self.isBlockTokenEnabled {
            my_size += 2;
        }
        if let Some(v) = self.keyUpdateInterval {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.tokenLifeTime {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.currentKey.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.allKeys {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.isBlockTokenEnabled {
            os.write_bool(1, v)?;
        }
        if let Some(v) = self.keyUpdateInterval {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.tokenLifeTime {
            os.write_uint64(3, v)?;
        }
        if let Some(ref v) = self.currentKey.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.allKeys {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ExportedBlockKeysProto {
    fn new() -> ExportedBlockKeysProto {
        ExportedBlockKeysProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExportedBlockKeysProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "isBlockTokenEnabled",
                    ExportedBlockKeysProto::get_isBlockTokenEnabled_for_reflect,
                    ExportedBlockKeysProto::mut_isBlockTokenEnabled_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "keyUpdateInterval",
                    ExportedBlockKeysProto::get_keyUpdateInterval_for_reflect,
                    ExportedBlockKeysProto::mut_keyUpdateInterval_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "tokenLifeTime",
                    ExportedBlockKeysProto::get_tokenLifeTime_for_reflect,
                    ExportedBlockKeysProto::mut_tokenLifeTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockKeyProto>>(
                    "currentKey",
                    ExportedBlockKeysProto::get_currentKey_for_reflect,
                    ExportedBlockKeysProto::mut_currentKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockKeyProto>>(
                    "allKeys",
                    ExportedBlockKeysProto::get_allKeys_for_reflect,
                    ExportedBlockKeysProto::mut_allKeys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExportedBlockKeysProto>(
                    "ExportedBlockKeysProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExportedBlockKeysProto {
    fn clear(&mut self) {
        self.clear_isBlockTokenEnabled();
        self.clear_keyUpdateInterval();
        self.clear_tokenLifeTime();
        self.clear_currentKey();
        self.clear_allKeys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExportedBlockKeysProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExportedBlockKeysProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RecoveringBlockProto {
    // message fields
    newGenStamp: ::std::option::Option<u64>,
    block: ::protobuf::SingularPtrField<LocatedBlockProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RecoveringBlockProto {}

impl RecoveringBlockProto {
    pub fn new() -> RecoveringBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RecoveringBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<RecoveringBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RecoveringBlockProto,
        };
        unsafe {
            instance.get(RecoveringBlockProto::new)
        }
    }

    // required uint64 newGenStamp = 1;

    pub fn clear_newGenStamp(&mut self) {
        self.newGenStamp = ::std::option::Option::None;
    }

    pub fn has_newGenStamp(&self) -> bool {
        self.newGenStamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_newGenStamp(&mut self, v: u64) {
        self.newGenStamp = ::std::option::Option::Some(v);
    }

    pub fn get_newGenStamp(&self) -> u64 {
        self.newGenStamp.unwrap_or(0)
    }

    fn get_newGenStamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.newGenStamp
    }

    fn mut_newGenStamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.newGenStamp
    }

    // required .hadoop.hdfs.LocatedBlockProto block = 2;

    pub fn clear_block(&mut self) {
        self.block.clear();
    }

    pub fn has_block(&self) -> bool {
        self.block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block(&mut self, v: LocatedBlockProto) {
        self.block = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block(&mut self) -> &mut LocatedBlockProto {
        if self.block.is_none() {
            self.block.set_default();
        }
        self.block.as_mut().unwrap()
    }

    // Take field
    pub fn take_block(&mut self) -> LocatedBlockProto {
        self.block.take().unwrap_or_else(|| LocatedBlockProto::new())
    }

    pub fn get_block(&self) -> &LocatedBlockProto {
        self.block.as_ref().unwrap_or_else(|| LocatedBlockProto::default_instance())
    }

    fn get_block_for_reflect(&self) -> &::protobuf::SingularPtrField<LocatedBlockProto> {
        &self.block
    }

    fn mut_block_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<LocatedBlockProto> {
        &mut self.block
    }
}

impl ::protobuf::Message for RecoveringBlockProto {
    fn is_initialized(&self) -> bool {
        if self.newGenStamp.is_none() {
            return false;
        }
        if self.block.is_none() {
            return false;
        }
        for v in &self.block {
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
                    let tmp = is.read_uint64()?;
                    self.newGenStamp = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.block)?;
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
        if let Some(v) = self.newGenStamp {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.block.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.newGenStamp {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.block.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for RecoveringBlockProto {
    fn new() -> RecoveringBlockProto {
        RecoveringBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RecoveringBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "newGenStamp",
                    RecoveringBlockProto::get_newGenStamp_for_reflect,
                    RecoveringBlockProto::mut_newGenStamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<LocatedBlockProto>>(
                    "block",
                    RecoveringBlockProto::get_block_for_reflect,
                    RecoveringBlockProto::mut_block_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RecoveringBlockProto>(
                    "RecoveringBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RecoveringBlockProto {
    fn clear(&mut self) {
        self.clear_newGenStamp();
        self.clear_block();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RecoveringBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RecoveringBlockProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VersionRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VersionRequestProto {}

impl VersionRequestProto {
    pub fn new() -> VersionRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VersionRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<VersionRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VersionRequestProto,
        };
        unsafe {
            instance.get(VersionRequestProto::new)
        }
    }
}

impl ::protobuf::Message for VersionRequestProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for VersionRequestProto {
    fn new() -> VersionRequestProto {
        VersionRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<VersionRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<VersionRequestProto>(
                    "VersionRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VersionRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VersionRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VersionRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VersionResponseProto {
    // message fields
    info: ::protobuf::SingularPtrField<NamespaceInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VersionResponseProto {}

impl VersionResponseProto {
    pub fn new() -> VersionResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VersionResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<VersionResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VersionResponseProto,
        };
        unsafe {
            instance.get(VersionResponseProto::new)
        }
    }

    // required .hadoop.hdfs.NamespaceInfoProto info = 1;

    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    pub fn has_info(&self) -> bool {
        self.info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: NamespaceInfoProto) {
        self.info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info(&mut self) -> &mut NamespaceInfoProto {
        if self.info.is_none() {
            self.info.set_default();
        }
        self.info.as_mut().unwrap()
    }

    // Take field
    pub fn take_info(&mut self) -> NamespaceInfoProto {
        self.info.take().unwrap_or_else(|| NamespaceInfoProto::new())
    }

    pub fn get_info(&self) -> &NamespaceInfoProto {
        self.info.as_ref().unwrap_or_else(|| NamespaceInfoProto::default_instance())
    }

    fn get_info_for_reflect(&self) -> &::protobuf::SingularPtrField<NamespaceInfoProto> {
        &self.info
    }

    fn mut_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<NamespaceInfoProto> {
        &mut self.info
    }
}

impl ::protobuf::Message for VersionResponseProto {
    fn is_initialized(&self) -> bool {
        if self.info.is_none() {
            return false;
        }
        for v in &self.info {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.info)?;
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
        if let Some(ref v) = self.info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.info.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for VersionResponseProto {
    fn new() -> VersionResponseProto {
        VersionResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<VersionResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<NamespaceInfoProto>>(
                    "info",
                    VersionResponseProto::get_info_for_reflect,
                    VersionResponseProto::mut_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VersionResponseProto>(
                    "VersionResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VersionResponseProto {
    fn clear(&mut self) {
        self.clear_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VersionResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VersionResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotInfoProto {
    // message fields
    snapshotName: ::protobuf::SingularField<::std::string::String>,
    snapshotRoot: ::protobuf::SingularField<::std::string::String>,
    permission: ::protobuf::SingularPtrField<FsPermissionProto>,
    owner: ::protobuf::SingularField<::std::string::String>,
    group: ::protobuf::SingularField<::std::string::String>,
    createTime: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotInfoProto {}

impl SnapshotInfoProto {
    pub fn new() -> SnapshotInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotInfoProto,
        };
        unsafe {
            instance.get(SnapshotInfoProto::new)
        }
    }

    // required string snapshotName = 1;

    pub fn clear_snapshotName(&mut self) {
        self.snapshotName.clear();
    }

    pub fn has_snapshotName(&self) -> bool {
        self.snapshotName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshotName(&mut self, v: ::std::string::String) {
        self.snapshotName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_snapshotName(&mut self) -> &mut ::std::string::String {
        if self.snapshotName.is_none() {
            self.snapshotName.set_default();
        }
        self.snapshotName.as_mut().unwrap()
    }

    // Take field
    pub fn take_snapshotName(&mut self) -> ::std::string::String {
        self.snapshotName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_snapshotName(&self) -> &str {
        match self.snapshotName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_snapshotName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.snapshotName
    }

    fn mut_snapshotName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.snapshotName
    }

    // required string snapshotRoot = 2;

    pub fn clear_snapshotRoot(&mut self) {
        self.snapshotRoot.clear();
    }

    pub fn has_snapshotRoot(&self) -> bool {
        self.snapshotRoot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshotRoot(&mut self, v: ::std::string::String) {
        self.snapshotRoot = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_snapshotRoot(&mut self) -> &mut ::std::string::String {
        if self.snapshotRoot.is_none() {
            self.snapshotRoot.set_default();
        }
        self.snapshotRoot.as_mut().unwrap()
    }

    // Take field
    pub fn take_snapshotRoot(&mut self) -> ::std::string::String {
        self.snapshotRoot.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_snapshotRoot(&self) -> &str {
        match self.snapshotRoot.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_snapshotRoot_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.snapshotRoot
    }

    fn mut_snapshotRoot_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.snapshotRoot
    }

    // required .hadoop.hdfs.FsPermissionProto permission = 3;

    pub fn clear_permission(&mut self) {
        self.permission.clear();
    }

    pub fn has_permission(&self) -> bool {
        self.permission.is_some()
    }

    // Param is passed by value, moved
    pub fn set_permission(&mut self, v: FsPermissionProto) {
        self.permission = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_permission(&mut self) -> &mut FsPermissionProto {
        if self.permission.is_none() {
            self.permission.set_default();
        }
        self.permission.as_mut().unwrap()
    }

    // Take field
    pub fn take_permission(&mut self) -> FsPermissionProto {
        self.permission.take().unwrap_or_else(|| FsPermissionProto::new())
    }

    pub fn get_permission(&self) -> &FsPermissionProto {
        self.permission.as_ref().unwrap_or_else(|| FsPermissionProto::default_instance())
    }

    fn get_permission_for_reflect(&self) -> &::protobuf::SingularPtrField<FsPermissionProto> {
        &self.permission
    }

    fn mut_permission_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FsPermissionProto> {
        &mut self.permission
    }

    // required string owner = 4;

    pub fn clear_owner(&mut self) {
        self.owner.clear();
    }

    pub fn has_owner(&self) -> bool {
        self.owner.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner(&mut self, v: ::std::string::String) {
        self.owner = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_owner(&mut self) -> &mut ::std::string::String {
        if self.owner.is_none() {
            self.owner.set_default();
        }
        self.owner.as_mut().unwrap()
    }

    // Take field
    pub fn take_owner(&mut self) -> ::std::string::String {
        self.owner.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_owner(&self) -> &str {
        match self.owner.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_owner_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.owner
    }

    fn mut_owner_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.owner
    }

    // required string group = 5;

    pub fn clear_group(&mut self) {
        self.group.clear();
    }

    pub fn has_group(&self) -> bool {
        self.group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group(&mut self, v: ::std::string::String) {
        self.group = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group(&mut self) -> &mut ::std::string::String {
        if self.group.is_none() {
            self.group.set_default();
        }
        self.group.as_mut().unwrap()
    }

    // Take field
    pub fn take_group(&mut self) -> ::std::string::String {
        self.group.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_group(&self) -> &str {
        match self.group.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_group_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.group
    }

    fn mut_group_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.group
    }

    // required string createTime = 6;

    pub fn clear_createTime(&mut self) {
        self.createTime.clear();
    }

    pub fn has_createTime(&self) -> bool {
        self.createTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_createTime(&mut self, v: ::std::string::String) {
        self.createTime = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_createTime(&mut self) -> &mut ::std::string::String {
        if self.createTime.is_none() {
            self.createTime.set_default();
        }
        self.createTime.as_mut().unwrap()
    }

    // Take field
    pub fn take_createTime(&mut self) -> ::std::string::String {
        self.createTime.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_createTime(&self) -> &str {
        match self.createTime.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_createTime_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.createTime
    }

    fn mut_createTime_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.createTime
    }
}

impl ::protobuf::Message for SnapshotInfoProto {
    fn is_initialized(&self) -> bool {
        if self.snapshotName.is_none() {
            return false;
        }
        if self.snapshotRoot.is_none() {
            return false;
        }
        if self.permission.is_none() {
            return false;
        }
        if self.owner.is_none() {
            return false;
        }
        if self.group.is_none() {
            return false;
        }
        if self.createTime.is_none() {
            return false;
        }
        for v in &self.permission {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.snapshotName)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.snapshotRoot)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.permission)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.owner)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.group)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.createTime)?;
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
        if let Some(ref v) = self.snapshotName.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.snapshotRoot.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.permission.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.owner.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.group.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.createTime.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.snapshotName.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.snapshotRoot.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.permission.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.owner.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.group.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.createTime.as_ref() {
            os.write_string(6, &v)?;
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

impl ::protobuf::MessageStatic for SnapshotInfoProto {
    fn new() -> SnapshotInfoProto {
        SnapshotInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "snapshotName",
                    SnapshotInfoProto::get_snapshotName_for_reflect,
                    SnapshotInfoProto::mut_snapshotName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "snapshotRoot",
                    SnapshotInfoProto::get_snapshotRoot_for_reflect,
                    SnapshotInfoProto::mut_snapshotRoot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FsPermissionProto>>(
                    "permission",
                    SnapshotInfoProto::get_permission_for_reflect,
                    SnapshotInfoProto::mut_permission_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "owner",
                    SnapshotInfoProto::get_owner_for_reflect,
                    SnapshotInfoProto::mut_owner_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "group",
                    SnapshotInfoProto::get_group_for_reflect,
                    SnapshotInfoProto::mut_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "createTime",
                    SnapshotInfoProto::get_createTime_for_reflect,
                    SnapshotInfoProto::mut_createTime_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotInfoProto>(
                    "SnapshotInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotInfoProto {
    fn clear(&mut self) {
        self.clear_snapshotName();
        self.clear_snapshotRoot();
        self.clear_permission();
        self.clear_owner();
        self.clear_group();
        self.clear_createTime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RollingUpgradeStatusProto {
    // message fields
    blockPoolId: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RollingUpgradeStatusProto {}

impl RollingUpgradeStatusProto {
    pub fn new() -> RollingUpgradeStatusProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RollingUpgradeStatusProto {
        static mut instance: ::protobuf::lazy::Lazy<RollingUpgradeStatusProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RollingUpgradeStatusProto,
        };
        unsafe {
            instance.get(RollingUpgradeStatusProto::new)
        }
    }

    // required string blockPoolId = 1;

    pub fn clear_blockPoolId(&mut self) {
        self.blockPoolId.clear();
    }

    pub fn has_blockPoolId(&self) -> bool {
        self.blockPoolId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPoolId(&mut self, v: ::std::string::String) {
        self.blockPoolId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockPoolId(&mut self) -> &mut ::std::string::String {
        if self.blockPoolId.is_none() {
            self.blockPoolId.set_default();
        }
        self.blockPoolId.as_mut().unwrap()
    }

    // Take field
    pub fn take_blockPoolId(&mut self) -> ::std::string::String {
        self.blockPoolId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_blockPoolId(&self) -> &str {
        match self.blockPoolId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_blockPoolId_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.blockPoolId
    }

    fn mut_blockPoolId_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.blockPoolId
    }
}

impl ::protobuf::Message for RollingUpgradeStatusProto {
    fn is_initialized(&self) -> bool {
        if self.blockPoolId.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.blockPoolId)?;
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
        if let Some(ref v) = self.blockPoolId.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.blockPoolId.as_ref() {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for RollingUpgradeStatusProto {
    fn new() -> RollingUpgradeStatusProto {
        RollingUpgradeStatusProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RollingUpgradeStatusProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "blockPoolId",
                    RollingUpgradeStatusProto::get_blockPoolId_for_reflect,
                    RollingUpgradeStatusProto::mut_blockPoolId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RollingUpgradeStatusProto>(
                    "RollingUpgradeStatusProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RollingUpgradeStatusProto {
    fn clear(&mut self) {
        self.clear_blockPoolId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RollingUpgradeStatusProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RollingUpgradeStatusProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum StorageTypeProto {
    DISK = 1,
    SSD = 2,
    ARCHIVE = 3,
    RAM_DISK = 4,
}

impl ::protobuf::ProtobufEnum for StorageTypeProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<StorageTypeProto> {
        match value {
            1 => ::std::option::Option::Some(StorageTypeProto::DISK),
            2 => ::std::option::Option::Some(StorageTypeProto::SSD),
            3 => ::std::option::Option::Some(StorageTypeProto::ARCHIVE),
            4 => ::std::option::Option::Some(StorageTypeProto::RAM_DISK),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [StorageTypeProto] = &[
            StorageTypeProto::DISK,
            StorageTypeProto::SSD,
            StorageTypeProto::ARCHIVE,
            StorageTypeProto::RAM_DISK,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<StorageTypeProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("StorageTypeProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for StorageTypeProto {
}

impl ::protobuf::reflect::ProtobufValue for StorageTypeProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CipherSuiteProto {
    UNKNOWN = 1,
    AES_CTR_NOPADDING = 2,
}

impl ::protobuf::ProtobufEnum for CipherSuiteProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CipherSuiteProto> {
        match value {
            1 => ::std::option::Option::Some(CipherSuiteProto::UNKNOWN),
            2 => ::std::option::Option::Some(CipherSuiteProto::AES_CTR_NOPADDING),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CipherSuiteProto] = &[
            CipherSuiteProto::UNKNOWN,
            CipherSuiteProto::AES_CTR_NOPADDING,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CipherSuiteProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CipherSuiteProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CipherSuiteProto {
}

impl ::protobuf::reflect::ProtobufValue for CipherSuiteProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CryptoProtocolVersionProto {
    UNKNOWN_PROTOCOL_VERSION = 1,
    ENCRYPTION_ZONES = 2,
}

impl ::protobuf::ProtobufEnum for CryptoProtocolVersionProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CryptoProtocolVersionProto> {
        match value {
            1 => ::std::option::Option::Some(CryptoProtocolVersionProto::UNKNOWN_PROTOCOL_VERSION),
            2 => ::std::option::Option::Some(CryptoProtocolVersionProto::ENCRYPTION_ZONES),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CryptoProtocolVersionProto] = &[
            CryptoProtocolVersionProto::UNKNOWN_PROTOCOL_VERSION,
            CryptoProtocolVersionProto::ENCRYPTION_ZONES,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CryptoProtocolVersionProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CryptoProtocolVersionProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CryptoProtocolVersionProto {
}

impl ::protobuf::reflect::ProtobufValue for CryptoProtocolVersionProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ChecksumTypeProto {
    CHECKSUM_NULL = 0,
    CHECKSUM_CRC32 = 1,
    CHECKSUM_CRC32C = 2,
}

impl ::protobuf::ProtobufEnum for ChecksumTypeProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ChecksumTypeProto> {
        match value {
            0 => ::std::option::Option::Some(ChecksumTypeProto::CHECKSUM_NULL),
            1 => ::std::option::Option::Some(ChecksumTypeProto::CHECKSUM_CRC32),
            2 => ::std::option::Option::Some(ChecksumTypeProto::CHECKSUM_CRC32C),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ChecksumTypeProto] = &[
            ChecksumTypeProto::CHECKSUM_NULL,
            ChecksumTypeProto::CHECKSUM_CRC32,
            ChecksumTypeProto::CHECKSUM_CRC32C,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ChecksumTypeProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ChecksumTypeProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ChecksumTypeProto {
}

impl ::protobuf::reflect::ProtobufValue for ChecksumTypeProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ReplicaStateProto {
    FINALIZED = 0,
    RBW = 1,
    RWR = 2,
    RUR = 3,
    TEMPORARY = 4,
}

impl ::protobuf::ProtobufEnum for ReplicaStateProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReplicaStateProto> {
        match value {
            0 => ::std::option::Option::Some(ReplicaStateProto::FINALIZED),
            1 => ::std::option::Option::Some(ReplicaStateProto::RBW),
            2 => ::std::option::Option::Some(ReplicaStateProto::RWR),
            3 => ::std::option::Option::Some(ReplicaStateProto::RUR),
            4 => ::std::option::Option::Some(ReplicaStateProto::TEMPORARY),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ReplicaStateProto] = &[
            ReplicaStateProto::FINALIZED,
            ReplicaStateProto::RBW,
            ReplicaStateProto::RWR,
            ReplicaStateProto::RUR,
            ReplicaStateProto::TEMPORARY,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ReplicaStateProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ReplicaStateProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ReplicaStateProto {
}

impl ::protobuf::reflect::ProtobufValue for ReplicaStateProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nhdfs.proto\x12\x0bhadoop.hdfs\x1a\x0eSecurity.proto\"c\n\x12Extended\
    BlockProto\x12\x0e\n\x06poolId\x18\x01\x20\x02(\t\x12\x0f\n\x07blockId\
    \x18\x02\x20\x02(\x04\x12\x17\n\x0fgenerationStamp\x18\x03\x20\x02(\x04\
    \x12\x13\n\x08numBytes\x18\x04\x20\x01(\x04:\x010\"\x99\x01\n\x0fDatanod\
    eIDProto\x12\x0e\n\x06ipAddr\x18\x01\x20\x02(\t\x12\x10\n\x08hostName\
    \x18\x02\x20\x02(\t\x12\x14\n\x0cdatanodeUuid\x18\x03\x20\x02(\t\x12\x10\
    \n\x08xferPort\x18\x04\x20\x02(\r\x12\x10\n\x08infoPort\x18\x05\x20\x02(\
    \r\x12\x0f\n\x07ipcPort\x18\x06\x20\x02(\r\x12\x19\n\x0einfoSecurePort\
    \x18\x07\x20\x01(\r:\x010\"X\n\x16DatanodeLocalInfoProto\x12\x17\n\x0fso\
    ftwareVersion\x18\x01\x20\x02(\t\x12\x15\n\rconfigVersion\x18\x02\x20\
    \x02(\t\x12\x0e\n\x06uptime\x18\x03\x20\x02(\x04\"G\n\x12DatanodeInfosPr\
    oto\x121\n\tdatanodes\x18\x01\x20\x03(\x0b2\x1e.hadoop.hdfs.DatanodeInfo\
    Proto\"\x9a\x03\n\x11DatanodeInfoProto\x12(\n\x02id\x18\x01\x20\x02(\x0b\
    2\x1c.hadoop.hdfs.DatanodeIDProto\x12\x13\n\x08capacity\x18\x02\x20\x01(\
    \x04:\x010\x12\x12\n\x07dfsUsed\x18\x03\x20\x01(\x04:\x010\x12\x14\n\tre\
    maining\x18\x04\x20\x01(\x04:\x010\x12\x18\n\rblockPoolUsed\x18\x05\x20\
    \x01(\x04:\x010\x12\x15\n\nlastUpdate\x18\x06\x20\x01(\x04:\x010\x12\x17\
    \n\x0cxceiverCount\x18\x07\x20\x01(\r:\x010\x12\x10\n\x08location\x18\
    \x08\x20\x01(\t\x12E\n\nadminState\x18\n\x20\x01(\x0e2).hadoop.hdfs.Data\
    nodeInfoProto.AdminState:\x06NORMAL\x12\x18\n\rcacheCapacity\x18\x0b\x20\
    \x01(\x04:\x010\x12\x14\n\tcacheUsed\x18\x0c\x20\x01(\x04:\x010\"I\n\nAd\
    minState\x12\n\n\x06NORMAL\x10\0\x12\x1b\n\x17DECOMMISSION_INPROGRESS\
    \x10\x01\x12\x12\n\x0eDECOMMISSIONED\x10\x02\"\xde\x01\n\x14DatanodeStor\
    ageProto\x12\x13\n\x0bstorageUuid\x18\x01\x20\x02(\t\x12E\n\x05state\x18\
    \x02\x20\x01(\x0e2..hadoop.hdfs.DatanodeStorageProto.StorageState:\x06NO\
    RMAL\x128\n\x0bstorageType\x18\x03\x20\x01(\x0e2\x1d.hadoop.hdfs.Storage\
    TypeProto:\x04DISK\"0\n\x0cStorageState\x12\n\n\x06NORMAL\x10\0\x12\x14\
    \n\x10READ_ONLY_SHARED\x10\x01\"\xd1\x01\n\x12StorageReportProto\x12\x17\
    \n\x0bstorageUuid\x18\x01\x20\x02(\tB\x02\x18\x01\x12\x15\n\x06failed\
    \x18\x02\x20\x01(\x08:\x05false\x12\x13\n\x08capacity\x18\x03\x20\x01(\
    \x04:\x010\x12\x12\n\x07dfsUsed\x18\x04\x20\x01(\x04:\x010\x12\x14\n\tre\
    maining\x18\x05\x20\x01(\x04:\x010\x12\x18\n\rblockPoolUsed\x18\x06\x20\
    \x01(\x04:\x010\x122\n\x07storage\x18\x07\x20\x01(\x0b2!.hadoop.hdfs.Dat\
    anodeStorageProto\"\x8a\x01\n\x13ContentSummaryProto\x12\x0e\n\x06length\
    \x18\x01\x20\x02(\x04\x12\x11\n\tfileCount\x18\x02\x20\x02(\x04\x12\x16\
    \n\x0edirectoryCount\x18\x03\x20\x02(\x04\x12\r\n\x05quota\x18\x04\x20\
    \x02(\x04\x12\x15\n\rspaceConsumed\x18\x05\x20\x02(\x04\x12\x12\n\nspace\
    Quota\x18\x06\x20\x02(\x04\"7\n\x16CorruptFileBlocksProto\x12\r\n\x05fil\
    es\x18\x01\x20\x03(\t\x12\x0e\n\x06cookie\x18\x02\x20\x02(\t\"!\n\x11FsP\
    ermissionProto\x12\x0c\n\x04perm\x18\x01\x20\x02(\r\"H\n\x11StorageTypes\
    Proto\x123\n\x0cstorageTypes\x18\x01\x20\x03(\x0e2\x1d.hadoop.hdfs.Stora\
    geTypeProto\"\xf4\x01\n\x17BlockStoragePolicyProto\x12\x10\n\x08policyId\
    \x18\x01\x20\x02(\r\x12\x0c\n\x04name\x18\x02\x20\x02(\t\x126\n\x0ecreat\
    ionPolicy\x18\x03\x20\x02(\x0b2\x1e.hadoop.hdfs.StorageTypesProto\x12>\n\
    \x16creationFallbackPolicy\x18\x04\x20\x01(\x0b2\x1e.hadoop.hdfs.Storage\
    TypesProto\x12A\n\x19replicationFallbackPolicy\x18\x05\x20\x01(\x0b2\x1e\
    .hadoop.hdfs.StorageTypesProto\")\n\x11StorageUuidsProto\x12\x14\n\x0cst\
    orageUuids\x18\x01\x20\x03(\t\"\x9c\x02\n\x11LocatedBlockProto\x12*\n\
    \x01b\x18\x01\x20\x02(\x0b2\x1f.hadoop.hdfs.ExtendedBlockProto\x12\x0e\n\
    \x06offset\x18\x02\x20\x02(\x04\x12,\n\x04locs\x18\x03\x20\x03(\x0b2\x1e\
    .hadoop.hdfs.DatanodeInfoProto\x12\x0f\n\x07corrupt\x18\x04\x20\x02(\x08\
    \x12-\n\nblockToken\x18\x05\x20\x02(\x0b2\x19.hadoop.common.TokenProto\
    \x12\x14\n\x08isCached\x18\x06\x20\x03(\x08B\x02\x10\x01\x123\n\x0cstora\
    geTypes\x18\x07\x20\x03(\x0e2\x1d.hadoop.hdfs.StorageTypeProto\x12\x12\n\
    \nstorageIDs\x18\x08\x20\x03(\t\"\x93\x01\n\x16DataEncryptionKeyProto\
    \x12\r\n\x05keyId\x18\x01\x20\x02(\r\x12\x13\n\x0bblockPoolId\x18\x02\
    \x20\x02(\t\x12\r\n\x05nonce\x18\x03\x20\x02(\x0c\x12\x15\n\rencryptionK\
    ey\x18\x04\x20\x02(\x0c\x12\x12\n\nexpiryDate\x18\x05\x20\x02(\x04\x12\
    \x1b\n\x13encryptionAlgorithm\x18\x06\x20\x01(\t\"\xd3\x01\n\x17FileEncr\
    yptionInfoProto\x12,\n\x05suite\x18\x01\x20\x02(\x0e2\x1d.hadoop.hdfs.Ci\
    pherSuiteProto\x12F\n\x15cryptoProtocolVersion\x18\x02\x20\x02(\x0e2'.ha\
    doop.hdfs.CryptoProtocolVersionProto\x12\x0b\n\x03key\x18\x03\x20\x02(\
    \x0c\x12\n\n\x02iv\x18\x04\x20\x02(\x0c\x12\x0f\n\x07keyName\x18\x05\x20\
    \x02(\t\x12\x18\n\x10ezKeyVersionName\x18\x06\x20\x02(\t\"O\n\x1aPerFile\
    EncryptionInfoProto\x12\x0b\n\x03key\x18\x01\x20\x02(\x0c\x12\n\n\x02iv\
    \x18\x02\x20\x02(\x0c\x12\x18\n\x10ezKeyVersionName\x18\x03\x20\x02(\t\"\
    \xa0\x01\n\x17ZoneEncryptionInfoProto\x12,\n\x05suite\x18\x01\x20\x02(\
    \x0e2\x1d.hadoop.hdfs.CipherSuiteProto\x12F\n\x15cryptoProtocolVersion\
    \x18\x02\x20\x02(\x0e2'.hadoop.hdfs.CryptoProtocolVersionProto\x12\x0f\n\
    \x07keyName\x18\x03\x20\x02(\t\"}\n\x11CipherOptionProto\x12,\n\x05suite\
    \x18\x01\x20\x02(\x0e2\x1d.hadoop.hdfs.CipherSuiteProto\x12\r\n\x05inKey\
    \x18\x02\x20\x01(\x0c\x12\x0c\n\x04inIv\x18\x03\x20\x01(\x0c\x12\x0e\n\
    \x06outKey\x18\x04\x20\x01(\x0c\x12\r\n\x05outIv\x18\x05\x20\x01(\x0c\"\
    \x85\x02\n\x12LocatedBlocksProto\x12\x12\n\nfileLength\x18\x01\x20\x02(\
    \x04\x12.\n\x06blocks\x18\x02\x20\x03(\x0b2\x1e.hadoop.hdfs.LocatedBlock\
    Proto\x12\x19\n\x11underConstruction\x18\x03\x20\x02(\x08\x121\n\tlastBl\
    ock\x18\x04\x20\x01(\x0b2\x1e.hadoop.hdfs.LocatedBlockProto\x12\x1b\n\
    \x13isLastBlockComplete\x18\x05\x20\x02(\x08\x12@\n\x12fileEncryptionInf\
    o\x18\x06\x20\x01(\x0b2$.hadoop.hdfs.FileEncryptionInfoProto\"\xa8\x04\n\
    \x13HdfsFileStatusProto\x12;\n\x08fileType\x18\x01\x20\x02(\x0e2).hadoop\
    .hdfs.HdfsFileStatusProto.FileType\x12\x0c\n\x04path\x18\x02\x20\x02(\
    \x0c\x12\x0e\n\x06length\x18\x03\x20\x02(\x04\x122\n\npermission\x18\x04\
    \x20\x02(\x0b2\x1e.hadoop.hdfs.FsPermissionProto\x12\r\n\x05owner\x18\
    \x05\x20\x02(\t\x12\r\n\x05group\x18\x06\x20\x02(\t\x12\x19\n\x11modific\
    ation_time\x18\x07\x20\x02(\x04\x12\x13\n\x0baccess_time\x18\x08\x20\x02\
    (\x04\x12\x0f\n\x07symlink\x18\t\x20\x01(\x0c\x12\x1c\n\x11block_replica\
    tion\x18\n\x20\x01(\r:\x010\x12\x14\n\tblocksize\x18\x0b\x20\x01(\x04:\
    \x010\x122\n\tlocations\x18\x0c\x20\x01(\x0b2\x1f.hadoop.hdfs.LocatedBlo\
    cksProto\x12\x11\n\x06fileId\x18\r\x20\x01(\x04:\x010\x12\x17\n\x0bchild\
    renNum\x18\x0e\x20\x01(\x05:\x02-1\x12@\n\x12fileEncryptionInfo\x18\x0f\
    \x20\x01(\x0b2$.hadoop.hdfs.FileEncryptionInfoProto\x12\x18\n\rstoragePo\
    licy\x18\x10\x20\x01(\r:\x010\"3\n\x08FileType\x12\n\n\x06IS_DIR\x10\x01\
    \x12\x0b\n\x07IS_FILE\x10\x02\x12\x0e\n\nIS_SYMLINK\x10\x03\"\x8e\x02\n\
    \x15FsServerDefaultsProto\x12\x11\n\tblockSize\x18\x01\x20\x02(\x04\x12\
    \x18\n\x10bytesPerChecksum\x18\x02\x20\x02(\r\x12\x17\n\x0fwritePacketSi\
    ze\x18\x03\x20\x02(\r\x12\x13\n\x0breplication\x18\x04\x20\x02(\r\x12\
    \x16\n\x0efileBufferSize\x18\x05\x20\x02(\r\x12\"\n\x13encryptDataTransf\
    er\x18\x06\x20\x01(\x08:\x05false\x12\x18\n\rtrashInterval\x18\x07\x20\
    \x01(\x04:\x010\x12D\n\x0cchecksumType\x18\x08\x20\x01(\x0e2\x1e.hadoop.\
    hdfs.ChecksumTypeProto:\x0eCHECKSUM_CRC32\"k\n\x15DirectoryListingProto\
    \x128\n\x0epartialListing\x18\x01\x20\x03(\x0b2\x20.hadoop.hdfs.HdfsFile\
    StatusProto\x12\x18\n\x10remainingEntries\x18\x02\x20\x02(\r\"\xa2\x01\n\
    !SnapshottableDirectoryStatusProto\x123\n\tdirStatus\x18\x01\x20\x02(\
    \x0b2\x20.hadoop.hdfs.HdfsFileStatusProto\x12\x16\n\x0esnapshot_quota\
    \x18\x02\x20\x02(\r\x12\x17\n\x0fsnapshot_number\x18\x03\x20\x02(\r\x12\
    \x17\n\x0fparent_fullpath\x18\x04\x20\x02(\x0c\"u\n\"SnapshottableDirect\
    oryListingProto\x12O\n\x17snapshottableDirListing\x18\x01\x20\x03(\x0b2.\
    .hadoop.hdfs.SnapshottableDirectoryStatusProto\"_\n\x1cSnapshotDiffRepor\
    tEntryProto\x12\x10\n\x08fullpath\x18\x01\x20\x02(\x0c\x12\x19\n\x11modi\
    ficationLabel\x18\x02\x20\x02(\t\x12\x12\n\ntargetPath\x18\x03\x20\x01(\
    \x0c\"\x9f\x01\n\x17SnapshotDiffReportProto\x12\x14\n\x0csnapshotRoot\
    \x18\x01\x20\x02(\t\x12\x14\n\x0cfromSnapshot\x18\x02\x20\x02(\t\x12\x12\
    \n\ntoSnapshot\x18\x03\x20\x02(\t\x12D\n\x11diffReportEntries\x18\x04\
    \x20\x03(\x0b2).hadoop.hdfs.SnapshotDiffReportEntryProto\"_\n\x10Storage\
    InfoProto\x12\x15\n\rlayoutVersion\x18\x01\x20\x02(\r\x12\x12\n\nnamespc\
    eID\x18\x02\x20\x02(\r\x12\x11\n\tclusterID\x18\x03\x20\x02(\t\x12\r\n\
    \x05cTime\x18\x04\x20\x02(\x04\"\x89\x02\n\x19NamenodeRegistrationProto\
    \x12\x12\n\nrpcAddress\x18\x01\x20\x02(\t\x12\x13\n\x0bhttpAddress\x18\
    \x02\x20\x02(\t\x122\n\x0bstorageInfo\x18\x03\x20\x02(\x0b2\x1d.hadoop.h\
    dfs.StorageInfoProto\x12P\n\x04role\x18\x04\x20\x01(\x0e28.hadoop.hdfs.N\
    amenodeRegistrationProto.NamenodeRoleProto:\x08NAMENODE\"=\n\x11Namenode\
    RoleProto\x12\x0c\n\x08NAMENODE\x10\x01\x12\n\n\x06BACKUP\x10\x02\x12\
    \x0e\n\nCHECKPOINT\x10\x03\"\x9d\x01\n\x18CheckpointSignatureProto\x12\
    \x13\n\x0bblockPoolId\x18\x01\x20\x02(\t\x12\x20\n\x18mostRecentCheckpoi\
    ntTxId\x18\x02\x20\x02(\x04\x12\x16\n\x0ecurSegmentTxId\x18\x03\x20\x02(\
    \x04\x122\n\x0bstorageInfo\x18\x04\x20\x02(\x0b2\x1d.hadoop.hdfs.Storage\
    InfoProto\"\xcc\x01\n\x14NamenodeCommandProto\x12\x0e\n\x06action\x18\
    \x01\x20\x02(\r\x124\n\x04type\x18\x02\x20\x02(\x0e2&.hadoop.hdfs.Nameno\
    deCommandProto.Type\x12:\n\rcheckpointCmd\x18\x03\x20\x01(\x0b2#.hadoop.\
    hdfs.CheckpointCommandProto\"2\n\x04Type\x12\x13\n\x0fNamenodeCommand\
    \x10\0\x12\x15\n\x11CheckPointCommand\x10\x01\"m\n\x16CheckpointCommandP\
    roto\x128\n\tsignature\x18\x01\x20\x02(\x0b2%.hadoop.hdfs.CheckpointSign\
    atureProto\x12\x19\n\x11needToReturnImage\x18\x02\x20\x02(\x08\"D\n\nBlo\
    ckProto\x12\x0f\n\x07blockId\x18\x01\x20\x02(\x04\x12\x10\n\x08genStamp\
    \x18\x02\x20\x02(\x04\x12\x13\n\x08numBytes\x18\x03\x20\x01(\x04:\x010\"\
    \xa3\x01\n\x17BlockWithLocationsProto\x12&\n\x05block\x18\x01\x20\x02(\
    \x0b2\x17.hadoop.hdfs.BlockProto\x12\x15\n\rdatanodeUuids\x18\x02\x20\
    \x03(\t\x12\x14\n\x0cstorageUuids\x18\x03\x20\x03(\t\x123\n\x0cstorageTy\
    pes\x18\x04\x20\x03(\x0e2\x1d.hadoop.hdfs.StorageTypeProto\"P\n\x18Block\
    sWithLocationsProto\x124\n\x06blocks\x18\x01\x20\x03(\x0b2$.hadoop.hdfs.\
    BlockWithLocationsProto\"U\n\x12RemoteEditLogProto\x12\x11\n\tstartTxId\
    \x18\x01\x20\x02(\x04\x12\x0f\n\x07endTxId\x18\x02\x20\x02(\x04\x12\x1b\
    \n\x0cisInProgress\x18\x03\x20\x01(\x08:\x05false\"K\n\x1aRemoteEditLogM\
    anifestProto\x12-\n\x04logs\x18\x01\x20\x03(\x0b2\x1f.hadoop.hdfs.Remote\
    EditLogProto\"\x9c\x01\n\x12NamespaceInfoProto\x12\x14\n\x0cbuildVersion\
    \x18\x01\x20\x02(\t\x12\x0e\n\x06unused\x18\x02\x20\x02(\r\x12\x13\n\x0b\
    blockPoolID\x18\x03\x20\x02(\t\x122\n\x0bstorageInfo\x18\x04\x20\x02(\
    \x0b2\x1d.hadoop.hdfs.StorageInfoProto\x12\x17\n\x0fsoftwareVersion\x18\
    \x05\x20\x02(\t\"D\n\rBlockKeyProto\x12\r\n\x05keyId\x18\x01\x20\x02(\r\
    \x12\x12\n\nexpiryDate\x18\x02\x20\x02(\x04\x12\x10\n\x08keyBytes\x18\
    \x03\x20\x01(\x0c\"\xc4\x01\n\x16ExportedBlockKeysProto\x12\x1b\n\x13isB\
    lockTokenEnabled\x18\x01\x20\x02(\x08\x12\x19\n\x11keyUpdateInterval\x18\
    \x02\x20\x02(\x04\x12\x15\n\rtokenLifeTime\x18\x03\x20\x02(\x04\x12.\n\n\
    currentKey\x18\x04\x20\x02(\x0b2\x1a.hadoop.hdfs.BlockKeyProto\x12+\n\
    \x07allKeys\x18\x05\x20\x03(\x0b2\x1a.hadoop.hdfs.BlockKeyProto\"Z\n\x14\
    RecoveringBlockProto\x12\x13\n\x0bnewGenStamp\x18\x01\x20\x02(\x04\x12-\
    \n\x05block\x18\x02\x20\x02(\x0b2\x1e.hadoop.hdfs.LocatedBlockProto\"\
    \x15\n\x13VersionRequestProto\"E\n\x14VersionResponseProto\x12-\n\x04inf\
    o\x18\x01\x20\x02(\x0b2\x1f.hadoop.hdfs.NamespaceInfoProto\"\xa5\x01\n\
    \x11SnapshotInfoProto\x12\x14\n\x0csnapshotName\x18\x01\x20\x02(\t\x12\
    \x14\n\x0csnapshotRoot\x18\x02\x20\x02(\t\x122\n\npermission\x18\x03\x20\
    \x02(\x0b2\x1e.hadoop.hdfs.FsPermissionProto\x12\r\n\x05owner\x18\x04\
    \x20\x02(\t\x12\r\n\x05group\x18\x05\x20\x02(\t\x12\x12\n\ncreateTime\
    \x18\x06\x20\x02(\t\"0\n\x19RollingUpgradeStatusProto\x12\x13\n\x0bblock\
    PoolId\x18\x01\x20\x02(\t*@\n\x10StorageTypeProto\x12\x08\n\x04DISK\x10\
    \x01\x12\x07\n\x03SSD\x10\x02\x12\x0b\n\x07ARCHIVE\x10\x03\x12\x0c\n\x08\
    RAM_DISK\x10\x04*6\n\x10CipherSuiteProto\x12\x0b\n\x07UNKNOWN\x10\x01\
    \x12\x15\n\x11AES_CTR_NOPADDING\x10\x02*P\n\x1aCryptoProtocolVersionProt\
    o\x12\x1c\n\x18UNKNOWN_PROTOCOL_VERSION\x10\x01\x12\x14\n\x10ENCRYPTION_\
    ZONES\x10\x02*O\n\x11ChecksumTypeProto\x12\x11\n\rCHECKSUM_NULL\x10\0\
    \x12\x12\n\x0eCHECKSUM_CRC32\x10\x01\x12\x13\n\x0fCHECKSUM_CRC32C\x10\
    \x02*L\n\x11ReplicaStateProto\x12\r\n\tFINALIZED\x10\0\x12\x07\n\x03RBW\
    \x10\x01\x12\x07\n\x03RWR\x10\x02\x12\x07\n\x03RUR\x10\x03\x12\r\n\tTEMP\
    ORARY\x10\x04B6\n%org.apache.hadoop.hdfs.protocol.protoB\nHdfsProtos\xa0\
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
