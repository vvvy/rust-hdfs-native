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
pub struct XAttrProto {
    // message fields
    namespace: ::std::option::Option<XAttrProto_XAttrNamespaceProto>,
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for XAttrProto {}

impl XAttrProto {
    pub fn new() -> XAttrProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static XAttrProto {
        static mut instance: ::protobuf::lazy::Lazy<XAttrProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const XAttrProto,
        };
        unsafe {
            instance.get(XAttrProto::new)
        }
    }

    // required .hadoop.hdfs.XAttrProto.XAttrNamespaceProto namespace = 1;

    pub fn clear_namespace(&mut self) {
        self.namespace = ::std::option::Option::None;
    }

    pub fn has_namespace(&self) -> bool {
        self.namespace.is_some()
    }

    // Param is passed by value, moved
    pub fn set_namespace(&mut self, v: XAttrProto_XAttrNamespaceProto) {
        self.namespace = ::std::option::Option::Some(v);
    }

    pub fn get_namespace(&self) -> XAttrProto_XAttrNamespaceProto {
        self.namespace.unwrap_or(XAttrProto_XAttrNamespaceProto::USER)
    }

    fn get_namespace_for_reflect(&self) -> &::std::option::Option<XAttrProto_XAttrNamespaceProto> {
        &self.namespace
    }

    fn mut_namespace_for_reflect(&mut self) -> &mut ::std::option::Option<XAttrProto_XAttrNamespaceProto> {
        &mut self.namespace
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

    // optional bytes value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.value
    }
}

impl ::protobuf::Message for XAttrProto {
    fn is_initialized(&self) -> bool {
        if self.namespace.is_none() {
            return false;
        }
        if self.name.is_none() {
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
                    self.namespace = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
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
        if let Some(v) = self.namespace {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.namespace {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.value.as_ref() {
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

impl ::protobuf::MessageStatic for XAttrProto {
    fn new() -> XAttrProto {
        XAttrProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<XAttrProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<XAttrProto_XAttrNamespaceProto>>(
                    "namespace",
                    XAttrProto::get_namespace_for_reflect,
                    XAttrProto::mut_namespace_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    XAttrProto::get_name_for_reflect,
                    XAttrProto::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    XAttrProto::get_value_for_reflect,
                    XAttrProto::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<XAttrProto>(
                    "XAttrProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for XAttrProto {
    fn clear(&mut self) {
        self.clear_namespace();
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for XAttrProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for XAttrProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum XAttrProto_XAttrNamespaceProto {
    USER = 0,
    TRUSTED = 1,
    SECURITY = 2,
    SYSTEM = 3,
    RAW = 4,
}

impl ::protobuf::ProtobufEnum for XAttrProto_XAttrNamespaceProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<XAttrProto_XAttrNamespaceProto> {
        match value {
            0 => ::std::option::Option::Some(XAttrProto_XAttrNamespaceProto::USER),
            1 => ::std::option::Option::Some(XAttrProto_XAttrNamespaceProto::TRUSTED),
            2 => ::std::option::Option::Some(XAttrProto_XAttrNamespaceProto::SECURITY),
            3 => ::std::option::Option::Some(XAttrProto_XAttrNamespaceProto::SYSTEM),
            4 => ::std::option::Option::Some(XAttrProto_XAttrNamespaceProto::RAW),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [XAttrProto_XAttrNamespaceProto] = &[
            XAttrProto_XAttrNamespaceProto::USER,
            XAttrProto_XAttrNamespaceProto::TRUSTED,
            XAttrProto_XAttrNamespaceProto::SECURITY,
            XAttrProto_XAttrNamespaceProto::SYSTEM,
            XAttrProto_XAttrNamespaceProto::RAW,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<XAttrProto_XAttrNamespaceProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("XAttrProto_XAttrNamespaceProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for XAttrProto_XAttrNamespaceProto {
}

impl ::protobuf::reflect::ProtobufValue for XAttrProto_XAttrNamespaceProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct XAttrEditLogProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    xAttrs: ::protobuf::RepeatedField<XAttrProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for XAttrEditLogProto {}

impl XAttrEditLogProto {
    pub fn new() -> XAttrEditLogProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static XAttrEditLogProto {
        static mut instance: ::protobuf::lazy::Lazy<XAttrEditLogProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const XAttrEditLogProto,
        };
        unsafe {
            instance.get(XAttrEditLogProto::new)
        }
    }

    // optional string src = 1;

    pub fn clear_src(&mut self) {
        self.src.clear();
    }

    pub fn has_src(&self) -> bool {
        self.src.is_some()
    }

    // Param is passed by value, moved
    pub fn set_src(&mut self, v: ::std::string::String) {
        self.src = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_src(&mut self) -> &mut ::std::string::String {
        if self.src.is_none() {
            self.src.set_default();
        }
        self.src.as_mut().unwrap()
    }

    // Take field
    pub fn take_src(&mut self) -> ::std::string::String {
        self.src.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_src(&self) -> &str {
        match self.src.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_src_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.src
    }

    fn mut_src_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.src
    }

    // repeated .hadoop.hdfs.XAttrProto xAttrs = 2;

    pub fn clear_xAttrs(&mut self) {
        self.xAttrs.clear();
    }

    // Param is passed by value, moved
    pub fn set_xAttrs(&mut self, v: ::protobuf::RepeatedField<XAttrProto>) {
        self.xAttrs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_xAttrs(&mut self) -> &mut ::protobuf::RepeatedField<XAttrProto> {
        &mut self.xAttrs
    }

    // Take field
    pub fn take_xAttrs(&mut self) -> ::protobuf::RepeatedField<XAttrProto> {
        ::std::mem::replace(&mut self.xAttrs, ::protobuf::RepeatedField::new())
    }

    pub fn get_xAttrs(&self) -> &[XAttrProto] {
        &self.xAttrs
    }

    fn get_xAttrs_for_reflect(&self) -> &::protobuf::RepeatedField<XAttrProto> {
        &self.xAttrs
    }

    fn mut_xAttrs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<XAttrProto> {
        &mut self.xAttrs
    }
}

impl ::protobuf::Message for XAttrEditLogProto {
    fn is_initialized(&self) -> bool {
        for v in &self.xAttrs {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.src)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.xAttrs)?;
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
        if let Some(ref v) = self.src.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.xAttrs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.src.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.xAttrs {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for XAttrEditLogProto {
    fn new() -> XAttrEditLogProto {
        XAttrEditLogProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<XAttrEditLogProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src",
                    XAttrEditLogProto::get_src_for_reflect,
                    XAttrEditLogProto::mut_src_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<XAttrProto>>(
                    "xAttrs",
                    XAttrEditLogProto::get_xAttrs_for_reflect,
                    XAttrEditLogProto::mut_xAttrs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<XAttrEditLogProto>(
                    "XAttrEditLogProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for XAttrEditLogProto {
    fn clear(&mut self) {
        self.clear_src();
        self.clear_xAttrs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for XAttrEditLogProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for XAttrEditLogProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetXAttrRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    xAttr: ::protobuf::SingularPtrField<XAttrProto>,
    flag: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetXAttrRequestProto {}

impl SetXAttrRequestProto {
    pub fn new() -> SetXAttrRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetXAttrRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<SetXAttrRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetXAttrRequestProto,
        };
        unsafe {
            instance.get(SetXAttrRequestProto::new)
        }
    }

    // required string src = 1;

    pub fn clear_src(&mut self) {
        self.src.clear();
    }

    pub fn has_src(&self) -> bool {
        self.src.is_some()
    }

    // Param is passed by value, moved
    pub fn set_src(&mut self, v: ::std::string::String) {
        self.src = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_src(&mut self) -> &mut ::std::string::String {
        if self.src.is_none() {
            self.src.set_default();
        }
        self.src.as_mut().unwrap()
    }

    // Take field
    pub fn take_src(&mut self) -> ::std::string::String {
        self.src.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_src(&self) -> &str {
        match self.src.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_src_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.src
    }

    fn mut_src_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.src
    }

    // optional .hadoop.hdfs.XAttrProto xAttr = 2;

    pub fn clear_xAttr(&mut self) {
        self.xAttr.clear();
    }

    pub fn has_xAttr(&self) -> bool {
        self.xAttr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xAttr(&mut self, v: XAttrProto) {
        self.xAttr = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_xAttr(&mut self) -> &mut XAttrProto {
        if self.xAttr.is_none() {
            self.xAttr.set_default();
        }
        self.xAttr.as_mut().unwrap()
    }

    // Take field
    pub fn take_xAttr(&mut self) -> XAttrProto {
        self.xAttr.take().unwrap_or_else(|| XAttrProto::new())
    }

    pub fn get_xAttr(&self) -> &XAttrProto {
        self.xAttr.as_ref().unwrap_or_else(|| XAttrProto::default_instance())
    }

    fn get_xAttr_for_reflect(&self) -> &::protobuf::SingularPtrField<XAttrProto> {
        &self.xAttr
    }

    fn mut_xAttr_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<XAttrProto> {
        &mut self.xAttr
    }

    // optional uint32 flag = 3;

    pub fn clear_flag(&mut self) {
        self.flag = ::std::option::Option::None;
    }

    pub fn has_flag(&self) -> bool {
        self.flag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flag(&mut self, v: u32) {
        self.flag = ::std::option::Option::Some(v);
    }

    pub fn get_flag(&self) -> u32 {
        self.flag.unwrap_or(0)
    }

    fn get_flag_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.flag
    }

    fn mut_flag_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.flag
    }
}

impl ::protobuf::Message for SetXAttrRequestProto {
    fn is_initialized(&self) -> bool {
        if self.src.is_none() {
            return false;
        }
        for v in &self.xAttr {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.src)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.xAttr)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.flag = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.src.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.xAttr.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.flag {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.src.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.xAttr.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.flag {
            os.write_uint32(3, v)?;
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

impl ::protobuf::MessageStatic for SetXAttrRequestProto {
    fn new() -> SetXAttrRequestProto {
        SetXAttrRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetXAttrRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src",
                    SetXAttrRequestProto::get_src_for_reflect,
                    SetXAttrRequestProto::mut_src_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<XAttrProto>>(
                    "xAttr",
                    SetXAttrRequestProto::get_xAttr_for_reflect,
                    SetXAttrRequestProto::mut_xAttr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "flag",
                    SetXAttrRequestProto::get_flag_for_reflect,
                    SetXAttrRequestProto::mut_flag_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetXAttrRequestProto>(
                    "SetXAttrRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetXAttrRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.clear_xAttr();
        self.clear_flag();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetXAttrRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetXAttrRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SetXAttrResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SetXAttrResponseProto {}

impl SetXAttrResponseProto {
    pub fn new() -> SetXAttrResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetXAttrResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<SetXAttrResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetXAttrResponseProto,
        };
        unsafe {
            instance.get(SetXAttrResponseProto::new)
        }
    }
}

impl ::protobuf::Message for SetXAttrResponseProto {
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

impl ::protobuf::MessageStatic for SetXAttrResponseProto {
    fn new() -> SetXAttrResponseProto {
        SetXAttrResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetXAttrResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<SetXAttrResponseProto>(
                    "SetXAttrResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetXAttrResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SetXAttrResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SetXAttrResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetXAttrsRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    xAttrs: ::protobuf::RepeatedField<XAttrProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetXAttrsRequestProto {}

impl GetXAttrsRequestProto {
    pub fn new() -> GetXAttrsRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetXAttrsRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetXAttrsRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetXAttrsRequestProto,
        };
        unsafe {
            instance.get(GetXAttrsRequestProto::new)
        }
    }

    // required string src = 1;

    pub fn clear_src(&mut self) {
        self.src.clear();
    }

    pub fn has_src(&self) -> bool {
        self.src.is_some()
    }

    // Param is passed by value, moved
    pub fn set_src(&mut self, v: ::std::string::String) {
        self.src = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_src(&mut self) -> &mut ::std::string::String {
        if self.src.is_none() {
            self.src.set_default();
        }
        self.src.as_mut().unwrap()
    }

    // Take field
    pub fn take_src(&mut self) -> ::std::string::String {
        self.src.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_src(&self) -> &str {
        match self.src.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_src_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.src
    }

    fn mut_src_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.src
    }

    // repeated .hadoop.hdfs.XAttrProto xAttrs = 2;

    pub fn clear_xAttrs(&mut self) {
        self.xAttrs.clear();
    }

    // Param is passed by value, moved
    pub fn set_xAttrs(&mut self, v: ::protobuf::RepeatedField<XAttrProto>) {
        self.xAttrs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_xAttrs(&mut self) -> &mut ::protobuf::RepeatedField<XAttrProto> {
        &mut self.xAttrs
    }

    // Take field
    pub fn take_xAttrs(&mut self) -> ::protobuf::RepeatedField<XAttrProto> {
        ::std::mem::replace(&mut self.xAttrs, ::protobuf::RepeatedField::new())
    }

    pub fn get_xAttrs(&self) -> &[XAttrProto] {
        &self.xAttrs
    }

    fn get_xAttrs_for_reflect(&self) -> &::protobuf::RepeatedField<XAttrProto> {
        &self.xAttrs
    }

    fn mut_xAttrs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<XAttrProto> {
        &mut self.xAttrs
    }
}

impl ::protobuf::Message for GetXAttrsRequestProto {
    fn is_initialized(&self) -> bool {
        if self.src.is_none() {
            return false;
        }
        for v in &self.xAttrs {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.src)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.xAttrs)?;
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
        if let Some(ref v) = self.src.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.xAttrs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.src.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.xAttrs {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for GetXAttrsRequestProto {
    fn new() -> GetXAttrsRequestProto {
        GetXAttrsRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetXAttrsRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src",
                    GetXAttrsRequestProto::get_src_for_reflect,
                    GetXAttrsRequestProto::mut_src_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<XAttrProto>>(
                    "xAttrs",
                    GetXAttrsRequestProto::get_xAttrs_for_reflect,
                    GetXAttrsRequestProto::mut_xAttrs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetXAttrsRequestProto>(
                    "GetXAttrsRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetXAttrsRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.clear_xAttrs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetXAttrsRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetXAttrsRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetXAttrsResponseProto {
    // message fields
    xAttrs: ::protobuf::RepeatedField<XAttrProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetXAttrsResponseProto {}

impl GetXAttrsResponseProto {
    pub fn new() -> GetXAttrsResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetXAttrsResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetXAttrsResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetXAttrsResponseProto,
        };
        unsafe {
            instance.get(GetXAttrsResponseProto::new)
        }
    }

    // repeated .hadoop.hdfs.XAttrProto xAttrs = 1;

    pub fn clear_xAttrs(&mut self) {
        self.xAttrs.clear();
    }

    // Param is passed by value, moved
    pub fn set_xAttrs(&mut self, v: ::protobuf::RepeatedField<XAttrProto>) {
        self.xAttrs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_xAttrs(&mut self) -> &mut ::protobuf::RepeatedField<XAttrProto> {
        &mut self.xAttrs
    }

    // Take field
    pub fn take_xAttrs(&mut self) -> ::protobuf::RepeatedField<XAttrProto> {
        ::std::mem::replace(&mut self.xAttrs, ::protobuf::RepeatedField::new())
    }

    pub fn get_xAttrs(&self) -> &[XAttrProto] {
        &self.xAttrs
    }

    fn get_xAttrs_for_reflect(&self) -> &::protobuf::RepeatedField<XAttrProto> {
        &self.xAttrs
    }

    fn mut_xAttrs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<XAttrProto> {
        &mut self.xAttrs
    }
}

impl ::protobuf::Message for GetXAttrsResponseProto {
    fn is_initialized(&self) -> bool {
        for v in &self.xAttrs {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.xAttrs)?;
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
        for value in &self.xAttrs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.xAttrs {
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

impl ::protobuf::MessageStatic for GetXAttrsResponseProto {
    fn new() -> GetXAttrsResponseProto {
        GetXAttrsResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetXAttrsResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<XAttrProto>>(
                    "xAttrs",
                    GetXAttrsResponseProto::get_xAttrs_for_reflect,
                    GetXAttrsResponseProto::mut_xAttrs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetXAttrsResponseProto>(
                    "GetXAttrsResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetXAttrsResponseProto {
    fn clear(&mut self) {
        self.clear_xAttrs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetXAttrsResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetXAttrsResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListXAttrsRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListXAttrsRequestProto {}

impl ListXAttrsRequestProto {
    pub fn new() -> ListXAttrsRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListXAttrsRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ListXAttrsRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListXAttrsRequestProto,
        };
        unsafe {
            instance.get(ListXAttrsRequestProto::new)
        }
    }

    // required string src = 1;

    pub fn clear_src(&mut self) {
        self.src.clear();
    }

    pub fn has_src(&self) -> bool {
        self.src.is_some()
    }

    // Param is passed by value, moved
    pub fn set_src(&mut self, v: ::std::string::String) {
        self.src = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_src(&mut self) -> &mut ::std::string::String {
        if self.src.is_none() {
            self.src.set_default();
        }
        self.src.as_mut().unwrap()
    }

    // Take field
    pub fn take_src(&mut self) -> ::std::string::String {
        self.src.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_src(&self) -> &str {
        match self.src.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_src_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.src
    }

    fn mut_src_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.src
    }
}

impl ::protobuf::Message for ListXAttrsRequestProto {
    fn is_initialized(&self) -> bool {
        if self.src.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.src)?;
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
        if let Some(ref v) = self.src.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.src.as_ref() {
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

impl ::protobuf::MessageStatic for ListXAttrsRequestProto {
    fn new() -> ListXAttrsRequestProto {
        ListXAttrsRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListXAttrsRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src",
                    ListXAttrsRequestProto::get_src_for_reflect,
                    ListXAttrsRequestProto::mut_src_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListXAttrsRequestProto>(
                    "ListXAttrsRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListXAttrsRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListXAttrsRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListXAttrsRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListXAttrsResponseProto {
    // message fields
    xAttrs: ::protobuf::RepeatedField<XAttrProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListXAttrsResponseProto {}

impl ListXAttrsResponseProto {
    pub fn new() -> ListXAttrsResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListXAttrsResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ListXAttrsResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListXAttrsResponseProto,
        };
        unsafe {
            instance.get(ListXAttrsResponseProto::new)
        }
    }

    // repeated .hadoop.hdfs.XAttrProto xAttrs = 1;

    pub fn clear_xAttrs(&mut self) {
        self.xAttrs.clear();
    }

    // Param is passed by value, moved
    pub fn set_xAttrs(&mut self, v: ::protobuf::RepeatedField<XAttrProto>) {
        self.xAttrs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_xAttrs(&mut self) -> &mut ::protobuf::RepeatedField<XAttrProto> {
        &mut self.xAttrs
    }

    // Take field
    pub fn take_xAttrs(&mut self) -> ::protobuf::RepeatedField<XAttrProto> {
        ::std::mem::replace(&mut self.xAttrs, ::protobuf::RepeatedField::new())
    }

    pub fn get_xAttrs(&self) -> &[XAttrProto] {
        &self.xAttrs
    }

    fn get_xAttrs_for_reflect(&self) -> &::protobuf::RepeatedField<XAttrProto> {
        &self.xAttrs
    }

    fn mut_xAttrs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<XAttrProto> {
        &mut self.xAttrs
    }
}

impl ::protobuf::Message for ListXAttrsResponseProto {
    fn is_initialized(&self) -> bool {
        for v in &self.xAttrs {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.xAttrs)?;
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
        for value in &self.xAttrs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.xAttrs {
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

impl ::protobuf::MessageStatic for ListXAttrsResponseProto {
    fn new() -> ListXAttrsResponseProto {
        ListXAttrsResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListXAttrsResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<XAttrProto>>(
                    "xAttrs",
                    ListXAttrsResponseProto::get_xAttrs_for_reflect,
                    ListXAttrsResponseProto::mut_xAttrs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListXAttrsResponseProto>(
                    "ListXAttrsResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListXAttrsResponseProto {
    fn clear(&mut self) {
        self.clear_xAttrs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListXAttrsResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListXAttrsResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RemoveXAttrRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    xAttr: ::protobuf::SingularPtrField<XAttrProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemoveXAttrRequestProto {}

impl RemoveXAttrRequestProto {
    pub fn new() -> RemoveXAttrRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveXAttrRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoveXAttrRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveXAttrRequestProto,
        };
        unsafe {
            instance.get(RemoveXAttrRequestProto::new)
        }
    }

    // required string src = 1;

    pub fn clear_src(&mut self) {
        self.src.clear();
    }

    pub fn has_src(&self) -> bool {
        self.src.is_some()
    }

    // Param is passed by value, moved
    pub fn set_src(&mut self, v: ::std::string::String) {
        self.src = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_src(&mut self) -> &mut ::std::string::String {
        if self.src.is_none() {
            self.src.set_default();
        }
        self.src.as_mut().unwrap()
    }

    // Take field
    pub fn take_src(&mut self) -> ::std::string::String {
        self.src.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_src(&self) -> &str {
        match self.src.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_src_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.src
    }

    fn mut_src_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.src
    }

    // optional .hadoop.hdfs.XAttrProto xAttr = 2;

    pub fn clear_xAttr(&mut self) {
        self.xAttr.clear();
    }

    pub fn has_xAttr(&self) -> bool {
        self.xAttr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xAttr(&mut self, v: XAttrProto) {
        self.xAttr = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_xAttr(&mut self) -> &mut XAttrProto {
        if self.xAttr.is_none() {
            self.xAttr.set_default();
        }
        self.xAttr.as_mut().unwrap()
    }

    // Take field
    pub fn take_xAttr(&mut self) -> XAttrProto {
        self.xAttr.take().unwrap_or_else(|| XAttrProto::new())
    }

    pub fn get_xAttr(&self) -> &XAttrProto {
        self.xAttr.as_ref().unwrap_or_else(|| XAttrProto::default_instance())
    }

    fn get_xAttr_for_reflect(&self) -> &::protobuf::SingularPtrField<XAttrProto> {
        &self.xAttr
    }

    fn mut_xAttr_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<XAttrProto> {
        &mut self.xAttr
    }
}

impl ::protobuf::Message for RemoveXAttrRequestProto {
    fn is_initialized(&self) -> bool {
        if self.src.is_none() {
            return false;
        }
        for v in &self.xAttr {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.src)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.xAttr)?;
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
        if let Some(ref v) = self.src.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.xAttr.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.src.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.xAttr.as_ref() {
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

impl ::protobuf::MessageStatic for RemoveXAttrRequestProto {
    fn new() -> RemoveXAttrRequestProto {
        RemoveXAttrRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveXAttrRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src",
                    RemoveXAttrRequestProto::get_src_for_reflect,
                    RemoveXAttrRequestProto::mut_src_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<XAttrProto>>(
                    "xAttr",
                    RemoveXAttrRequestProto::get_xAttr_for_reflect,
                    RemoveXAttrRequestProto::mut_xAttr_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RemoveXAttrRequestProto>(
                    "RemoveXAttrRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveXAttrRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.clear_xAttr();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RemoveXAttrRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemoveXAttrRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RemoveXAttrResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RemoveXAttrResponseProto {}

impl RemoveXAttrResponseProto {
    pub fn new() -> RemoveXAttrResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveXAttrResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoveXAttrResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveXAttrResponseProto,
        };
        unsafe {
            instance.get(RemoveXAttrResponseProto::new)
        }
    }
}

impl ::protobuf::Message for RemoveXAttrResponseProto {
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

impl ::protobuf::MessageStatic for RemoveXAttrResponseProto {
    fn new() -> RemoveXAttrResponseProto {
        RemoveXAttrResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveXAttrResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RemoveXAttrResponseProto>(
                    "RemoveXAttrResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveXAttrResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RemoveXAttrResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RemoveXAttrResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum XAttrSetFlagProto {
    XATTR_CREATE = 1,
    XATTR_REPLACE = 2,
}

impl ::protobuf::ProtobufEnum for XAttrSetFlagProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<XAttrSetFlagProto> {
        match value {
            1 => ::std::option::Option::Some(XAttrSetFlagProto::XATTR_CREATE),
            2 => ::std::option::Option::Some(XAttrSetFlagProto::XATTR_REPLACE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [XAttrSetFlagProto] = &[
            XAttrSetFlagProto::XATTR_CREATE,
            XAttrSetFlagProto::XATTR_REPLACE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<XAttrSetFlagProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("XAttrSetFlagProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for XAttrSetFlagProto {
}

impl ::protobuf::reflect::ProtobufValue for XAttrSetFlagProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bxattr.proto\x12\x0bhadoop.hdfs\"\xba\x01\n\nXAttrProto\x12>\n\tnam\
    espace\x18\x01\x20\x02(\x0e2+.hadoop.hdfs.XAttrProto.XAttrNamespaceProto\
    \x12\x0c\n\x04name\x18\x02\x20\x02(\t\x12\r\n\x05value\x18\x03\x20\x01(\
    \x0c\"O\n\x13XAttrNamespaceProto\x12\x08\n\x04USER\x10\0\x12\x0b\n\x07TR\
    USTED\x10\x01\x12\x0c\n\x08SECURITY\x10\x02\x12\n\n\x06SYSTEM\x10\x03\
    \x12\x07\n\x03RAW\x10\x04\"I\n\x11XAttrEditLogProto\x12\x0b\n\x03src\x18\
    \x01\x20\x01(\t\x12'\n\x06xAttrs\x18\x02\x20\x03(\x0b2\x17.hadoop.hdfs.X\
    AttrProto\"Y\n\x14SetXAttrRequestProto\x12\x0b\n\x03src\x18\x01\x20\x02(\
    \t\x12&\n\x05xAttr\x18\x02\x20\x01(\x0b2\x17.hadoop.hdfs.XAttrProto\x12\
    \x0c\n\x04flag\x18\x03\x20\x01(\r\"\x17\n\x15SetXAttrResponseProto\"M\n\
    \x15GetXAttrsRequestProto\x12\x0b\n\x03src\x18\x01\x20\x02(\t\x12'\n\x06\
    xAttrs\x18\x02\x20\x03(\x0b2\x17.hadoop.hdfs.XAttrProto\"A\n\x16GetXAttr\
    sResponseProto\x12'\n\x06xAttrs\x18\x01\x20\x03(\x0b2\x17.hadoop.hdfs.XA\
    ttrProto\"%\n\x16ListXAttrsRequestProto\x12\x0b\n\x03src\x18\x01\x20\x02\
    (\t\"B\n\x17ListXAttrsResponseProto\x12'\n\x06xAttrs\x18\x01\x20\x03(\
    \x0b2\x17.hadoop.hdfs.XAttrProto\"N\n\x17RemoveXAttrRequestProto\x12\x0b\
    \n\x03src\x18\x01\x20\x02(\t\x12&\n\x05xAttr\x18\x02\x20\x01(\x0b2\x17.h\
    adoop.hdfs.XAttrProto\"\x1a\n\x18RemoveXAttrResponseProto*8\n\x11XAttrSe\
    tFlagProto\x12\x10\n\x0cXATTR_CREATE\x10\x01\x12\x11\n\rXATTR_REPLACE\
    \x10\x02B7\n%org.apache.hadoop.hdfs.protocol.protoB\x0bXAttrProtos\xa0\
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
