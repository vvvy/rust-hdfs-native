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
pub struct FileSummary {
    // message fields
    ondiskVersion: ::std::option::Option<u32>,
    layoutVersion: ::std::option::Option<u32>,
    codec: ::protobuf::SingularField<::std::string::String>,
    sections: ::protobuf::RepeatedField<FileSummary_Section>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FileSummary {}

impl FileSummary {
    pub fn new() -> FileSummary {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FileSummary {
        static mut instance: ::protobuf::lazy::Lazy<FileSummary> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FileSummary,
        };
        unsafe {
            instance.get(FileSummary::new)
        }
    }

    // required uint32 ondiskVersion = 1;

    pub fn clear_ondiskVersion(&mut self) {
        self.ondiskVersion = ::std::option::Option::None;
    }

    pub fn has_ondiskVersion(&self) -> bool {
        self.ondiskVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ondiskVersion(&mut self, v: u32) {
        self.ondiskVersion = ::std::option::Option::Some(v);
    }

    pub fn get_ondiskVersion(&self) -> u32 {
        self.ondiskVersion.unwrap_or(0)
    }

    fn get_ondiskVersion_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ondiskVersion
    }

    fn mut_ondiskVersion_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ondiskVersion
    }

    // required uint32 layoutVersion = 2;

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

    // optional string codec = 3;

    pub fn clear_codec(&mut self) {
        self.codec.clear();
    }

    pub fn has_codec(&self) -> bool {
        self.codec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_codec(&mut self, v: ::std::string::String) {
        self.codec = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_codec(&mut self) -> &mut ::std::string::String {
        if self.codec.is_none() {
            self.codec.set_default();
        }
        self.codec.as_mut().unwrap()
    }

    // Take field
    pub fn take_codec(&mut self) -> ::std::string::String {
        self.codec.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_codec(&self) -> &str {
        match self.codec.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_codec_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.codec
    }

    fn mut_codec_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.codec
    }

    // repeated .hadoop.hdfs.fsimage.FileSummary.Section sections = 4;

    pub fn clear_sections(&mut self) {
        self.sections.clear();
    }

    // Param is passed by value, moved
    pub fn set_sections(&mut self, v: ::protobuf::RepeatedField<FileSummary_Section>) {
        self.sections = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sections(&mut self) -> &mut ::protobuf::RepeatedField<FileSummary_Section> {
        &mut self.sections
    }

    // Take field
    pub fn take_sections(&mut self) -> ::protobuf::RepeatedField<FileSummary_Section> {
        ::std::mem::replace(&mut self.sections, ::protobuf::RepeatedField::new())
    }

    pub fn get_sections(&self) -> &[FileSummary_Section] {
        &self.sections
    }

    fn get_sections_for_reflect(&self) -> &::protobuf::RepeatedField<FileSummary_Section> {
        &self.sections
    }

    fn mut_sections_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<FileSummary_Section> {
        &mut self.sections
    }
}

impl ::protobuf::Message for FileSummary {
    fn is_initialized(&self) -> bool {
        if self.ondiskVersion.is_none() {
            return false;
        }
        if self.layoutVersion.is_none() {
            return false;
        }
        for v in &self.sections {
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
                    self.ondiskVersion = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.layoutVersion = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.codec)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.sections)?;
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
        if let Some(v) = self.ondiskVersion {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.layoutVersion {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.codec.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        for value in &self.sections {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ondiskVersion {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.layoutVersion {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.codec.as_ref() {
            os.write_string(3, &v)?;
        }
        for v in &self.sections {
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

impl ::protobuf::MessageStatic for FileSummary {
    fn new() -> FileSummary {
        FileSummary::new()
    }

    fn descriptor_static(_: ::std::option::Option<FileSummary>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ondiskVersion",
                    FileSummary::get_ondiskVersion_for_reflect,
                    FileSummary::mut_ondiskVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "layoutVersion",
                    FileSummary::get_layoutVersion_for_reflect,
                    FileSummary::mut_layoutVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "codec",
                    FileSummary::get_codec_for_reflect,
                    FileSummary::mut_codec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FileSummary_Section>>(
                    "sections",
                    FileSummary::get_sections_for_reflect,
                    FileSummary::mut_sections_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FileSummary>(
                    "FileSummary",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FileSummary {
    fn clear(&mut self) {
        self.clear_ondiskVersion();
        self.clear_layoutVersion();
        self.clear_codec();
        self.clear_sections();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FileSummary {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FileSummary {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FileSummary_Section {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    length: ::std::option::Option<u64>,
    offset: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FileSummary_Section {}

impl FileSummary_Section {
    pub fn new() -> FileSummary_Section {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FileSummary_Section {
        static mut instance: ::protobuf::lazy::Lazy<FileSummary_Section> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FileSummary_Section,
        };
        unsafe {
            instance.get(FileSummary_Section::new)
        }
    }

    // optional string name = 1;

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

    // optional uint64 length = 2;

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

    // optional uint64 offset = 3;

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
}

impl ::protobuf::Message for FileSummary_Section {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.length = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.offset = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.length {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.offset {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.length {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.offset {
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

impl ::protobuf::MessageStatic for FileSummary_Section {
    fn new() -> FileSummary_Section {
        FileSummary_Section::new()
    }

    fn descriptor_static(_: ::std::option::Option<FileSummary_Section>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    FileSummary_Section::get_name_for_reflect,
                    FileSummary_Section::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "length",
                    FileSummary_Section::get_length_for_reflect,
                    FileSummary_Section::mut_length_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "offset",
                    FileSummary_Section::get_offset_for_reflect,
                    FileSummary_Section::mut_offset_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FileSummary_Section>(
                    "FileSummary_Section",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FileSummary_Section {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_length();
        self.clear_offset();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FileSummary_Section {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FileSummary_Section {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NameSystemSection {
    // message fields
    namespaceId: ::std::option::Option<u32>,
    genstampV1: ::std::option::Option<u64>,
    genstampV2: ::std::option::Option<u64>,
    genstampV1Limit: ::std::option::Option<u64>,
    lastAllocatedBlockId: ::std::option::Option<u64>,
    transactionId: ::std::option::Option<u64>,
    rollingUpgradeStartTime: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NameSystemSection {}

impl NameSystemSection {
    pub fn new() -> NameSystemSection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NameSystemSection {
        static mut instance: ::protobuf::lazy::Lazy<NameSystemSection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NameSystemSection,
        };
        unsafe {
            instance.get(NameSystemSection::new)
        }
    }

    // optional uint32 namespaceId = 1;

    pub fn clear_namespaceId(&mut self) {
        self.namespaceId = ::std::option::Option::None;
    }

    pub fn has_namespaceId(&self) -> bool {
        self.namespaceId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_namespaceId(&mut self, v: u32) {
        self.namespaceId = ::std::option::Option::Some(v);
    }

    pub fn get_namespaceId(&self) -> u32 {
        self.namespaceId.unwrap_or(0)
    }

    fn get_namespaceId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.namespaceId
    }

    fn mut_namespaceId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.namespaceId
    }

    // optional uint64 genstampV1 = 2;

    pub fn clear_genstampV1(&mut self) {
        self.genstampV1 = ::std::option::Option::None;
    }

    pub fn has_genstampV1(&self) -> bool {
        self.genstampV1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_genstampV1(&mut self, v: u64) {
        self.genstampV1 = ::std::option::Option::Some(v);
    }

    pub fn get_genstampV1(&self) -> u64 {
        self.genstampV1.unwrap_or(0)
    }

    fn get_genstampV1_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.genstampV1
    }

    fn mut_genstampV1_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.genstampV1
    }

    // optional uint64 genstampV2 = 3;

    pub fn clear_genstampV2(&mut self) {
        self.genstampV2 = ::std::option::Option::None;
    }

    pub fn has_genstampV2(&self) -> bool {
        self.genstampV2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_genstampV2(&mut self, v: u64) {
        self.genstampV2 = ::std::option::Option::Some(v);
    }

    pub fn get_genstampV2(&self) -> u64 {
        self.genstampV2.unwrap_or(0)
    }

    fn get_genstampV2_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.genstampV2
    }

    fn mut_genstampV2_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.genstampV2
    }

    // optional uint64 genstampV1Limit = 4;

    pub fn clear_genstampV1Limit(&mut self) {
        self.genstampV1Limit = ::std::option::Option::None;
    }

    pub fn has_genstampV1Limit(&self) -> bool {
        self.genstampV1Limit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_genstampV1Limit(&mut self, v: u64) {
        self.genstampV1Limit = ::std::option::Option::Some(v);
    }

    pub fn get_genstampV1Limit(&self) -> u64 {
        self.genstampV1Limit.unwrap_or(0)
    }

    fn get_genstampV1Limit_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.genstampV1Limit
    }

    fn mut_genstampV1Limit_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.genstampV1Limit
    }

    // optional uint64 lastAllocatedBlockId = 5;

    pub fn clear_lastAllocatedBlockId(&mut self) {
        self.lastAllocatedBlockId = ::std::option::Option::None;
    }

    pub fn has_lastAllocatedBlockId(&self) -> bool {
        self.lastAllocatedBlockId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastAllocatedBlockId(&mut self, v: u64) {
        self.lastAllocatedBlockId = ::std::option::Option::Some(v);
    }

    pub fn get_lastAllocatedBlockId(&self) -> u64 {
        self.lastAllocatedBlockId.unwrap_or(0)
    }

    fn get_lastAllocatedBlockId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lastAllocatedBlockId
    }

    fn mut_lastAllocatedBlockId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lastAllocatedBlockId
    }

    // optional uint64 transactionId = 6;

    pub fn clear_transactionId(&mut self) {
        self.transactionId = ::std::option::Option::None;
    }

    pub fn has_transactionId(&self) -> bool {
        self.transactionId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transactionId(&mut self, v: u64) {
        self.transactionId = ::std::option::Option::Some(v);
    }

    pub fn get_transactionId(&self) -> u64 {
        self.transactionId.unwrap_or(0)
    }

    fn get_transactionId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.transactionId
    }

    fn mut_transactionId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.transactionId
    }

    // optional uint64 rollingUpgradeStartTime = 7;

    pub fn clear_rollingUpgradeStartTime(&mut self) {
        self.rollingUpgradeStartTime = ::std::option::Option::None;
    }

    pub fn has_rollingUpgradeStartTime(&self) -> bool {
        self.rollingUpgradeStartTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rollingUpgradeStartTime(&mut self, v: u64) {
        self.rollingUpgradeStartTime = ::std::option::Option::Some(v);
    }

    pub fn get_rollingUpgradeStartTime(&self) -> u64 {
        self.rollingUpgradeStartTime.unwrap_or(0)
    }

    fn get_rollingUpgradeStartTime_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.rollingUpgradeStartTime
    }

    fn mut_rollingUpgradeStartTime_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.rollingUpgradeStartTime
    }
}

impl ::protobuf::Message for NameSystemSection {
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
                    let tmp = is.read_uint32()?;
                    self.namespaceId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.genstampV1 = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.genstampV2 = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.genstampV1Limit = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.lastAllocatedBlockId = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.transactionId = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.rollingUpgradeStartTime = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.namespaceId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.genstampV1 {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.genstampV2 {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.genstampV1Limit {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.lastAllocatedBlockId {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.transactionId {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.rollingUpgradeStartTime {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.namespaceId {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.genstampV1 {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.genstampV2 {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.genstampV1Limit {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.lastAllocatedBlockId {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.transactionId {
            os.write_uint64(6, v)?;
        }
        if let Some(v) = self.rollingUpgradeStartTime {
            os.write_uint64(7, v)?;
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

impl ::protobuf::MessageStatic for NameSystemSection {
    fn new() -> NameSystemSection {
        NameSystemSection::new()
    }

    fn descriptor_static(_: ::std::option::Option<NameSystemSection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "namespaceId",
                    NameSystemSection::get_namespaceId_for_reflect,
                    NameSystemSection::mut_namespaceId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "genstampV1",
                    NameSystemSection::get_genstampV1_for_reflect,
                    NameSystemSection::mut_genstampV1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "genstampV2",
                    NameSystemSection::get_genstampV2_for_reflect,
                    NameSystemSection::mut_genstampV2_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "genstampV1Limit",
                    NameSystemSection::get_genstampV1Limit_for_reflect,
                    NameSystemSection::mut_genstampV1Limit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lastAllocatedBlockId",
                    NameSystemSection::get_lastAllocatedBlockId_for_reflect,
                    NameSystemSection::mut_lastAllocatedBlockId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "transactionId",
                    NameSystemSection::get_transactionId_for_reflect,
                    NameSystemSection::mut_transactionId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "rollingUpgradeStartTime",
                    NameSystemSection::get_rollingUpgradeStartTime_for_reflect,
                    NameSystemSection::mut_rollingUpgradeStartTime_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NameSystemSection>(
                    "NameSystemSection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NameSystemSection {
    fn clear(&mut self) {
        self.clear_namespaceId();
        self.clear_genstampV1();
        self.clear_genstampV2();
        self.clear_genstampV1Limit();
        self.clear_lastAllocatedBlockId();
        self.clear_transactionId();
        self.clear_rollingUpgradeStartTime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NameSystemSection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NameSystemSection {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct INodeSection {
    // message fields
    lastInodeId: ::std::option::Option<u64>,
    numInodes: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for INodeSection {}

impl INodeSection {
    pub fn new() -> INodeSection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static INodeSection {
        static mut instance: ::protobuf::lazy::Lazy<INodeSection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const INodeSection,
        };
        unsafe {
            instance.get(INodeSection::new)
        }
    }

    // optional uint64 lastInodeId = 1;

    pub fn clear_lastInodeId(&mut self) {
        self.lastInodeId = ::std::option::Option::None;
    }

    pub fn has_lastInodeId(&self) -> bool {
        self.lastInodeId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastInodeId(&mut self, v: u64) {
        self.lastInodeId = ::std::option::Option::Some(v);
    }

    pub fn get_lastInodeId(&self) -> u64 {
        self.lastInodeId.unwrap_or(0)
    }

    fn get_lastInodeId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lastInodeId
    }

    fn mut_lastInodeId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lastInodeId
    }

    // optional uint64 numInodes = 2;

    pub fn clear_numInodes(&mut self) {
        self.numInodes = ::std::option::Option::None;
    }

    pub fn has_numInodes(&self) -> bool {
        self.numInodes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numInodes(&mut self, v: u64) {
        self.numInodes = ::std::option::Option::Some(v);
    }

    pub fn get_numInodes(&self) -> u64 {
        self.numInodes.unwrap_or(0)
    }

    fn get_numInodes_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.numInodes
    }

    fn mut_numInodes_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.numInodes
    }
}

impl ::protobuf::Message for INodeSection {
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
                    let tmp = is.read_uint64()?;
                    self.lastInodeId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.numInodes = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.lastInodeId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numInodes {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.lastInodeId {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.numInodes {
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

impl ::protobuf::MessageStatic for INodeSection {
    fn new() -> INodeSection {
        INodeSection::new()
    }

    fn descriptor_static(_: ::std::option::Option<INodeSection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lastInodeId",
                    INodeSection::get_lastInodeId_for_reflect,
                    INodeSection::mut_lastInodeId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "numInodes",
                    INodeSection::get_numInodes_for_reflect,
                    INodeSection::mut_numInodes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<INodeSection>(
                    "INodeSection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for INodeSection {
    fn clear(&mut self) {
        self.clear_lastInodeId();
        self.clear_numInodes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for INodeSection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for INodeSection {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct INodeSection_FileUnderConstructionFeature {
    // message fields
    clientName: ::protobuf::SingularField<::std::string::String>,
    clientMachine: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for INodeSection_FileUnderConstructionFeature {}

impl INodeSection_FileUnderConstructionFeature {
    pub fn new() -> INodeSection_FileUnderConstructionFeature {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static INodeSection_FileUnderConstructionFeature {
        static mut instance: ::protobuf::lazy::Lazy<INodeSection_FileUnderConstructionFeature> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const INodeSection_FileUnderConstructionFeature,
        };
        unsafe {
            instance.get(INodeSection_FileUnderConstructionFeature::new)
        }
    }

    // optional string clientName = 1;

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

    // optional string clientMachine = 2;

    pub fn clear_clientMachine(&mut self) {
        self.clientMachine.clear();
    }

    pub fn has_clientMachine(&self) -> bool {
        self.clientMachine.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientMachine(&mut self, v: ::std::string::String) {
        self.clientMachine = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientMachine(&mut self) -> &mut ::std::string::String {
        if self.clientMachine.is_none() {
            self.clientMachine.set_default();
        }
        self.clientMachine.as_mut().unwrap()
    }

    // Take field
    pub fn take_clientMachine(&mut self) -> ::std::string::String {
        self.clientMachine.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_clientMachine(&self) -> &str {
        match self.clientMachine.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_clientMachine_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.clientMachine
    }

    fn mut_clientMachine_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.clientMachine
    }
}

impl ::protobuf::Message for INodeSection_FileUnderConstructionFeature {
    fn is_initialized(&self) -> bool {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.clientMachine)?;
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
        if let Some(ref v) = self.clientMachine.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.clientName.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.clientMachine.as_ref() {
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

impl ::protobuf::MessageStatic for INodeSection_FileUnderConstructionFeature {
    fn new() -> INodeSection_FileUnderConstructionFeature {
        INodeSection_FileUnderConstructionFeature::new()
    }

    fn descriptor_static(_: ::std::option::Option<INodeSection_FileUnderConstructionFeature>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "clientName",
                    INodeSection_FileUnderConstructionFeature::get_clientName_for_reflect,
                    INodeSection_FileUnderConstructionFeature::mut_clientName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "clientMachine",
                    INodeSection_FileUnderConstructionFeature::get_clientMachine_for_reflect,
                    INodeSection_FileUnderConstructionFeature::mut_clientMachine_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<INodeSection_FileUnderConstructionFeature>(
                    "INodeSection_FileUnderConstructionFeature",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for INodeSection_FileUnderConstructionFeature {
    fn clear(&mut self) {
        self.clear_clientName();
        self.clear_clientMachine();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for INodeSection_FileUnderConstructionFeature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for INodeSection_FileUnderConstructionFeature {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct INodeSection_AclFeatureProto {
    // message fields
    entries: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for INodeSection_AclFeatureProto {}

impl INodeSection_AclFeatureProto {
    pub fn new() -> INodeSection_AclFeatureProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static INodeSection_AclFeatureProto {
        static mut instance: ::protobuf::lazy::Lazy<INodeSection_AclFeatureProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const INodeSection_AclFeatureProto,
        };
        unsafe {
            instance.get(INodeSection_AclFeatureProto::new)
        }
    }

    // repeated fixed32 entries = 2;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::std::vec::Vec<u32>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.entries, ::std::vec::Vec::new())
    }

    pub fn get_entries(&self) -> &[u32] {
        &self.entries
    }

    fn get_entries_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.entries
    }

    fn mut_entries_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.entries
    }
}

impl ::protobuf::Message for INodeSection_AclFeatureProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.entries)?;
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
        if !self.entries.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.entries.len() as u32) + (self.entries.len() * 4) as u32;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.entries.is_empty() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.entries.len() * 4) as u32)?;
            for v in &self.entries {
                os.write_fixed32_no_tag(*v)?;
            };
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

impl ::protobuf::MessageStatic for INodeSection_AclFeatureProto {
    fn new() -> INodeSection_AclFeatureProto {
        INodeSection_AclFeatureProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<INodeSection_AclFeatureProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "entries",
                    INodeSection_AclFeatureProto::get_entries_for_reflect,
                    INodeSection_AclFeatureProto::mut_entries_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<INodeSection_AclFeatureProto>(
                    "INodeSection_AclFeatureProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for INodeSection_AclFeatureProto {
    fn clear(&mut self) {
        self.clear_entries();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for INodeSection_AclFeatureProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for INodeSection_AclFeatureProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct INodeSection_XAttrCompactProto {
    // message fields
    name: ::std::option::Option<u32>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for INodeSection_XAttrCompactProto {}

impl INodeSection_XAttrCompactProto {
    pub fn new() -> INodeSection_XAttrCompactProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static INodeSection_XAttrCompactProto {
        static mut instance: ::protobuf::lazy::Lazy<INodeSection_XAttrCompactProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const INodeSection_XAttrCompactProto,
        };
        unsafe {
            instance.get(INodeSection_XAttrCompactProto::new)
        }
    }

    // required fixed32 name = 1;

    pub fn clear_name(&mut self) {
        self.name = ::std::option::Option::None;
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: u32) {
        self.name = ::std::option::Option::Some(v);
    }

    pub fn get_name(&self) -> u32 {
        self.name.unwrap_or(0)
    }

    fn get_name_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.name
    }

    // optional bytes value = 2;

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

impl ::protobuf::Message for INodeSection_XAttrCompactProto {
    fn is_initialized(&self) -> bool {
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
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.name = ::std::option::Option::Some(tmp);
                },
                2 => {
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
        if let Some(v) = self.name {
            my_size += 5;
        }
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name {
            os.write_fixed32(1, v)?;
        }
        if let Some(ref v) = self.value.as_ref() {
            os.write_bytes(2, &v)?;
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

impl ::protobuf::MessageStatic for INodeSection_XAttrCompactProto {
    fn new() -> INodeSection_XAttrCompactProto {
        INodeSection_XAttrCompactProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<INodeSection_XAttrCompactProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "name",
                    INodeSection_XAttrCompactProto::get_name_for_reflect,
                    INodeSection_XAttrCompactProto::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    INodeSection_XAttrCompactProto::get_value_for_reflect,
                    INodeSection_XAttrCompactProto::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<INodeSection_XAttrCompactProto>(
                    "INodeSection_XAttrCompactProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for INodeSection_XAttrCompactProto {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for INodeSection_XAttrCompactProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for INodeSection_XAttrCompactProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct INodeSection_XAttrFeatureProto {
    // message fields
    xAttrs: ::protobuf::RepeatedField<INodeSection_XAttrCompactProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for INodeSection_XAttrFeatureProto {}

impl INodeSection_XAttrFeatureProto {
    pub fn new() -> INodeSection_XAttrFeatureProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static INodeSection_XAttrFeatureProto {
        static mut instance: ::protobuf::lazy::Lazy<INodeSection_XAttrFeatureProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const INodeSection_XAttrFeatureProto,
        };
        unsafe {
            instance.get(INodeSection_XAttrFeatureProto::new)
        }
    }

    // repeated .hadoop.hdfs.fsimage.INodeSection.XAttrCompactProto xAttrs = 1;

    pub fn clear_xAttrs(&mut self) {
        self.xAttrs.clear();
    }

    // Param is passed by value, moved
    pub fn set_xAttrs(&mut self, v: ::protobuf::RepeatedField<INodeSection_XAttrCompactProto>) {
        self.xAttrs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_xAttrs(&mut self) -> &mut ::protobuf::RepeatedField<INodeSection_XAttrCompactProto> {
        &mut self.xAttrs
    }

    // Take field
    pub fn take_xAttrs(&mut self) -> ::protobuf::RepeatedField<INodeSection_XAttrCompactProto> {
        ::std::mem::replace(&mut self.xAttrs, ::protobuf::RepeatedField::new())
    }

    pub fn get_xAttrs(&self) -> &[INodeSection_XAttrCompactProto] {
        &self.xAttrs
    }

    fn get_xAttrs_for_reflect(&self) -> &::protobuf::RepeatedField<INodeSection_XAttrCompactProto> {
        &self.xAttrs
    }

    fn mut_xAttrs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<INodeSection_XAttrCompactProto> {
        &mut self.xAttrs
    }
}

impl ::protobuf::Message for INodeSection_XAttrFeatureProto {
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

impl ::protobuf::MessageStatic for INodeSection_XAttrFeatureProto {
    fn new() -> INodeSection_XAttrFeatureProto {
        INodeSection_XAttrFeatureProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<INodeSection_XAttrFeatureProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<INodeSection_XAttrCompactProto>>(
                    "xAttrs",
                    INodeSection_XAttrFeatureProto::get_xAttrs_for_reflect,
                    INodeSection_XAttrFeatureProto::mut_xAttrs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<INodeSection_XAttrFeatureProto>(
                    "INodeSection_XAttrFeatureProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for INodeSection_XAttrFeatureProto {
    fn clear(&mut self) {
        self.clear_xAttrs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for INodeSection_XAttrFeatureProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for INodeSection_XAttrFeatureProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct INodeSection_INodeFile {
    // message fields
    replication: ::std::option::Option<u32>,
    modificationTime: ::std::option::Option<u64>,
    accessTime: ::std::option::Option<u64>,
    preferredBlockSize: ::std::option::Option<u64>,
    permission: ::std::option::Option<u64>,
    blocks: ::protobuf::RepeatedField<super::hdfs::BlockProto>,
    fileUC: ::protobuf::SingularPtrField<INodeSection_FileUnderConstructionFeature>,
    acl: ::protobuf::SingularPtrField<INodeSection_AclFeatureProto>,
    xAttrs: ::protobuf::SingularPtrField<INodeSection_XAttrFeatureProto>,
    storagePolicyID: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for INodeSection_INodeFile {}

impl INodeSection_INodeFile {
    pub fn new() -> INodeSection_INodeFile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static INodeSection_INodeFile {
        static mut instance: ::protobuf::lazy::Lazy<INodeSection_INodeFile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const INodeSection_INodeFile,
        };
        unsafe {
            instance.get(INodeSection_INodeFile::new)
        }
    }

    // optional uint32 replication = 1;

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

    // optional uint64 modificationTime = 2;

    pub fn clear_modificationTime(&mut self) {
        self.modificationTime = ::std::option::Option::None;
    }

    pub fn has_modificationTime(&self) -> bool {
        self.modificationTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modificationTime(&mut self, v: u64) {
        self.modificationTime = ::std::option::Option::Some(v);
    }

    pub fn get_modificationTime(&self) -> u64 {
        self.modificationTime.unwrap_or(0)
    }

    fn get_modificationTime_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.modificationTime
    }

    fn mut_modificationTime_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.modificationTime
    }

    // optional uint64 accessTime = 3;

    pub fn clear_accessTime(&mut self) {
        self.accessTime = ::std::option::Option::None;
    }

    pub fn has_accessTime(&self) -> bool {
        self.accessTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accessTime(&mut self, v: u64) {
        self.accessTime = ::std::option::Option::Some(v);
    }

    pub fn get_accessTime(&self) -> u64 {
        self.accessTime.unwrap_or(0)
    }

    fn get_accessTime_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.accessTime
    }

    fn mut_accessTime_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.accessTime
    }

    // optional uint64 preferredBlockSize = 4;

    pub fn clear_preferredBlockSize(&mut self) {
        self.preferredBlockSize = ::std::option::Option::None;
    }

    pub fn has_preferredBlockSize(&self) -> bool {
        self.preferredBlockSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_preferredBlockSize(&mut self, v: u64) {
        self.preferredBlockSize = ::std::option::Option::Some(v);
    }

    pub fn get_preferredBlockSize(&self) -> u64 {
        self.preferredBlockSize.unwrap_or(0)
    }

    fn get_preferredBlockSize_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.preferredBlockSize
    }

    fn mut_preferredBlockSize_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.preferredBlockSize
    }

    // optional fixed64 permission = 5;

    pub fn clear_permission(&mut self) {
        self.permission = ::std::option::Option::None;
    }

    pub fn has_permission(&self) -> bool {
        self.permission.is_some()
    }

    // Param is passed by value, moved
    pub fn set_permission(&mut self, v: u64) {
        self.permission = ::std::option::Option::Some(v);
    }

    pub fn get_permission(&self) -> u64 {
        self.permission.unwrap_or(0)
    }

    fn get_permission_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.permission
    }

    fn mut_permission_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.permission
    }

    // repeated .hadoop.hdfs.BlockProto blocks = 6;

    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    // Param is passed by value, moved
    pub fn set_blocks(&mut self, v: ::protobuf::RepeatedField<super::hdfs::BlockProto>) {
        self.blocks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blocks(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::BlockProto> {
        &mut self.blocks
    }

    // Take field
    pub fn take_blocks(&mut self) -> ::protobuf::RepeatedField<super::hdfs::BlockProto> {
        ::std::mem::replace(&mut self.blocks, ::protobuf::RepeatedField::new())
    }

    pub fn get_blocks(&self) -> &[super::hdfs::BlockProto] {
        &self.blocks
    }

    fn get_blocks_for_reflect(&self) -> &::protobuf::RepeatedField<super::hdfs::BlockProto> {
        &self.blocks
    }

    fn mut_blocks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::BlockProto> {
        &mut self.blocks
    }

    // optional .hadoop.hdfs.fsimage.INodeSection.FileUnderConstructionFeature fileUC = 7;

    pub fn clear_fileUC(&mut self) {
        self.fileUC.clear();
    }

    pub fn has_fileUC(&self) -> bool {
        self.fileUC.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileUC(&mut self, v: INodeSection_FileUnderConstructionFeature) {
        self.fileUC = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fileUC(&mut self) -> &mut INodeSection_FileUnderConstructionFeature {
        if self.fileUC.is_none() {
            self.fileUC.set_default();
        }
        self.fileUC.as_mut().unwrap()
    }

    // Take field
    pub fn take_fileUC(&mut self) -> INodeSection_FileUnderConstructionFeature {
        self.fileUC.take().unwrap_or_else(|| INodeSection_FileUnderConstructionFeature::new())
    }

    pub fn get_fileUC(&self) -> &INodeSection_FileUnderConstructionFeature {
        self.fileUC.as_ref().unwrap_or_else(|| INodeSection_FileUnderConstructionFeature::default_instance())
    }

    fn get_fileUC_for_reflect(&self) -> &::protobuf::SingularPtrField<INodeSection_FileUnderConstructionFeature> {
        &self.fileUC
    }

    fn mut_fileUC_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<INodeSection_FileUnderConstructionFeature> {
        &mut self.fileUC
    }

    // optional .hadoop.hdfs.fsimage.INodeSection.AclFeatureProto acl = 8;

    pub fn clear_acl(&mut self) {
        self.acl.clear();
    }

    pub fn has_acl(&self) -> bool {
        self.acl.is_some()
    }

    // Param is passed by value, moved
    pub fn set_acl(&mut self, v: INodeSection_AclFeatureProto) {
        self.acl = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_acl(&mut self) -> &mut INodeSection_AclFeatureProto {
        if self.acl.is_none() {
            self.acl.set_default();
        }
        self.acl.as_mut().unwrap()
    }

    // Take field
    pub fn take_acl(&mut self) -> INodeSection_AclFeatureProto {
        self.acl.take().unwrap_or_else(|| INodeSection_AclFeatureProto::new())
    }

    pub fn get_acl(&self) -> &INodeSection_AclFeatureProto {
        self.acl.as_ref().unwrap_or_else(|| INodeSection_AclFeatureProto::default_instance())
    }

    fn get_acl_for_reflect(&self) -> &::protobuf::SingularPtrField<INodeSection_AclFeatureProto> {
        &self.acl
    }

    fn mut_acl_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<INodeSection_AclFeatureProto> {
        &mut self.acl
    }

    // optional .hadoop.hdfs.fsimage.INodeSection.XAttrFeatureProto xAttrs = 9;

    pub fn clear_xAttrs(&mut self) {
        self.xAttrs.clear();
    }

    pub fn has_xAttrs(&self) -> bool {
        self.xAttrs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xAttrs(&mut self, v: INodeSection_XAttrFeatureProto) {
        self.xAttrs = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_xAttrs(&mut self) -> &mut INodeSection_XAttrFeatureProto {
        if self.xAttrs.is_none() {
            self.xAttrs.set_default();
        }
        self.xAttrs.as_mut().unwrap()
    }

    // Take field
    pub fn take_xAttrs(&mut self) -> INodeSection_XAttrFeatureProto {
        self.xAttrs.take().unwrap_or_else(|| INodeSection_XAttrFeatureProto::new())
    }

    pub fn get_xAttrs(&self) -> &INodeSection_XAttrFeatureProto {
        self.xAttrs.as_ref().unwrap_or_else(|| INodeSection_XAttrFeatureProto::default_instance())
    }

    fn get_xAttrs_for_reflect(&self) -> &::protobuf::SingularPtrField<INodeSection_XAttrFeatureProto> {
        &self.xAttrs
    }

    fn mut_xAttrs_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<INodeSection_XAttrFeatureProto> {
        &mut self.xAttrs
    }

    // optional uint32 storagePolicyID = 10;

    pub fn clear_storagePolicyID(&mut self) {
        self.storagePolicyID = ::std::option::Option::None;
    }

    pub fn has_storagePolicyID(&self) -> bool {
        self.storagePolicyID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storagePolicyID(&mut self, v: u32) {
        self.storagePolicyID = ::std::option::Option::Some(v);
    }

    pub fn get_storagePolicyID(&self) -> u32 {
        self.storagePolicyID.unwrap_or(0)
    }

    fn get_storagePolicyID_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.storagePolicyID
    }

    fn mut_storagePolicyID_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.storagePolicyID
    }
}

impl ::protobuf::Message for INodeSection_INodeFile {
    fn is_initialized(&self) -> bool {
        for v in &self.blocks {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.fileUC {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.acl {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.replication = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.modificationTime = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.accessTime = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.preferredBlockSize = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.permission = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.blocks)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fileUC)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.acl)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.xAttrs)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.storagePolicyID = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.replication {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.modificationTime {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.accessTime {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.preferredBlockSize {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.permission {
            my_size += 9;
        }
        for value in &self.blocks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.fileUC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.acl.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.xAttrs.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.storagePolicyID {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.replication {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.modificationTime {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.accessTime {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.preferredBlockSize {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.permission {
            os.write_fixed64(5, v)?;
        }
        for v in &self.blocks {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.fileUC.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.acl.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.xAttrs.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.storagePolicyID {
            os.write_uint32(10, v)?;
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

impl ::protobuf::MessageStatic for INodeSection_INodeFile {
    fn new() -> INodeSection_INodeFile {
        INodeSection_INodeFile::new()
    }

    fn descriptor_static(_: ::std::option::Option<INodeSection_INodeFile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "replication",
                    INodeSection_INodeFile::get_replication_for_reflect,
                    INodeSection_INodeFile::mut_replication_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "modificationTime",
                    INodeSection_INodeFile::get_modificationTime_for_reflect,
                    INodeSection_INodeFile::mut_modificationTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "accessTime",
                    INodeSection_INodeFile::get_accessTime_for_reflect,
                    INodeSection_INodeFile::mut_accessTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "preferredBlockSize",
                    INodeSection_INodeFile::get_preferredBlockSize_for_reflect,
                    INodeSection_INodeFile::mut_preferredBlockSize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "permission",
                    INodeSection_INodeFile::get_permission_for_reflect,
                    INodeSection_INodeFile::mut_permission_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::BlockProto>>(
                    "blocks",
                    INodeSection_INodeFile::get_blocks_for_reflect,
                    INodeSection_INodeFile::mut_blocks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<INodeSection_FileUnderConstructionFeature>>(
                    "fileUC",
                    INodeSection_INodeFile::get_fileUC_for_reflect,
                    INodeSection_INodeFile::mut_fileUC_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<INodeSection_AclFeatureProto>>(
                    "acl",
                    INodeSection_INodeFile::get_acl_for_reflect,
                    INodeSection_INodeFile::mut_acl_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<INodeSection_XAttrFeatureProto>>(
                    "xAttrs",
                    INodeSection_INodeFile::get_xAttrs_for_reflect,
                    INodeSection_INodeFile::mut_xAttrs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "storagePolicyID",
                    INodeSection_INodeFile::get_storagePolicyID_for_reflect,
                    INodeSection_INodeFile::mut_storagePolicyID_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<INodeSection_INodeFile>(
                    "INodeSection_INodeFile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for INodeSection_INodeFile {
    fn clear(&mut self) {
        self.clear_replication();
        self.clear_modificationTime();
        self.clear_accessTime();
        self.clear_preferredBlockSize();
        self.clear_permission();
        self.clear_blocks();
        self.clear_fileUC();
        self.clear_acl();
        self.clear_xAttrs();
        self.clear_storagePolicyID();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for INodeSection_INodeFile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for INodeSection_INodeFile {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct INodeSection_INodeDirectory {
    // message fields
    modificationTime: ::std::option::Option<u64>,
    nsQuota: ::std::option::Option<u64>,
    dsQuota: ::std::option::Option<u64>,
    permission: ::std::option::Option<u64>,
    acl: ::protobuf::SingularPtrField<INodeSection_AclFeatureProto>,
    xAttrs: ::protobuf::SingularPtrField<INodeSection_XAttrFeatureProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for INodeSection_INodeDirectory {}

impl INodeSection_INodeDirectory {
    pub fn new() -> INodeSection_INodeDirectory {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static INodeSection_INodeDirectory {
        static mut instance: ::protobuf::lazy::Lazy<INodeSection_INodeDirectory> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const INodeSection_INodeDirectory,
        };
        unsafe {
            instance.get(INodeSection_INodeDirectory::new)
        }
    }

    // optional uint64 modificationTime = 1;

    pub fn clear_modificationTime(&mut self) {
        self.modificationTime = ::std::option::Option::None;
    }

    pub fn has_modificationTime(&self) -> bool {
        self.modificationTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modificationTime(&mut self, v: u64) {
        self.modificationTime = ::std::option::Option::Some(v);
    }

    pub fn get_modificationTime(&self) -> u64 {
        self.modificationTime.unwrap_or(0)
    }

    fn get_modificationTime_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.modificationTime
    }

    fn mut_modificationTime_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.modificationTime
    }

    // optional uint64 nsQuota = 2;

    pub fn clear_nsQuota(&mut self) {
        self.nsQuota = ::std::option::Option::None;
    }

    pub fn has_nsQuota(&self) -> bool {
        self.nsQuota.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nsQuota(&mut self, v: u64) {
        self.nsQuota = ::std::option::Option::Some(v);
    }

    pub fn get_nsQuota(&self) -> u64 {
        self.nsQuota.unwrap_or(0)
    }

    fn get_nsQuota_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.nsQuota
    }

    fn mut_nsQuota_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.nsQuota
    }

    // optional uint64 dsQuota = 3;

    pub fn clear_dsQuota(&mut self) {
        self.dsQuota = ::std::option::Option::None;
    }

    pub fn has_dsQuota(&self) -> bool {
        self.dsQuota.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dsQuota(&mut self, v: u64) {
        self.dsQuota = ::std::option::Option::Some(v);
    }

    pub fn get_dsQuota(&self) -> u64 {
        self.dsQuota.unwrap_or(0)
    }

    fn get_dsQuota_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.dsQuota
    }

    fn mut_dsQuota_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.dsQuota
    }

    // optional fixed64 permission = 4;

    pub fn clear_permission(&mut self) {
        self.permission = ::std::option::Option::None;
    }

    pub fn has_permission(&self) -> bool {
        self.permission.is_some()
    }

    // Param is passed by value, moved
    pub fn set_permission(&mut self, v: u64) {
        self.permission = ::std::option::Option::Some(v);
    }

    pub fn get_permission(&self) -> u64 {
        self.permission.unwrap_or(0)
    }

    fn get_permission_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.permission
    }

    fn mut_permission_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.permission
    }

    // optional .hadoop.hdfs.fsimage.INodeSection.AclFeatureProto acl = 5;

    pub fn clear_acl(&mut self) {
        self.acl.clear();
    }

    pub fn has_acl(&self) -> bool {
        self.acl.is_some()
    }

    // Param is passed by value, moved
    pub fn set_acl(&mut self, v: INodeSection_AclFeatureProto) {
        self.acl = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_acl(&mut self) -> &mut INodeSection_AclFeatureProto {
        if self.acl.is_none() {
            self.acl.set_default();
        }
        self.acl.as_mut().unwrap()
    }

    // Take field
    pub fn take_acl(&mut self) -> INodeSection_AclFeatureProto {
        self.acl.take().unwrap_or_else(|| INodeSection_AclFeatureProto::new())
    }

    pub fn get_acl(&self) -> &INodeSection_AclFeatureProto {
        self.acl.as_ref().unwrap_or_else(|| INodeSection_AclFeatureProto::default_instance())
    }

    fn get_acl_for_reflect(&self) -> &::protobuf::SingularPtrField<INodeSection_AclFeatureProto> {
        &self.acl
    }

    fn mut_acl_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<INodeSection_AclFeatureProto> {
        &mut self.acl
    }

    // optional .hadoop.hdfs.fsimage.INodeSection.XAttrFeatureProto xAttrs = 6;

    pub fn clear_xAttrs(&mut self) {
        self.xAttrs.clear();
    }

    pub fn has_xAttrs(&self) -> bool {
        self.xAttrs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xAttrs(&mut self, v: INodeSection_XAttrFeatureProto) {
        self.xAttrs = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_xAttrs(&mut self) -> &mut INodeSection_XAttrFeatureProto {
        if self.xAttrs.is_none() {
            self.xAttrs.set_default();
        }
        self.xAttrs.as_mut().unwrap()
    }

    // Take field
    pub fn take_xAttrs(&mut self) -> INodeSection_XAttrFeatureProto {
        self.xAttrs.take().unwrap_or_else(|| INodeSection_XAttrFeatureProto::new())
    }

    pub fn get_xAttrs(&self) -> &INodeSection_XAttrFeatureProto {
        self.xAttrs.as_ref().unwrap_or_else(|| INodeSection_XAttrFeatureProto::default_instance())
    }

    fn get_xAttrs_for_reflect(&self) -> &::protobuf::SingularPtrField<INodeSection_XAttrFeatureProto> {
        &self.xAttrs
    }

    fn mut_xAttrs_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<INodeSection_XAttrFeatureProto> {
        &mut self.xAttrs
    }
}

impl ::protobuf::Message for INodeSection_INodeDirectory {
    fn is_initialized(&self) -> bool {
        for v in &self.acl {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.modificationTime = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.nsQuota = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.dsQuota = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.permission = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.acl)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.xAttrs)?;
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
        if let Some(v) = self.modificationTime {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.nsQuota {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.dsQuota {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.permission {
            my_size += 9;
        }
        if let Some(ref v) = self.acl.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.xAttrs.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.modificationTime {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.nsQuota {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.dsQuota {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.permission {
            os.write_fixed64(4, v)?;
        }
        if let Some(ref v) = self.acl.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.xAttrs.as_ref() {
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

impl ::protobuf::MessageStatic for INodeSection_INodeDirectory {
    fn new() -> INodeSection_INodeDirectory {
        INodeSection_INodeDirectory::new()
    }

    fn descriptor_static(_: ::std::option::Option<INodeSection_INodeDirectory>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "modificationTime",
                    INodeSection_INodeDirectory::get_modificationTime_for_reflect,
                    INodeSection_INodeDirectory::mut_modificationTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "nsQuota",
                    INodeSection_INodeDirectory::get_nsQuota_for_reflect,
                    INodeSection_INodeDirectory::mut_nsQuota_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "dsQuota",
                    INodeSection_INodeDirectory::get_dsQuota_for_reflect,
                    INodeSection_INodeDirectory::mut_dsQuota_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "permission",
                    INodeSection_INodeDirectory::get_permission_for_reflect,
                    INodeSection_INodeDirectory::mut_permission_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<INodeSection_AclFeatureProto>>(
                    "acl",
                    INodeSection_INodeDirectory::get_acl_for_reflect,
                    INodeSection_INodeDirectory::mut_acl_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<INodeSection_XAttrFeatureProto>>(
                    "xAttrs",
                    INodeSection_INodeDirectory::get_xAttrs_for_reflect,
                    INodeSection_INodeDirectory::mut_xAttrs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<INodeSection_INodeDirectory>(
                    "INodeSection_INodeDirectory",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for INodeSection_INodeDirectory {
    fn clear(&mut self) {
        self.clear_modificationTime();
        self.clear_nsQuota();
        self.clear_dsQuota();
        self.clear_permission();
        self.clear_acl();
        self.clear_xAttrs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for INodeSection_INodeDirectory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for INodeSection_INodeDirectory {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct INodeSection_INodeSymlink {
    // message fields
    permission: ::std::option::Option<u64>,
    target: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    modificationTime: ::std::option::Option<u64>,
    accessTime: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for INodeSection_INodeSymlink {}

impl INodeSection_INodeSymlink {
    pub fn new() -> INodeSection_INodeSymlink {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static INodeSection_INodeSymlink {
        static mut instance: ::protobuf::lazy::Lazy<INodeSection_INodeSymlink> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const INodeSection_INodeSymlink,
        };
        unsafe {
            instance.get(INodeSection_INodeSymlink::new)
        }
    }

    // optional fixed64 permission = 1;

    pub fn clear_permission(&mut self) {
        self.permission = ::std::option::Option::None;
    }

    pub fn has_permission(&self) -> bool {
        self.permission.is_some()
    }

    // Param is passed by value, moved
    pub fn set_permission(&mut self, v: u64) {
        self.permission = ::std::option::Option::Some(v);
    }

    pub fn get_permission(&self) -> u64 {
        self.permission.unwrap_or(0)
    }

    fn get_permission_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.permission
    }

    fn mut_permission_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.permission
    }

    // optional bytes target = 2;

    pub fn clear_target(&mut self) {
        self.target.clear();
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: ::std::vec::Vec<u8>) {
        self.target = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.target.is_none() {
            self.target.set_default();
        }
        self.target.as_mut().unwrap()
    }

    // Take field
    pub fn take_target(&mut self) -> ::std::vec::Vec<u8> {
        self.target.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_target(&self) -> &[u8] {
        match self.target.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_target_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.target
    }

    fn mut_target_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.target
    }

    // optional uint64 modificationTime = 3;

    pub fn clear_modificationTime(&mut self) {
        self.modificationTime = ::std::option::Option::None;
    }

    pub fn has_modificationTime(&self) -> bool {
        self.modificationTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modificationTime(&mut self, v: u64) {
        self.modificationTime = ::std::option::Option::Some(v);
    }

    pub fn get_modificationTime(&self) -> u64 {
        self.modificationTime.unwrap_or(0)
    }

    fn get_modificationTime_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.modificationTime
    }

    fn mut_modificationTime_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.modificationTime
    }

    // optional uint64 accessTime = 4;

    pub fn clear_accessTime(&mut self) {
        self.accessTime = ::std::option::Option::None;
    }

    pub fn has_accessTime(&self) -> bool {
        self.accessTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accessTime(&mut self, v: u64) {
        self.accessTime = ::std::option::Option::Some(v);
    }

    pub fn get_accessTime(&self) -> u64 {
        self.accessTime.unwrap_or(0)
    }

    fn get_accessTime_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.accessTime
    }

    fn mut_accessTime_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.accessTime
    }
}

impl ::protobuf::Message for INodeSection_INodeSymlink {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_fixed64()?;
                    self.permission = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.target)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.modificationTime = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.accessTime = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.permission {
            my_size += 9;
        }
        if let Some(ref v) = self.target.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(v) = self.modificationTime {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.accessTime {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.permission {
            os.write_fixed64(1, v)?;
        }
        if let Some(ref v) = self.target.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(v) = self.modificationTime {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.accessTime {
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

impl ::protobuf::MessageStatic for INodeSection_INodeSymlink {
    fn new() -> INodeSection_INodeSymlink {
        INodeSection_INodeSymlink::new()
    }

    fn descriptor_static(_: ::std::option::Option<INodeSection_INodeSymlink>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "permission",
                    INodeSection_INodeSymlink::get_permission_for_reflect,
                    INodeSection_INodeSymlink::mut_permission_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "target",
                    INodeSection_INodeSymlink::get_target_for_reflect,
                    INodeSection_INodeSymlink::mut_target_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "modificationTime",
                    INodeSection_INodeSymlink::get_modificationTime_for_reflect,
                    INodeSection_INodeSymlink::mut_modificationTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "accessTime",
                    INodeSection_INodeSymlink::get_accessTime_for_reflect,
                    INodeSection_INodeSymlink::mut_accessTime_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<INodeSection_INodeSymlink>(
                    "INodeSection_INodeSymlink",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for INodeSection_INodeSymlink {
    fn clear(&mut self) {
        self.clear_permission();
        self.clear_target();
        self.clear_modificationTime();
        self.clear_accessTime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for INodeSection_INodeSymlink {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for INodeSection_INodeSymlink {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct INodeSection_INode {
    // message fields
    field_type: ::std::option::Option<INodeSection_INode_Type>,
    id: ::std::option::Option<u64>,
    name: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    file: ::protobuf::SingularPtrField<INodeSection_INodeFile>,
    directory: ::protobuf::SingularPtrField<INodeSection_INodeDirectory>,
    symlink: ::protobuf::SingularPtrField<INodeSection_INodeSymlink>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for INodeSection_INode {}

impl INodeSection_INode {
    pub fn new() -> INodeSection_INode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static INodeSection_INode {
        static mut instance: ::protobuf::lazy::Lazy<INodeSection_INode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const INodeSection_INode,
        };
        unsafe {
            instance.get(INodeSection_INode::new)
        }
    }

    // required .hadoop.hdfs.fsimage.INodeSection.INode.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: INodeSection_INode_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> INodeSection_INode_Type {
        self.field_type.unwrap_or(INodeSection_INode_Type::FILE)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<INodeSection_INode_Type> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<INodeSection_INode_Type> {
        &mut self.field_type
    }

    // required uint64 id = 2;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.id
    }

    // optional bytes name = 3;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        self.name.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_name(&self) -> &[u8] {
        match self.name.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.name
    }

    // optional .hadoop.hdfs.fsimage.INodeSection.INodeFile file = 4;

    pub fn clear_file(&mut self) {
        self.file.clear();
    }

    pub fn has_file(&self) -> bool {
        self.file.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file(&mut self, v: INodeSection_INodeFile) {
        self.file = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file(&mut self) -> &mut INodeSection_INodeFile {
        if self.file.is_none() {
            self.file.set_default();
        }
        self.file.as_mut().unwrap()
    }

    // Take field
    pub fn take_file(&mut self) -> INodeSection_INodeFile {
        self.file.take().unwrap_or_else(|| INodeSection_INodeFile::new())
    }

    pub fn get_file(&self) -> &INodeSection_INodeFile {
        self.file.as_ref().unwrap_or_else(|| INodeSection_INodeFile::default_instance())
    }

    fn get_file_for_reflect(&self) -> &::protobuf::SingularPtrField<INodeSection_INodeFile> {
        &self.file
    }

    fn mut_file_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<INodeSection_INodeFile> {
        &mut self.file
    }

    // optional .hadoop.hdfs.fsimage.INodeSection.INodeDirectory directory = 5;

    pub fn clear_directory(&mut self) {
        self.directory.clear();
    }

    pub fn has_directory(&self) -> bool {
        self.directory.is_some()
    }

    // Param is passed by value, moved
    pub fn set_directory(&mut self, v: INodeSection_INodeDirectory) {
        self.directory = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_directory(&mut self) -> &mut INodeSection_INodeDirectory {
        if self.directory.is_none() {
            self.directory.set_default();
        }
        self.directory.as_mut().unwrap()
    }

    // Take field
    pub fn take_directory(&mut self) -> INodeSection_INodeDirectory {
        self.directory.take().unwrap_or_else(|| INodeSection_INodeDirectory::new())
    }

    pub fn get_directory(&self) -> &INodeSection_INodeDirectory {
        self.directory.as_ref().unwrap_or_else(|| INodeSection_INodeDirectory::default_instance())
    }

    fn get_directory_for_reflect(&self) -> &::protobuf::SingularPtrField<INodeSection_INodeDirectory> {
        &self.directory
    }

    fn mut_directory_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<INodeSection_INodeDirectory> {
        &mut self.directory
    }

    // optional .hadoop.hdfs.fsimage.INodeSection.INodeSymlink symlink = 6;

    pub fn clear_symlink(&mut self) {
        self.symlink.clear();
    }

    pub fn has_symlink(&self) -> bool {
        self.symlink.is_some()
    }

    // Param is passed by value, moved
    pub fn set_symlink(&mut self, v: INodeSection_INodeSymlink) {
        self.symlink = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_symlink(&mut self) -> &mut INodeSection_INodeSymlink {
        if self.symlink.is_none() {
            self.symlink.set_default();
        }
        self.symlink.as_mut().unwrap()
    }

    // Take field
    pub fn take_symlink(&mut self) -> INodeSection_INodeSymlink {
        self.symlink.take().unwrap_or_else(|| INodeSection_INodeSymlink::new())
    }

    pub fn get_symlink(&self) -> &INodeSection_INodeSymlink {
        self.symlink.as_ref().unwrap_or_else(|| INodeSection_INodeSymlink::default_instance())
    }

    fn get_symlink_for_reflect(&self) -> &::protobuf::SingularPtrField<INodeSection_INodeSymlink> {
        &self.symlink
    }

    fn mut_symlink_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<INodeSection_INodeSymlink> {
        &mut self.symlink
    }
}

impl ::protobuf::Message for INodeSection_INode {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        }
        if self.id.is_none() {
            return false;
        }
        for v in &self.file {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.directory {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.symlink {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.file)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.directory)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.symlink)?;
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        if let Some(ref v) = self.file.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.directory.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.symlink.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.id {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_bytes(3, &v)?;
        }
        if let Some(ref v) = self.file.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.directory.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.symlink.as_ref() {
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

impl ::protobuf::MessageStatic for INodeSection_INode {
    fn new() -> INodeSection_INode {
        INodeSection_INode::new()
    }

    fn descriptor_static(_: ::std::option::Option<INodeSection_INode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<INodeSection_INode_Type>>(
                    "type",
                    INodeSection_INode::get_field_type_for_reflect,
                    INodeSection_INode::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "id",
                    INodeSection_INode::get_id_for_reflect,
                    INodeSection_INode::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    INodeSection_INode::get_name_for_reflect,
                    INodeSection_INode::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<INodeSection_INodeFile>>(
                    "file",
                    INodeSection_INode::get_file_for_reflect,
                    INodeSection_INode::mut_file_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<INodeSection_INodeDirectory>>(
                    "directory",
                    INodeSection_INode::get_directory_for_reflect,
                    INodeSection_INode::mut_directory_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<INodeSection_INodeSymlink>>(
                    "symlink",
                    INodeSection_INode::get_symlink_for_reflect,
                    INodeSection_INode::mut_symlink_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<INodeSection_INode>(
                    "INodeSection_INode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for INodeSection_INode {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_id();
        self.clear_name();
        self.clear_file();
        self.clear_directory();
        self.clear_symlink();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for INodeSection_INode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for INodeSection_INode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum INodeSection_INode_Type {
    FILE = 1,
    DIRECTORY = 2,
    SYMLINK = 3,
}

impl ::protobuf::ProtobufEnum for INodeSection_INode_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<INodeSection_INode_Type> {
        match value {
            1 => ::std::option::Option::Some(INodeSection_INode_Type::FILE),
            2 => ::std::option::Option::Some(INodeSection_INode_Type::DIRECTORY),
            3 => ::std::option::Option::Some(INodeSection_INode_Type::SYMLINK),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [INodeSection_INode_Type] = &[
            INodeSection_INode_Type::FILE,
            INodeSection_INode_Type::DIRECTORY,
            INodeSection_INode_Type::SYMLINK,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<INodeSection_INode_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("INodeSection_INode_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for INodeSection_INode_Type {
}

impl ::protobuf::reflect::ProtobufValue for INodeSection_INode_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FilesUnderConstructionSection {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FilesUnderConstructionSection {}

impl FilesUnderConstructionSection {
    pub fn new() -> FilesUnderConstructionSection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FilesUnderConstructionSection {
        static mut instance: ::protobuf::lazy::Lazy<FilesUnderConstructionSection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FilesUnderConstructionSection,
        };
        unsafe {
            instance.get(FilesUnderConstructionSection::new)
        }
    }
}

impl ::protobuf::Message for FilesUnderConstructionSection {
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

impl ::protobuf::MessageStatic for FilesUnderConstructionSection {
    fn new() -> FilesUnderConstructionSection {
        FilesUnderConstructionSection::new()
    }

    fn descriptor_static(_: ::std::option::Option<FilesUnderConstructionSection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<FilesUnderConstructionSection>(
                    "FilesUnderConstructionSection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FilesUnderConstructionSection {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FilesUnderConstructionSection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FilesUnderConstructionSection {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FilesUnderConstructionSection_FileUnderConstructionEntry {
    // message fields
    inodeId: ::std::option::Option<u64>,
    fullPath: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FilesUnderConstructionSection_FileUnderConstructionEntry {}

impl FilesUnderConstructionSection_FileUnderConstructionEntry {
    pub fn new() -> FilesUnderConstructionSection_FileUnderConstructionEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FilesUnderConstructionSection_FileUnderConstructionEntry {
        static mut instance: ::protobuf::lazy::Lazy<FilesUnderConstructionSection_FileUnderConstructionEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FilesUnderConstructionSection_FileUnderConstructionEntry,
        };
        unsafe {
            instance.get(FilesUnderConstructionSection_FileUnderConstructionEntry::new)
        }
    }

    // optional uint64 inodeId = 1;

    pub fn clear_inodeId(&mut self) {
        self.inodeId = ::std::option::Option::None;
    }

    pub fn has_inodeId(&self) -> bool {
        self.inodeId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inodeId(&mut self, v: u64) {
        self.inodeId = ::std::option::Option::Some(v);
    }

    pub fn get_inodeId(&self) -> u64 {
        self.inodeId.unwrap_or(0)
    }

    fn get_inodeId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.inodeId
    }

    fn mut_inodeId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.inodeId
    }

    // optional string fullPath = 2;

    pub fn clear_fullPath(&mut self) {
        self.fullPath.clear();
    }

    pub fn has_fullPath(&self) -> bool {
        self.fullPath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fullPath(&mut self, v: ::std::string::String) {
        self.fullPath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fullPath(&mut self) -> &mut ::std::string::String {
        if self.fullPath.is_none() {
            self.fullPath.set_default();
        }
        self.fullPath.as_mut().unwrap()
    }

    // Take field
    pub fn take_fullPath(&mut self) -> ::std::string::String {
        self.fullPath.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_fullPath(&self) -> &str {
        match self.fullPath.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_fullPath_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.fullPath
    }

    fn mut_fullPath_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.fullPath
    }
}

impl ::protobuf::Message for FilesUnderConstructionSection_FileUnderConstructionEntry {
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
                    let tmp = is.read_uint64()?;
                    self.inodeId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.fullPath)?;
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
        if let Some(v) = self.inodeId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.fullPath.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.inodeId {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.fullPath.as_ref() {
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

impl ::protobuf::MessageStatic for FilesUnderConstructionSection_FileUnderConstructionEntry {
    fn new() -> FilesUnderConstructionSection_FileUnderConstructionEntry {
        FilesUnderConstructionSection_FileUnderConstructionEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<FilesUnderConstructionSection_FileUnderConstructionEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "inodeId",
                    FilesUnderConstructionSection_FileUnderConstructionEntry::get_inodeId_for_reflect,
                    FilesUnderConstructionSection_FileUnderConstructionEntry::mut_inodeId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "fullPath",
                    FilesUnderConstructionSection_FileUnderConstructionEntry::get_fullPath_for_reflect,
                    FilesUnderConstructionSection_FileUnderConstructionEntry::mut_fullPath_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FilesUnderConstructionSection_FileUnderConstructionEntry>(
                    "FilesUnderConstructionSection_FileUnderConstructionEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FilesUnderConstructionSection_FileUnderConstructionEntry {
    fn clear(&mut self) {
        self.clear_inodeId();
        self.clear_fullPath();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FilesUnderConstructionSection_FileUnderConstructionEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FilesUnderConstructionSection_FileUnderConstructionEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct INodeDirectorySection {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for INodeDirectorySection {}

impl INodeDirectorySection {
    pub fn new() -> INodeDirectorySection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static INodeDirectorySection {
        static mut instance: ::protobuf::lazy::Lazy<INodeDirectorySection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const INodeDirectorySection,
        };
        unsafe {
            instance.get(INodeDirectorySection::new)
        }
    }
}

impl ::protobuf::Message for INodeDirectorySection {
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

impl ::protobuf::MessageStatic for INodeDirectorySection {
    fn new() -> INodeDirectorySection {
        INodeDirectorySection::new()
    }

    fn descriptor_static(_: ::std::option::Option<INodeDirectorySection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<INodeDirectorySection>(
                    "INodeDirectorySection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for INodeDirectorySection {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for INodeDirectorySection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for INodeDirectorySection {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct INodeDirectorySection_DirEntry {
    // message fields
    parent: ::std::option::Option<u64>,
    children: ::std::vec::Vec<u64>,
    refChildren: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for INodeDirectorySection_DirEntry {}

impl INodeDirectorySection_DirEntry {
    pub fn new() -> INodeDirectorySection_DirEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static INodeDirectorySection_DirEntry {
        static mut instance: ::protobuf::lazy::Lazy<INodeDirectorySection_DirEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const INodeDirectorySection_DirEntry,
        };
        unsafe {
            instance.get(INodeDirectorySection_DirEntry::new)
        }
    }

    // optional uint64 parent = 1;

    pub fn clear_parent(&mut self) {
        self.parent = ::std::option::Option::None;
    }

    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parent(&mut self, v: u64) {
        self.parent = ::std::option::Option::Some(v);
    }

    pub fn get_parent(&self) -> u64 {
        self.parent.unwrap_or(0)
    }

    fn get_parent_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.parent
    }

    fn mut_parent_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.parent
    }

    // repeated uint64 children = 2;

    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    // Param is passed by value, moved
    pub fn set_children(&mut self, v: ::std::vec::Vec<u64>) {
        self.children = v;
    }

    // Mutable pointer to the field.
    pub fn mut_children(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.children
    }

    // Take field
    pub fn take_children(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.children, ::std::vec::Vec::new())
    }

    pub fn get_children(&self) -> &[u64] {
        &self.children
    }

    fn get_children_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.children
    }

    fn mut_children_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.children
    }

    // repeated uint32 refChildren = 3;

    pub fn clear_refChildren(&mut self) {
        self.refChildren.clear();
    }

    // Param is passed by value, moved
    pub fn set_refChildren(&mut self, v: ::std::vec::Vec<u32>) {
        self.refChildren = v;
    }

    // Mutable pointer to the field.
    pub fn mut_refChildren(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.refChildren
    }

    // Take field
    pub fn take_refChildren(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.refChildren, ::std::vec::Vec::new())
    }

    pub fn get_refChildren(&self) -> &[u32] {
        &self.refChildren
    }

    fn get_refChildren_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.refChildren
    }

    fn mut_refChildren_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.refChildren
    }
}

impl ::protobuf::Message for INodeDirectorySection_DirEntry {
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
                    let tmp = is.read_uint64()?;
                    self.parent = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.children)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.refChildren)?;
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
        if let Some(v) = self.parent {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.children.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(2, &self.children);
        }
        if !self.refChildren.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(3, &self.refChildren);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.parent {
            os.write_uint64(1, v)?;
        }
        if !self.children.is_empty() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.children))?;
            for v in &self.children {
                os.write_uint64_no_tag(*v)?;
            };
        }
        if !self.refChildren.is_empty() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.refChildren))?;
            for v in &self.refChildren {
                os.write_uint32_no_tag(*v)?;
            };
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

impl ::protobuf::MessageStatic for INodeDirectorySection_DirEntry {
    fn new() -> INodeDirectorySection_DirEntry {
        INodeDirectorySection_DirEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<INodeDirectorySection_DirEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "parent",
                    INodeDirectorySection_DirEntry::get_parent_for_reflect,
                    INodeDirectorySection_DirEntry::mut_parent_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "children",
                    INodeDirectorySection_DirEntry::get_children_for_reflect,
                    INodeDirectorySection_DirEntry::mut_children_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "refChildren",
                    INodeDirectorySection_DirEntry::get_refChildren_for_reflect,
                    INodeDirectorySection_DirEntry::mut_refChildren_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<INodeDirectorySection_DirEntry>(
                    "INodeDirectorySection_DirEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for INodeDirectorySection_DirEntry {
    fn clear(&mut self) {
        self.clear_parent();
        self.clear_children();
        self.clear_refChildren();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for INodeDirectorySection_DirEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for INodeDirectorySection_DirEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct INodeReferenceSection {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for INodeReferenceSection {}

impl INodeReferenceSection {
    pub fn new() -> INodeReferenceSection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static INodeReferenceSection {
        static mut instance: ::protobuf::lazy::Lazy<INodeReferenceSection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const INodeReferenceSection,
        };
        unsafe {
            instance.get(INodeReferenceSection::new)
        }
    }
}

impl ::protobuf::Message for INodeReferenceSection {
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

impl ::protobuf::MessageStatic for INodeReferenceSection {
    fn new() -> INodeReferenceSection {
        INodeReferenceSection::new()
    }

    fn descriptor_static(_: ::std::option::Option<INodeReferenceSection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<INodeReferenceSection>(
                    "INodeReferenceSection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for INodeReferenceSection {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for INodeReferenceSection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for INodeReferenceSection {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct INodeReferenceSection_INodeReference {
    // message fields
    referredId: ::std::option::Option<u64>,
    name: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    dstSnapshotId: ::std::option::Option<u32>,
    lastSnapshotId: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for INodeReferenceSection_INodeReference {}

impl INodeReferenceSection_INodeReference {
    pub fn new() -> INodeReferenceSection_INodeReference {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static INodeReferenceSection_INodeReference {
        static mut instance: ::protobuf::lazy::Lazy<INodeReferenceSection_INodeReference> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const INodeReferenceSection_INodeReference,
        };
        unsafe {
            instance.get(INodeReferenceSection_INodeReference::new)
        }
    }

    // optional uint64 referredId = 1;

    pub fn clear_referredId(&mut self) {
        self.referredId = ::std::option::Option::None;
    }

    pub fn has_referredId(&self) -> bool {
        self.referredId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_referredId(&mut self, v: u64) {
        self.referredId = ::std::option::Option::Some(v);
    }

    pub fn get_referredId(&self) -> u64 {
        self.referredId.unwrap_or(0)
    }

    fn get_referredId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.referredId
    }

    fn mut_referredId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.referredId
    }

    // optional bytes name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        self.name.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_name(&self) -> &[u8] {
        match self.name.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.name
    }

    // optional uint32 dstSnapshotId = 3;

    pub fn clear_dstSnapshotId(&mut self) {
        self.dstSnapshotId = ::std::option::Option::None;
    }

    pub fn has_dstSnapshotId(&self) -> bool {
        self.dstSnapshotId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dstSnapshotId(&mut self, v: u32) {
        self.dstSnapshotId = ::std::option::Option::Some(v);
    }

    pub fn get_dstSnapshotId(&self) -> u32 {
        self.dstSnapshotId.unwrap_or(0)
    }

    fn get_dstSnapshotId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dstSnapshotId
    }

    fn mut_dstSnapshotId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dstSnapshotId
    }

    // optional uint32 lastSnapshotId = 4;

    pub fn clear_lastSnapshotId(&mut self) {
        self.lastSnapshotId = ::std::option::Option::None;
    }

    pub fn has_lastSnapshotId(&self) -> bool {
        self.lastSnapshotId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastSnapshotId(&mut self, v: u32) {
        self.lastSnapshotId = ::std::option::Option::Some(v);
    }

    pub fn get_lastSnapshotId(&self) -> u32 {
        self.lastSnapshotId.unwrap_or(0)
    }

    fn get_lastSnapshotId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.lastSnapshotId
    }

    fn mut_lastSnapshotId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.lastSnapshotId
    }
}

impl ::protobuf::Message for INodeReferenceSection_INodeReference {
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
                    let tmp = is.read_uint64()?;
                    self.referredId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.dstSnapshotId = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.lastSnapshotId = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.referredId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(v) = self.dstSnapshotId {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.lastSnapshotId {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.referredId {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(v) = self.dstSnapshotId {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.lastSnapshotId {
            os.write_uint32(4, v)?;
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

impl ::protobuf::MessageStatic for INodeReferenceSection_INodeReference {
    fn new() -> INodeReferenceSection_INodeReference {
        INodeReferenceSection_INodeReference::new()
    }

    fn descriptor_static(_: ::std::option::Option<INodeReferenceSection_INodeReference>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "referredId",
                    INodeReferenceSection_INodeReference::get_referredId_for_reflect,
                    INodeReferenceSection_INodeReference::mut_referredId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    INodeReferenceSection_INodeReference::get_name_for_reflect,
                    INodeReferenceSection_INodeReference::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dstSnapshotId",
                    INodeReferenceSection_INodeReference::get_dstSnapshotId_for_reflect,
                    INodeReferenceSection_INodeReference::mut_dstSnapshotId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "lastSnapshotId",
                    INodeReferenceSection_INodeReference::get_lastSnapshotId_for_reflect,
                    INodeReferenceSection_INodeReference::mut_lastSnapshotId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<INodeReferenceSection_INodeReference>(
                    "INodeReferenceSection_INodeReference",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for INodeReferenceSection_INodeReference {
    fn clear(&mut self) {
        self.clear_referredId();
        self.clear_name();
        self.clear_dstSnapshotId();
        self.clear_lastSnapshotId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for INodeReferenceSection_INodeReference {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for INodeReferenceSection_INodeReference {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotSection {
    // message fields
    snapshotCounter: ::std::option::Option<u32>,
    snapshottableDir: ::std::vec::Vec<u64>,
    numSnapshots: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotSection {}

impl SnapshotSection {
    pub fn new() -> SnapshotSection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotSection {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotSection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotSection,
        };
        unsafe {
            instance.get(SnapshotSection::new)
        }
    }

    // optional uint32 snapshotCounter = 1;

    pub fn clear_snapshotCounter(&mut self) {
        self.snapshotCounter = ::std::option::Option::None;
    }

    pub fn has_snapshotCounter(&self) -> bool {
        self.snapshotCounter.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshotCounter(&mut self, v: u32) {
        self.snapshotCounter = ::std::option::Option::Some(v);
    }

    pub fn get_snapshotCounter(&self) -> u32 {
        self.snapshotCounter.unwrap_or(0)
    }

    fn get_snapshotCounter_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.snapshotCounter
    }

    fn mut_snapshotCounter_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.snapshotCounter
    }

    // repeated uint64 snapshottableDir = 2;

    pub fn clear_snapshottableDir(&mut self) {
        self.snapshottableDir.clear();
    }

    // Param is passed by value, moved
    pub fn set_snapshottableDir(&mut self, v: ::std::vec::Vec<u64>) {
        self.snapshottableDir = v;
    }

    // Mutable pointer to the field.
    pub fn mut_snapshottableDir(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.snapshottableDir
    }

    // Take field
    pub fn take_snapshottableDir(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.snapshottableDir, ::std::vec::Vec::new())
    }

    pub fn get_snapshottableDir(&self) -> &[u64] {
        &self.snapshottableDir
    }

    fn get_snapshottableDir_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.snapshottableDir
    }

    fn mut_snapshottableDir_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.snapshottableDir
    }

    // optional uint32 numSnapshots = 3;

    pub fn clear_numSnapshots(&mut self) {
        self.numSnapshots = ::std::option::Option::None;
    }

    pub fn has_numSnapshots(&self) -> bool {
        self.numSnapshots.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numSnapshots(&mut self, v: u32) {
        self.numSnapshots = ::std::option::Option::Some(v);
    }

    pub fn get_numSnapshots(&self) -> u32 {
        self.numSnapshots.unwrap_or(0)
    }

    fn get_numSnapshots_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.numSnapshots
    }

    fn mut_numSnapshots_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.numSnapshots
    }
}

impl ::protobuf::Message for SnapshotSection {
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
                    let tmp = is.read_uint32()?;
                    self.snapshotCounter = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.snapshottableDir)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.numSnapshots = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.snapshotCounter {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.snapshottableDir.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(2, &self.snapshottableDir);
        }
        if let Some(v) = self.numSnapshots {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.snapshotCounter {
            os.write_uint32(1, v)?;
        }
        if !self.snapshottableDir.is_empty() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.snapshottableDir))?;
            for v in &self.snapshottableDir {
                os.write_uint64_no_tag(*v)?;
            };
        }
        if let Some(v) = self.numSnapshots {
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

impl ::protobuf::MessageStatic for SnapshotSection {
    fn new() -> SnapshotSection {
        SnapshotSection::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotSection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "snapshotCounter",
                    SnapshotSection::get_snapshotCounter_for_reflect,
                    SnapshotSection::mut_snapshotCounter_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "snapshottableDir",
                    SnapshotSection::get_snapshottableDir_for_reflect,
                    SnapshotSection::mut_snapshottableDir_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "numSnapshots",
                    SnapshotSection::get_numSnapshots_for_reflect,
                    SnapshotSection::mut_numSnapshots_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotSection>(
                    "SnapshotSection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotSection {
    fn clear(&mut self) {
        self.clear_snapshotCounter();
        self.clear_snapshottableDir();
        self.clear_numSnapshots();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotSection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotSection {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotSection_Snapshot {
    // message fields
    snapshotId: ::std::option::Option<u32>,
    root: ::protobuf::SingularPtrField<INodeSection_INode>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotSection_Snapshot {}

impl SnapshotSection_Snapshot {
    pub fn new() -> SnapshotSection_Snapshot {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotSection_Snapshot {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotSection_Snapshot> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotSection_Snapshot,
        };
        unsafe {
            instance.get(SnapshotSection_Snapshot::new)
        }
    }

    // optional uint32 snapshotId = 1;

    pub fn clear_snapshotId(&mut self) {
        self.snapshotId = ::std::option::Option::None;
    }

    pub fn has_snapshotId(&self) -> bool {
        self.snapshotId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshotId(&mut self, v: u32) {
        self.snapshotId = ::std::option::Option::Some(v);
    }

    pub fn get_snapshotId(&self) -> u32 {
        self.snapshotId.unwrap_or(0)
    }

    fn get_snapshotId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.snapshotId
    }

    fn mut_snapshotId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.snapshotId
    }

    // optional .hadoop.hdfs.fsimage.INodeSection.INode root = 2;

    pub fn clear_root(&mut self) {
        self.root.clear();
    }

    pub fn has_root(&self) -> bool {
        self.root.is_some()
    }

    // Param is passed by value, moved
    pub fn set_root(&mut self, v: INodeSection_INode) {
        self.root = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_root(&mut self) -> &mut INodeSection_INode {
        if self.root.is_none() {
            self.root.set_default();
        }
        self.root.as_mut().unwrap()
    }

    // Take field
    pub fn take_root(&mut self) -> INodeSection_INode {
        self.root.take().unwrap_or_else(|| INodeSection_INode::new())
    }

    pub fn get_root(&self) -> &INodeSection_INode {
        self.root.as_ref().unwrap_or_else(|| INodeSection_INode::default_instance())
    }

    fn get_root_for_reflect(&self) -> &::protobuf::SingularPtrField<INodeSection_INode> {
        &self.root
    }

    fn mut_root_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<INodeSection_INode> {
        &mut self.root
    }
}

impl ::protobuf::Message for SnapshotSection_Snapshot {
    fn is_initialized(&self) -> bool {
        for v in &self.root {
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
                    self.snapshotId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.root)?;
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
        if let Some(v) = self.snapshotId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.root.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.snapshotId {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.root.as_ref() {
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

impl ::protobuf::MessageStatic for SnapshotSection_Snapshot {
    fn new() -> SnapshotSection_Snapshot {
        SnapshotSection_Snapshot::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotSection_Snapshot>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "snapshotId",
                    SnapshotSection_Snapshot::get_snapshotId_for_reflect,
                    SnapshotSection_Snapshot::mut_snapshotId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<INodeSection_INode>>(
                    "root",
                    SnapshotSection_Snapshot::get_root_for_reflect,
                    SnapshotSection_Snapshot::mut_root_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotSection_Snapshot>(
                    "SnapshotSection_Snapshot",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotSection_Snapshot {
    fn clear(&mut self) {
        self.clear_snapshotId();
        self.clear_root();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotSection_Snapshot {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotSection_Snapshot {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotDiffSection {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotDiffSection {}

impl SnapshotDiffSection {
    pub fn new() -> SnapshotDiffSection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotDiffSection {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotDiffSection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotDiffSection,
        };
        unsafe {
            instance.get(SnapshotDiffSection::new)
        }
    }
}

impl ::protobuf::Message for SnapshotDiffSection {
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

impl ::protobuf::MessageStatic for SnapshotDiffSection {
    fn new() -> SnapshotDiffSection {
        SnapshotDiffSection::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotDiffSection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotDiffSection>(
                    "SnapshotDiffSection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotDiffSection {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotDiffSection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotDiffSection {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotDiffSection_CreatedListEntry {
    // message fields
    name: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotDiffSection_CreatedListEntry {}

impl SnapshotDiffSection_CreatedListEntry {
    pub fn new() -> SnapshotDiffSection_CreatedListEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotDiffSection_CreatedListEntry {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotDiffSection_CreatedListEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotDiffSection_CreatedListEntry,
        };
        unsafe {
            instance.get(SnapshotDiffSection_CreatedListEntry::new)
        }
    }

    // optional bytes name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        self.name.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_name(&self) -> &[u8] {
        match self.name.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.name
    }
}

impl ::protobuf::Message for SnapshotDiffSection_CreatedListEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.name)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_bytes(1, &v)?;
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

impl ::protobuf::MessageStatic for SnapshotDiffSection_CreatedListEntry {
    fn new() -> SnapshotDiffSection_CreatedListEntry {
        SnapshotDiffSection_CreatedListEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotDiffSection_CreatedListEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    SnapshotDiffSection_CreatedListEntry::get_name_for_reflect,
                    SnapshotDiffSection_CreatedListEntry::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotDiffSection_CreatedListEntry>(
                    "SnapshotDiffSection_CreatedListEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotDiffSection_CreatedListEntry {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotDiffSection_CreatedListEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotDiffSection_CreatedListEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotDiffSection_DirectoryDiff {
    // message fields
    snapshotId: ::std::option::Option<u32>,
    childrenSize: ::std::option::Option<u32>,
    isSnapshotRoot: ::std::option::Option<bool>,
    name: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    snapshotCopy: ::protobuf::SingularPtrField<INodeSection_INodeDirectory>,
    createdListSize: ::std::option::Option<u32>,
    deletedINode: ::std::vec::Vec<u64>,
    deletedINodeRef: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotDiffSection_DirectoryDiff {}

impl SnapshotDiffSection_DirectoryDiff {
    pub fn new() -> SnapshotDiffSection_DirectoryDiff {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotDiffSection_DirectoryDiff {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotDiffSection_DirectoryDiff> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotDiffSection_DirectoryDiff,
        };
        unsafe {
            instance.get(SnapshotDiffSection_DirectoryDiff::new)
        }
    }

    // optional uint32 snapshotId = 1;

    pub fn clear_snapshotId(&mut self) {
        self.snapshotId = ::std::option::Option::None;
    }

    pub fn has_snapshotId(&self) -> bool {
        self.snapshotId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshotId(&mut self, v: u32) {
        self.snapshotId = ::std::option::Option::Some(v);
    }

    pub fn get_snapshotId(&self) -> u32 {
        self.snapshotId.unwrap_or(0)
    }

    fn get_snapshotId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.snapshotId
    }

    fn mut_snapshotId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.snapshotId
    }

    // optional uint32 childrenSize = 2;

    pub fn clear_childrenSize(&mut self) {
        self.childrenSize = ::std::option::Option::None;
    }

    pub fn has_childrenSize(&self) -> bool {
        self.childrenSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_childrenSize(&mut self, v: u32) {
        self.childrenSize = ::std::option::Option::Some(v);
    }

    pub fn get_childrenSize(&self) -> u32 {
        self.childrenSize.unwrap_or(0)
    }

    fn get_childrenSize_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.childrenSize
    }

    fn mut_childrenSize_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.childrenSize
    }

    // optional bool isSnapshotRoot = 3;

    pub fn clear_isSnapshotRoot(&mut self) {
        self.isSnapshotRoot = ::std::option::Option::None;
    }

    pub fn has_isSnapshotRoot(&self) -> bool {
        self.isSnapshotRoot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_isSnapshotRoot(&mut self, v: bool) {
        self.isSnapshotRoot = ::std::option::Option::Some(v);
    }

    pub fn get_isSnapshotRoot(&self) -> bool {
        self.isSnapshotRoot.unwrap_or(false)
    }

    fn get_isSnapshotRoot_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.isSnapshotRoot
    }

    fn mut_isSnapshotRoot_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.isSnapshotRoot
    }

    // optional bytes name = 4;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        self.name.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_name(&self) -> &[u8] {
        match self.name.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.name
    }

    // optional .hadoop.hdfs.fsimage.INodeSection.INodeDirectory snapshotCopy = 5;

    pub fn clear_snapshotCopy(&mut self) {
        self.snapshotCopy.clear();
    }

    pub fn has_snapshotCopy(&self) -> bool {
        self.snapshotCopy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshotCopy(&mut self, v: INodeSection_INodeDirectory) {
        self.snapshotCopy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_snapshotCopy(&mut self) -> &mut INodeSection_INodeDirectory {
        if self.snapshotCopy.is_none() {
            self.snapshotCopy.set_default();
        }
        self.snapshotCopy.as_mut().unwrap()
    }

    // Take field
    pub fn take_snapshotCopy(&mut self) -> INodeSection_INodeDirectory {
        self.snapshotCopy.take().unwrap_or_else(|| INodeSection_INodeDirectory::new())
    }

    pub fn get_snapshotCopy(&self) -> &INodeSection_INodeDirectory {
        self.snapshotCopy.as_ref().unwrap_or_else(|| INodeSection_INodeDirectory::default_instance())
    }

    fn get_snapshotCopy_for_reflect(&self) -> &::protobuf::SingularPtrField<INodeSection_INodeDirectory> {
        &self.snapshotCopy
    }

    fn mut_snapshotCopy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<INodeSection_INodeDirectory> {
        &mut self.snapshotCopy
    }

    // optional uint32 createdListSize = 6;

    pub fn clear_createdListSize(&mut self) {
        self.createdListSize = ::std::option::Option::None;
    }

    pub fn has_createdListSize(&self) -> bool {
        self.createdListSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_createdListSize(&mut self, v: u32) {
        self.createdListSize = ::std::option::Option::Some(v);
    }

    pub fn get_createdListSize(&self) -> u32 {
        self.createdListSize.unwrap_or(0)
    }

    fn get_createdListSize_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.createdListSize
    }

    fn mut_createdListSize_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.createdListSize
    }

    // repeated uint64 deletedINode = 7;

    pub fn clear_deletedINode(&mut self) {
        self.deletedINode.clear();
    }

    // Param is passed by value, moved
    pub fn set_deletedINode(&mut self, v: ::std::vec::Vec<u64>) {
        self.deletedINode = v;
    }

    // Mutable pointer to the field.
    pub fn mut_deletedINode(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.deletedINode
    }

    // Take field
    pub fn take_deletedINode(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.deletedINode, ::std::vec::Vec::new())
    }

    pub fn get_deletedINode(&self) -> &[u64] {
        &self.deletedINode
    }

    fn get_deletedINode_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.deletedINode
    }

    fn mut_deletedINode_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.deletedINode
    }

    // repeated uint32 deletedINodeRef = 8;

    pub fn clear_deletedINodeRef(&mut self) {
        self.deletedINodeRef.clear();
    }

    // Param is passed by value, moved
    pub fn set_deletedINodeRef(&mut self, v: ::std::vec::Vec<u32>) {
        self.deletedINodeRef = v;
    }

    // Mutable pointer to the field.
    pub fn mut_deletedINodeRef(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.deletedINodeRef
    }

    // Take field
    pub fn take_deletedINodeRef(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.deletedINodeRef, ::std::vec::Vec::new())
    }

    pub fn get_deletedINodeRef(&self) -> &[u32] {
        &self.deletedINodeRef
    }

    fn get_deletedINodeRef_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.deletedINodeRef
    }

    fn mut_deletedINodeRef_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.deletedINodeRef
    }
}

impl ::protobuf::Message for SnapshotDiffSection_DirectoryDiff {
    fn is_initialized(&self) -> bool {
        for v in &self.snapshotCopy {
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
                    self.snapshotId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.childrenSize = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.isSnapshotRoot = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.name)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.snapshotCopy)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.createdListSize = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.deletedINode)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.deletedINodeRef)?;
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
        if let Some(v) = self.snapshotId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.childrenSize {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.isSnapshotRoot {
            my_size += 2;
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        if let Some(ref v) = self.snapshotCopy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.createdListSize {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.deletedINode.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(7, &self.deletedINode);
        }
        if !self.deletedINodeRef.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(8, &self.deletedINodeRef);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.snapshotId {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.childrenSize {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.isSnapshotRoot {
            os.write_bool(3, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_bytes(4, &v)?;
        }
        if let Some(ref v) = self.snapshotCopy.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.createdListSize {
            os.write_uint32(6, v)?;
        }
        if !self.deletedINode.is_empty() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.deletedINode))?;
            for v in &self.deletedINode {
                os.write_uint64_no_tag(*v)?;
            };
        }
        if !self.deletedINodeRef.is_empty() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.deletedINodeRef))?;
            for v in &self.deletedINodeRef {
                os.write_uint32_no_tag(*v)?;
            };
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

impl ::protobuf::MessageStatic for SnapshotDiffSection_DirectoryDiff {
    fn new() -> SnapshotDiffSection_DirectoryDiff {
        SnapshotDiffSection_DirectoryDiff::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotDiffSection_DirectoryDiff>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "snapshotId",
                    SnapshotDiffSection_DirectoryDiff::get_snapshotId_for_reflect,
                    SnapshotDiffSection_DirectoryDiff::mut_snapshotId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "childrenSize",
                    SnapshotDiffSection_DirectoryDiff::get_childrenSize_for_reflect,
                    SnapshotDiffSection_DirectoryDiff::mut_childrenSize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "isSnapshotRoot",
                    SnapshotDiffSection_DirectoryDiff::get_isSnapshotRoot_for_reflect,
                    SnapshotDiffSection_DirectoryDiff::mut_isSnapshotRoot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    SnapshotDiffSection_DirectoryDiff::get_name_for_reflect,
                    SnapshotDiffSection_DirectoryDiff::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<INodeSection_INodeDirectory>>(
                    "snapshotCopy",
                    SnapshotDiffSection_DirectoryDiff::get_snapshotCopy_for_reflect,
                    SnapshotDiffSection_DirectoryDiff::mut_snapshotCopy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "createdListSize",
                    SnapshotDiffSection_DirectoryDiff::get_createdListSize_for_reflect,
                    SnapshotDiffSection_DirectoryDiff::mut_createdListSize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "deletedINode",
                    SnapshotDiffSection_DirectoryDiff::get_deletedINode_for_reflect,
                    SnapshotDiffSection_DirectoryDiff::mut_deletedINode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "deletedINodeRef",
                    SnapshotDiffSection_DirectoryDiff::get_deletedINodeRef_for_reflect,
                    SnapshotDiffSection_DirectoryDiff::mut_deletedINodeRef_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotDiffSection_DirectoryDiff>(
                    "SnapshotDiffSection_DirectoryDiff",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotDiffSection_DirectoryDiff {
    fn clear(&mut self) {
        self.clear_snapshotId();
        self.clear_childrenSize();
        self.clear_isSnapshotRoot();
        self.clear_name();
        self.clear_snapshotCopy();
        self.clear_createdListSize();
        self.clear_deletedINode();
        self.clear_deletedINodeRef();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotDiffSection_DirectoryDiff {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotDiffSection_DirectoryDiff {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotDiffSection_FileDiff {
    // message fields
    snapshotId: ::std::option::Option<u32>,
    fileSize: ::std::option::Option<u64>,
    name: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    snapshotCopy: ::protobuf::SingularPtrField<INodeSection_INodeFile>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotDiffSection_FileDiff {}

impl SnapshotDiffSection_FileDiff {
    pub fn new() -> SnapshotDiffSection_FileDiff {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotDiffSection_FileDiff {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotDiffSection_FileDiff> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotDiffSection_FileDiff,
        };
        unsafe {
            instance.get(SnapshotDiffSection_FileDiff::new)
        }
    }

    // optional uint32 snapshotId = 1;

    pub fn clear_snapshotId(&mut self) {
        self.snapshotId = ::std::option::Option::None;
    }

    pub fn has_snapshotId(&self) -> bool {
        self.snapshotId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshotId(&mut self, v: u32) {
        self.snapshotId = ::std::option::Option::Some(v);
    }

    pub fn get_snapshotId(&self) -> u32 {
        self.snapshotId.unwrap_or(0)
    }

    fn get_snapshotId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.snapshotId
    }

    fn mut_snapshotId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.snapshotId
    }

    // optional uint64 fileSize = 2;

    pub fn clear_fileSize(&mut self) {
        self.fileSize = ::std::option::Option::None;
    }

    pub fn has_fileSize(&self) -> bool {
        self.fileSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileSize(&mut self, v: u64) {
        self.fileSize = ::std::option::Option::Some(v);
    }

    pub fn get_fileSize(&self) -> u64 {
        self.fileSize.unwrap_or(0)
    }

    fn get_fileSize_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.fileSize
    }

    fn mut_fileSize_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.fileSize
    }

    // optional bytes name = 3;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::vec::Vec<u8>) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::vec::Vec<u8> {
        self.name.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_name(&self) -> &[u8] {
        match self.name.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.name
    }

    // optional .hadoop.hdfs.fsimage.INodeSection.INodeFile snapshotCopy = 4;

    pub fn clear_snapshotCopy(&mut self) {
        self.snapshotCopy.clear();
    }

    pub fn has_snapshotCopy(&self) -> bool {
        self.snapshotCopy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshotCopy(&mut self, v: INodeSection_INodeFile) {
        self.snapshotCopy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_snapshotCopy(&mut self) -> &mut INodeSection_INodeFile {
        if self.snapshotCopy.is_none() {
            self.snapshotCopy.set_default();
        }
        self.snapshotCopy.as_mut().unwrap()
    }

    // Take field
    pub fn take_snapshotCopy(&mut self) -> INodeSection_INodeFile {
        self.snapshotCopy.take().unwrap_or_else(|| INodeSection_INodeFile::new())
    }

    pub fn get_snapshotCopy(&self) -> &INodeSection_INodeFile {
        self.snapshotCopy.as_ref().unwrap_or_else(|| INodeSection_INodeFile::default_instance())
    }

    fn get_snapshotCopy_for_reflect(&self) -> &::protobuf::SingularPtrField<INodeSection_INodeFile> {
        &self.snapshotCopy
    }

    fn mut_snapshotCopy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<INodeSection_INodeFile> {
        &mut self.snapshotCopy
    }
}

impl ::protobuf::Message for SnapshotDiffSection_FileDiff {
    fn is_initialized(&self) -> bool {
        for v in &self.snapshotCopy {
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
                    self.snapshotId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.fileSize = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.snapshotCopy)?;
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
        if let Some(v) = self.snapshotId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.fileSize {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        if let Some(ref v) = self.snapshotCopy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.snapshotId {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.fileSize {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_bytes(3, &v)?;
        }
        if let Some(ref v) = self.snapshotCopy.as_ref() {
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

impl ::protobuf::MessageStatic for SnapshotDiffSection_FileDiff {
    fn new() -> SnapshotDiffSection_FileDiff {
        SnapshotDiffSection_FileDiff::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotDiffSection_FileDiff>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "snapshotId",
                    SnapshotDiffSection_FileDiff::get_snapshotId_for_reflect,
                    SnapshotDiffSection_FileDiff::mut_snapshotId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "fileSize",
                    SnapshotDiffSection_FileDiff::get_fileSize_for_reflect,
                    SnapshotDiffSection_FileDiff::mut_fileSize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "name",
                    SnapshotDiffSection_FileDiff::get_name_for_reflect,
                    SnapshotDiffSection_FileDiff::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<INodeSection_INodeFile>>(
                    "snapshotCopy",
                    SnapshotDiffSection_FileDiff::get_snapshotCopy_for_reflect,
                    SnapshotDiffSection_FileDiff::mut_snapshotCopy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotDiffSection_FileDiff>(
                    "SnapshotDiffSection_FileDiff",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotDiffSection_FileDiff {
    fn clear(&mut self) {
        self.clear_snapshotId();
        self.clear_fileSize();
        self.clear_name();
        self.clear_snapshotCopy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotDiffSection_FileDiff {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotDiffSection_FileDiff {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotDiffSection_DiffEntry {
    // message fields
    field_type: ::std::option::Option<SnapshotDiffSection_DiffEntry_Type>,
    inodeId: ::std::option::Option<u64>,
    numOfDiff: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotDiffSection_DiffEntry {}

impl SnapshotDiffSection_DiffEntry {
    pub fn new() -> SnapshotDiffSection_DiffEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotDiffSection_DiffEntry {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotDiffSection_DiffEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotDiffSection_DiffEntry,
        };
        unsafe {
            instance.get(SnapshotDiffSection_DiffEntry::new)
        }
    }

    // required .hadoop.hdfs.fsimage.SnapshotDiffSection.DiffEntry.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: SnapshotDiffSection_DiffEntry_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> SnapshotDiffSection_DiffEntry_Type {
        self.field_type.unwrap_or(SnapshotDiffSection_DiffEntry_Type::FILEDIFF)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<SnapshotDiffSection_DiffEntry_Type> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<SnapshotDiffSection_DiffEntry_Type> {
        &mut self.field_type
    }

    // optional uint64 inodeId = 2;

    pub fn clear_inodeId(&mut self) {
        self.inodeId = ::std::option::Option::None;
    }

    pub fn has_inodeId(&self) -> bool {
        self.inodeId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inodeId(&mut self, v: u64) {
        self.inodeId = ::std::option::Option::Some(v);
    }

    pub fn get_inodeId(&self) -> u64 {
        self.inodeId.unwrap_or(0)
    }

    fn get_inodeId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.inodeId
    }

    fn mut_inodeId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.inodeId
    }

    // optional uint32 numOfDiff = 3;

    pub fn clear_numOfDiff(&mut self) {
        self.numOfDiff = ::std::option::Option::None;
    }

    pub fn has_numOfDiff(&self) -> bool {
        self.numOfDiff.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numOfDiff(&mut self, v: u32) {
        self.numOfDiff = ::std::option::Option::Some(v);
    }

    pub fn get_numOfDiff(&self) -> u32 {
        self.numOfDiff.unwrap_or(0)
    }

    fn get_numOfDiff_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.numOfDiff
    }

    fn mut_numOfDiff_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.numOfDiff
    }
}

impl ::protobuf::Message for SnapshotDiffSection_DiffEntry {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
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
                    let tmp = is.read_uint64()?;
                    self.inodeId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.numOfDiff = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.inodeId {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numOfDiff {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.inodeId {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.numOfDiff {
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

impl ::protobuf::MessageStatic for SnapshotDiffSection_DiffEntry {
    fn new() -> SnapshotDiffSection_DiffEntry {
        SnapshotDiffSection_DiffEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotDiffSection_DiffEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<SnapshotDiffSection_DiffEntry_Type>>(
                    "type",
                    SnapshotDiffSection_DiffEntry::get_field_type_for_reflect,
                    SnapshotDiffSection_DiffEntry::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "inodeId",
                    SnapshotDiffSection_DiffEntry::get_inodeId_for_reflect,
                    SnapshotDiffSection_DiffEntry::mut_inodeId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "numOfDiff",
                    SnapshotDiffSection_DiffEntry::get_numOfDiff_for_reflect,
                    SnapshotDiffSection_DiffEntry::mut_numOfDiff_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotDiffSection_DiffEntry>(
                    "SnapshotDiffSection_DiffEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotDiffSection_DiffEntry {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_inodeId();
        self.clear_numOfDiff();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotDiffSection_DiffEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotDiffSection_DiffEntry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SnapshotDiffSection_DiffEntry_Type {
    FILEDIFF = 1,
    DIRECTORYDIFF = 2,
}

impl ::protobuf::ProtobufEnum for SnapshotDiffSection_DiffEntry_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SnapshotDiffSection_DiffEntry_Type> {
        match value {
            1 => ::std::option::Option::Some(SnapshotDiffSection_DiffEntry_Type::FILEDIFF),
            2 => ::std::option::Option::Some(SnapshotDiffSection_DiffEntry_Type::DIRECTORYDIFF),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SnapshotDiffSection_DiffEntry_Type] = &[
            SnapshotDiffSection_DiffEntry_Type::FILEDIFF,
            SnapshotDiffSection_DiffEntry_Type::DIRECTORYDIFF,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<SnapshotDiffSection_DiffEntry_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SnapshotDiffSection_DiffEntry_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SnapshotDiffSection_DiffEntry_Type {
}

impl ::protobuf::reflect::ProtobufValue for SnapshotDiffSection_DiffEntry_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StringTableSection {
    // message fields
    numEntry: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StringTableSection {}

impl StringTableSection {
    pub fn new() -> StringTableSection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StringTableSection {
        static mut instance: ::protobuf::lazy::Lazy<StringTableSection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StringTableSection,
        };
        unsafe {
            instance.get(StringTableSection::new)
        }
    }

    // optional uint32 numEntry = 1;

    pub fn clear_numEntry(&mut self) {
        self.numEntry = ::std::option::Option::None;
    }

    pub fn has_numEntry(&self) -> bool {
        self.numEntry.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numEntry(&mut self, v: u32) {
        self.numEntry = ::std::option::Option::Some(v);
    }

    pub fn get_numEntry(&self) -> u32 {
        self.numEntry.unwrap_or(0)
    }

    fn get_numEntry_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.numEntry
    }

    fn mut_numEntry_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.numEntry
    }
}

impl ::protobuf::Message for StringTableSection {
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
                    let tmp = is.read_uint32()?;
                    self.numEntry = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.numEntry {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.numEntry {
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

impl ::protobuf::MessageStatic for StringTableSection {
    fn new() -> StringTableSection {
        StringTableSection::new()
    }

    fn descriptor_static(_: ::std::option::Option<StringTableSection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "numEntry",
                    StringTableSection::get_numEntry_for_reflect,
                    StringTableSection::mut_numEntry_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StringTableSection>(
                    "StringTableSection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StringTableSection {
    fn clear(&mut self) {
        self.clear_numEntry();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StringTableSection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StringTableSection {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StringTableSection_Entry {
    // message fields
    id: ::std::option::Option<u32>,
    str: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StringTableSection_Entry {}

impl StringTableSection_Entry {
    pub fn new() -> StringTableSection_Entry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StringTableSection_Entry {
        static mut instance: ::protobuf::lazy::Lazy<StringTableSection_Entry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StringTableSection_Entry,
        };
        unsafe {
            instance.get(StringTableSection_Entry::new)
        }
    }

    // optional uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.id
    }

    // optional string str = 2;

    pub fn clear_str(&mut self) {
        self.str.clear();
    }

    pub fn has_str(&self) -> bool {
        self.str.is_some()
    }

    // Param is passed by value, moved
    pub fn set_str(&mut self, v: ::std::string::String) {
        self.str = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_str(&mut self) -> &mut ::std::string::String {
        if self.str.is_none() {
            self.str.set_default();
        }
        self.str.as_mut().unwrap()
    }

    // Take field
    pub fn take_str(&mut self) -> ::std::string::String {
        self.str.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_str(&self) -> &str {
        match self.str.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_str_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.str
    }

    fn mut_str_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.str
    }
}

impl ::protobuf::Message for StringTableSection_Entry {
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
                    let tmp = is.read_uint32()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.str)?;
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.str.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.str.as_ref() {
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

impl ::protobuf::MessageStatic for StringTableSection_Entry {
    fn new() -> StringTableSection_Entry {
        StringTableSection_Entry::new()
    }

    fn descriptor_static(_: ::std::option::Option<StringTableSection_Entry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    StringTableSection_Entry::get_id_for_reflect,
                    StringTableSection_Entry::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "str",
                    StringTableSection_Entry::get_str_for_reflect,
                    StringTableSection_Entry::mut_str_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StringTableSection_Entry>(
                    "StringTableSection_Entry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StringTableSection_Entry {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_str();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StringTableSection_Entry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StringTableSection_Entry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SecretManagerSection {
    // message fields
    currentId: ::std::option::Option<u32>,
    tokenSequenceNumber: ::std::option::Option<u32>,
    numKeys: ::std::option::Option<u32>,
    numTokens: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SecretManagerSection {}

impl SecretManagerSection {
    pub fn new() -> SecretManagerSection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SecretManagerSection {
        static mut instance: ::protobuf::lazy::Lazy<SecretManagerSection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SecretManagerSection,
        };
        unsafe {
            instance.get(SecretManagerSection::new)
        }
    }

    // optional uint32 currentId = 1;

    pub fn clear_currentId(&mut self) {
        self.currentId = ::std::option::Option::None;
    }

    pub fn has_currentId(&self) -> bool {
        self.currentId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentId(&mut self, v: u32) {
        self.currentId = ::std::option::Option::Some(v);
    }

    pub fn get_currentId(&self) -> u32 {
        self.currentId.unwrap_or(0)
    }

    fn get_currentId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.currentId
    }

    fn mut_currentId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.currentId
    }

    // optional uint32 tokenSequenceNumber = 2;

    pub fn clear_tokenSequenceNumber(&mut self) {
        self.tokenSequenceNumber = ::std::option::Option::None;
    }

    pub fn has_tokenSequenceNumber(&self) -> bool {
        self.tokenSequenceNumber.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tokenSequenceNumber(&mut self, v: u32) {
        self.tokenSequenceNumber = ::std::option::Option::Some(v);
    }

    pub fn get_tokenSequenceNumber(&self) -> u32 {
        self.tokenSequenceNumber.unwrap_or(0)
    }

    fn get_tokenSequenceNumber_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tokenSequenceNumber
    }

    fn mut_tokenSequenceNumber_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tokenSequenceNumber
    }

    // optional uint32 numKeys = 3;

    pub fn clear_numKeys(&mut self) {
        self.numKeys = ::std::option::Option::None;
    }

    pub fn has_numKeys(&self) -> bool {
        self.numKeys.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numKeys(&mut self, v: u32) {
        self.numKeys = ::std::option::Option::Some(v);
    }

    pub fn get_numKeys(&self) -> u32 {
        self.numKeys.unwrap_or(0)
    }

    fn get_numKeys_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.numKeys
    }

    fn mut_numKeys_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.numKeys
    }

    // optional uint32 numTokens = 4;

    pub fn clear_numTokens(&mut self) {
        self.numTokens = ::std::option::Option::None;
    }

    pub fn has_numTokens(&self) -> bool {
        self.numTokens.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numTokens(&mut self, v: u32) {
        self.numTokens = ::std::option::Option::Some(v);
    }

    pub fn get_numTokens(&self) -> u32 {
        self.numTokens.unwrap_or(0)
    }

    fn get_numTokens_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.numTokens
    }

    fn mut_numTokens_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.numTokens
    }
}

impl ::protobuf::Message for SecretManagerSection {
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
                    let tmp = is.read_uint32()?;
                    self.currentId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.tokenSequenceNumber = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.numKeys = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.numTokens = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.currentId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.tokenSequenceNumber {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numKeys {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numTokens {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.currentId {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.tokenSequenceNumber {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.numKeys {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.numTokens {
            os.write_uint32(4, v)?;
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

impl ::protobuf::MessageStatic for SecretManagerSection {
    fn new() -> SecretManagerSection {
        SecretManagerSection::new()
    }

    fn descriptor_static(_: ::std::option::Option<SecretManagerSection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "currentId",
                    SecretManagerSection::get_currentId_for_reflect,
                    SecretManagerSection::mut_currentId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tokenSequenceNumber",
                    SecretManagerSection::get_tokenSequenceNumber_for_reflect,
                    SecretManagerSection::mut_tokenSequenceNumber_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "numKeys",
                    SecretManagerSection::get_numKeys_for_reflect,
                    SecretManagerSection::mut_numKeys_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "numTokens",
                    SecretManagerSection::get_numTokens_for_reflect,
                    SecretManagerSection::mut_numTokens_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SecretManagerSection>(
                    "SecretManagerSection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SecretManagerSection {
    fn clear(&mut self) {
        self.clear_currentId();
        self.clear_tokenSequenceNumber();
        self.clear_numKeys();
        self.clear_numTokens();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SecretManagerSection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SecretManagerSection {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SecretManagerSection_DelegationKey {
    // message fields
    id: ::std::option::Option<u32>,
    expiryDate: ::std::option::Option<u64>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SecretManagerSection_DelegationKey {}

impl SecretManagerSection_DelegationKey {
    pub fn new() -> SecretManagerSection_DelegationKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SecretManagerSection_DelegationKey {
        static mut instance: ::protobuf::lazy::Lazy<SecretManagerSection_DelegationKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SecretManagerSection_DelegationKey,
        };
        unsafe {
            instance.get(SecretManagerSection_DelegationKey::new)
        }
    }

    // optional uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.id
    }

    // optional uint64 expiryDate = 2;

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

    // optional bytes key = 3;

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
}

impl ::protobuf::Message for SecretManagerSection_DelegationKey {
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
                    let tmp = is.read_uint32()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.expiryDate = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.expiryDate {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.expiryDate {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.key.as_ref() {
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

impl ::protobuf::MessageStatic for SecretManagerSection_DelegationKey {
    fn new() -> SecretManagerSection_DelegationKey {
        SecretManagerSection_DelegationKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<SecretManagerSection_DelegationKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    SecretManagerSection_DelegationKey::get_id_for_reflect,
                    SecretManagerSection_DelegationKey::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "expiryDate",
                    SecretManagerSection_DelegationKey::get_expiryDate_for_reflect,
                    SecretManagerSection_DelegationKey::mut_expiryDate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    SecretManagerSection_DelegationKey::get_key_for_reflect,
                    SecretManagerSection_DelegationKey::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SecretManagerSection_DelegationKey>(
                    "SecretManagerSection_DelegationKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SecretManagerSection_DelegationKey {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_expiryDate();
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SecretManagerSection_DelegationKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SecretManagerSection_DelegationKey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SecretManagerSection_PersistToken {
    // message fields
    version: ::std::option::Option<u32>,
    owner: ::protobuf::SingularField<::std::string::String>,
    renewer: ::protobuf::SingularField<::std::string::String>,
    realUser: ::protobuf::SingularField<::std::string::String>,
    issueDate: ::std::option::Option<u64>,
    maxDate: ::std::option::Option<u64>,
    sequenceNumber: ::std::option::Option<u32>,
    masterKeyId: ::std::option::Option<u32>,
    expiryDate: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SecretManagerSection_PersistToken {}

impl SecretManagerSection_PersistToken {
    pub fn new() -> SecretManagerSection_PersistToken {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SecretManagerSection_PersistToken {
        static mut instance: ::protobuf::lazy::Lazy<SecretManagerSection_PersistToken> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SecretManagerSection_PersistToken,
        };
        unsafe {
            instance.get(SecretManagerSection_PersistToken::new)
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

    // optional string owner = 2;

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

    // optional string renewer = 3;

    pub fn clear_renewer(&mut self) {
        self.renewer.clear();
    }

    pub fn has_renewer(&self) -> bool {
        self.renewer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_renewer(&mut self, v: ::std::string::String) {
        self.renewer = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_renewer(&mut self) -> &mut ::std::string::String {
        if self.renewer.is_none() {
            self.renewer.set_default();
        }
        self.renewer.as_mut().unwrap()
    }

    // Take field
    pub fn take_renewer(&mut self) -> ::std::string::String {
        self.renewer.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_renewer(&self) -> &str {
        match self.renewer.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_renewer_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.renewer
    }

    fn mut_renewer_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.renewer
    }

    // optional string realUser = 4;

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

    // optional uint64 issueDate = 5;

    pub fn clear_issueDate(&mut self) {
        self.issueDate = ::std::option::Option::None;
    }

    pub fn has_issueDate(&self) -> bool {
        self.issueDate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_issueDate(&mut self, v: u64) {
        self.issueDate = ::std::option::Option::Some(v);
    }

    pub fn get_issueDate(&self) -> u64 {
        self.issueDate.unwrap_or(0)
    }

    fn get_issueDate_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.issueDate
    }

    fn mut_issueDate_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.issueDate
    }

    // optional uint64 maxDate = 6;

    pub fn clear_maxDate(&mut self) {
        self.maxDate = ::std::option::Option::None;
    }

    pub fn has_maxDate(&self) -> bool {
        self.maxDate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maxDate(&mut self, v: u64) {
        self.maxDate = ::std::option::Option::Some(v);
    }

    pub fn get_maxDate(&self) -> u64 {
        self.maxDate.unwrap_or(0)
    }

    fn get_maxDate_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.maxDate
    }

    fn mut_maxDate_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.maxDate
    }

    // optional uint32 sequenceNumber = 7;

    pub fn clear_sequenceNumber(&mut self) {
        self.sequenceNumber = ::std::option::Option::None;
    }

    pub fn has_sequenceNumber(&self) -> bool {
        self.sequenceNumber.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sequenceNumber(&mut self, v: u32) {
        self.sequenceNumber = ::std::option::Option::Some(v);
    }

    pub fn get_sequenceNumber(&self) -> u32 {
        self.sequenceNumber.unwrap_or(0)
    }

    fn get_sequenceNumber_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sequenceNumber
    }

    fn mut_sequenceNumber_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sequenceNumber
    }

    // optional uint32 masterKeyId = 8;

    pub fn clear_masterKeyId(&mut self) {
        self.masterKeyId = ::std::option::Option::None;
    }

    pub fn has_masterKeyId(&self) -> bool {
        self.masterKeyId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_masterKeyId(&mut self, v: u32) {
        self.masterKeyId = ::std::option::Option::Some(v);
    }

    pub fn get_masterKeyId(&self) -> u32 {
        self.masterKeyId.unwrap_or(0)
    }

    fn get_masterKeyId_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.masterKeyId
    }

    fn mut_masterKeyId_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.masterKeyId
    }

    // optional uint64 expiryDate = 9;

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
}

impl ::protobuf::Message for SecretManagerSection_PersistToken {
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
                    let tmp = is.read_uint32()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.owner)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.renewer)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.realUser)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.issueDate = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.maxDate = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.sequenceNumber = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.masterKeyId = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.expiryDate = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.owner.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.renewer.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.realUser.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(v) = self.issueDate {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.maxDate {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.sequenceNumber {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.masterKeyId {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.expiryDate {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.owner.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.renewer.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.realUser.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(v) = self.issueDate {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.maxDate {
            os.write_uint64(6, v)?;
        }
        if let Some(v) = self.sequenceNumber {
            os.write_uint32(7, v)?;
        }
        if let Some(v) = self.masterKeyId {
            os.write_uint32(8, v)?;
        }
        if let Some(v) = self.expiryDate {
            os.write_uint64(9, v)?;
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

impl ::protobuf::MessageStatic for SecretManagerSection_PersistToken {
    fn new() -> SecretManagerSection_PersistToken {
        SecretManagerSection_PersistToken::new()
    }

    fn descriptor_static(_: ::std::option::Option<SecretManagerSection_PersistToken>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version",
                    SecretManagerSection_PersistToken::get_version_for_reflect,
                    SecretManagerSection_PersistToken::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "owner",
                    SecretManagerSection_PersistToken::get_owner_for_reflect,
                    SecretManagerSection_PersistToken::mut_owner_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "renewer",
                    SecretManagerSection_PersistToken::get_renewer_for_reflect,
                    SecretManagerSection_PersistToken::mut_renewer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "realUser",
                    SecretManagerSection_PersistToken::get_realUser_for_reflect,
                    SecretManagerSection_PersistToken::mut_realUser_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "issueDate",
                    SecretManagerSection_PersistToken::get_issueDate_for_reflect,
                    SecretManagerSection_PersistToken::mut_issueDate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "maxDate",
                    SecretManagerSection_PersistToken::get_maxDate_for_reflect,
                    SecretManagerSection_PersistToken::mut_maxDate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sequenceNumber",
                    SecretManagerSection_PersistToken::get_sequenceNumber_for_reflect,
                    SecretManagerSection_PersistToken::mut_sequenceNumber_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "masterKeyId",
                    SecretManagerSection_PersistToken::get_masterKeyId_for_reflect,
                    SecretManagerSection_PersistToken::mut_masterKeyId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "expiryDate",
                    SecretManagerSection_PersistToken::get_expiryDate_for_reflect,
                    SecretManagerSection_PersistToken::mut_expiryDate_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SecretManagerSection_PersistToken>(
                    "SecretManagerSection_PersistToken",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SecretManagerSection_PersistToken {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_owner();
        self.clear_renewer();
        self.clear_realUser();
        self.clear_issueDate();
        self.clear_maxDate();
        self.clear_sequenceNumber();
        self.clear_masterKeyId();
        self.clear_expiryDate();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SecretManagerSection_PersistToken {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SecretManagerSection_PersistToken {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CacheManagerSection {
    // message fields
    nextDirectiveId: ::std::option::Option<u64>,
    numPools: ::std::option::Option<u32>,
    numDirectives: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CacheManagerSection {}

impl CacheManagerSection {
    pub fn new() -> CacheManagerSection {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CacheManagerSection {
        static mut instance: ::protobuf::lazy::Lazy<CacheManagerSection> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CacheManagerSection,
        };
        unsafe {
            instance.get(CacheManagerSection::new)
        }
    }

    // required uint64 nextDirectiveId = 1;

    pub fn clear_nextDirectiveId(&mut self) {
        self.nextDirectiveId = ::std::option::Option::None;
    }

    pub fn has_nextDirectiveId(&self) -> bool {
        self.nextDirectiveId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nextDirectiveId(&mut self, v: u64) {
        self.nextDirectiveId = ::std::option::Option::Some(v);
    }

    pub fn get_nextDirectiveId(&self) -> u64 {
        self.nextDirectiveId.unwrap_or(0)
    }

    fn get_nextDirectiveId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.nextDirectiveId
    }

    fn mut_nextDirectiveId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.nextDirectiveId
    }

    // required uint32 numPools = 2;

    pub fn clear_numPools(&mut self) {
        self.numPools = ::std::option::Option::None;
    }

    pub fn has_numPools(&self) -> bool {
        self.numPools.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numPools(&mut self, v: u32) {
        self.numPools = ::std::option::Option::Some(v);
    }

    pub fn get_numPools(&self) -> u32 {
        self.numPools.unwrap_or(0)
    }

    fn get_numPools_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.numPools
    }

    fn mut_numPools_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.numPools
    }

    // required uint32 numDirectives = 3;

    pub fn clear_numDirectives(&mut self) {
        self.numDirectives = ::std::option::Option::None;
    }

    pub fn has_numDirectives(&self) -> bool {
        self.numDirectives.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numDirectives(&mut self, v: u32) {
        self.numDirectives = ::std::option::Option::Some(v);
    }

    pub fn get_numDirectives(&self) -> u32 {
        self.numDirectives.unwrap_or(0)
    }

    fn get_numDirectives_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.numDirectives
    }

    fn mut_numDirectives_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.numDirectives
    }
}

impl ::protobuf::Message for CacheManagerSection {
    fn is_initialized(&self) -> bool {
        if self.nextDirectiveId.is_none() {
            return false;
        }
        if self.numPools.is_none() {
            return false;
        }
        if self.numDirectives.is_none() {
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
                    self.nextDirectiveId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.numPools = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.numDirectives = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.nextDirectiveId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numPools {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numDirectives {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.nextDirectiveId {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.numPools {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.numDirectives {
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

impl ::protobuf::MessageStatic for CacheManagerSection {
    fn new() -> CacheManagerSection {
        CacheManagerSection::new()
    }

    fn descriptor_static(_: ::std::option::Option<CacheManagerSection>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "nextDirectiveId",
                    CacheManagerSection::get_nextDirectiveId_for_reflect,
                    CacheManagerSection::mut_nextDirectiveId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "numPools",
                    CacheManagerSection::get_numPools_for_reflect,
                    CacheManagerSection::mut_numPools_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "numDirectives",
                    CacheManagerSection::get_numDirectives_for_reflect,
                    CacheManagerSection::mut_numDirectives_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CacheManagerSection>(
                    "CacheManagerSection",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CacheManagerSection {
    fn clear(&mut self) {
        self.clear_nextDirectiveId();
        self.clear_numPools();
        self.clear_numDirectives();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CacheManagerSection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CacheManagerSection {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rfsimage.proto\x12\x13hadoop.hdfs.fsimage\x1a\nhdfs.proto\x1a\tacl.pr\
    oto\x1a\x0bxattr.proto\"\xbf\x01\n\x0bFileSummary\x12\x15\n\rondiskVersi\
    on\x18\x01\x20\x02(\r\x12\x15\n\rlayoutVersion\x18\x02\x20\x02(\r\x12\r\
    \n\x05codec\x18\x03\x20\x01(\t\x12:\n\x08sections\x18\x04\x20\x03(\x0b2(\
    .hadoop.hdfs.fsimage.FileSummary.Section\x1a7\n\x07Section\x12\x0c\n\x04\
    name\x18\x01\x20\x01(\t\x12\x0e\n\x06length\x18\x02\x20\x01(\x04\x12\x0e\
    \n\x06offset\x18\x03\x20\x01(\x04\"\xbf\x01\n\x11NameSystemSection\x12\
    \x13\n\x0bnamespaceId\x18\x01\x20\x01(\r\x12\x12\n\ngenstampV1\x18\x02\
    \x20\x01(\x04\x12\x12\n\ngenstampV2\x18\x03\x20\x01(\x04\x12\x17\n\x0fge\
    nstampV1Limit\x18\x04\x20\x01(\x04\x12\x1c\n\x14lastAllocatedBlockId\x18\
    \x05\x20\x01(\x04\x12\x15\n\rtransactionId\x18\x06\x20\x01(\x04\x12\x1f\
    \n\x17rollingUpgradeStartTime\x18\x07\x20\x01(\x04\"\xe6\n\n\x0cINodeSec\
    tion\x12\x13\n\x0blastInodeId\x18\x01\x20\x01(\x04\x12\x11\n\tnumInodes\
    \x18\x02\x20\x01(\x04\x1aI\n\x1cFileUnderConstructionFeature\x12\x12\n\n\
    clientName\x18\x01\x20\x01(\t\x12\x15\n\rclientMachine\x18\x02\x20\x01(\
    \t\x1a&\n\x0fAclFeatureProto\x12\x13\n\x07entries\x18\x02\x20\x03(\x07B\
    \x02\x10\x01\x1a0\n\x11XAttrCompactProto\x12\x0c\n\x04name\x18\x01\x20\
    \x02(\x07\x12\r\n\x05value\x18\x02\x20\x01(\x0c\x1aX\n\x11XAttrFeaturePr\
    oto\x12C\n\x06xAttrs\x18\x01\x20\x03(\x0b23.hadoop.hdfs.fsimage.INodeSec\
    tion.XAttrCompactProto\x1a\x95\x03\n\tINodeFile\x12\x13\n\x0breplication\
    \x18\x01\x20\x01(\r\x12\x18\n\x10modificationTime\x18\x02\x20\x01(\x04\
    \x12\x12\n\naccessTime\x18\x03\x20\x01(\x04\x12\x1a\n\x12preferredBlockS\
    ize\x18\x04\x20\x01(\x04\x12\x12\n\npermission\x18\x05\x20\x01(\x06\x12'\
    \n\x06blocks\x18\x06\x20\x03(\x0b2\x17.hadoop.hdfs.BlockProto\x12N\n\x06\
    fileUC\x18\x07\x20\x01(\x0b2>.hadoop.hdfs.fsimage.INodeSection.FileUnder\
    ConstructionFeature\x12>\n\x03acl\x18\x08\x20\x01(\x0b21.hadoop.hdfs.fsi\
    mage.INodeSection.AclFeatureProto\x12C\n\x06xAttrs\x18\t\x20\x01(\x0b23.\
    hadoop.hdfs.fsimage.INodeSection.XAttrFeatureProto\x12\x17\n\x0fstorageP\
    olicyID\x18\n\x20\x01(\r\x1a\xe5\x01\n\x0eINodeDirectory\x12\x18\n\x10mo\
    dificationTime\x18\x01\x20\x01(\x04\x12\x0f\n\x07nsQuota\x18\x02\x20\x01\
    (\x04\x12\x0f\n\x07dsQuota\x18\x03\x20\x01(\x04\x12\x12\n\npermission\
    \x18\x04\x20\x01(\x06\x12>\n\x03acl\x18\x05\x20\x01(\x0b21.hadoop.hdfs.f\
    simage.INodeSection.AclFeatureProto\x12C\n\x06xAttrs\x18\x06\x20\x01(\
    \x0b23.hadoop.hdfs.fsimage.INodeSection.XAttrFeatureProto\x1a`\n\x0cINod\
    eSymlink\x12\x12\n\npermission\x18\x01\x20\x01(\x06\x12\x0e\n\x06target\
    \x18\x02\x20\x01(\x0c\x12\x18\n\x10modificationTime\x18\x03\x20\x01(\x04\
    \x12\x12\n\naccessTime\x18\x04\x20\x01(\x04\x1a\xcc\x02\n\x05INode\x12:\
    \n\x04type\x18\x01\x20\x02(\x0e2,.hadoop.hdfs.fsimage.INodeSection.INode\
    .Type\x12\n\n\x02id\x18\x02\x20\x02(\x04\x12\x0c\n\x04name\x18\x03\x20\
    \x01(\x0c\x129\n\x04file\x18\x04\x20\x01(\x0b2+.hadoop.hdfs.fsimage.INod\
    eSection.INodeFile\x12C\n\tdirectory\x18\x05\x20\x01(\x0b20.hadoop.hdfs.\
    fsimage.INodeSection.INodeDirectory\x12?\n\x07symlink\x18\x06\x20\x01(\
    \x0b2..hadoop.hdfs.fsimage.INodeSection.INodeSymlink\",\n\x04Type\x12\
    \x08\n\x04FILE\x10\x01\x12\r\n\tDIRECTORY\x10\x02\x12\x0b\n\x07SYMLINK\
    \x10\x03\"`\n\x1dFilesUnderConstructionSection\x1a?\n\x1aFileUnderConstr\
    uctionEntry\x12\x0f\n\x07inodeId\x18\x01\x20\x01(\x04\x12\x10\n\x08fullP\
    ath\x18\x02\x20\x01(\t\"b\n\x15INodeDirectorySection\x1aI\n\x08DirEntry\
    \x12\x0e\n\x06parent\x18\x01\x20\x01(\x04\x12\x14\n\x08children\x18\x02\
    \x20\x03(\x04B\x02\x10\x01\x12\x17\n\x0brefChildren\x18\x03\x20\x03(\rB\
    \x02\x10\x01\"z\n\x15INodeReferenceSection\x1aa\n\x0eINodeReference\x12\
    \x12\n\nreferredId\x18\x01\x20\x01(\x04\x12\x0c\n\x04name\x18\x02\x20\
    \x01(\x0c\x12\x15\n\rdstSnapshotId\x18\x03\x20\x01(\r\x12\x16\n\x0elastS\
    napshotId\x18\x04\x20\x01(\r\"\xb5\x01\n\x0fSnapshotSection\x12\x17\n\
    \x0fsnapshotCounter\x18\x01\x20\x01(\r\x12\x1c\n\x10snapshottableDir\x18\
    \x02\x20\x03(\x04B\x02\x10\x01\x12\x14\n\x0cnumSnapshots\x18\x03\x20\x01\
    (\r\x1aU\n\x08Snapshot\x12\x12\n\nsnapshotId\x18\x01\x20\x01(\r\x125\n\
    \x04root\x18\x02\x20\x01(\x0b2'.hadoop.hdfs.fsimage.INodeSection.INode\"\
    \xd7\x04\n\x13SnapshotDiffSection\x1a\x20\n\x10CreatedListEntry\x12\x0c\
    \n\x04name\x18\x01\x20\x01(\x0c\x1a\xf7\x01\n\rDirectoryDiff\x12\x12\n\n\
    snapshotId\x18\x01\x20\x01(\r\x12\x14\n\x0cchildrenSize\x18\x02\x20\x01(\
    \r\x12\x16\n\x0eisSnapshotRoot\x18\x03\x20\x01(\x08\x12\x0c\n\x04name\
    \x18\x04\x20\x01(\x0c\x12F\n\x0csnapshotCopy\x18\x05\x20\x01(\x0b20.hado\
    op.hdfs.fsimage.INodeSection.INodeDirectory\x12\x17\n\x0fcreatedListSize\
    \x18\x06\x20\x01(\r\x12\x18\n\x0cdeletedINode\x18\x07\x20\x03(\x04B\x02\
    \x10\x01\x12\x1b\n\x0fdeletedINodeRef\x18\x08\x20\x03(\rB\x02\x10\x01\
    \x1a\x81\x01\n\x08FileDiff\x12\x12\n\nsnapshotId\x18\x01\x20\x01(\r\x12\
    \x10\n\x08fileSize\x18\x02\x20\x01(\x04\x12\x0c\n\x04name\x18\x03\x20\
    \x01(\x0c\x12A\n\x0csnapshotCopy\x18\x04\x20\x01(\x0b2+.hadoop.hdfs.fsim\
    age.INodeSection.INodeFile\x1a\x9f\x01\n\tDiffEntry\x12E\n\x04type\x18\
    \x01\x20\x02(\x0e27.hadoop.hdfs.fsimage.SnapshotDiffSection.DiffEntry.Ty\
    pe\x12\x0f\n\x07inodeId\x18\x02\x20\x01(\x04\x12\x11\n\tnumOfDiff\x18\
    \x03\x20\x01(\r\"'\n\x04Type\x12\x0c\n\x08FILEDIFF\x10\x01\x12\x11\n\rDI\
    RECTORYDIFF\x10\x02\"H\n\x12StringTableSection\x12\x10\n\x08numEntry\x18\
    \x01\x20\x01(\r\x1a\x20\n\x05Entry\x12\n\n\x02id\x18\x01\x20\x01(\r\x12\
    \x0b\n\x03str\x18\x02\x20\x01(\t\"\xe1\x02\n\x14SecretManagerSection\x12\
    \x11\n\tcurrentId\x18\x01\x20\x01(\r\x12\x1b\n\x13tokenSequenceNumber\
    \x18\x02\x20\x01(\r\x12\x0f\n\x07numKeys\x18\x03\x20\x01(\r\x12\x11\n\tn\
    umTokens\x18\x04\x20\x01(\r\x1a<\n\rDelegationKey\x12\n\n\x02id\x18\x01\
    \x20\x01(\r\x12\x12\n\nexpiryDate\x18\x02\x20\x01(\x04\x12\x0b\n\x03key\
    \x18\x03\x20\x01(\x0c\x1a\xb6\x01\n\x0cPersistToken\x12\x0f\n\x07version\
    \x18\x01\x20\x01(\r\x12\r\n\x05owner\x18\x02\x20\x01(\t\x12\x0f\n\x07ren\
    ewer\x18\x03\x20\x01(\t\x12\x10\n\x08realUser\x18\x04\x20\x01(\t\x12\x11\
    \n\tissueDate\x18\x05\x20\x01(\x04\x12\x0f\n\x07maxDate\x18\x06\x20\x01(\
    \x04\x12\x16\n\x0esequenceNumber\x18\x07\x20\x01(\r\x12\x13\n\x0bmasterK\
    eyId\x18\x08\x20\x01(\r\x12\x12\n\nexpiryDate\x18\t\x20\x01(\x04\"W\n\
    \x13CacheManagerSection\x12\x17\n\x0fnextDirectiveId\x18\x01\x20\x02(\
    \x04\x12\x10\n\x08numPools\x18\x02\x20\x02(\r\x12\x15\n\rnumDirectives\
    \x18\x03\x20\x02(\rB6\n&org.apache.hadoop.hdfs.server.namenodeB\x0cFsIma\
    geProto\
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
