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
pub struct UserInformationProto {
    // message fields
    effectiveUser: ::protobuf::SingularField<::std::string::String>,
    realUser: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UserInformationProto {}

impl UserInformationProto {
    pub fn new() -> UserInformationProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UserInformationProto {
        static mut instance: ::protobuf::lazy::Lazy<UserInformationProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UserInformationProto,
        };
        unsafe {
            instance.get(UserInformationProto::new)
        }
    }

    // optional string effectiveUser = 1;

    pub fn clear_effectiveUser(&mut self) {
        self.effectiveUser.clear();
    }

    pub fn has_effectiveUser(&self) -> bool {
        self.effectiveUser.is_some()
    }

    // Param is passed by value, moved
    pub fn set_effectiveUser(&mut self, v: ::std::string::String) {
        self.effectiveUser = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_effectiveUser(&mut self) -> &mut ::std::string::String {
        if self.effectiveUser.is_none() {
            self.effectiveUser.set_default();
        }
        self.effectiveUser.as_mut().unwrap()
    }

    // Take field
    pub fn take_effectiveUser(&mut self) -> ::std::string::String {
        self.effectiveUser.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_effectiveUser(&self) -> &str {
        match self.effectiveUser.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_effectiveUser_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.effectiveUser
    }

    fn mut_effectiveUser_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.effectiveUser
    }

    // optional string realUser = 2;

    pub fn clear_realUser(&mut self) {
        self.realUser.clear();
    }

    pub fn has_realUser(&self) -> bool {
        self.realUser.is_some()
    }

    // Param is passed by value, moved
    pub fn set_realUser(&mut self, v: ::std::string::String) {
        self.realUser = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_realUser(&mut self) -> &mut ::std::string::String {
        if self.realUser.is_none() {
            self.realUser.set_default();
        }
        self.realUser.as_mut().unwrap()
    }

    // Take field
    pub fn take_realUser(&mut self) -> ::std::string::String {
        self.realUser.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_realUser(&self) -> &str {
        match self.realUser.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_realUser_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.realUser
    }

    fn mut_realUser_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.realUser
    }
}

impl ::protobuf::Message for UserInformationProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.effectiveUser)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.realUser)?;
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
        if let Some(ref v) = self.effectiveUser.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.realUser.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.effectiveUser.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.realUser.as_ref() {
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

impl ::protobuf::MessageStatic for UserInformationProto {
    fn new() -> UserInformationProto {
        UserInformationProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<UserInformationProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "effectiveUser",
                    UserInformationProto::get_effectiveUser_for_reflect,
                    UserInformationProto::mut_effectiveUser_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "realUser",
                    UserInformationProto::get_realUser_for_reflect,
                    UserInformationProto::mut_realUser_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UserInformationProto>(
                    "UserInformationProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UserInformationProto {
    fn clear(&mut self) {
        self.clear_effectiveUser();
        self.clear_realUser();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserInformationProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserInformationProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IpcConnectionContextProto {
    // message fields
    userInfo: ::protobuf::SingularPtrField<UserInformationProto>,
    protocol: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IpcConnectionContextProto {}

impl IpcConnectionContextProto {
    pub fn new() -> IpcConnectionContextProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IpcConnectionContextProto {
        static mut instance: ::protobuf::lazy::Lazy<IpcConnectionContextProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IpcConnectionContextProto,
        };
        unsafe {
            instance.get(IpcConnectionContextProto::new)
        }
    }

    // optional .hadoop.common.UserInformationProto userInfo = 2;

    pub fn clear_userInfo(&mut self) {
        self.userInfo.clear();
    }

    pub fn has_userInfo(&self) -> bool {
        self.userInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_userInfo(&mut self, v: UserInformationProto) {
        self.userInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_userInfo(&mut self) -> &mut UserInformationProto {
        if self.userInfo.is_none() {
            self.userInfo.set_default();
        }
        self.userInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_userInfo(&mut self) -> UserInformationProto {
        self.userInfo.take().unwrap_or_else(|| UserInformationProto::new())
    }

    pub fn get_userInfo(&self) -> &UserInformationProto {
        self.userInfo.as_ref().unwrap_or_else(|| UserInformationProto::default_instance())
    }

    fn get_userInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<UserInformationProto> {
        &self.userInfo
    }

    fn mut_userInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<UserInformationProto> {
        &mut self.userInfo
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
}

impl ::protobuf::Message for IpcConnectionContextProto {
    fn is_initialized(&self) -> bool {
        for v in &self.userInfo {
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
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.userInfo)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.protocol)?;
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
        if let Some(ref v) = self.userInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.protocol.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.userInfo.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.protocol.as_ref() {
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

impl ::protobuf::MessageStatic for IpcConnectionContextProto {
    fn new() -> IpcConnectionContextProto {
        IpcConnectionContextProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<IpcConnectionContextProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UserInformationProto>>(
                    "userInfo",
                    IpcConnectionContextProto::get_userInfo_for_reflect,
                    IpcConnectionContextProto::mut_userInfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "protocol",
                    IpcConnectionContextProto::get_protocol_for_reflect,
                    IpcConnectionContextProto::mut_protocol_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IpcConnectionContextProto>(
                    "IpcConnectionContextProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IpcConnectionContextProto {
    fn clear(&mut self) {
        self.clear_userInfo();
        self.clear_protocol();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IpcConnectionContextProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IpcConnectionContextProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aIpcConnectionContext.proto\x12\rhadoop.common\"?\n\x14UserInformat\
    ionProto\x12\x15\n\reffectiveUser\x18\x01\x20\x01(\t\x12\x10\n\x08realUs\
    er\x18\x02\x20\x01(\t\"d\n\x19IpcConnectionContextProto\x125\n\x08userIn\
    fo\x18\x02\x20\x01(\x0b2#.hadoop.common.UserInformationProto\x12\x10\n\
    \x08protocol\x18\x03\x20\x01(\tB?\n\x1eorg.apache.hadoop.ipc.protobufB\
    \x1aIpcConnectionContextProtos\xa0\x01\x01\
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
