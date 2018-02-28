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
pub struct DataTransferEncryptorMessageProto {
    // message fields
    status: ::std::option::Option<DataTransferEncryptorMessageProto_DataTransferEncryptorStatus>,
    payload: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    message: ::protobuf::SingularField<::std::string::String>,
    cipherOption: ::protobuf::RepeatedField<super::hdfs::CipherOptionProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataTransferEncryptorMessageProto {}

impl DataTransferEncryptorMessageProto {
    pub fn new() -> DataTransferEncryptorMessageProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataTransferEncryptorMessageProto {
        static mut instance: ::protobuf::lazy::Lazy<DataTransferEncryptorMessageProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataTransferEncryptorMessageProto,
        };
        unsafe {
            instance.get(DataTransferEncryptorMessageProto::new)
        }
    }

    // required .hadoop.hdfs.DataTransferEncryptorMessageProto.DataTransferEncryptorStatus status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: DataTransferEncryptorMessageProto_DataTransferEncryptorStatus) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> DataTransferEncryptorMessageProto_DataTransferEncryptorStatus {
        self.status.unwrap_or(DataTransferEncryptorMessageProto_DataTransferEncryptorStatus::SUCCESS)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<DataTransferEncryptorMessageProto_DataTransferEncryptorStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<DataTransferEncryptorMessageProto_DataTransferEncryptorStatus> {
        &mut self.status
    }

    // optional bytes payload = 2;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.payload.is_none() {
            self.payload.set_default();
        }
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        self.payload.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_payload(&self) -> &[u8] {
        match self.payload.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_payload_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.payload
    }

    fn mut_payload_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.payload
    }

    // optional string message = 3;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        }
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.message
    }

    // repeated .hadoop.hdfs.CipherOptionProto cipherOption = 4;

    pub fn clear_cipherOption(&mut self) {
        self.cipherOption.clear();
    }

    // Param is passed by value, moved
    pub fn set_cipherOption(&mut self, v: ::protobuf::RepeatedField<super::hdfs::CipherOptionProto>) {
        self.cipherOption = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cipherOption(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::CipherOptionProto> {
        &mut self.cipherOption
    }

    // Take field
    pub fn take_cipherOption(&mut self) -> ::protobuf::RepeatedField<super::hdfs::CipherOptionProto> {
        ::std::mem::replace(&mut self.cipherOption, ::protobuf::RepeatedField::new())
    }

    pub fn get_cipherOption(&self) -> &[super::hdfs::CipherOptionProto] {
        &self.cipherOption
    }

    fn get_cipherOption_for_reflect(&self) -> &::protobuf::RepeatedField<super::hdfs::CipherOptionProto> {
        &self.cipherOption
    }

    fn mut_cipherOption_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::CipherOptionProto> {
        &mut self.cipherOption
    }
}

impl ::protobuf::Message for DataTransferEncryptorMessageProto {
    fn is_initialized(&self) -> bool {
        if self.status.is_none() {
            return false;
        }
        for v in &self.cipherOption {
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
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.payload)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cipherOption)?;
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
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.payload.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(ref v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        for value in &self.cipherOption {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.payload.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(ref v) = self.message.as_ref() {
            os.write_string(3, &v)?;
        }
        for v in &self.cipherOption {
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

impl ::protobuf::MessageStatic for DataTransferEncryptorMessageProto {
    fn new() -> DataTransferEncryptorMessageProto {
        DataTransferEncryptorMessageProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataTransferEncryptorMessageProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DataTransferEncryptorMessageProto_DataTransferEncryptorStatus>>(
                    "status",
                    DataTransferEncryptorMessageProto::get_status_for_reflect,
                    DataTransferEncryptorMessageProto::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "payload",
                    DataTransferEncryptorMessageProto::get_payload_for_reflect,
                    DataTransferEncryptorMessageProto::mut_payload_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    DataTransferEncryptorMessageProto::get_message_for_reflect,
                    DataTransferEncryptorMessageProto::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::CipherOptionProto>>(
                    "cipherOption",
                    DataTransferEncryptorMessageProto::get_cipherOption_for_reflect,
                    DataTransferEncryptorMessageProto::mut_cipherOption_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataTransferEncryptorMessageProto>(
                    "DataTransferEncryptorMessageProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataTransferEncryptorMessageProto {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_payload();
        self.clear_message();
        self.clear_cipherOption();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataTransferEncryptorMessageProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataTransferEncryptorMessageProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DataTransferEncryptorMessageProto_DataTransferEncryptorStatus {
    SUCCESS = 0,
    ERROR_UNKNOWN_KEY = 1,
    ERROR = 2,
}

impl ::protobuf::ProtobufEnum for DataTransferEncryptorMessageProto_DataTransferEncryptorStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DataTransferEncryptorMessageProto_DataTransferEncryptorStatus> {
        match value {
            0 => ::std::option::Option::Some(DataTransferEncryptorMessageProto_DataTransferEncryptorStatus::SUCCESS),
            1 => ::std::option::Option::Some(DataTransferEncryptorMessageProto_DataTransferEncryptorStatus::ERROR_UNKNOWN_KEY),
            2 => ::std::option::Option::Some(DataTransferEncryptorMessageProto_DataTransferEncryptorStatus::ERROR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DataTransferEncryptorMessageProto_DataTransferEncryptorStatus] = &[
            DataTransferEncryptorMessageProto_DataTransferEncryptorStatus::SUCCESS,
            DataTransferEncryptorMessageProto_DataTransferEncryptorStatus::ERROR_UNKNOWN_KEY,
            DataTransferEncryptorMessageProto_DataTransferEncryptorStatus::ERROR,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DataTransferEncryptorMessageProto_DataTransferEncryptorStatus>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DataTransferEncryptorMessageProto_DataTransferEncryptorStatus", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DataTransferEncryptorMessageProto_DataTransferEncryptorStatus {
}

impl ::protobuf::reflect::ProtobufValue for DataTransferEncryptorMessageProto_DataTransferEncryptorStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BaseHeaderProto {
    // message fields
    block: ::protobuf::SingularPtrField<super::hdfs::ExtendedBlockProto>,
    token: ::protobuf::SingularPtrField<super::Security::TokenProto>,
    traceInfo: ::protobuf::SingularPtrField<DataTransferTraceInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BaseHeaderProto {}

impl BaseHeaderProto {
    pub fn new() -> BaseHeaderProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BaseHeaderProto {
        static mut instance: ::protobuf::lazy::Lazy<BaseHeaderProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BaseHeaderProto,
        };
        unsafe {
            instance.get(BaseHeaderProto::new)
        }
    }

    // required .hadoop.hdfs.ExtendedBlockProto block = 1;

    pub fn clear_block(&mut self) {
        self.block.clear();
    }

    pub fn has_block(&self) -> bool {
        self.block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block(&mut self, v: super::hdfs::ExtendedBlockProto) {
        self.block = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block(&mut self) -> &mut super::hdfs::ExtendedBlockProto {
        if self.block.is_none() {
            self.block.set_default();
        }
        self.block.as_mut().unwrap()
    }

    // Take field
    pub fn take_block(&mut self) -> super::hdfs::ExtendedBlockProto {
        self.block.take().unwrap_or_else(|| super::hdfs::ExtendedBlockProto::new())
    }

    pub fn get_block(&self) -> &super::hdfs::ExtendedBlockProto {
        self.block.as_ref().unwrap_or_else(|| super::hdfs::ExtendedBlockProto::default_instance())
    }

    fn get_block_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::ExtendedBlockProto> {
        &self.block
    }

    fn mut_block_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::ExtendedBlockProto> {
        &mut self.block
    }

    // optional .hadoop.common.TokenProto token = 2;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: super::Security::TokenProto) {
        self.token = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut super::Security::TokenProto {
        if self.token.is_none() {
            self.token.set_default();
        }
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> super::Security::TokenProto {
        self.token.take().unwrap_or_else(|| super::Security::TokenProto::new())
    }

    pub fn get_token(&self) -> &super::Security::TokenProto {
        self.token.as_ref().unwrap_or_else(|| super::Security::TokenProto::default_instance())
    }

    fn get_token_for_reflect(&self) -> &::protobuf::SingularPtrField<super::Security::TokenProto> {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::Security::TokenProto> {
        &mut self.token
    }

    // optional .hadoop.hdfs.DataTransferTraceInfoProto traceInfo = 3;

    pub fn clear_traceInfo(&mut self) {
        self.traceInfo.clear();
    }

    pub fn has_traceInfo(&self) -> bool {
        self.traceInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_traceInfo(&mut self, v: DataTransferTraceInfoProto) {
        self.traceInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_traceInfo(&mut self) -> &mut DataTransferTraceInfoProto {
        if self.traceInfo.is_none() {
            self.traceInfo.set_default();
        }
        self.traceInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_traceInfo(&mut self) -> DataTransferTraceInfoProto {
        self.traceInfo.take().unwrap_or_else(|| DataTransferTraceInfoProto::new())
    }

    pub fn get_traceInfo(&self) -> &DataTransferTraceInfoProto {
        self.traceInfo.as_ref().unwrap_or_else(|| DataTransferTraceInfoProto::default_instance())
    }

    fn get_traceInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<DataTransferTraceInfoProto> {
        &self.traceInfo
    }

    fn mut_traceInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DataTransferTraceInfoProto> {
        &mut self.traceInfo
    }
}

impl ::protobuf::Message for BaseHeaderProto {
    fn is_initialized(&self) -> bool {
        if self.block.is_none() {
            return false;
        }
        for v in &self.block {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.token {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.block)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.token)?;
                },
                3 => {
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
        if let Some(ref v) = self.block.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.token.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
        if let Some(ref v) = self.block.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.token.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.traceInfo.as_ref() {
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

impl ::protobuf::MessageStatic for BaseHeaderProto {
    fn new() -> BaseHeaderProto {
        BaseHeaderProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BaseHeaderProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::ExtendedBlockProto>>(
                    "block",
                    BaseHeaderProto::get_block_for_reflect,
                    BaseHeaderProto::mut_block_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::Security::TokenProto>>(
                    "token",
                    BaseHeaderProto::get_token_for_reflect,
                    BaseHeaderProto::mut_token_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DataTransferTraceInfoProto>>(
                    "traceInfo",
                    BaseHeaderProto::get_traceInfo_for_reflect,
                    BaseHeaderProto::mut_traceInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BaseHeaderProto>(
                    "BaseHeaderProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BaseHeaderProto {
    fn clear(&mut self) {
        self.clear_block();
        self.clear_token();
        self.clear_traceInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BaseHeaderProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BaseHeaderProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataTransferTraceInfoProto {
    // message fields
    traceId: ::std::option::Option<u64>,
    parentId: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DataTransferTraceInfoProto {}

impl DataTransferTraceInfoProto {
    pub fn new() -> DataTransferTraceInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataTransferTraceInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<DataTransferTraceInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataTransferTraceInfoProto,
        };
        unsafe {
            instance.get(DataTransferTraceInfoProto::new)
        }
    }

    // required uint64 traceId = 1;

    pub fn clear_traceId(&mut self) {
        self.traceId = ::std::option::Option::None;
    }

    pub fn has_traceId(&self) -> bool {
        self.traceId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_traceId(&mut self, v: u64) {
        self.traceId = ::std::option::Option::Some(v);
    }

    pub fn get_traceId(&self) -> u64 {
        self.traceId.unwrap_or(0)
    }

    fn get_traceId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.traceId
    }

    fn mut_traceId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.traceId
    }

    // required uint64 parentId = 2;

    pub fn clear_parentId(&mut self) {
        self.parentId = ::std::option::Option::None;
    }

    pub fn has_parentId(&self) -> bool {
        self.parentId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parentId(&mut self, v: u64) {
        self.parentId = ::std::option::Option::Some(v);
    }

    pub fn get_parentId(&self) -> u64 {
        self.parentId.unwrap_or(0)
    }

    fn get_parentId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.parentId
    }

    fn mut_parentId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.parentId
    }
}

impl ::protobuf::Message for DataTransferTraceInfoProto {
    fn is_initialized(&self) -> bool {
        if self.traceId.is_none() {
            return false;
        }
        if self.parentId.is_none() {
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
                    self.traceId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
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
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.parentId {
            os.write_uint64(2, v)?;
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

impl ::protobuf::MessageStatic for DataTransferTraceInfoProto {
    fn new() -> DataTransferTraceInfoProto {
        DataTransferTraceInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataTransferTraceInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "traceId",
                    DataTransferTraceInfoProto::get_traceId_for_reflect,
                    DataTransferTraceInfoProto::mut_traceId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "parentId",
                    DataTransferTraceInfoProto::get_parentId_for_reflect,
                    DataTransferTraceInfoProto::mut_parentId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataTransferTraceInfoProto>(
                    "DataTransferTraceInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataTransferTraceInfoProto {
    fn clear(&mut self) {
        self.clear_traceId();
        self.clear_parentId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataTransferTraceInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataTransferTraceInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClientOperationHeaderProto {
    // message fields
    baseHeader: ::protobuf::SingularPtrField<BaseHeaderProto>,
    clientName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientOperationHeaderProto {}

impl ClientOperationHeaderProto {
    pub fn new() -> ClientOperationHeaderProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientOperationHeaderProto {
        static mut instance: ::protobuf::lazy::Lazy<ClientOperationHeaderProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientOperationHeaderProto,
        };
        unsafe {
            instance.get(ClientOperationHeaderProto::new)
        }
    }

    // required .hadoop.hdfs.BaseHeaderProto baseHeader = 1;

    pub fn clear_baseHeader(&mut self) {
        self.baseHeader.clear();
    }

    pub fn has_baseHeader(&self) -> bool {
        self.baseHeader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_baseHeader(&mut self, v: BaseHeaderProto) {
        self.baseHeader = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_baseHeader(&mut self) -> &mut BaseHeaderProto {
        if self.baseHeader.is_none() {
            self.baseHeader.set_default();
        }
        self.baseHeader.as_mut().unwrap()
    }

    // Take field
    pub fn take_baseHeader(&mut self) -> BaseHeaderProto {
        self.baseHeader.take().unwrap_or_else(|| BaseHeaderProto::new())
    }

    pub fn get_baseHeader(&self) -> &BaseHeaderProto {
        self.baseHeader.as_ref().unwrap_or_else(|| BaseHeaderProto::default_instance())
    }

    fn get_baseHeader_for_reflect(&self) -> &::protobuf::SingularPtrField<BaseHeaderProto> {
        &self.baseHeader
    }

    fn mut_baseHeader_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BaseHeaderProto> {
        &mut self.baseHeader
    }

    // required string clientName = 2;

    pub fn clear_clientName(&mut self) {
        self.clientName.clear();
    }

    pub fn has_clientName(&self) -> bool {
        self.clientName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientName(&mut self, v: ::std::string::String) {
        self.clientName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientName(&mut self) -> &mut ::std::string::String {
        if self.clientName.is_none() {
            self.clientName.set_default();
        }
        self.clientName.as_mut().unwrap()
    }

    // Take field
    pub fn take_clientName(&mut self) -> ::std::string::String {
        self.clientName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_clientName(&self) -> &str {
        match self.clientName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_clientName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.clientName
    }

    fn mut_clientName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.clientName
    }
}

impl ::protobuf::Message for ClientOperationHeaderProto {
    fn is_initialized(&self) -> bool {
        if self.baseHeader.is_none() {
            return false;
        }
        if self.clientName.is_none() {
            return false;
        }
        for v in &self.baseHeader {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.baseHeader)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.clientName)?;
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
        if let Some(ref v) = self.baseHeader.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.clientName.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.baseHeader.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.clientName.as_ref() {
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

impl ::protobuf::MessageStatic for ClientOperationHeaderProto {
    fn new() -> ClientOperationHeaderProto {
        ClientOperationHeaderProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientOperationHeaderProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BaseHeaderProto>>(
                    "baseHeader",
                    ClientOperationHeaderProto::get_baseHeader_for_reflect,
                    ClientOperationHeaderProto::mut_baseHeader_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "clientName",
                    ClientOperationHeaderProto::get_clientName_for_reflect,
                    ClientOperationHeaderProto::mut_clientName_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientOperationHeaderProto>(
                    "ClientOperationHeaderProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientOperationHeaderProto {
    fn clear(&mut self) {
        self.clear_baseHeader();
        self.clear_clientName();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientOperationHeaderProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientOperationHeaderProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CachingStrategyProto {
    // message fields
    dropBehind: ::std::option::Option<bool>,
    readahead: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CachingStrategyProto {}

impl CachingStrategyProto {
    pub fn new() -> CachingStrategyProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CachingStrategyProto {
        static mut instance: ::protobuf::lazy::Lazy<CachingStrategyProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CachingStrategyProto,
        };
        unsafe {
            instance.get(CachingStrategyProto::new)
        }
    }

    // optional bool dropBehind = 1;

    pub fn clear_dropBehind(&mut self) {
        self.dropBehind = ::std::option::Option::None;
    }

    pub fn has_dropBehind(&self) -> bool {
        self.dropBehind.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dropBehind(&mut self, v: bool) {
        self.dropBehind = ::std::option::Option::Some(v);
    }

    pub fn get_dropBehind(&self) -> bool {
        self.dropBehind.unwrap_or(false)
    }

    fn get_dropBehind_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.dropBehind
    }

    fn mut_dropBehind_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.dropBehind
    }

    // optional int64 readahead = 2;

    pub fn clear_readahead(&mut self) {
        self.readahead = ::std::option::Option::None;
    }

    pub fn has_readahead(&self) -> bool {
        self.readahead.is_some()
    }

    // Param is passed by value, moved
    pub fn set_readahead(&mut self, v: i64) {
        self.readahead = ::std::option::Option::Some(v);
    }

    pub fn get_readahead(&self) -> i64 {
        self.readahead.unwrap_or(0)
    }

    fn get_readahead_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.readahead
    }

    fn mut_readahead_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.readahead
    }
}

impl ::protobuf::Message for CachingStrategyProto {
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
                    let tmp = is.read_bool()?;
                    self.dropBehind = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.readahead = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.dropBehind {
            my_size += 2;
        }
        if let Some(v) = self.readahead {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dropBehind {
            os.write_bool(1, v)?;
        }
        if let Some(v) = self.readahead {
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

impl ::protobuf::MessageStatic for CachingStrategyProto {
    fn new() -> CachingStrategyProto {
        CachingStrategyProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CachingStrategyProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "dropBehind",
                    CachingStrategyProto::get_dropBehind_for_reflect,
                    CachingStrategyProto::mut_dropBehind_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "readahead",
                    CachingStrategyProto::get_readahead_for_reflect,
                    CachingStrategyProto::mut_readahead_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CachingStrategyProto>(
                    "CachingStrategyProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CachingStrategyProto {
    fn clear(&mut self) {
        self.clear_dropBehind();
        self.clear_readahead();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CachingStrategyProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CachingStrategyProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpReadBlockProto {
    // message fields
    header: ::protobuf::SingularPtrField<ClientOperationHeaderProto>,
    offset: ::std::option::Option<u64>,
    len: ::std::option::Option<u64>,
    sendChecksums: ::std::option::Option<bool>,
    cachingStrategy: ::protobuf::SingularPtrField<CachingStrategyProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpReadBlockProto {}

impl OpReadBlockProto {
    pub fn new() -> OpReadBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpReadBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<OpReadBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpReadBlockProto,
        };
        unsafe {
            instance.get(OpReadBlockProto::new)
        }
    }

    // required .hadoop.hdfs.ClientOperationHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ClientOperationHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ClientOperationHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ClientOperationHeaderProto {
        self.header.take().unwrap_or_else(|| ClientOperationHeaderProto::new())
    }

    pub fn get_header(&self) -> &ClientOperationHeaderProto {
        self.header.as_ref().unwrap_or_else(|| ClientOperationHeaderProto::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ClientOperationHeaderProto> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ClientOperationHeaderProto> {
        &mut self.header
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

    // required uint64 len = 3;

    pub fn clear_len(&mut self) {
        self.len = ::std::option::Option::None;
    }

    pub fn has_len(&self) -> bool {
        self.len.is_some()
    }

    // Param is passed by value, moved
    pub fn set_len(&mut self, v: u64) {
        self.len = ::std::option::Option::Some(v);
    }

    pub fn get_len(&self) -> u64 {
        self.len.unwrap_or(0)
    }

    fn get_len_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.len
    }

    fn mut_len_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.len
    }

    // optional bool sendChecksums = 4;

    pub fn clear_sendChecksums(&mut self) {
        self.sendChecksums = ::std::option::Option::None;
    }

    pub fn has_sendChecksums(&self) -> bool {
        self.sendChecksums.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sendChecksums(&mut self, v: bool) {
        self.sendChecksums = ::std::option::Option::Some(v);
    }

    pub fn get_sendChecksums(&self) -> bool {
        self.sendChecksums.unwrap_or(true)
    }

    fn get_sendChecksums_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.sendChecksums
    }

    fn mut_sendChecksums_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.sendChecksums
    }

    // optional .hadoop.hdfs.CachingStrategyProto cachingStrategy = 5;

    pub fn clear_cachingStrategy(&mut self) {
        self.cachingStrategy.clear();
    }

    pub fn has_cachingStrategy(&self) -> bool {
        self.cachingStrategy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cachingStrategy(&mut self, v: CachingStrategyProto) {
        self.cachingStrategy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cachingStrategy(&mut self) -> &mut CachingStrategyProto {
        if self.cachingStrategy.is_none() {
            self.cachingStrategy.set_default();
        }
        self.cachingStrategy.as_mut().unwrap()
    }

    // Take field
    pub fn take_cachingStrategy(&mut self) -> CachingStrategyProto {
        self.cachingStrategy.take().unwrap_or_else(|| CachingStrategyProto::new())
    }

    pub fn get_cachingStrategy(&self) -> &CachingStrategyProto {
        self.cachingStrategy.as_ref().unwrap_or_else(|| CachingStrategyProto::default_instance())
    }

    fn get_cachingStrategy_for_reflect(&self) -> &::protobuf::SingularPtrField<CachingStrategyProto> {
        &self.cachingStrategy
    }

    fn mut_cachingStrategy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CachingStrategyProto> {
        &mut self.cachingStrategy
    }
}

impl ::protobuf::Message for OpReadBlockProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        }
        if self.offset.is_none() {
            return false;
        }
        if self.len.is_none() {
            return false;
        }
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cachingStrategy {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.offset = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.len = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.sendChecksums = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cachingStrategy)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.offset {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.len {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.sendChecksums {
            my_size += 2;
        }
        if let Some(ref v) = self.cachingStrategy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.offset {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.len {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.sendChecksums {
            os.write_bool(4, v)?;
        }
        if let Some(ref v) = self.cachingStrategy.as_ref() {
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

impl ::protobuf::MessageStatic for OpReadBlockProto {
    fn new() -> OpReadBlockProto {
        OpReadBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpReadBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClientOperationHeaderProto>>(
                    "header",
                    OpReadBlockProto::get_header_for_reflect,
                    OpReadBlockProto::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "offset",
                    OpReadBlockProto::get_offset_for_reflect,
                    OpReadBlockProto::mut_offset_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "len",
                    OpReadBlockProto::get_len_for_reflect,
                    OpReadBlockProto::mut_len_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "sendChecksums",
                    OpReadBlockProto::get_sendChecksums_for_reflect,
                    OpReadBlockProto::mut_sendChecksums_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CachingStrategyProto>>(
                    "cachingStrategy",
                    OpReadBlockProto::get_cachingStrategy_for_reflect,
                    OpReadBlockProto::mut_cachingStrategy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpReadBlockProto>(
                    "OpReadBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpReadBlockProto {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_offset();
        self.clear_len();
        self.clear_sendChecksums();
        self.clear_cachingStrategy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpReadBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpReadBlockProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChecksumProto {
    // message fields
    field_type: ::std::option::Option<super::hdfs::ChecksumTypeProto>,
    bytesPerChecksum: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChecksumProto {}

impl ChecksumProto {
    pub fn new() -> ChecksumProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChecksumProto {
        static mut instance: ::protobuf::lazy::Lazy<ChecksumProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChecksumProto,
        };
        unsafe {
            instance.get(ChecksumProto::new)
        }
    }

    // required .hadoop.hdfs.ChecksumTypeProto type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: super::hdfs::ChecksumTypeProto) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> super::hdfs::ChecksumTypeProto {
        self.field_type.unwrap_or(super::hdfs::ChecksumTypeProto::CHECKSUM_NULL)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<super::hdfs::ChecksumTypeProto> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<super::hdfs::ChecksumTypeProto> {
        &mut self.field_type
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
}

impl ::protobuf::Message for ChecksumProto {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        }
        if self.bytesPerChecksum.is_none() {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.bytesPerChecksum = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.bytesPerChecksum {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.bytesPerChecksum {
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

impl ::protobuf::MessageStatic for ChecksumProto {
    fn new() -> ChecksumProto {
        ChecksumProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChecksumProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::hdfs::ChecksumTypeProto>>(
                    "type",
                    ChecksumProto::get_field_type_for_reflect,
                    ChecksumProto::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "bytesPerChecksum",
                    ChecksumProto::get_bytesPerChecksum_for_reflect,
                    ChecksumProto::mut_bytesPerChecksum_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChecksumProto>(
                    "ChecksumProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChecksumProto {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_bytesPerChecksum();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChecksumProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChecksumProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpWriteBlockProto {
    // message fields
    header: ::protobuf::SingularPtrField<ClientOperationHeaderProto>,
    targets: ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto>,
    source: ::protobuf::SingularPtrField<super::hdfs::DatanodeInfoProto>,
    stage: ::std::option::Option<OpWriteBlockProto_BlockConstructionStage>,
    pipelineSize: ::std::option::Option<u32>,
    minBytesRcvd: ::std::option::Option<u64>,
    maxBytesRcvd: ::std::option::Option<u64>,
    latestGenerationStamp: ::std::option::Option<u64>,
    requestedChecksum: ::protobuf::SingularPtrField<ChecksumProto>,
    cachingStrategy: ::protobuf::SingularPtrField<CachingStrategyProto>,
    storageType: ::std::option::Option<super::hdfs::StorageTypeProto>,
    targetStorageTypes: ::std::vec::Vec<super::hdfs::StorageTypeProto>,
    allowLazyPersist: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpWriteBlockProto {}

impl OpWriteBlockProto {
    pub fn new() -> OpWriteBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpWriteBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<OpWriteBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpWriteBlockProto,
        };
        unsafe {
            instance.get(OpWriteBlockProto::new)
        }
    }

    // required .hadoop.hdfs.ClientOperationHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ClientOperationHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ClientOperationHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ClientOperationHeaderProto {
        self.header.take().unwrap_or_else(|| ClientOperationHeaderProto::new())
    }

    pub fn get_header(&self) -> &ClientOperationHeaderProto {
        self.header.as_ref().unwrap_or_else(|| ClientOperationHeaderProto::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ClientOperationHeaderProto> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ClientOperationHeaderProto> {
        &mut self.header
    }

    // repeated .hadoop.hdfs.DatanodeInfoProto targets = 2;

    pub fn clear_targets(&mut self) {
        self.targets.clear();
    }

    // Param is passed by value, moved
    pub fn set_targets(&mut self, v: ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto>) {
        self.targets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targets(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto> {
        &mut self.targets
    }

    // Take field
    pub fn take_targets(&mut self) -> ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto> {
        ::std::mem::replace(&mut self.targets, ::protobuf::RepeatedField::new())
    }

    pub fn get_targets(&self) -> &[super::hdfs::DatanodeInfoProto] {
        &self.targets
    }

    fn get_targets_for_reflect(&self) -> &::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto> {
        &self.targets
    }

    fn mut_targets_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto> {
        &mut self.targets
    }

    // optional .hadoop.hdfs.DatanodeInfoProto source = 3;

    pub fn clear_source(&mut self) {
        self.source.clear();
    }

    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source(&mut self, v: super::hdfs::DatanodeInfoProto) {
        self.source = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source(&mut self) -> &mut super::hdfs::DatanodeInfoProto {
        if self.source.is_none() {
            self.source.set_default();
        }
        self.source.as_mut().unwrap()
    }

    // Take field
    pub fn take_source(&mut self) -> super::hdfs::DatanodeInfoProto {
        self.source.take().unwrap_or_else(|| super::hdfs::DatanodeInfoProto::new())
    }

    pub fn get_source(&self) -> &super::hdfs::DatanodeInfoProto {
        self.source.as_ref().unwrap_or_else(|| super::hdfs::DatanodeInfoProto::default_instance())
    }

    fn get_source_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::DatanodeInfoProto> {
        &self.source
    }

    fn mut_source_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::DatanodeInfoProto> {
        &mut self.source
    }

    // required .hadoop.hdfs.OpWriteBlockProto.BlockConstructionStage stage = 4;

    pub fn clear_stage(&mut self) {
        self.stage = ::std::option::Option::None;
    }

    pub fn has_stage(&self) -> bool {
        self.stage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stage(&mut self, v: OpWriteBlockProto_BlockConstructionStage) {
        self.stage = ::std::option::Option::Some(v);
    }

    pub fn get_stage(&self) -> OpWriteBlockProto_BlockConstructionStage {
        self.stage.unwrap_or(OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_APPEND)
    }

    fn get_stage_for_reflect(&self) -> &::std::option::Option<OpWriteBlockProto_BlockConstructionStage> {
        &self.stage
    }

    fn mut_stage_for_reflect(&mut self) -> &mut ::std::option::Option<OpWriteBlockProto_BlockConstructionStage> {
        &mut self.stage
    }

    // required uint32 pipelineSize = 5;

    pub fn clear_pipelineSize(&mut self) {
        self.pipelineSize = ::std::option::Option::None;
    }

    pub fn has_pipelineSize(&self) -> bool {
        self.pipelineSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pipelineSize(&mut self, v: u32) {
        self.pipelineSize = ::std::option::Option::Some(v);
    }

    pub fn get_pipelineSize(&self) -> u32 {
        self.pipelineSize.unwrap_or(0)
    }

    fn get_pipelineSize_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.pipelineSize
    }

    fn mut_pipelineSize_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.pipelineSize
    }

    // required uint64 minBytesRcvd = 6;

    pub fn clear_minBytesRcvd(&mut self) {
        self.minBytesRcvd = ::std::option::Option::None;
    }

    pub fn has_minBytesRcvd(&self) -> bool {
        self.minBytesRcvd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minBytesRcvd(&mut self, v: u64) {
        self.minBytesRcvd = ::std::option::Option::Some(v);
    }

    pub fn get_minBytesRcvd(&self) -> u64 {
        self.minBytesRcvd.unwrap_or(0)
    }

    fn get_minBytesRcvd_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.minBytesRcvd
    }

    fn mut_minBytesRcvd_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.minBytesRcvd
    }

    // required uint64 maxBytesRcvd = 7;

    pub fn clear_maxBytesRcvd(&mut self) {
        self.maxBytesRcvd = ::std::option::Option::None;
    }

    pub fn has_maxBytesRcvd(&self) -> bool {
        self.maxBytesRcvd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maxBytesRcvd(&mut self, v: u64) {
        self.maxBytesRcvd = ::std::option::Option::Some(v);
    }

    pub fn get_maxBytesRcvd(&self) -> u64 {
        self.maxBytesRcvd.unwrap_or(0)
    }

    fn get_maxBytesRcvd_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.maxBytesRcvd
    }

    fn mut_maxBytesRcvd_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.maxBytesRcvd
    }

    // required uint64 latestGenerationStamp = 8;

    pub fn clear_latestGenerationStamp(&mut self) {
        self.latestGenerationStamp = ::std::option::Option::None;
    }

    pub fn has_latestGenerationStamp(&self) -> bool {
        self.latestGenerationStamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latestGenerationStamp(&mut self, v: u64) {
        self.latestGenerationStamp = ::std::option::Option::Some(v);
    }

    pub fn get_latestGenerationStamp(&self) -> u64 {
        self.latestGenerationStamp.unwrap_or(0)
    }

    fn get_latestGenerationStamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.latestGenerationStamp
    }

    fn mut_latestGenerationStamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.latestGenerationStamp
    }

    // required .hadoop.hdfs.ChecksumProto requestedChecksum = 9;

    pub fn clear_requestedChecksum(&mut self) {
        self.requestedChecksum.clear();
    }

    pub fn has_requestedChecksum(&self) -> bool {
        self.requestedChecksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_requestedChecksum(&mut self, v: ChecksumProto) {
        self.requestedChecksum = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_requestedChecksum(&mut self) -> &mut ChecksumProto {
        if self.requestedChecksum.is_none() {
            self.requestedChecksum.set_default();
        }
        self.requestedChecksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_requestedChecksum(&mut self) -> ChecksumProto {
        self.requestedChecksum.take().unwrap_or_else(|| ChecksumProto::new())
    }

    pub fn get_requestedChecksum(&self) -> &ChecksumProto {
        self.requestedChecksum.as_ref().unwrap_or_else(|| ChecksumProto::default_instance())
    }

    fn get_requestedChecksum_for_reflect(&self) -> &::protobuf::SingularPtrField<ChecksumProto> {
        &self.requestedChecksum
    }

    fn mut_requestedChecksum_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ChecksumProto> {
        &mut self.requestedChecksum
    }

    // optional .hadoop.hdfs.CachingStrategyProto cachingStrategy = 10;

    pub fn clear_cachingStrategy(&mut self) {
        self.cachingStrategy.clear();
    }

    pub fn has_cachingStrategy(&self) -> bool {
        self.cachingStrategy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cachingStrategy(&mut self, v: CachingStrategyProto) {
        self.cachingStrategy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cachingStrategy(&mut self) -> &mut CachingStrategyProto {
        if self.cachingStrategy.is_none() {
            self.cachingStrategy.set_default();
        }
        self.cachingStrategy.as_mut().unwrap()
    }

    // Take field
    pub fn take_cachingStrategy(&mut self) -> CachingStrategyProto {
        self.cachingStrategy.take().unwrap_or_else(|| CachingStrategyProto::new())
    }

    pub fn get_cachingStrategy(&self) -> &CachingStrategyProto {
        self.cachingStrategy.as_ref().unwrap_or_else(|| CachingStrategyProto::default_instance())
    }

    fn get_cachingStrategy_for_reflect(&self) -> &::protobuf::SingularPtrField<CachingStrategyProto> {
        &self.cachingStrategy
    }

    fn mut_cachingStrategy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CachingStrategyProto> {
        &mut self.cachingStrategy
    }

    // optional .hadoop.hdfs.StorageTypeProto storageType = 11;

    pub fn clear_storageType(&mut self) {
        self.storageType = ::std::option::Option::None;
    }

    pub fn has_storageType(&self) -> bool {
        self.storageType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageType(&mut self, v: super::hdfs::StorageTypeProto) {
        self.storageType = ::std::option::Option::Some(v);
    }

    pub fn get_storageType(&self) -> super::hdfs::StorageTypeProto {
        self.storageType.unwrap_or(super::hdfs::StorageTypeProto::DISK)
    }

    fn get_storageType_for_reflect(&self) -> &::std::option::Option<super::hdfs::StorageTypeProto> {
        &self.storageType
    }

    fn mut_storageType_for_reflect(&mut self) -> &mut ::std::option::Option<super::hdfs::StorageTypeProto> {
        &mut self.storageType
    }

    // repeated .hadoop.hdfs.StorageTypeProto targetStorageTypes = 12;

    pub fn clear_targetStorageTypes(&mut self) {
        self.targetStorageTypes.clear();
    }

    // Param is passed by value, moved
    pub fn set_targetStorageTypes(&mut self, v: ::std::vec::Vec<super::hdfs::StorageTypeProto>) {
        self.targetStorageTypes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targetStorageTypes(&mut self) -> &mut ::std::vec::Vec<super::hdfs::StorageTypeProto> {
        &mut self.targetStorageTypes
    }

    // Take field
    pub fn take_targetStorageTypes(&mut self) -> ::std::vec::Vec<super::hdfs::StorageTypeProto> {
        ::std::mem::replace(&mut self.targetStorageTypes, ::std::vec::Vec::new())
    }

    pub fn get_targetStorageTypes(&self) -> &[super::hdfs::StorageTypeProto] {
        &self.targetStorageTypes
    }

    fn get_targetStorageTypes_for_reflect(&self) -> &::std::vec::Vec<super::hdfs::StorageTypeProto> {
        &self.targetStorageTypes
    }

    fn mut_targetStorageTypes_for_reflect(&mut self) -> &mut ::std::vec::Vec<super::hdfs::StorageTypeProto> {
        &mut self.targetStorageTypes
    }

    // optional bool allowLazyPersist = 13;

    pub fn clear_allowLazyPersist(&mut self) {
        self.allowLazyPersist = ::std::option::Option::None;
    }

    pub fn has_allowLazyPersist(&self) -> bool {
        self.allowLazyPersist.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allowLazyPersist(&mut self, v: bool) {
        self.allowLazyPersist = ::std::option::Option::Some(v);
    }

    pub fn get_allowLazyPersist(&self) -> bool {
        self.allowLazyPersist.unwrap_or(false)
    }

    fn get_allowLazyPersist_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.allowLazyPersist
    }

    fn mut_allowLazyPersist_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.allowLazyPersist
    }
}

impl ::protobuf::Message for OpWriteBlockProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        }
        if self.stage.is_none() {
            return false;
        }
        if self.pipelineSize.is_none() {
            return false;
        }
        if self.minBytesRcvd.is_none() {
            return false;
        }
        if self.maxBytesRcvd.is_none() {
            return false;
        }
        if self.latestGenerationStamp.is_none() {
            return false;
        }
        if self.requestedChecksum.is_none() {
            return false;
        }
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.targets {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.source {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.requestedChecksum {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.cachingStrategy {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.targets)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.source)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.stage = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.pipelineSize = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.minBytesRcvd = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.maxBytesRcvd = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.latestGenerationStamp = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.requestedChecksum)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cachingStrategy)?;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.storageType = ::std::option::Option::Some(tmp);
                },
                12 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.targetStorageTypes)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.allowLazyPersist = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.targets {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.source.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.stage {
            my_size += ::protobuf::rt::enum_size(4, v);
        }
        if let Some(v) = self.pipelineSize {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.minBytesRcvd {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.maxBytesRcvd {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.latestGenerationStamp {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.requestedChecksum.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.cachingStrategy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.storageType {
            my_size += ::protobuf::rt::enum_size(11, v);
        }
        for value in &self.targetStorageTypes {
            my_size += ::protobuf::rt::enum_size(12, *value);
        };
        if let Some(v) = self.allowLazyPersist {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.targets {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.source.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.stage {
            os.write_enum(4, v.value())?;
        }
        if let Some(v) = self.pipelineSize {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.minBytesRcvd {
            os.write_uint64(6, v)?;
        }
        if let Some(v) = self.maxBytesRcvd {
            os.write_uint64(7, v)?;
        }
        if let Some(v) = self.latestGenerationStamp {
            os.write_uint64(8, v)?;
        }
        if let Some(ref v) = self.requestedChecksum.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.cachingStrategy.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.storageType {
            os.write_enum(11, v.value())?;
        }
        for v in &self.targetStorageTypes {
            os.write_enum(12, v.value())?;
        };
        if let Some(v) = self.allowLazyPersist {
            os.write_bool(13, v)?;
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

impl ::protobuf::MessageStatic for OpWriteBlockProto {
    fn new() -> OpWriteBlockProto {
        OpWriteBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpWriteBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClientOperationHeaderProto>>(
                    "header",
                    OpWriteBlockProto::get_header_for_reflect,
                    OpWriteBlockProto::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeInfoProto>>(
                    "targets",
                    OpWriteBlockProto::get_targets_for_reflect,
                    OpWriteBlockProto::mut_targets_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeInfoProto>>(
                    "source",
                    OpWriteBlockProto::get_source_for_reflect,
                    OpWriteBlockProto::mut_source_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<OpWriteBlockProto_BlockConstructionStage>>(
                    "stage",
                    OpWriteBlockProto::get_stage_for_reflect,
                    OpWriteBlockProto::mut_stage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "pipelineSize",
                    OpWriteBlockProto::get_pipelineSize_for_reflect,
                    OpWriteBlockProto::mut_pipelineSize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "minBytesRcvd",
                    OpWriteBlockProto::get_minBytesRcvd_for_reflect,
                    OpWriteBlockProto::mut_minBytesRcvd_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "maxBytesRcvd",
                    OpWriteBlockProto::get_maxBytesRcvd_for_reflect,
                    OpWriteBlockProto::mut_maxBytesRcvd_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "latestGenerationStamp",
                    OpWriteBlockProto::get_latestGenerationStamp_for_reflect,
                    OpWriteBlockProto::mut_latestGenerationStamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ChecksumProto>>(
                    "requestedChecksum",
                    OpWriteBlockProto::get_requestedChecksum_for_reflect,
                    OpWriteBlockProto::mut_requestedChecksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CachingStrategyProto>>(
                    "cachingStrategy",
                    OpWriteBlockProto::get_cachingStrategy_for_reflect,
                    OpWriteBlockProto::mut_cachingStrategy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::hdfs::StorageTypeProto>>(
                    "storageType",
                    OpWriteBlockProto::get_storageType_for_reflect,
                    OpWriteBlockProto::mut_storageType_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::hdfs::StorageTypeProto>>(
                    "targetStorageTypes",
                    OpWriteBlockProto::get_targetStorageTypes_for_reflect,
                    OpWriteBlockProto::mut_targetStorageTypes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allowLazyPersist",
                    OpWriteBlockProto::get_allowLazyPersist_for_reflect,
                    OpWriteBlockProto::mut_allowLazyPersist_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpWriteBlockProto>(
                    "OpWriteBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpWriteBlockProto {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_targets();
        self.clear_source();
        self.clear_stage();
        self.clear_pipelineSize();
        self.clear_minBytesRcvd();
        self.clear_maxBytesRcvd();
        self.clear_latestGenerationStamp();
        self.clear_requestedChecksum();
        self.clear_cachingStrategy();
        self.clear_storageType();
        self.clear_targetStorageTypes();
        self.clear_allowLazyPersist();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpWriteBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpWriteBlockProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OpWriteBlockProto_BlockConstructionStage {
    PIPELINE_SETUP_APPEND = 0,
    PIPELINE_SETUP_APPEND_RECOVERY = 1,
    DATA_STREAMING = 2,
    PIPELINE_SETUP_STREAMING_RECOVERY = 3,
    PIPELINE_CLOSE = 4,
    PIPELINE_CLOSE_RECOVERY = 5,
    PIPELINE_SETUP_CREATE = 6,
    TRANSFER_RBW = 7,
    TRANSFER_FINALIZED = 8,
}

impl ::protobuf::ProtobufEnum for OpWriteBlockProto_BlockConstructionStage {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OpWriteBlockProto_BlockConstructionStage> {
        match value {
            0 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_APPEND),
            1 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_APPEND_RECOVERY),
            2 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::DATA_STREAMING),
            3 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_STREAMING_RECOVERY),
            4 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::PIPELINE_CLOSE),
            5 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::PIPELINE_CLOSE_RECOVERY),
            6 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_CREATE),
            7 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::TRANSFER_RBW),
            8 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::TRANSFER_FINALIZED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [OpWriteBlockProto_BlockConstructionStage] = &[
            OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_APPEND,
            OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_APPEND_RECOVERY,
            OpWriteBlockProto_BlockConstructionStage::DATA_STREAMING,
            OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_STREAMING_RECOVERY,
            OpWriteBlockProto_BlockConstructionStage::PIPELINE_CLOSE,
            OpWriteBlockProto_BlockConstructionStage::PIPELINE_CLOSE_RECOVERY,
            OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_CREATE,
            OpWriteBlockProto_BlockConstructionStage::TRANSFER_RBW,
            OpWriteBlockProto_BlockConstructionStage::TRANSFER_FINALIZED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<OpWriteBlockProto_BlockConstructionStage>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("OpWriteBlockProto_BlockConstructionStage", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for OpWriteBlockProto_BlockConstructionStage {
}

impl ::protobuf::reflect::ProtobufValue for OpWriteBlockProto_BlockConstructionStage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpTransferBlockProto {
    // message fields
    header: ::protobuf::SingularPtrField<ClientOperationHeaderProto>,
    targets: ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto>,
    targetStorageTypes: ::std::vec::Vec<super::hdfs::StorageTypeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpTransferBlockProto {}

impl OpTransferBlockProto {
    pub fn new() -> OpTransferBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpTransferBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<OpTransferBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpTransferBlockProto,
        };
        unsafe {
            instance.get(OpTransferBlockProto::new)
        }
    }

    // required .hadoop.hdfs.ClientOperationHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ClientOperationHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ClientOperationHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ClientOperationHeaderProto {
        self.header.take().unwrap_or_else(|| ClientOperationHeaderProto::new())
    }

    pub fn get_header(&self) -> &ClientOperationHeaderProto {
        self.header.as_ref().unwrap_or_else(|| ClientOperationHeaderProto::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<ClientOperationHeaderProto> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ClientOperationHeaderProto> {
        &mut self.header
    }

    // repeated .hadoop.hdfs.DatanodeInfoProto targets = 2;

    pub fn clear_targets(&mut self) {
        self.targets.clear();
    }

    // Param is passed by value, moved
    pub fn set_targets(&mut self, v: ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto>) {
        self.targets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targets(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto> {
        &mut self.targets
    }

    // Take field
    pub fn take_targets(&mut self) -> ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto> {
        ::std::mem::replace(&mut self.targets, ::protobuf::RepeatedField::new())
    }

    pub fn get_targets(&self) -> &[super::hdfs::DatanodeInfoProto] {
        &self.targets
    }

    fn get_targets_for_reflect(&self) -> &::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto> {
        &self.targets
    }

    fn mut_targets_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::DatanodeInfoProto> {
        &mut self.targets
    }

    // repeated .hadoop.hdfs.StorageTypeProto targetStorageTypes = 3;

    pub fn clear_targetStorageTypes(&mut self) {
        self.targetStorageTypes.clear();
    }

    // Param is passed by value, moved
    pub fn set_targetStorageTypes(&mut self, v: ::std::vec::Vec<super::hdfs::StorageTypeProto>) {
        self.targetStorageTypes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targetStorageTypes(&mut self) -> &mut ::std::vec::Vec<super::hdfs::StorageTypeProto> {
        &mut self.targetStorageTypes
    }

    // Take field
    pub fn take_targetStorageTypes(&mut self) -> ::std::vec::Vec<super::hdfs::StorageTypeProto> {
        ::std::mem::replace(&mut self.targetStorageTypes, ::std::vec::Vec::new())
    }

    pub fn get_targetStorageTypes(&self) -> &[super::hdfs::StorageTypeProto] {
        &self.targetStorageTypes
    }

    fn get_targetStorageTypes_for_reflect(&self) -> &::std::vec::Vec<super::hdfs::StorageTypeProto> {
        &self.targetStorageTypes
    }

    fn mut_targetStorageTypes_for_reflect(&mut self) -> &mut ::std::vec::Vec<super::hdfs::StorageTypeProto> {
        &mut self.targetStorageTypes
    }
}

impl ::protobuf::Message for OpTransferBlockProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        }
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.targets {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.targets)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.targetStorageTypes)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.targets {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.targetStorageTypes {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.targets {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.targetStorageTypes {
            os.write_enum(3, v.value())?;
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

impl ::protobuf::MessageStatic for OpTransferBlockProto {
    fn new() -> OpTransferBlockProto {
        OpTransferBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpTransferBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClientOperationHeaderProto>>(
                    "header",
                    OpTransferBlockProto::get_header_for_reflect,
                    OpTransferBlockProto::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeInfoProto>>(
                    "targets",
                    OpTransferBlockProto::get_targets_for_reflect,
                    OpTransferBlockProto::mut_targets_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::hdfs::StorageTypeProto>>(
                    "targetStorageTypes",
                    OpTransferBlockProto::get_targetStorageTypes_for_reflect,
                    OpTransferBlockProto::mut_targetStorageTypes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpTransferBlockProto>(
                    "OpTransferBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpTransferBlockProto {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_targets();
        self.clear_targetStorageTypes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpTransferBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpTransferBlockProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpReplaceBlockProto {
    // message fields
    header: ::protobuf::SingularPtrField<BaseHeaderProto>,
    delHint: ::protobuf::SingularField<::std::string::String>,
    source: ::protobuf::SingularPtrField<super::hdfs::DatanodeInfoProto>,
    storageType: ::std::option::Option<super::hdfs::StorageTypeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpReplaceBlockProto {}

impl OpReplaceBlockProto {
    pub fn new() -> OpReplaceBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpReplaceBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<OpReplaceBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpReplaceBlockProto,
        };
        unsafe {
            instance.get(OpReplaceBlockProto::new)
        }
    }

    // required .hadoop.hdfs.BaseHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: BaseHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut BaseHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> BaseHeaderProto {
        self.header.take().unwrap_or_else(|| BaseHeaderProto::new())
    }

    pub fn get_header(&self) -> &BaseHeaderProto {
        self.header.as_ref().unwrap_or_else(|| BaseHeaderProto::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<BaseHeaderProto> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BaseHeaderProto> {
        &mut self.header
    }

    // required string delHint = 2;

    pub fn clear_delHint(&mut self) {
        self.delHint.clear();
    }

    pub fn has_delHint(&self) -> bool {
        self.delHint.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delHint(&mut self, v: ::std::string::String) {
        self.delHint = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delHint(&mut self) -> &mut ::std::string::String {
        if self.delHint.is_none() {
            self.delHint.set_default();
        }
        self.delHint.as_mut().unwrap()
    }

    // Take field
    pub fn take_delHint(&mut self) -> ::std::string::String {
        self.delHint.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_delHint(&self) -> &str {
        match self.delHint.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_delHint_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.delHint
    }

    fn mut_delHint_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.delHint
    }

    // required .hadoop.hdfs.DatanodeInfoProto source = 3;

    pub fn clear_source(&mut self) {
        self.source.clear();
    }

    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source(&mut self, v: super::hdfs::DatanodeInfoProto) {
        self.source = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source(&mut self) -> &mut super::hdfs::DatanodeInfoProto {
        if self.source.is_none() {
            self.source.set_default();
        }
        self.source.as_mut().unwrap()
    }

    // Take field
    pub fn take_source(&mut self) -> super::hdfs::DatanodeInfoProto {
        self.source.take().unwrap_or_else(|| super::hdfs::DatanodeInfoProto::new())
    }

    pub fn get_source(&self) -> &super::hdfs::DatanodeInfoProto {
        self.source.as_ref().unwrap_or_else(|| super::hdfs::DatanodeInfoProto::default_instance())
    }

    fn get_source_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::DatanodeInfoProto> {
        &self.source
    }

    fn mut_source_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::DatanodeInfoProto> {
        &mut self.source
    }

    // optional .hadoop.hdfs.StorageTypeProto storageType = 4;

    pub fn clear_storageType(&mut self) {
        self.storageType = ::std::option::Option::None;
    }

    pub fn has_storageType(&self) -> bool {
        self.storageType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageType(&mut self, v: super::hdfs::StorageTypeProto) {
        self.storageType = ::std::option::Option::Some(v);
    }

    pub fn get_storageType(&self) -> super::hdfs::StorageTypeProto {
        self.storageType.unwrap_or(super::hdfs::StorageTypeProto::DISK)
    }

    fn get_storageType_for_reflect(&self) -> &::std::option::Option<super::hdfs::StorageTypeProto> {
        &self.storageType
    }

    fn mut_storageType_for_reflect(&mut self) -> &mut ::std::option::Option<super::hdfs::StorageTypeProto> {
        &mut self.storageType
    }
}

impl ::protobuf::Message for OpReplaceBlockProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        }
        if self.delHint.is_none() {
            return false;
        }
        if self.source.is_none() {
            return false;
        }
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.source {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.delHint)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.source)?;
                },
                4 => {
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.delHint.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.source.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.storageType {
            my_size += ::protobuf::rt::enum_size(4, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.delHint.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.source.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.storageType {
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

impl ::protobuf::MessageStatic for OpReplaceBlockProto {
    fn new() -> OpReplaceBlockProto {
        OpReplaceBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpReplaceBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BaseHeaderProto>>(
                    "header",
                    OpReplaceBlockProto::get_header_for_reflect,
                    OpReplaceBlockProto::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "delHint",
                    OpReplaceBlockProto::get_delHint_for_reflect,
                    OpReplaceBlockProto::mut_delHint_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeInfoProto>>(
                    "source",
                    OpReplaceBlockProto::get_source_for_reflect,
                    OpReplaceBlockProto::mut_source_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::hdfs::StorageTypeProto>>(
                    "storageType",
                    OpReplaceBlockProto::get_storageType_for_reflect,
                    OpReplaceBlockProto::mut_storageType_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpReplaceBlockProto>(
                    "OpReplaceBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpReplaceBlockProto {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_delHint();
        self.clear_source();
        self.clear_storageType();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpReplaceBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpReplaceBlockProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpCopyBlockProto {
    // message fields
    header: ::protobuf::SingularPtrField<BaseHeaderProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpCopyBlockProto {}

impl OpCopyBlockProto {
    pub fn new() -> OpCopyBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpCopyBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<OpCopyBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpCopyBlockProto,
        };
        unsafe {
            instance.get(OpCopyBlockProto::new)
        }
    }

    // required .hadoop.hdfs.BaseHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: BaseHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut BaseHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> BaseHeaderProto {
        self.header.take().unwrap_or_else(|| BaseHeaderProto::new())
    }

    pub fn get_header(&self) -> &BaseHeaderProto {
        self.header.as_ref().unwrap_or_else(|| BaseHeaderProto::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<BaseHeaderProto> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BaseHeaderProto> {
        &mut self.header
    }
}

impl ::protobuf::Message for OpCopyBlockProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        }
        for v in &self.header {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
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

impl ::protobuf::MessageStatic for OpCopyBlockProto {
    fn new() -> OpCopyBlockProto {
        OpCopyBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpCopyBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BaseHeaderProto>>(
                    "header",
                    OpCopyBlockProto::get_header_for_reflect,
                    OpCopyBlockProto::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpCopyBlockProto>(
                    "OpCopyBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpCopyBlockProto {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpCopyBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpCopyBlockProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpBlockChecksumProto {
    // message fields
    header: ::protobuf::SingularPtrField<BaseHeaderProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpBlockChecksumProto {}

impl OpBlockChecksumProto {
    pub fn new() -> OpBlockChecksumProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpBlockChecksumProto {
        static mut instance: ::protobuf::lazy::Lazy<OpBlockChecksumProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpBlockChecksumProto,
        };
        unsafe {
            instance.get(OpBlockChecksumProto::new)
        }
    }

    // required .hadoop.hdfs.BaseHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: BaseHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut BaseHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> BaseHeaderProto {
        self.header.take().unwrap_or_else(|| BaseHeaderProto::new())
    }

    pub fn get_header(&self) -> &BaseHeaderProto {
        self.header.as_ref().unwrap_or_else(|| BaseHeaderProto::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<BaseHeaderProto> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BaseHeaderProto> {
        &mut self.header
    }
}

impl ::protobuf::Message for OpBlockChecksumProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        }
        for v in &self.header {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
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

impl ::protobuf::MessageStatic for OpBlockChecksumProto {
    fn new() -> OpBlockChecksumProto {
        OpBlockChecksumProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpBlockChecksumProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BaseHeaderProto>>(
                    "header",
                    OpBlockChecksumProto::get_header_for_reflect,
                    OpBlockChecksumProto::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpBlockChecksumProto>(
                    "OpBlockChecksumProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpBlockChecksumProto {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpBlockChecksumProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpBlockChecksumProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ShortCircuitShmIdProto {
    // message fields
    hi: ::std::option::Option<i64>,
    lo: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ShortCircuitShmIdProto {}

impl ShortCircuitShmIdProto {
    pub fn new() -> ShortCircuitShmIdProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShortCircuitShmIdProto {
        static mut instance: ::protobuf::lazy::Lazy<ShortCircuitShmIdProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShortCircuitShmIdProto,
        };
        unsafe {
            instance.get(ShortCircuitShmIdProto::new)
        }
    }

    // required int64 hi = 1;

    pub fn clear_hi(&mut self) {
        self.hi = ::std::option::Option::None;
    }

    pub fn has_hi(&self) -> bool {
        self.hi.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hi(&mut self, v: i64) {
        self.hi = ::std::option::Option::Some(v);
    }

    pub fn get_hi(&self) -> i64 {
        self.hi.unwrap_or(0)
    }

    fn get_hi_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.hi
    }

    fn mut_hi_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.hi
    }

    // required int64 lo = 2;

    pub fn clear_lo(&mut self) {
        self.lo = ::std::option::Option::None;
    }

    pub fn has_lo(&self) -> bool {
        self.lo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lo(&mut self, v: i64) {
        self.lo = ::std::option::Option::Some(v);
    }

    pub fn get_lo(&self) -> i64 {
        self.lo.unwrap_or(0)
    }

    fn get_lo_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.lo
    }

    fn mut_lo_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.lo
    }
}

impl ::protobuf::Message for ShortCircuitShmIdProto {
    fn is_initialized(&self) -> bool {
        if self.hi.is_none() {
            return false;
        }
        if self.lo.is_none() {
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
                    let tmp = is.read_int64()?;
                    self.hi = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.lo = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.hi {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.lo {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hi {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.lo {
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

impl ::protobuf::MessageStatic for ShortCircuitShmIdProto {
    fn new() -> ShortCircuitShmIdProto {
        ShortCircuitShmIdProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShortCircuitShmIdProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "hi",
                    ShortCircuitShmIdProto::get_hi_for_reflect,
                    ShortCircuitShmIdProto::mut_hi_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "lo",
                    ShortCircuitShmIdProto::get_lo_for_reflect,
                    ShortCircuitShmIdProto::mut_lo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShortCircuitShmIdProto>(
                    "ShortCircuitShmIdProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShortCircuitShmIdProto {
    fn clear(&mut self) {
        self.clear_hi();
        self.clear_lo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ShortCircuitShmIdProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShortCircuitShmIdProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ShortCircuitShmSlotProto {
    // message fields
    shmId: ::protobuf::SingularPtrField<ShortCircuitShmIdProto>,
    slotIdx: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ShortCircuitShmSlotProto {}

impl ShortCircuitShmSlotProto {
    pub fn new() -> ShortCircuitShmSlotProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShortCircuitShmSlotProto {
        static mut instance: ::protobuf::lazy::Lazy<ShortCircuitShmSlotProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShortCircuitShmSlotProto,
        };
        unsafe {
            instance.get(ShortCircuitShmSlotProto::new)
        }
    }

    // required .hadoop.hdfs.ShortCircuitShmIdProto shmId = 1;

    pub fn clear_shmId(&mut self) {
        self.shmId.clear();
    }

    pub fn has_shmId(&self) -> bool {
        self.shmId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shmId(&mut self, v: ShortCircuitShmIdProto) {
        self.shmId = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shmId(&mut self) -> &mut ShortCircuitShmIdProto {
        if self.shmId.is_none() {
            self.shmId.set_default();
        }
        self.shmId.as_mut().unwrap()
    }

    // Take field
    pub fn take_shmId(&mut self) -> ShortCircuitShmIdProto {
        self.shmId.take().unwrap_or_else(|| ShortCircuitShmIdProto::new())
    }

    pub fn get_shmId(&self) -> &ShortCircuitShmIdProto {
        self.shmId.as_ref().unwrap_or_else(|| ShortCircuitShmIdProto::default_instance())
    }

    fn get_shmId_for_reflect(&self) -> &::protobuf::SingularPtrField<ShortCircuitShmIdProto> {
        &self.shmId
    }

    fn mut_shmId_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ShortCircuitShmIdProto> {
        &mut self.shmId
    }

    // required int32 slotIdx = 2;

    pub fn clear_slotIdx(&mut self) {
        self.slotIdx = ::std::option::Option::None;
    }

    pub fn has_slotIdx(&self) -> bool {
        self.slotIdx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slotIdx(&mut self, v: i32) {
        self.slotIdx = ::std::option::Option::Some(v);
    }

    pub fn get_slotIdx(&self) -> i32 {
        self.slotIdx.unwrap_or(0)
    }

    fn get_slotIdx_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.slotIdx
    }

    fn mut_slotIdx_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.slotIdx
    }
}

impl ::protobuf::Message for ShortCircuitShmSlotProto {
    fn is_initialized(&self) -> bool {
        if self.shmId.is_none() {
            return false;
        }
        if self.slotIdx.is_none() {
            return false;
        }
        for v in &self.shmId {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.shmId)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.slotIdx = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.shmId.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.slotIdx {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.shmId.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.slotIdx {
            os.write_int32(2, v)?;
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

impl ::protobuf::MessageStatic for ShortCircuitShmSlotProto {
    fn new() -> ShortCircuitShmSlotProto {
        ShortCircuitShmSlotProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShortCircuitShmSlotProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ShortCircuitShmIdProto>>(
                    "shmId",
                    ShortCircuitShmSlotProto::get_shmId_for_reflect,
                    ShortCircuitShmSlotProto::mut_shmId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "slotIdx",
                    ShortCircuitShmSlotProto::get_slotIdx_for_reflect,
                    ShortCircuitShmSlotProto::mut_slotIdx_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShortCircuitShmSlotProto>(
                    "ShortCircuitShmSlotProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShortCircuitShmSlotProto {
    fn clear(&mut self) {
        self.clear_shmId();
        self.clear_slotIdx();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ShortCircuitShmSlotProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShortCircuitShmSlotProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpRequestShortCircuitAccessProto {
    // message fields
    header: ::protobuf::SingularPtrField<BaseHeaderProto>,
    maxVersion: ::std::option::Option<u32>,
    slotId: ::protobuf::SingularPtrField<ShortCircuitShmSlotProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpRequestShortCircuitAccessProto {}

impl OpRequestShortCircuitAccessProto {
    pub fn new() -> OpRequestShortCircuitAccessProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpRequestShortCircuitAccessProto {
        static mut instance: ::protobuf::lazy::Lazy<OpRequestShortCircuitAccessProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpRequestShortCircuitAccessProto,
        };
        unsafe {
            instance.get(OpRequestShortCircuitAccessProto::new)
        }
    }

    // required .hadoop.hdfs.BaseHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: BaseHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut BaseHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> BaseHeaderProto {
        self.header.take().unwrap_or_else(|| BaseHeaderProto::new())
    }

    pub fn get_header(&self) -> &BaseHeaderProto {
        self.header.as_ref().unwrap_or_else(|| BaseHeaderProto::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<BaseHeaderProto> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BaseHeaderProto> {
        &mut self.header
    }

    // required uint32 maxVersion = 2;

    pub fn clear_maxVersion(&mut self) {
        self.maxVersion = ::std::option::Option::None;
    }

    pub fn has_maxVersion(&self) -> bool {
        self.maxVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maxVersion(&mut self, v: u32) {
        self.maxVersion = ::std::option::Option::Some(v);
    }

    pub fn get_maxVersion(&self) -> u32 {
        self.maxVersion.unwrap_or(0)
    }

    fn get_maxVersion_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.maxVersion
    }

    fn mut_maxVersion_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.maxVersion
    }

    // optional .hadoop.hdfs.ShortCircuitShmSlotProto slotId = 3;

    pub fn clear_slotId(&mut self) {
        self.slotId.clear();
    }

    pub fn has_slotId(&self) -> bool {
        self.slotId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slotId(&mut self, v: ShortCircuitShmSlotProto) {
        self.slotId = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slotId(&mut self) -> &mut ShortCircuitShmSlotProto {
        if self.slotId.is_none() {
            self.slotId.set_default();
        }
        self.slotId.as_mut().unwrap()
    }

    // Take field
    pub fn take_slotId(&mut self) -> ShortCircuitShmSlotProto {
        self.slotId.take().unwrap_or_else(|| ShortCircuitShmSlotProto::new())
    }

    pub fn get_slotId(&self) -> &ShortCircuitShmSlotProto {
        self.slotId.as_ref().unwrap_or_else(|| ShortCircuitShmSlotProto::default_instance())
    }

    fn get_slotId_for_reflect(&self) -> &::protobuf::SingularPtrField<ShortCircuitShmSlotProto> {
        &self.slotId
    }

    fn mut_slotId_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ShortCircuitShmSlotProto> {
        &mut self.slotId
    }
}

impl ::protobuf::Message for OpRequestShortCircuitAccessProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        }
        if self.maxVersion.is_none() {
            return false;
        }
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.slotId {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.maxVersion = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.slotId)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.maxVersion {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.slotId.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.maxVersion {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.slotId.as_ref() {
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

impl ::protobuf::MessageStatic for OpRequestShortCircuitAccessProto {
    fn new() -> OpRequestShortCircuitAccessProto {
        OpRequestShortCircuitAccessProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpRequestShortCircuitAccessProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BaseHeaderProto>>(
                    "header",
                    OpRequestShortCircuitAccessProto::get_header_for_reflect,
                    OpRequestShortCircuitAccessProto::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "maxVersion",
                    OpRequestShortCircuitAccessProto::get_maxVersion_for_reflect,
                    OpRequestShortCircuitAccessProto::mut_maxVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ShortCircuitShmSlotProto>>(
                    "slotId",
                    OpRequestShortCircuitAccessProto::get_slotId_for_reflect,
                    OpRequestShortCircuitAccessProto::mut_slotId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpRequestShortCircuitAccessProto>(
                    "OpRequestShortCircuitAccessProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpRequestShortCircuitAccessProto {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_maxVersion();
        self.clear_slotId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpRequestShortCircuitAccessProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpRequestShortCircuitAccessProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReleaseShortCircuitAccessRequestProto {
    // message fields
    slotId: ::protobuf::SingularPtrField<ShortCircuitShmSlotProto>,
    traceInfo: ::protobuf::SingularPtrField<DataTransferTraceInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReleaseShortCircuitAccessRequestProto {}

impl ReleaseShortCircuitAccessRequestProto {
    pub fn new() -> ReleaseShortCircuitAccessRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReleaseShortCircuitAccessRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ReleaseShortCircuitAccessRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReleaseShortCircuitAccessRequestProto,
        };
        unsafe {
            instance.get(ReleaseShortCircuitAccessRequestProto::new)
        }
    }

    // required .hadoop.hdfs.ShortCircuitShmSlotProto slotId = 1;

    pub fn clear_slotId(&mut self) {
        self.slotId.clear();
    }

    pub fn has_slotId(&self) -> bool {
        self.slotId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slotId(&mut self, v: ShortCircuitShmSlotProto) {
        self.slotId = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slotId(&mut self) -> &mut ShortCircuitShmSlotProto {
        if self.slotId.is_none() {
            self.slotId.set_default();
        }
        self.slotId.as_mut().unwrap()
    }

    // Take field
    pub fn take_slotId(&mut self) -> ShortCircuitShmSlotProto {
        self.slotId.take().unwrap_or_else(|| ShortCircuitShmSlotProto::new())
    }

    pub fn get_slotId(&self) -> &ShortCircuitShmSlotProto {
        self.slotId.as_ref().unwrap_or_else(|| ShortCircuitShmSlotProto::default_instance())
    }

    fn get_slotId_for_reflect(&self) -> &::protobuf::SingularPtrField<ShortCircuitShmSlotProto> {
        &self.slotId
    }

    fn mut_slotId_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ShortCircuitShmSlotProto> {
        &mut self.slotId
    }

    // optional .hadoop.hdfs.DataTransferTraceInfoProto traceInfo = 2;

    pub fn clear_traceInfo(&mut self) {
        self.traceInfo.clear();
    }

    pub fn has_traceInfo(&self) -> bool {
        self.traceInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_traceInfo(&mut self, v: DataTransferTraceInfoProto) {
        self.traceInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_traceInfo(&mut self) -> &mut DataTransferTraceInfoProto {
        if self.traceInfo.is_none() {
            self.traceInfo.set_default();
        }
        self.traceInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_traceInfo(&mut self) -> DataTransferTraceInfoProto {
        self.traceInfo.take().unwrap_or_else(|| DataTransferTraceInfoProto::new())
    }

    pub fn get_traceInfo(&self) -> &DataTransferTraceInfoProto {
        self.traceInfo.as_ref().unwrap_or_else(|| DataTransferTraceInfoProto::default_instance())
    }

    fn get_traceInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<DataTransferTraceInfoProto> {
        &self.traceInfo
    }

    fn mut_traceInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DataTransferTraceInfoProto> {
        &mut self.traceInfo
    }
}

impl ::protobuf::Message for ReleaseShortCircuitAccessRequestProto {
    fn is_initialized(&self) -> bool {
        if self.slotId.is_none() {
            return false;
        }
        for v in &self.slotId {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.slotId)?;
                },
                2 => {
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
        if let Some(ref v) = self.slotId.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
        if let Some(ref v) = self.slotId.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.traceInfo.as_ref() {
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

impl ::protobuf::MessageStatic for ReleaseShortCircuitAccessRequestProto {
    fn new() -> ReleaseShortCircuitAccessRequestProto {
        ReleaseShortCircuitAccessRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReleaseShortCircuitAccessRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ShortCircuitShmSlotProto>>(
                    "slotId",
                    ReleaseShortCircuitAccessRequestProto::get_slotId_for_reflect,
                    ReleaseShortCircuitAccessRequestProto::mut_slotId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DataTransferTraceInfoProto>>(
                    "traceInfo",
                    ReleaseShortCircuitAccessRequestProto::get_traceInfo_for_reflect,
                    ReleaseShortCircuitAccessRequestProto::mut_traceInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReleaseShortCircuitAccessRequestProto>(
                    "ReleaseShortCircuitAccessRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReleaseShortCircuitAccessRequestProto {
    fn clear(&mut self) {
        self.clear_slotId();
        self.clear_traceInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReleaseShortCircuitAccessRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReleaseShortCircuitAccessRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReleaseShortCircuitAccessResponseProto {
    // message fields
    status: ::std::option::Option<Status>,
    error: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReleaseShortCircuitAccessResponseProto {}

impl ReleaseShortCircuitAccessResponseProto {
    pub fn new() -> ReleaseShortCircuitAccessResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReleaseShortCircuitAccessResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ReleaseShortCircuitAccessResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReleaseShortCircuitAccessResponseProto,
        };
        unsafe {
            instance.get(ReleaseShortCircuitAccessResponseProto::new)
        }
    }

    // required .hadoop.hdfs.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> Status {
        self.status.unwrap_or(Status::SUCCESS)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<Status> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<Status> {
        &mut self.status
    }

    // optional string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        self.error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        match self.error.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error
    }
}

impl ::protobuf::Message for ReleaseShortCircuitAccessResponseProto {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_enum()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error)?;
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
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.error.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.error.as_ref() {
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

impl ::protobuf::MessageStatic for ReleaseShortCircuitAccessResponseProto {
    fn new() -> ReleaseShortCircuitAccessResponseProto {
        ReleaseShortCircuitAccessResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReleaseShortCircuitAccessResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    ReleaseShortCircuitAccessResponseProto::get_status_for_reflect,
                    ReleaseShortCircuitAccessResponseProto::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    ReleaseShortCircuitAccessResponseProto::get_error_for_reflect,
                    ReleaseShortCircuitAccessResponseProto::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReleaseShortCircuitAccessResponseProto>(
                    "ReleaseShortCircuitAccessResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReleaseShortCircuitAccessResponseProto {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReleaseShortCircuitAccessResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReleaseShortCircuitAccessResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ShortCircuitShmRequestProto {
    // message fields
    clientName: ::protobuf::SingularField<::std::string::String>,
    traceInfo: ::protobuf::SingularPtrField<DataTransferTraceInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ShortCircuitShmRequestProto {}

impl ShortCircuitShmRequestProto {
    pub fn new() -> ShortCircuitShmRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShortCircuitShmRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ShortCircuitShmRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShortCircuitShmRequestProto,
        };
        unsafe {
            instance.get(ShortCircuitShmRequestProto::new)
        }
    }

    // required string clientName = 1;

    pub fn clear_clientName(&mut self) {
        self.clientName.clear();
    }

    pub fn has_clientName(&self) -> bool {
        self.clientName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientName(&mut self, v: ::std::string::String) {
        self.clientName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientName(&mut self) -> &mut ::std::string::String {
        if self.clientName.is_none() {
            self.clientName.set_default();
        }
        self.clientName.as_mut().unwrap()
    }

    // Take field
    pub fn take_clientName(&mut self) -> ::std::string::String {
        self.clientName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_clientName(&self) -> &str {
        match self.clientName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_clientName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.clientName
    }

    fn mut_clientName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.clientName
    }

    // optional .hadoop.hdfs.DataTransferTraceInfoProto traceInfo = 2;

    pub fn clear_traceInfo(&mut self) {
        self.traceInfo.clear();
    }

    pub fn has_traceInfo(&self) -> bool {
        self.traceInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_traceInfo(&mut self, v: DataTransferTraceInfoProto) {
        self.traceInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_traceInfo(&mut self) -> &mut DataTransferTraceInfoProto {
        if self.traceInfo.is_none() {
            self.traceInfo.set_default();
        }
        self.traceInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_traceInfo(&mut self) -> DataTransferTraceInfoProto {
        self.traceInfo.take().unwrap_or_else(|| DataTransferTraceInfoProto::new())
    }

    pub fn get_traceInfo(&self) -> &DataTransferTraceInfoProto {
        self.traceInfo.as_ref().unwrap_or_else(|| DataTransferTraceInfoProto::default_instance())
    }

    fn get_traceInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<DataTransferTraceInfoProto> {
        &self.traceInfo
    }

    fn mut_traceInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DataTransferTraceInfoProto> {
        &mut self.traceInfo
    }
}

impl ::protobuf::Message for ShortCircuitShmRequestProto {
    fn is_initialized(&self) -> bool {
        if self.clientName.is_none() {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.clientName)?;
                },
                2 => {
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
        if let Some(ref v) = self.clientName.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
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
        if let Some(ref v) = self.clientName.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.traceInfo.as_ref() {
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

impl ::protobuf::MessageStatic for ShortCircuitShmRequestProto {
    fn new() -> ShortCircuitShmRequestProto {
        ShortCircuitShmRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShortCircuitShmRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "clientName",
                    ShortCircuitShmRequestProto::get_clientName_for_reflect,
                    ShortCircuitShmRequestProto::mut_clientName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DataTransferTraceInfoProto>>(
                    "traceInfo",
                    ShortCircuitShmRequestProto::get_traceInfo_for_reflect,
                    ShortCircuitShmRequestProto::mut_traceInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShortCircuitShmRequestProto>(
                    "ShortCircuitShmRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShortCircuitShmRequestProto {
    fn clear(&mut self) {
        self.clear_clientName();
        self.clear_traceInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ShortCircuitShmRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShortCircuitShmRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ShortCircuitShmResponseProto {
    // message fields
    status: ::std::option::Option<Status>,
    error: ::protobuf::SingularField<::std::string::String>,
    id: ::protobuf::SingularPtrField<ShortCircuitShmIdProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ShortCircuitShmResponseProto {}

impl ShortCircuitShmResponseProto {
    pub fn new() -> ShortCircuitShmResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShortCircuitShmResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ShortCircuitShmResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShortCircuitShmResponseProto,
        };
        unsafe {
            instance.get(ShortCircuitShmResponseProto::new)
        }
    }

    // required .hadoop.hdfs.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> Status {
        self.status.unwrap_or(Status::SUCCESS)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<Status> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<Status> {
        &mut self.status
    }

    // optional string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        self.error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        match self.error.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error
    }

    // optional .hadoop.hdfs.ShortCircuitShmIdProto id = 3;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ShortCircuitShmIdProto) {
        self.id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ShortCircuitShmIdProto {
        if self.id.is_none() {
            self.id.set_default();
        }
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ShortCircuitShmIdProto {
        self.id.take().unwrap_or_else(|| ShortCircuitShmIdProto::new())
    }

    pub fn get_id(&self) -> &ShortCircuitShmIdProto {
        self.id.as_ref().unwrap_or_else(|| ShortCircuitShmIdProto::default_instance())
    }

    fn get_id_for_reflect(&self) -> &::protobuf::SingularPtrField<ShortCircuitShmIdProto> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ShortCircuitShmIdProto> {
        &mut self.id
    }
}

impl ::protobuf::Message for ShortCircuitShmResponseProto {
    fn is_initialized(&self) -> bool {
        if self.status.is_none() {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.id)?;
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
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.error.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.error.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.id.as_ref() {
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

impl ::protobuf::MessageStatic for ShortCircuitShmResponseProto {
    fn new() -> ShortCircuitShmResponseProto {
        ShortCircuitShmResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShortCircuitShmResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    ShortCircuitShmResponseProto::get_status_for_reflect,
                    ShortCircuitShmResponseProto::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    ShortCircuitShmResponseProto::get_error_for_reflect,
                    ShortCircuitShmResponseProto::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ShortCircuitShmIdProto>>(
                    "id",
                    ShortCircuitShmResponseProto::get_id_for_reflect,
                    ShortCircuitShmResponseProto::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShortCircuitShmResponseProto>(
                    "ShortCircuitShmResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShortCircuitShmResponseProto {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_error();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ShortCircuitShmResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShortCircuitShmResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PacketHeaderProto {
    // message fields
    offsetInBlock: ::std::option::Option<i64>,
    seqno: ::std::option::Option<i64>,
    lastPacketInBlock: ::std::option::Option<bool>,
    dataLen: ::std::option::Option<i32>,
    syncBlock: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PacketHeaderProto {}

impl PacketHeaderProto {
    pub fn new() -> PacketHeaderProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PacketHeaderProto {
        static mut instance: ::protobuf::lazy::Lazy<PacketHeaderProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PacketHeaderProto,
        };
        unsafe {
            instance.get(PacketHeaderProto::new)
        }
    }

    // required sfixed64 offsetInBlock = 1;

    pub fn clear_offsetInBlock(&mut self) {
        self.offsetInBlock = ::std::option::Option::None;
    }

    pub fn has_offsetInBlock(&self) -> bool {
        self.offsetInBlock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_offsetInBlock(&mut self, v: i64) {
        self.offsetInBlock = ::std::option::Option::Some(v);
    }

    pub fn get_offsetInBlock(&self) -> i64 {
        self.offsetInBlock.unwrap_or(0)
    }

    fn get_offsetInBlock_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.offsetInBlock
    }

    fn mut_offsetInBlock_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.offsetInBlock
    }

    // required sfixed64 seqno = 2;

    pub fn clear_seqno(&mut self) {
        self.seqno = ::std::option::Option::None;
    }

    pub fn has_seqno(&self) -> bool {
        self.seqno.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seqno(&mut self, v: i64) {
        self.seqno = ::std::option::Option::Some(v);
    }

    pub fn get_seqno(&self) -> i64 {
        self.seqno.unwrap_or(0)
    }

    fn get_seqno_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.seqno
    }

    fn mut_seqno_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.seqno
    }

    // required bool lastPacketInBlock = 3;

    pub fn clear_lastPacketInBlock(&mut self) {
        self.lastPacketInBlock = ::std::option::Option::None;
    }

    pub fn has_lastPacketInBlock(&self) -> bool {
        self.lastPacketInBlock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastPacketInBlock(&mut self, v: bool) {
        self.lastPacketInBlock = ::std::option::Option::Some(v);
    }

    pub fn get_lastPacketInBlock(&self) -> bool {
        self.lastPacketInBlock.unwrap_or(false)
    }

    fn get_lastPacketInBlock_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.lastPacketInBlock
    }

    fn mut_lastPacketInBlock_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.lastPacketInBlock
    }

    // required sfixed32 dataLen = 4;

    pub fn clear_dataLen(&mut self) {
        self.dataLen = ::std::option::Option::None;
    }

    pub fn has_dataLen(&self) -> bool {
        self.dataLen.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dataLen(&mut self, v: i32) {
        self.dataLen = ::std::option::Option::Some(v);
    }

    pub fn get_dataLen(&self) -> i32 {
        self.dataLen.unwrap_or(0)
    }

    fn get_dataLen_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dataLen
    }

    fn mut_dataLen_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dataLen
    }

    // optional bool syncBlock = 5;

    pub fn clear_syncBlock(&mut self) {
        self.syncBlock = ::std::option::Option::None;
    }

    pub fn has_syncBlock(&self) -> bool {
        self.syncBlock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_syncBlock(&mut self, v: bool) {
        self.syncBlock = ::std::option::Option::Some(v);
    }

    pub fn get_syncBlock(&self) -> bool {
        self.syncBlock.unwrap_or(false)
    }

    fn get_syncBlock_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.syncBlock
    }

    fn mut_syncBlock_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.syncBlock
    }
}

impl ::protobuf::Message for PacketHeaderProto {
    fn is_initialized(&self) -> bool {
        if self.offsetInBlock.is_none() {
            return false;
        }
        if self.seqno.is_none() {
            return false;
        }
        if self.lastPacketInBlock.is_none() {
            return false;
        }
        if self.dataLen.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sfixed64()?;
                    self.offsetInBlock = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sfixed64()?;
                    self.seqno = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.lastPacketInBlock = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sfixed32()?;
                    self.dataLen = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.syncBlock = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.offsetInBlock {
            my_size += 9;
        }
        if let Some(v) = self.seqno {
            my_size += 9;
        }
        if let Some(v) = self.lastPacketInBlock {
            my_size += 2;
        }
        if let Some(v) = self.dataLen {
            my_size += 5;
        }
        if let Some(v) = self.syncBlock {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.offsetInBlock {
            os.write_sfixed64(1, v)?;
        }
        if let Some(v) = self.seqno {
            os.write_sfixed64(2, v)?;
        }
        if let Some(v) = self.lastPacketInBlock {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.dataLen {
            os.write_sfixed32(4, v)?;
        }
        if let Some(v) = self.syncBlock {
            os.write_bool(5, v)?;
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

impl ::protobuf::MessageStatic for PacketHeaderProto {
    fn new() -> PacketHeaderProto {
        PacketHeaderProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<PacketHeaderProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSfixed64>(
                    "offsetInBlock",
                    PacketHeaderProto::get_offsetInBlock_for_reflect,
                    PacketHeaderProto::mut_offsetInBlock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSfixed64>(
                    "seqno",
                    PacketHeaderProto::get_seqno_for_reflect,
                    PacketHeaderProto::mut_seqno_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "lastPacketInBlock",
                    PacketHeaderProto::get_lastPacketInBlock_for_reflect,
                    PacketHeaderProto::mut_lastPacketInBlock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSfixed32>(
                    "dataLen",
                    PacketHeaderProto::get_dataLen_for_reflect,
                    PacketHeaderProto::mut_dataLen_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "syncBlock",
                    PacketHeaderProto::get_syncBlock_for_reflect,
                    PacketHeaderProto::mut_syncBlock_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PacketHeaderProto>(
                    "PacketHeaderProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PacketHeaderProto {
    fn clear(&mut self) {
        self.clear_offsetInBlock();
        self.clear_seqno();
        self.clear_lastPacketInBlock();
        self.clear_dataLen();
        self.clear_syncBlock();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PacketHeaderProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PacketHeaderProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PipelineAckProto {
    // message fields
    seqno: ::std::option::Option<i64>,
    status: ::std::vec::Vec<Status>,
    downstreamAckTimeNanos: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PipelineAckProto {}

impl PipelineAckProto {
    pub fn new() -> PipelineAckProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PipelineAckProto {
        static mut instance: ::protobuf::lazy::Lazy<PipelineAckProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PipelineAckProto,
        };
        unsafe {
            instance.get(PipelineAckProto::new)
        }
    }

    // required sint64 seqno = 1;

    pub fn clear_seqno(&mut self) {
        self.seqno = ::std::option::Option::None;
    }

    pub fn has_seqno(&self) -> bool {
        self.seqno.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seqno(&mut self, v: i64) {
        self.seqno = ::std::option::Option::Some(v);
    }

    pub fn get_seqno(&self) -> i64 {
        self.seqno.unwrap_or(0)
    }

    fn get_seqno_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.seqno
    }

    fn mut_seqno_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.seqno
    }

    // repeated .hadoop.hdfs.Status status = 2;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: ::std::vec::Vec<Status>) {
        self.status = v;
    }

    // Mutable pointer to the field.
    pub fn mut_status(&mut self) -> &mut ::std::vec::Vec<Status> {
        &mut self.status
    }

    // Take field
    pub fn take_status(&mut self) -> ::std::vec::Vec<Status> {
        ::std::mem::replace(&mut self.status, ::std::vec::Vec::new())
    }

    pub fn get_status(&self) -> &[Status] {
        &self.status
    }

    fn get_status_for_reflect(&self) -> &::std::vec::Vec<Status> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::vec::Vec<Status> {
        &mut self.status
    }

    // optional uint64 downstreamAckTimeNanos = 3;

    pub fn clear_downstreamAckTimeNanos(&mut self) {
        self.downstreamAckTimeNanos = ::std::option::Option::None;
    }

    pub fn has_downstreamAckTimeNanos(&self) -> bool {
        self.downstreamAckTimeNanos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_downstreamAckTimeNanos(&mut self, v: u64) {
        self.downstreamAckTimeNanos = ::std::option::Option::Some(v);
    }

    pub fn get_downstreamAckTimeNanos(&self) -> u64 {
        self.downstreamAckTimeNanos.unwrap_or(0u64)
    }

    fn get_downstreamAckTimeNanos_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.downstreamAckTimeNanos
    }

    fn mut_downstreamAckTimeNanos_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.downstreamAckTimeNanos
    }
}

impl ::protobuf::Message for PipelineAckProto {
    fn is_initialized(&self) -> bool {
        if self.seqno.is_none() {
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
                    let tmp = is.read_sint64()?;
                    self.seqno = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.status)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.downstreamAckTimeNanos = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.seqno {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, v);
        }
        for value in &self.status {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        if let Some(v) = self.downstreamAckTimeNanos {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.seqno {
            os.write_sint64(1, v)?;
        }
        for v in &self.status {
            os.write_enum(2, v.value())?;
        };
        if let Some(v) = self.downstreamAckTimeNanos {
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

impl ::protobuf::MessageStatic for PipelineAckProto {
    fn new() -> PipelineAckProto {
        PipelineAckProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<PipelineAckProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                    "seqno",
                    PipelineAckProto::get_seqno_for_reflect,
                    PipelineAckProto::mut_seqno_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    PipelineAckProto::get_status_for_reflect,
                    PipelineAckProto::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "downstreamAckTimeNanos",
                    PipelineAckProto::get_downstreamAckTimeNanos_for_reflect,
                    PipelineAckProto::mut_downstreamAckTimeNanos_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PipelineAckProto>(
                    "PipelineAckProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PipelineAckProto {
    fn clear(&mut self) {
        self.clear_seqno();
        self.clear_status();
        self.clear_downstreamAckTimeNanos();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PipelineAckProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PipelineAckProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReadOpChecksumInfoProto {
    // message fields
    checksum: ::protobuf::SingularPtrField<ChecksumProto>,
    chunkOffset: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReadOpChecksumInfoProto {}

impl ReadOpChecksumInfoProto {
    pub fn new() -> ReadOpChecksumInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadOpChecksumInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<ReadOpChecksumInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadOpChecksumInfoProto,
        };
        unsafe {
            instance.get(ReadOpChecksumInfoProto::new)
        }
    }

    // required .hadoop.hdfs.ChecksumProto checksum = 1;

    pub fn clear_checksum(&mut self) {
        self.checksum.clear();
    }

    pub fn has_checksum(&self) -> bool {
        self.checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checksum(&mut self, v: ChecksumProto) {
        self.checksum = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_checksum(&mut self) -> &mut ChecksumProto {
        if self.checksum.is_none() {
            self.checksum.set_default();
        }
        self.checksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_checksum(&mut self) -> ChecksumProto {
        self.checksum.take().unwrap_or_else(|| ChecksumProto::new())
    }

    pub fn get_checksum(&self) -> &ChecksumProto {
        self.checksum.as_ref().unwrap_or_else(|| ChecksumProto::default_instance())
    }

    fn get_checksum_for_reflect(&self) -> &::protobuf::SingularPtrField<ChecksumProto> {
        &self.checksum
    }

    fn mut_checksum_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ChecksumProto> {
        &mut self.checksum
    }

    // required uint64 chunkOffset = 2;

    pub fn clear_chunkOffset(&mut self) {
        self.chunkOffset = ::std::option::Option::None;
    }

    pub fn has_chunkOffset(&self) -> bool {
        self.chunkOffset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chunkOffset(&mut self, v: u64) {
        self.chunkOffset = ::std::option::Option::Some(v);
    }

    pub fn get_chunkOffset(&self) -> u64 {
        self.chunkOffset.unwrap_or(0)
    }

    fn get_chunkOffset_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.chunkOffset
    }

    fn mut_chunkOffset_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.chunkOffset
    }
}

impl ::protobuf::Message for ReadOpChecksumInfoProto {
    fn is_initialized(&self) -> bool {
        if self.checksum.is_none() {
            return false;
        }
        if self.chunkOffset.is_none() {
            return false;
        }
        for v in &self.checksum {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.checksum)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.chunkOffset = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.checksum.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.chunkOffset {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.checksum.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.chunkOffset {
            os.write_uint64(2, v)?;
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

impl ::protobuf::MessageStatic for ReadOpChecksumInfoProto {
    fn new() -> ReadOpChecksumInfoProto {
        ReadOpChecksumInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadOpChecksumInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ChecksumProto>>(
                    "checksum",
                    ReadOpChecksumInfoProto::get_checksum_for_reflect,
                    ReadOpChecksumInfoProto::mut_checksum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "chunkOffset",
                    ReadOpChecksumInfoProto::get_chunkOffset_for_reflect,
                    ReadOpChecksumInfoProto::mut_chunkOffset_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadOpChecksumInfoProto>(
                    "ReadOpChecksumInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadOpChecksumInfoProto {
    fn clear(&mut self) {
        self.clear_checksum();
        self.clear_chunkOffset();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReadOpChecksumInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReadOpChecksumInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockOpResponseProto {
    // message fields
    status: ::std::option::Option<Status>,
    firstBadLink: ::protobuf::SingularField<::std::string::String>,
    checksumResponse: ::protobuf::SingularPtrField<OpBlockChecksumResponseProto>,
    readOpChecksumInfo: ::protobuf::SingularPtrField<ReadOpChecksumInfoProto>,
    message: ::protobuf::SingularField<::std::string::String>,
    shortCircuitAccessVersion: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockOpResponseProto {}

impl BlockOpResponseProto {
    pub fn new() -> BlockOpResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockOpResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockOpResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockOpResponseProto,
        };
        unsafe {
            instance.get(BlockOpResponseProto::new)
        }
    }

    // required .hadoop.hdfs.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> Status {
        self.status.unwrap_or(Status::SUCCESS)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<Status> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<Status> {
        &mut self.status
    }

    // optional string firstBadLink = 2;

    pub fn clear_firstBadLink(&mut self) {
        self.firstBadLink.clear();
    }

    pub fn has_firstBadLink(&self) -> bool {
        self.firstBadLink.is_some()
    }

    // Param is passed by value, moved
    pub fn set_firstBadLink(&mut self, v: ::std::string::String) {
        self.firstBadLink = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_firstBadLink(&mut self) -> &mut ::std::string::String {
        if self.firstBadLink.is_none() {
            self.firstBadLink.set_default();
        }
        self.firstBadLink.as_mut().unwrap()
    }

    // Take field
    pub fn take_firstBadLink(&mut self) -> ::std::string::String {
        self.firstBadLink.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_firstBadLink(&self) -> &str {
        match self.firstBadLink.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_firstBadLink_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.firstBadLink
    }

    fn mut_firstBadLink_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.firstBadLink
    }

    // optional .hadoop.hdfs.OpBlockChecksumResponseProto checksumResponse = 3;

    pub fn clear_checksumResponse(&mut self) {
        self.checksumResponse.clear();
    }

    pub fn has_checksumResponse(&self) -> bool {
        self.checksumResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checksumResponse(&mut self, v: OpBlockChecksumResponseProto) {
        self.checksumResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_checksumResponse(&mut self) -> &mut OpBlockChecksumResponseProto {
        if self.checksumResponse.is_none() {
            self.checksumResponse.set_default();
        }
        self.checksumResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_checksumResponse(&mut self) -> OpBlockChecksumResponseProto {
        self.checksumResponse.take().unwrap_or_else(|| OpBlockChecksumResponseProto::new())
    }

    pub fn get_checksumResponse(&self) -> &OpBlockChecksumResponseProto {
        self.checksumResponse.as_ref().unwrap_or_else(|| OpBlockChecksumResponseProto::default_instance())
    }

    fn get_checksumResponse_for_reflect(&self) -> &::protobuf::SingularPtrField<OpBlockChecksumResponseProto> {
        &self.checksumResponse
    }

    fn mut_checksumResponse_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OpBlockChecksumResponseProto> {
        &mut self.checksumResponse
    }

    // optional .hadoop.hdfs.ReadOpChecksumInfoProto readOpChecksumInfo = 4;

    pub fn clear_readOpChecksumInfo(&mut self) {
        self.readOpChecksumInfo.clear();
    }

    pub fn has_readOpChecksumInfo(&self) -> bool {
        self.readOpChecksumInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_readOpChecksumInfo(&mut self, v: ReadOpChecksumInfoProto) {
        self.readOpChecksumInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_readOpChecksumInfo(&mut self) -> &mut ReadOpChecksumInfoProto {
        if self.readOpChecksumInfo.is_none() {
            self.readOpChecksumInfo.set_default();
        }
        self.readOpChecksumInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_readOpChecksumInfo(&mut self) -> ReadOpChecksumInfoProto {
        self.readOpChecksumInfo.take().unwrap_or_else(|| ReadOpChecksumInfoProto::new())
    }

    pub fn get_readOpChecksumInfo(&self) -> &ReadOpChecksumInfoProto {
        self.readOpChecksumInfo.as_ref().unwrap_or_else(|| ReadOpChecksumInfoProto::default_instance())
    }

    fn get_readOpChecksumInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<ReadOpChecksumInfoProto> {
        &self.readOpChecksumInfo
    }

    fn mut_readOpChecksumInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ReadOpChecksumInfoProto> {
        &mut self.readOpChecksumInfo
    }

    // optional string message = 5;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        }
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.message
    }

    // optional uint32 shortCircuitAccessVersion = 6;

    pub fn clear_shortCircuitAccessVersion(&mut self) {
        self.shortCircuitAccessVersion = ::std::option::Option::None;
    }

    pub fn has_shortCircuitAccessVersion(&self) -> bool {
        self.shortCircuitAccessVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shortCircuitAccessVersion(&mut self, v: u32) {
        self.shortCircuitAccessVersion = ::std::option::Option::Some(v);
    }

    pub fn get_shortCircuitAccessVersion(&self) -> u32 {
        self.shortCircuitAccessVersion.unwrap_or(0)
    }

    fn get_shortCircuitAccessVersion_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.shortCircuitAccessVersion
    }

    fn mut_shortCircuitAccessVersion_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.shortCircuitAccessVersion
    }
}

impl ::protobuf::Message for BlockOpResponseProto {
    fn is_initialized(&self) -> bool {
        if self.status.is_none() {
            return false;
        }
        for v in &self.checksumResponse {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.readOpChecksumInfo {
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
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.firstBadLink)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.checksumResponse)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.readOpChecksumInfo)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.shortCircuitAccessVersion = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.firstBadLink.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.checksumResponse.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.readOpChecksumInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(v) = self.shortCircuitAccessVersion {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.firstBadLink.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.checksumResponse.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.readOpChecksumInfo.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.message.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(v) = self.shortCircuitAccessVersion {
            os.write_uint32(6, v)?;
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

impl ::protobuf::MessageStatic for BlockOpResponseProto {
    fn new() -> BlockOpResponseProto {
        BlockOpResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockOpResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    BlockOpResponseProto::get_status_for_reflect,
                    BlockOpResponseProto::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "firstBadLink",
                    BlockOpResponseProto::get_firstBadLink_for_reflect,
                    BlockOpResponseProto::mut_firstBadLink_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OpBlockChecksumResponseProto>>(
                    "checksumResponse",
                    BlockOpResponseProto::get_checksumResponse_for_reflect,
                    BlockOpResponseProto::mut_checksumResponse_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ReadOpChecksumInfoProto>>(
                    "readOpChecksumInfo",
                    BlockOpResponseProto::get_readOpChecksumInfo_for_reflect,
                    BlockOpResponseProto::mut_readOpChecksumInfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    BlockOpResponseProto::get_message_for_reflect,
                    BlockOpResponseProto::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "shortCircuitAccessVersion",
                    BlockOpResponseProto::get_shortCircuitAccessVersion_for_reflect,
                    BlockOpResponseProto::mut_shortCircuitAccessVersion_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockOpResponseProto>(
                    "BlockOpResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockOpResponseProto {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_firstBadLink();
        self.clear_checksumResponse();
        self.clear_readOpChecksumInfo();
        self.clear_message();
        self.clear_shortCircuitAccessVersion();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockOpResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockOpResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClientReadStatusProto {
    // message fields
    status: ::std::option::Option<Status>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientReadStatusProto {}

impl ClientReadStatusProto {
    pub fn new() -> ClientReadStatusProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientReadStatusProto {
        static mut instance: ::protobuf::lazy::Lazy<ClientReadStatusProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientReadStatusProto,
        };
        unsafe {
            instance.get(ClientReadStatusProto::new)
        }
    }

    // required .hadoop.hdfs.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> Status {
        self.status.unwrap_or(Status::SUCCESS)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<Status> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<Status> {
        &mut self.status
    }
}

impl ::protobuf::Message for ClientReadStatusProto {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_enum()?;
                    self.status = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for ClientReadStatusProto {
    fn new() -> ClientReadStatusProto {
        ClientReadStatusProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientReadStatusProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    ClientReadStatusProto::get_status_for_reflect,
                    ClientReadStatusProto::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientReadStatusProto>(
                    "ClientReadStatusProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientReadStatusProto {
    fn clear(&mut self) {
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClientReadStatusProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientReadStatusProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DNTransferAckProto {
    // message fields
    status: ::std::option::Option<Status>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DNTransferAckProto {}

impl DNTransferAckProto {
    pub fn new() -> DNTransferAckProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DNTransferAckProto {
        static mut instance: ::protobuf::lazy::Lazy<DNTransferAckProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DNTransferAckProto,
        };
        unsafe {
            instance.get(DNTransferAckProto::new)
        }
    }

    // required .hadoop.hdfs.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> Status {
        self.status.unwrap_or(Status::SUCCESS)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<Status> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<Status> {
        &mut self.status
    }
}

impl ::protobuf::Message for DNTransferAckProto {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_enum()?;
                    self.status = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for DNTransferAckProto {
    fn new() -> DNTransferAckProto {
        DNTransferAckProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DNTransferAckProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    DNTransferAckProto::get_status_for_reflect,
                    DNTransferAckProto::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DNTransferAckProto>(
                    "DNTransferAckProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DNTransferAckProto {
    fn clear(&mut self) {
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DNTransferAckProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DNTransferAckProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OpBlockChecksumResponseProto {
    // message fields
    bytesPerCrc: ::std::option::Option<u32>,
    crcPerBlock: ::std::option::Option<u64>,
    md5: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    crcType: ::std::option::Option<super::hdfs::ChecksumTypeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OpBlockChecksumResponseProto {}

impl OpBlockChecksumResponseProto {
    pub fn new() -> OpBlockChecksumResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpBlockChecksumResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<OpBlockChecksumResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpBlockChecksumResponseProto,
        };
        unsafe {
            instance.get(OpBlockChecksumResponseProto::new)
        }
    }

    // required uint32 bytesPerCrc = 1;

    pub fn clear_bytesPerCrc(&mut self) {
        self.bytesPerCrc = ::std::option::Option::None;
    }

    pub fn has_bytesPerCrc(&self) -> bool {
        self.bytesPerCrc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytesPerCrc(&mut self, v: u32) {
        self.bytesPerCrc = ::std::option::Option::Some(v);
    }

    pub fn get_bytesPerCrc(&self) -> u32 {
        self.bytesPerCrc.unwrap_or(0)
    }

    fn get_bytesPerCrc_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.bytesPerCrc
    }

    fn mut_bytesPerCrc_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.bytesPerCrc
    }

    // required uint64 crcPerBlock = 2;

    pub fn clear_crcPerBlock(&mut self) {
        self.crcPerBlock = ::std::option::Option::None;
    }

    pub fn has_crcPerBlock(&self) -> bool {
        self.crcPerBlock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crcPerBlock(&mut self, v: u64) {
        self.crcPerBlock = ::std::option::Option::Some(v);
    }

    pub fn get_crcPerBlock(&self) -> u64 {
        self.crcPerBlock.unwrap_or(0)
    }

    fn get_crcPerBlock_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.crcPerBlock
    }

    fn mut_crcPerBlock_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.crcPerBlock
    }

    // required bytes md5 = 3;

    pub fn clear_md5(&mut self) {
        self.md5.clear();
    }

    pub fn has_md5(&self) -> bool {
        self.md5.is_some()
    }

    // Param is passed by value, moved
    pub fn set_md5(&mut self, v: ::std::vec::Vec<u8>) {
        self.md5 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_md5(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.md5.is_none() {
            self.md5.set_default();
        }
        self.md5.as_mut().unwrap()
    }

    // Take field
    pub fn take_md5(&mut self) -> ::std::vec::Vec<u8> {
        self.md5.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_md5(&self) -> &[u8] {
        match self.md5.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_md5_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.md5
    }

    fn mut_md5_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.md5
    }

    // optional .hadoop.hdfs.ChecksumTypeProto crcType = 4;

    pub fn clear_crcType(&mut self) {
        self.crcType = ::std::option::Option::None;
    }

    pub fn has_crcType(&self) -> bool {
        self.crcType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crcType(&mut self, v: super::hdfs::ChecksumTypeProto) {
        self.crcType = ::std::option::Option::Some(v);
    }

    pub fn get_crcType(&self) -> super::hdfs::ChecksumTypeProto {
        self.crcType.unwrap_or(super::hdfs::ChecksumTypeProto::CHECKSUM_NULL)
    }

    fn get_crcType_for_reflect(&self) -> &::std::option::Option<super::hdfs::ChecksumTypeProto> {
        &self.crcType
    }

    fn mut_crcType_for_reflect(&mut self) -> &mut ::std::option::Option<super::hdfs::ChecksumTypeProto> {
        &mut self.crcType
    }
}

impl ::protobuf::Message for OpBlockChecksumResponseProto {
    fn is_initialized(&self) -> bool {
        if self.bytesPerCrc.is_none() {
            return false;
        }
        if self.crcPerBlock.is_none() {
            return false;
        }
        if self.md5.is_none() {
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
                    self.bytesPerCrc = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.crcPerBlock = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.md5)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.crcType = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.bytesPerCrc {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.crcPerBlock {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.md5.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        if let Some(v) = self.crcType {
            my_size += ::protobuf::rt::enum_size(4, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bytesPerCrc {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.crcPerBlock {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.md5.as_ref() {
            os.write_bytes(3, &v)?;
        }
        if let Some(v) = self.crcType {
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

impl ::protobuf::MessageStatic for OpBlockChecksumResponseProto {
    fn new() -> OpBlockChecksumResponseProto {
        OpBlockChecksumResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpBlockChecksumResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "bytesPerCrc",
                    OpBlockChecksumResponseProto::get_bytesPerCrc_for_reflect,
                    OpBlockChecksumResponseProto::mut_bytesPerCrc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "crcPerBlock",
                    OpBlockChecksumResponseProto::get_crcPerBlock_for_reflect,
                    OpBlockChecksumResponseProto::mut_crcPerBlock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "md5",
                    OpBlockChecksumResponseProto::get_md5_for_reflect,
                    OpBlockChecksumResponseProto::mut_md5_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::hdfs::ChecksumTypeProto>>(
                    "crcType",
                    OpBlockChecksumResponseProto::get_crcType_for_reflect,
                    OpBlockChecksumResponseProto::mut_crcType_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpBlockChecksumResponseProto>(
                    "OpBlockChecksumResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpBlockChecksumResponseProto {
    fn clear(&mut self) {
        self.clear_bytesPerCrc();
        self.clear_crcPerBlock();
        self.clear_md5();
        self.clear_crcType();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OpBlockChecksumResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OpBlockChecksumResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Status {
    SUCCESS = 0,
    ERROR = 1,
    ERROR_CHECKSUM = 2,
    ERROR_INVALID = 3,
    ERROR_EXISTS = 4,
    ERROR_ACCESS_TOKEN = 5,
    CHECKSUM_OK = 6,
    ERROR_UNSUPPORTED = 7,
    OOB_RESTART = 8,
    OOB_RESERVED1 = 9,
    OOB_RESERVED2 = 10,
    OOB_RESERVED3 = 11,
    IN_PROGRESS = 12,
}

impl ::protobuf::ProtobufEnum for Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Status> {
        match value {
            0 => ::std::option::Option::Some(Status::SUCCESS),
            1 => ::std::option::Option::Some(Status::ERROR),
            2 => ::std::option::Option::Some(Status::ERROR_CHECKSUM),
            3 => ::std::option::Option::Some(Status::ERROR_INVALID),
            4 => ::std::option::Option::Some(Status::ERROR_EXISTS),
            5 => ::std::option::Option::Some(Status::ERROR_ACCESS_TOKEN),
            6 => ::std::option::Option::Some(Status::CHECKSUM_OK),
            7 => ::std::option::Option::Some(Status::ERROR_UNSUPPORTED),
            8 => ::std::option::Option::Some(Status::OOB_RESTART),
            9 => ::std::option::Option::Some(Status::OOB_RESERVED1),
            10 => ::std::option::Option::Some(Status::OOB_RESERVED2),
            11 => ::std::option::Option::Some(Status::OOB_RESERVED3),
            12 => ::std::option::Option::Some(Status::IN_PROGRESS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Status] = &[
            Status::SUCCESS,
            Status::ERROR,
            Status::ERROR_CHECKSUM,
            Status::ERROR_INVALID,
            Status::ERROR_EXISTS,
            Status::ERROR_ACCESS_TOKEN,
            Status::CHECKSUM_OK,
            Status::ERROR_UNSUPPORTED,
            Status::OOB_RESTART,
            Status::OOB_RESERVED1,
            Status::OOB_RESERVED2,
            Status::OOB_RESERVED3,
            Status::IN_PROGRESS,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Status {
}

impl ::protobuf::reflect::ProtobufValue for Status {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12datatransfer.proto\x12\x0bhadoop.hdfs\x1a\x0eSecurity.proto\x1a\nh\
    dfs.proto\"\xa5\x02\n!DataTransferEncryptorMessageProto\x12Z\n\x06status\
    \x18\x01\x20\x02(\x0e2J.hadoop.hdfs.DataTransferEncryptorMessageProto.Da\
    taTransferEncryptorStatus\x12\x0f\n\x07payload\x18\x02\x20\x01(\x0c\x12\
    \x0f\n\x07message\x18\x03\x20\x01(\t\x124\n\x0ccipherOption\x18\x04\x20\
    \x03(\x0b2\x1e.hadoop.hdfs.CipherOptionProto\"L\n\x1bDataTransferEncrypt\
    orStatus\x12\x0b\n\x07SUCCESS\x10\0\x12\x15\n\x11ERROR_UNKNOWN_KEY\x10\
    \x01\x12\t\n\x05ERROR\x10\x02\"\xa7\x01\n\x0fBaseHeaderProto\x12.\n\x05b\
    lock\x18\x01\x20\x02(\x0b2\x1f.hadoop.hdfs.ExtendedBlockProto\x12(\n\x05\
    token\x18\x02\x20\x01(\x0b2\x19.hadoop.common.TokenProto\x12:\n\ttraceIn\
    fo\x18\x03\x20\x01(\x0b2'.hadoop.hdfs.DataTransferTraceInfoProto\"?\n\
    \x1aDataTransferTraceInfoProto\x12\x0f\n\x07traceId\x18\x01\x20\x02(\x04\
    \x12\x10\n\x08parentId\x18\x02\x20\x02(\x04\"b\n\x1aClientOperationHeade\
    rProto\x120\n\nbaseHeader\x18\x01\x20\x02(\x0b2\x1c.hadoop.hdfs.BaseHead\
    erProto\x12\x12\n\nclientName\x18\x02\x20\x02(\t\"=\n\x14CachingStrategy\
    Proto\x12\x12\n\ndropBehind\x18\x01\x20\x01(\x08\x12\x11\n\treadahead\
    \x18\x02\x20\x01(\x03\"\xc1\x01\n\x10OpReadBlockProto\x127\n\x06header\
    \x18\x01\x20\x02(\x0b2'.hadoop.hdfs.ClientOperationHeaderProto\x12\x0e\n\
    \x06offset\x18\x02\x20\x02(\x04\x12\x0b\n\x03len\x18\x03\x20\x02(\x04\
    \x12\x1b\n\rsendChecksums\x18\x04\x20\x01(\x08:\x04true\x12:\n\x0fcachin\
    gStrategy\x18\x05\x20\x01(\x0b2!.hadoop.hdfs.CachingStrategyProto\"W\n\r\
    ChecksumProto\x12,\n\x04type\x18\x01\x20\x02(\x0e2\x1e.hadoop.hdfs.Check\
    sumTypeProto\x12\x18\n\x10bytesPerChecksum\x18\x02\x20\x02(\r\"\xe8\x06\
    \n\x11OpWriteBlockProto\x127\n\x06header\x18\x01\x20\x02(\x0b2'.hadoop.h\
    dfs.ClientOperationHeaderProto\x12/\n\x07targets\x18\x02\x20\x03(\x0b2\
    \x1e.hadoop.hdfs.DatanodeInfoProto\x12.\n\x06source\x18\x03\x20\x01(\x0b\
    2\x1e.hadoop.hdfs.DatanodeInfoProto\x12D\n\x05stage\x18\x04\x20\x02(\x0e\
    25.hadoop.hdfs.OpWriteBlockProto.BlockConstructionStage\x12\x14\n\x0cpip\
    elineSize\x18\x05\x20\x02(\r\x12\x14\n\x0cminBytesRcvd\x18\x06\x20\x02(\
    \x04\x12\x14\n\x0cmaxBytesRcvd\x18\x07\x20\x02(\x04\x12\x1d\n\x15latestG\
    enerationStamp\x18\x08\x20\x02(\x04\x125\n\x11requestedChecksum\x18\t\
    \x20\x02(\x0b2\x1a.hadoop.hdfs.ChecksumProto\x12:\n\x0fcachingStrategy\
    \x18\n\x20\x01(\x0b2!.hadoop.hdfs.CachingStrategyProto\x128\n\x0bstorage\
    Type\x18\x0b\x20\x01(\x0e2\x1d.hadoop.hdfs.StorageTypeProto:\x04DISK\x12\
    9\n\x12targetStorageTypes\x18\x0c\x20\x03(\x0e2\x1d.hadoop.hdfs.StorageT\
    ypeProto\x12\x1f\n\x10allowLazyPersist\x18\r\x20\x01(\x08:\x05false\"\
    \x88\x02\n\x16BlockConstructionStage\x12\x19\n\x15PIPELINE_SETUP_APPEND\
    \x10\0\x12\"\n\x1ePIPELINE_SETUP_APPEND_RECOVERY\x10\x01\x12\x12\n\x0eDA\
    TA_STREAMING\x10\x02\x12%\n!PIPELINE_SETUP_STREAMING_RECOVERY\x10\x03\
    \x12\x12\n\x0ePIPELINE_CLOSE\x10\x04\x12\x1b\n\x17PIPELINE_CLOSE_RECOVER\
    Y\x10\x05\x12\x19\n\x15PIPELINE_SETUP_CREATE\x10\x06\x12\x10\n\x0cTRANSF\
    ER_RBW\x10\x07\x12\x16\n\x12TRANSFER_FINALIZED\x10\x08\"\xbb\x01\n\x14Op\
    TransferBlockProto\x127\n\x06header\x18\x01\x20\x02(\x0b2'.hadoop.hdfs.C\
    lientOperationHeaderProto\x12/\n\x07targets\x18\x02\x20\x03(\x0b2\x1e.ha\
    doop.hdfs.DatanodeInfoProto\x129\n\x12targetStorageTypes\x18\x03\x20\x03\
    (\x0e2\x1d.hadoop.hdfs.StorageTypeProto\"\xbe\x01\n\x13OpReplaceBlockPro\
    to\x12,\n\x06header\x18\x01\x20\x02(\x0b2\x1c.hadoop.hdfs.BaseHeaderProt\
    o\x12\x0f\n\x07delHint\x18\x02\x20\x02(\t\x12.\n\x06source\x18\x03\x20\
    \x02(\x0b2\x1e.hadoop.hdfs.DatanodeInfoProto\x128\n\x0bstorageType\x18\
    \x04\x20\x01(\x0e2\x1d.hadoop.hdfs.StorageTypeProto:\x04DISK\"@\n\x10OpC\
    opyBlockProto\x12,\n\x06header\x18\x01\x20\x02(\x0b2\x1c.hadoop.hdfs.Bas\
    eHeaderProto\"D\n\x14OpBlockChecksumProto\x12,\n\x06header\x18\x01\x20\
    \x02(\x0b2\x1c.hadoop.hdfs.BaseHeaderProto\"0\n\x16ShortCircuitShmIdProt\
    o\x12\n\n\x02hi\x18\x01\x20\x02(\x03\x12\n\n\x02lo\x18\x02\x20\x02(\x03\
    \"_\n\x18ShortCircuitShmSlotProto\x122\n\x05shmId\x18\x01\x20\x02(\x0b2#\
    .hadoop.hdfs.ShortCircuitShmIdProto\x12\x0f\n\x07slotIdx\x18\x02\x20\x02\
    (\x05\"\x9b\x01\n\x20OpRequestShortCircuitAccessProto\x12,\n\x06header\
    \x18\x01\x20\x02(\x0b2\x1c.hadoop.hdfs.BaseHeaderProto\x12\x12\n\nmaxVer\
    sion\x18\x02\x20\x02(\r\x125\n\x06slotId\x18\x03\x20\x01(\x0b2%.hadoop.h\
    dfs.ShortCircuitShmSlotProto\"\x9a\x01\n%ReleaseShortCircuitAccessReques\
    tProto\x125\n\x06slotId\x18\x01\x20\x02(\x0b2%.hadoop.hdfs.ShortCircuitS\
    hmSlotProto\x12:\n\ttraceInfo\x18\x02\x20\x01(\x0b2'.hadoop.hdfs.DataTra\
    nsferTraceInfoProto\"\\\n&ReleaseShortCircuitAccessResponseProto\x12#\n\
    \x06status\x18\x01\x20\x02(\x0e2\x13.hadoop.hdfs.Status\x12\r\n\x05error\
    \x18\x02\x20\x01(\t\"m\n\x1bShortCircuitShmRequestProto\x12\x12\n\nclien\
    tName\x18\x01\x20\x02(\t\x12:\n\ttraceInfo\x18\x02\x20\x01(\x0b2'.hadoop\
    .hdfs.DataTransferTraceInfoProto\"\x83\x01\n\x1cShortCircuitShmResponseP\
    roto\x12#\n\x06status\x18\x01\x20\x02(\x0e2\x13.hadoop.hdfs.Status\x12\r\
    \n\x05error\x18\x02\x20\x01(\t\x12/\n\x02id\x18\x03\x20\x01(\x0b2#.hadoo\
    p.hdfs.ShortCircuitShmIdProto\"\x7f\n\x11PacketHeaderProto\x12\x15\n\rof\
    fsetInBlock\x18\x01\x20\x02(\x10\x12\r\n\x05seqno\x18\x02\x20\x02(\x10\
    \x12\x19\n\x11lastPacketInBlock\x18\x03\x20\x02(\x08\x12\x0f\n\x07dataLe\
    n\x18\x04\x20\x02(\x0f\x12\x18\n\tsyncBlock\x18\x05\x20\x01(\x08:\x05fal\
    se\"i\n\x10PipelineAckProto\x12\r\n\x05seqno\x18\x01\x20\x02(\x12\x12#\n\
    \x06status\x18\x02\x20\x03(\x0e2\x13.hadoop.hdfs.Status\x12!\n\x16downst\
    reamAckTimeNanos\x18\x03\x20\x01(\x04:\x010\"\\\n\x17ReadOpChecksumInfoP\
    roto\x12,\n\x08checksum\x18\x01\x20\x02(\x0b2\x1a.hadoop.hdfs.ChecksumPr\
    oto\x12\x13\n\x0bchunkOffset\x18\x02\x20\x02(\x04\"\x8c\x02\n\x14BlockOp\
    ResponseProto\x12#\n\x06status\x18\x01\x20\x02(\x0e2\x13.hadoop.hdfs.Sta\
    tus\x12\x14\n\x0cfirstBadLink\x18\x02\x20\x01(\t\x12C\n\x10checksumRespo\
    nse\x18\x03\x20\x01(\x0b2).hadoop.hdfs.OpBlockChecksumResponseProto\x12@\
    \n\x12readOpChecksumInfo\x18\x04\x20\x01(\x0b2$.hadoop.hdfs.ReadOpChecks\
    umInfoProto\x12\x0f\n\x07message\x18\x05\x20\x01(\t\x12!\n\x19shortCircu\
    itAccessVersion\x18\x06\x20\x01(\r\"<\n\x15ClientReadStatusProto\x12#\n\
    \x06status\x18\x01\x20\x02(\x0e2\x13.hadoop.hdfs.Status\"9\n\x12DNTransf\
    erAckProto\x12#\n\x06status\x18\x01\x20\x02(\x0e2\x13.hadoop.hdfs.Status\
    \"\x86\x01\n\x1cOpBlockChecksumResponseProto\x12\x13\n\x0bbytesPerCrc\
    \x18\x01\x20\x02(\r\x12\x13\n\x0bcrcPerBlock\x18\x02\x20\x02(\x04\x12\
    \x0b\n\x03md5\x18\x03\x20\x02(\x0c\x12/\n\x07crcType\x18\x04\x20\x01(\
    \x0e2\x1e.hadoop.hdfs.ChecksumTypeProto*\xf4\x01\n\x06Status\x12\x0b\n\
    \x07SUCCESS\x10\0\x12\t\n\x05ERROR\x10\x01\x12\x12\n\x0eERROR_CHECKSUM\
    \x10\x02\x12\x11\n\rERROR_INVALID\x10\x03\x12\x10\n\x0cERROR_EXISTS\x10\
    \x04\x12\x16\n\x12ERROR_ACCESS_TOKEN\x10\x05\x12\x0f\n\x0bCHECKSUM_OK\
    \x10\x06\x12\x15\n\x11ERROR_UNSUPPORTED\x10\x07\x12\x0f\n\x0bOOB_RESTART\
    \x10\x08\x12\x11\n\rOOB_RESERVED1\x10\t\x12\x11\n\rOOB_RESERVED2\x10\n\
    \x12\x11\n\rOOB_RESERVED3\x10\x0b\x12\x0f\n\x0bIN_PROGRESS\x10\x0cB>\n%o\
    rg.apache.hadoop.hdfs.protocol.protoB\x12DataTransferProtos\xa0\x01\x01\
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
