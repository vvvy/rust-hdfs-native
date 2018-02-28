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
pub struct ActiveNodeInfo {
    // message fields
    nameserviceId: ::protobuf::SingularField<::std::string::String>,
    namenodeId: ::protobuf::SingularField<::std::string::String>,
    hostname: ::protobuf::SingularField<::std::string::String>,
    port: ::std::option::Option<i32>,
    zkfcPort: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActiveNodeInfo {}

impl ActiveNodeInfo {
    pub fn new() -> ActiveNodeInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActiveNodeInfo {
        static mut instance: ::protobuf::lazy::Lazy<ActiveNodeInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActiveNodeInfo,
        };
        unsafe {
            instance.get(ActiveNodeInfo::new)
        }
    }

    // required string nameserviceId = 1;

    pub fn clear_nameserviceId(&mut self) {
        self.nameserviceId.clear();
    }

    pub fn has_nameserviceId(&self) -> bool {
        self.nameserviceId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nameserviceId(&mut self, v: ::std::string::String) {
        self.nameserviceId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nameserviceId(&mut self) -> &mut ::std::string::String {
        if self.nameserviceId.is_none() {
            self.nameserviceId.set_default();
        }
        self.nameserviceId.as_mut().unwrap()
    }

    // Take field
    pub fn take_nameserviceId(&mut self) -> ::std::string::String {
        self.nameserviceId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_nameserviceId(&self) -> &str {
        match self.nameserviceId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_nameserviceId_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.nameserviceId
    }

    fn mut_nameserviceId_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.nameserviceId
    }

    // required string namenodeId = 2;

    pub fn clear_namenodeId(&mut self) {
        self.namenodeId.clear();
    }

    pub fn has_namenodeId(&self) -> bool {
        self.namenodeId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_namenodeId(&mut self, v: ::std::string::String) {
        self.namenodeId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_namenodeId(&mut self) -> &mut ::std::string::String {
        if self.namenodeId.is_none() {
            self.namenodeId.set_default();
        }
        self.namenodeId.as_mut().unwrap()
    }

    // Take field
    pub fn take_namenodeId(&mut self) -> ::std::string::String {
        self.namenodeId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_namenodeId(&self) -> &str {
        match self.namenodeId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_namenodeId_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.namenodeId
    }

    fn mut_namenodeId_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.namenodeId
    }

    // required string hostname = 3;

    pub fn clear_hostname(&mut self) {
        self.hostname.clear();
    }

    pub fn has_hostname(&self) -> bool {
        self.hostname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostname(&mut self, v: ::std::string::String) {
        self.hostname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostname(&mut self) -> &mut ::std::string::String {
        if self.hostname.is_none() {
            self.hostname.set_default();
        }
        self.hostname.as_mut().unwrap()
    }

    // Take field
    pub fn take_hostname(&mut self) -> ::std::string::String {
        self.hostname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hostname(&self) -> &str {
        match self.hostname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_hostname_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.hostname
    }

    fn mut_hostname_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.hostname
    }

    // required int32 port = 4;

    pub fn clear_port(&mut self) {
        self.port = ::std::option::Option::None;
    }

    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: i32) {
        self.port = ::std::option::Option::Some(v);
    }

    pub fn get_port(&self) -> i32 {
        self.port.unwrap_or(0)
    }

    fn get_port_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.port
    }

    fn mut_port_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.port
    }

    // required int32 zkfcPort = 5;

    pub fn clear_zkfcPort(&mut self) {
        self.zkfcPort = ::std::option::Option::None;
    }

    pub fn has_zkfcPort(&self) -> bool {
        self.zkfcPort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_zkfcPort(&mut self, v: i32) {
        self.zkfcPort = ::std::option::Option::Some(v);
    }

    pub fn get_zkfcPort(&self) -> i32 {
        self.zkfcPort.unwrap_or(0)
    }

    fn get_zkfcPort_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.zkfcPort
    }

    fn mut_zkfcPort_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.zkfcPort
    }
}

impl ::protobuf::Message for ActiveNodeInfo {
    fn is_initialized(&self) -> bool {
        if self.nameserviceId.is_none() {
            return false;
        }
        if self.namenodeId.is_none() {
            return false;
        }
        if self.hostname.is_none() {
            return false;
        }
        if self.port.is_none() {
            return false;
        }
        if self.zkfcPort.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.nameserviceId)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.namenodeId)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hostname)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.port = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.zkfcPort = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.nameserviceId.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.namenodeId.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.hostname.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.port {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.zkfcPort {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.nameserviceId.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.namenodeId.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.hostname.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.port {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.zkfcPort {
            os.write_int32(5, v)?;
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

impl ::protobuf::MessageStatic for ActiveNodeInfo {
    fn new() -> ActiveNodeInfo {
        ActiveNodeInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActiveNodeInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "nameserviceId",
                    ActiveNodeInfo::get_nameserviceId_for_reflect,
                    ActiveNodeInfo::mut_nameserviceId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "namenodeId",
                    ActiveNodeInfo::get_namenodeId_for_reflect,
                    ActiveNodeInfo::mut_namenodeId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hostname",
                    ActiveNodeInfo::get_hostname_for_reflect,
                    ActiveNodeInfo::mut_hostname_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "port",
                    ActiveNodeInfo::get_port_for_reflect,
                    ActiveNodeInfo::mut_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "zkfcPort",
                    ActiveNodeInfo::get_zkfcPort_for_reflect,
                    ActiveNodeInfo::mut_zkfcPort_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActiveNodeInfo>(
                    "ActiveNodeInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActiveNodeInfo {
    fn clear(&mut self) {
        self.clear_nameserviceId();
        self.clear_namenodeId();
        self.clear_hostname();
        self.clear_port();
        self.clear_zkfcPort();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActiveNodeInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActiveNodeInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0eHAZKInfo.proto\x12\x0bhadoop.hdfs\"m\n\x0eActiveNodeInfo\x12\x15\n\
    \rnameserviceId\x18\x01\x20\x02(\t\x12\x12\n\nnamenodeId\x18\x02\x20\x02\
    (\t\x12\x10\n\x08hostname\x18\x03\x20\x02(\t\x12\x0c\n\x04port\x18\x04\
    \x20\x02(\x05\x12\x10\n\x08zkfcPort\x18\x05\x20\x02(\x05BA\n/org.apache.\
    hadoop.hdfs.server.namenode.ha.protoB\x0eHAZKInfoProtos\
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
