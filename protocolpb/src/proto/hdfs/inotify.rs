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
pub struct EventProto {
    // message fields
    field_type: ::std::option::Option<EventType>,
    contents: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EventProto {}

impl EventProto {
    pub fn new() -> EventProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EventProto {
        static mut instance: ::protobuf::lazy::Lazy<EventProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EventProto,
        };
        unsafe {
            instance.get(EventProto::new)
        }
    }

    // required .hadoop.hdfs.EventType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: EventType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> EventType {
        self.field_type.unwrap_or(EventType::EVENT_CREATE)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<EventType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<EventType> {
        &mut self.field_type
    }

    // required bytes contents = 2;

    pub fn clear_contents(&mut self) {
        self.contents.clear();
    }

    pub fn has_contents(&self) -> bool {
        self.contents.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contents(&mut self, v: ::std::vec::Vec<u8>) {
        self.contents = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contents(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.contents.is_none() {
            self.contents.set_default();
        }
        self.contents.as_mut().unwrap()
    }

    // Take field
    pub fn take_contents(&mut self) -> ::std::vec::Vec<u8> {
        self.contents.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_contents(&self) -> &[u8] {
        match self.contents.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_contents_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.contents
    }

    fn mut_contents_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.contents
    }
}

impl ::protobuf::Message for EventProto {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        }
        if self.contents.is_none() {
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
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.contents)?;
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
        if let Some(ref v) = self.contents.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.contents.as_ref() {
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

impl ::protobuf::MessageStatic for EventProto {
    fn new() -> EventProto {
        EventProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<EventProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<EventType>>(
                    "type",
                    EventProto::get_field_type_for_reflect,
                    EventProto::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "contents",
                    EventProto::get_contents_for_reflect,
                    EventProto::mut_contents_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EventProto>(
                    "EventProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EventProto {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_contents();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EventProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EventProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CreateEventProto {
    // message fields
    field_type: ::std::option::Option<INodeType>,
    path: ::protobuf::SingularField<::std::string::String>,
    ctime: ::std::option::Option<i64>,
    ownerName: ::protobuf::SingularField<::std::string::String>,
    groupName: ::protobuf::SingularField<::std::string::String>,
    perms: ::protobuf::SingularPtrField<super::hdfs::FsPermissionProto>,
    replication: ::std::option::Option<i32>,
    symlinkTarget: ::protobuf::SingularField<::std::string::String>,
    overwrite: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateEventProto {}

impl CreateEventProto {
    pub fn new() -> CreateEventProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateEventProto {
        static mut instance: ::protobuf::lazy::Lazy<CreateEventProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateEventProto,
        };
        unsafe {
            instance.get(CreateEventProto::new)
        }
    }

    // required .hadoop.hdfs.INodeType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: INodeType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> INodeType {
        self.field_type.unwrap_or(INodeType::I_TYPE_FILE)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<INodeType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<INodeType> {
        &mut self.field_type
    }

    // required string path = 2;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        }
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }

    // required int64 ctime = 3;

    pub fn clear_ctime(&mut self) {
        self.ctime = ::std::option::Option::None;
    }

    pub fn has_ctime(&self) -> bool {
        self.ctime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ctime(&mut self, v: i64) {
        self.ctime = ::std::option::Option::Some(v);
    }

    pub fn get_ctime(&self) -> i64 {
        self.ctime.unwrap_or(0)
    }

    fn get_ctime_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.ctime
    }

    fn mut_ctime_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.ctime
    }

    // required string ownerName = 4;

    pub fn clear_ownerName(&mut self) {
        self.ownerName.clear();
    }

    pub fn has_ownerName(&self) -> bool {
        self.ownerName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ownerName(&mut self, v: ::std::string::String) {
        self.ownerName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ownerName(&mut self) -> &mut ::std::string::String {
        if self.ownerName.is_none() {
            self.ownerName.set_default();
        }
        self.ownerName.as_mut().unwrap()
    }

    // Take field
    pub fn take_ownerName(&mut self) -> ::std::string::String {
        self.ownerName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ownerName(&self) -> &str {
        match self.ownerName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ownerName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ownerName
    }

    fn mut_ownerName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ownerName
    }

    // required string groupName = 5;

    pub fn clear_groupName(&mut self) {
        self.groupName.clear();
    }

    pub fn has_groupName(&self) -> bool {
        self.groupName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_groupName(&mut self, v: ::std::string::String) {
        self.groupName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_groupName(&mut self) -> &mut ::std::string::String {
        if self.groupName.is_none() {
            self.groupName.set_default();
        }
        self.groupName.as_mut().unwrap()
    }

    // Take field
    pub fn take_groupName(&mut self) -> ::std::string::String {
        self.groupName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_groupName(&self) -> &str {
        match self.groupName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_groupName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.groupName
    }

    fn mut_groupName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.groupName
    }

    // required .hadoop.hdfs.FsPermissionProto perms = 6;

    pub fn clear_perms(&mut self) {
        self.perms.clear();
    }

    pub fn has_perms(&self) -> bool {
        self.perms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_perms(&mut self, v: super::hdfs::FsPermissionProto) {
        self.perms = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_perms(&mut self) -> &mut super::hdfs::FsPermissionProto {
        if self.perms.is_none() {
            self.perms.set_default();
        }
        self.perms.as_mut().unwrap()
    }

    // Take field
    pub fn take_perms(&mut self) -> super::hdfs::FsPermissionProto {
        self.perms.take().unwrap_or_else(|| super::hdfs::FsPermissionProto::new())
    }

    pub fn get_perms(&self) -> &super::hdfs::FsPermissionProto {
        self.perms.as_ref().unwrap_or_else(|| super::hdfs::FsPermissionProto::default_instance())
    }

    fn get_perms_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::FsPermissionProto> {
        &self.perms
    }

    fn mut_perms_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::FsPermissionProto> {
        &mut self.perms
    }

    // optional int32 replication = 7;

    pub fn clear_replication(&mut self) {
        self.replication = ::std::option::Option::None;
    }

    pub fn has_replication(&self) -> bool {
        self.replication.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replication(&mut self, v: i32) {
        self.replication = ::std::option::Option::Some(v);
    }

    pub fn get_replication(&self) -> i32 {
        self.replication.unwrap_or(0)
    }

    fn get_replication_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.replication
    }

    fn mut_replication_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.replication
    }

    // optional string symlinkTarget = 8;

    pub fn clear_symlinkTarget(&mut self) {
        self.symlinkTarget.clear();
    }

    pub fn has_symlinkTarget(&self) -> bool {
        self.symlinkTarget.is_some()
    }

    // Param is passed by value, moved
    pub fn set_symlinkTarget(&mut self, v: ::std::string::String) {
        self.symlinkTarget = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_symlinkTarget(&mut self) -> &mut ::std::string::String {
        if self.symlinkTarget.is_none() {
            self.symlinkTarget.set_default();
        }
        self.symlinkTarget.as_mut().unwrap()
    }

    // Take field
    pub fn take_symlinkTarget(&mut self) -> ::std::string::String {
        self.symlinkTarget.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_symlinkTarget(&self) -> &str {
        match self.symlinkTarget.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_symlinkTarget_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.symlinkTarget
    }

    fn mut_symlinkTarget_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.symlinkTarget
    }

    // optional bool overwrite = 9;

    pub fn clear_overwrite(&mut self) {
        self.overwrite = ::std::option::Option::None;
    }

    pub fn has_overwrite(&self) -> bool {
        self.overwrite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_overwrite(&mut self, v: bool) {
        self.overwrite = ::std::option::Option::Some(v);
    }

    pub fn get_overwrite(&self) -> bool {
        self.overwrite.unwrap_or(false)
    }

    fn get_overwrite_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.overwrite
    }

    fn mut_overwrite_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.overwrite
    }
}

impl ::protobuf::Message for CreateEventProto {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        }
        if self.path.is_none() {
            return false;
        }
        if self.ctime.is_none() {
            return false;
        }
        if self.ownerName.is_none() {
            return false;
        }
        if self.groupName.is_none() {
            return false;
        }
        if self.perms.is_none() {
            return false;
        }
        for v in &self.perms {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.ctime = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ownerName)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.groupName)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.perms)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.replication = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.symlinkTarget)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.overwrite = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.ctime {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.ownerName.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.groupName.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.perms.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.replication {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.symlinkTarget.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        if let Some(v) = self.overwrite {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.path.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.ctime {
            os.write_int64(3, v)?;
        }
        if let Some(ref v) = self.ownerName.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.groupName.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.perms.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.replication {
            os.write_int32(7, v)?;
        }
        if let Some(ref v) = self.symlinkTarget.as_ref() {
            os.write_string(8, &v)?;
        }
        if let Some(v) = self.overwrite {
            os.write_bool(9, v)?;
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

impl ::protobuf::MessageStatic for CreateEventProto {
    fn new() -> CreateEventProto {
        CreateEventProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateEventProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<INodeType>>(
                    "type",
                    CreateEventProto::get_field_type_for_reflect,
                    CreateEventProto::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    CreateEventProto::get_path_for_reflect,
                    CreateEventProto::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "ctime",
                    CreateEventProto::get_ctime_for_reflect,
                    CreateEventProto::mut_ctime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ownerName",
                    CreateEventProto::get_ownerName_for_reflect,
                    CreateEventProto::mut_ownerName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "groupName",
                    CreateEventProto::get_groupName_for_reflect,
                    CreateEventProto::mut_groupName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::FsPermissionProto>>(
                    "perms",
                    CreateEventProto::get_perms_for_reflect,
                    CreateEventProto::mut_perms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "replication",
                    CreateEventProto::get_replication_for_reflect,
                    CreateEventProto::mut_replication_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "symlinkTarget",
                    CreateEventProto::get_symlinkTarget_for_reflect,
                    CreateEventProto::mut_symlinkTarget_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "overwrite",
                    CreateEventProto::get_overwrite_for_reflect,
                    CreateEventProto::mut_overwrite_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateEventProto>(
                    "CreateEventProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateEventProto {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_path();
        self.clear_ctime();
        self.clear_ownerName();
        self.clear_groupName();
        self.clear_perms();
        self.clear_replication();
        self.clear_symlinkTarget();
        self.clear_overwrite();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateEventProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateEventProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CloseEventProto {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    fileSize: ::std::option::Option<i64>,
    timestamp: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CloseEventProto {}

impl CloseEventProto {
    pub fn new() -> CloseEventProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CloseEventProto {
        static mut instance: ::protobuf::lazy::Lazy<CloseEventProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CloseEventProto,
        };
        unsafe {
            instance.get(CloseEventProto::new)
        }
    }

    // required string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        }
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }

    // required int64 fileSize = 2;

    pub fn clear_fileSize(&mut self) {
        self.fileSize = ::std::option::Option::None;
    }

    pub fn has_fileSize(&self) -> bool {
        self.fileSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileSize(&mut self, v: i64) {
        self.fileSize = ::std::option::Option::Some(v);
    }

    pub fn get_fileSize(&self) -> i64 {
        self.fileSize.unwrap_or(0)
    }

    fn get_fileSize_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.fileSize
    }

    fn mut_fileSize_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.fileSize
    }

    // required int64 timestamp = 3;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: i64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp.unwrap_or(0)
    }

    fn get_timestamp_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.timestamp
    }
}

impl ::protobuf::Message for CloseEventProto {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        }
        if self.fileSize.is_none() {
            return false;
        }
        if self.timestamp.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.fileSize = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.fileSize {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.fileSize {
            os.write_int64(2, v)?;
        }
        if let Some(v) = self.timestamp {
            os.write_int64(3, v)?;
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

impl ::protobuf::MessageStatic for CloseEventProto {
    fn new() -> CloseEventProto {
        CloseEventProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CloseEventProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    CloseEventProto::get_path_for_reflect,
                    CloseEventProto::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "fileSize",
                    CloseEventProto::get_fileSize_for_reflect,
                    CloseEventProto::mut_fileSize_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "timestamp",
                    CloseEventProto::get_timestamp_for_reflect,
                    CloseEventProto::mut_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CloseEventProto>(
                    "CloseEventProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CloseEventProto {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_fileSize();
        self.clear_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CloseEventProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CloseEventProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AppendEventProto {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AppendEventProto {}

impl AppendEventProto {
    pub fn new() -> AppendEventProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AppendEventProto {
        static mut instance: ::protobuf::lazy::Lazy<AppendEventProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AppendEventProto,
        };
        unsafe {
            instance.get(AppendEventProto::new)
        }
    }

    // required string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        }
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }
}

impl ::protobuf::Message for AppendEventProto {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
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
        if let Some(ref v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.path.as_ref() {
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

impl ::protobuf::MessageStatic for AppendEventProto {
    fn new() -> AppendEventProto {
        AppendEventProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<AppendEventProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    AppendEventProto::get_path_for_reflect,
                    AppendEventProto::mut_path_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AppendEventProto>(
                    "AppendEventProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AppendEventProto {
    fn clear(&mut self) {
        self.clear_path();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AppendEventProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AppendEventProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RenameEventProto {
    // message fields
    srcPath: ::protobuf::SingularField<::std::string::String>,
    destPath: ::protobuf::SingularField<::std::string::String>,
    timestamp: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RenameEventProto {}

impl RenameEventProto {
    pub fn new() -> RenameEventProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RenameEventProto {
        static mut instance: ::protobuf::lazy::Lazy<RenameEventProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RenameEventProto,
        };
        unsafe {
            instance.get(RenameEventProto::new)
        }
    }

    // required string srcPath = 1;

    pub fn clear_srcPath(&mut self) {
        self.srcPath.clear();
    }

    pub fn has_srcPath(&self) -> bool {
        self.srcPath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_srcPath(&mut self, v: ::std::string::String) {
        self.srcPath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_srcPath(&mut self) -> &mut ::std::string::String {
        if self.srcPath.is_none() {
            self.srcPath.set_default();
        }
        self.srcPath.as_mut().unwrap()
    }

    // Take field
    pub fn take_srcPath(&mut self) -> ::std::string::String {
        self.srcPath.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_srcPath(&self) -> &str {
        match self.srcPath.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_srcPath_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.srcPath
    }

    fn mut_srcPath_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.srcPath
    }

    // required string destPath = 2;

    pub fn clear_destPath(&mut self) {
        self.destPath.clear();
    }

    pub fn has_destPath(&self) -> bool {
        self.destPath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_destPath(&mut self, v: ::std::string::String) {
        self.destPath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_destPath(&mut self) -> &mut ::std::string::String {
        if self.destPath.is_none() {
            self.destPath.set_default();
        }
        self.destPath.as_mut().unwrap()
    }

    // Take field
    pub fn take_destPath(&mut self) -> ::std::string::String {
        self.destPath.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_destPath(&self) -> &str {
        match self.destPath.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_destPath_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.destPath
    }

    fn mut_destPath_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.destPath
    }

    // required int64 timestamp = 3;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: i64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp.unwrap_or(0)
    }

    fn get_timestamp_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.timestamp
    }
}

impl ::protobuf::Message for RenameEventProto {
    fn is_initialized(&self) -> bool {
        if self.srcPath.is_none() {
            return false;
        }
        if self.destPath.is_none() {
            return false;
        }
        if self.timestamp.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.srcPath)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.destPath)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.srcPath.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.destPath.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.srcPath.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.destPath.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.timestamp {
            os.write_int64(3, v)?;
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

impl ::protobuf::MessageStatic for RenameEventProto {
    fn new() -> RenameEventProto {
        RenameEventProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RenameEventProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "srcPath",
                    RenameEventProto::get_srcPath_for_reflect,
                    RenameEventProto::mut_srcPath_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "destPath",
                    RenameEventProto::get_destPath_for_reflect,
                    RenameEventProto::mut_destPath_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "timestamp",
                    RenameEventProto::get_timestamp_for_reflect,
                    RenameEventProto::mut_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RenameEventProto>(
                    "RenameEventProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RenameEventProto {
    fn clear(&mut self) {
        self.clear_srcPath();
        self.clear_destPath();
        self.clear_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RenameEventProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RenameEventProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MetadataUpdateEventProto {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    field_type: ::std::option::Option<MetadataUpdateType>,
    mtime: ::std::option::Option<i64>,
    atime: ::std::option::Option<i64>,
    replication: ::std::option::Option<i32>,
    ownerName: ::protobuf::SingularField<::std::string::String>,
    groupName: ::protobuf::SingularField<::std::string::String>,
    perms: ::protobuf::SingularPtrField<super::hdfs::FsPermissionProto>,
    acls: ::protobuf::RepeatedField<super::acl::AclEntryProto>,
    xAttrs: ::protobuf::RepeatedField<super::xattr::XAttrProto>,
    xAttrsRemoved: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MetadataUpdateEventProto {}

impl MetadataUpdateEventProto {
    pub fn new() -> MetadataUpdateEventProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MetadataUpdateEventProto {
        static mut instance: ::protobuf::lazy::Lazy<MetadataUpdateEventProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MetadataUpdateEventProto,
        };
        unsafe {
            instance.get(MetadataUpdateEventProto::new)
        }
    }

    // required string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        }
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }

    // required .hadoop.hdfs.MetadataUpdateType type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: MetadataUpdateType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> MetadataUpdateType {
        self.field_type.unwrap_or(MetadataUpdateType::META_TYPE_TIMES)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<MetadataUpdateType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<MetadataUpdateType> {
        &mut self.field_type
    }

    // optional int64 mtime = 3;

    pub fn clear_mtime(&mut self) {
        self.mtime = ::std::option::Option::None;
    }

    pub fn has_mtime(&self) -> bool {
        self.mtime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mtime(&mut self, v: i64) {
        self.mtime = ::std::option::Option::Some(v);
    }

    pub fn get_mtime(&self) -> i64 {
        self.mtime.unwrap_or(0)
    }

    fn get_mtime_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.mtime
    }

    fn mut_mtime_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.mtime
    }

    // optional int64 atime = 4;

    pub fn clear_atime(&mut self) {
        self.atime = ::std::option::Option::None;
    }

    pub fn has_atime(&self) -> bool {
        self.atime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_atime(&mut self, v: i64) {
        self.atime = ::std::option::Option::Some(v);
    }

    pub fn get_atime(&self) -> i64 {
        self.atime.unwrap_or(0)
    }

    fn get_atime_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.atime
    }

    fn mut_atime_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.atime
    }

    // optional int32 replication = 5;

    pub fn clear_replication(&mut self) {
        self.replication = ::std::option::Option::None;
    }

    pub fn has_replication(&self) -> bool {
        self.replication.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replication(&mut self, v: i32) {
        self.replication = ::std::option::Option::Some(v);
    }

    pub fn get_replication(&self) -> i32 {
        self.replication.unwrap_or(0)
    }

    fn get_replication_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.replication
    }

    fn mut_replication_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.replication
    }

    // optional string ownerName = 6;

    pub fn clear_ownerName(&mut self) {
        self.ownerName.clear();
    }

    pub fn has_ownerName(&self) -> bool {
        self.ownerName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ownerName(&mut self, v: ::std::string::String) {
        self.ownerName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ownerName(&mut self) -> &mut ::std::string::String {
        if self.ownerName.is_none() {
            self.ownerName.set_default();
        }
        self.ownerName.as_mut().unwrap()
    }

    // Take field
    pub fn take_ownerName(&mut self) -> ::std::string::String {
        self.ownerName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ownerName(&self) -> &str {
        match self.ownerName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_ownerName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.ownerName
    }

    fn mut_ownerName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.ownerName
    }

    // optional string groupName = 7;

    pub fn clear_groupName(&mut self) {
        self.groupName.clear();
    }

    pub fn has_groupName(&self) -> bool {
        self.groupName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_groupName(&mut self, v: ::std::string::String) {
        self.groupName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_groupName(&mut self) -> &mut ::std::string::String {
        if self.groupName.is_none() {
            self.groupName.set_default();
        }
        self.groupName.as_mut().unwrap()
    }

    // Take field
    pub fn take_groupName(&mut self) -> ::std::string::String {
        self.groupName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_groupName(&self) -> &str {
        match self.groupName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_groupName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.groupName
    }

    fn mut_groupName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.groupName
    }

    // optional .hadoop.hdfs.FsPermissionProto perms = 8;

    pub fn clear_perms(&mut self) {
        self.perms.clear();
    }

    pub fn has_perms(&self) -> bool {
        self.perms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_perms(&mut self, v: super::hdfs::FsPermissionProto) {
        self.perms = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_perms(&mut self) -> &mut super::hdfs::FsPermissionProto {
        if self.perms.is_none() {
            self.perms.set_default();
        }
        self.perms.as_mut().unwrap()
    }

    // Take field
    pub fn take_perms(&mut self) -> super::hdfs::FsPermissionProto {
        self.perms.take().unwrap_or_else(|| super::hdfs::FsPermissionProto::new())
    }

    pub fn get_perms(&self) -> &super::hdfs::FsPermissionProto {
        self.perms.as_ref().unwrap_or_else(|| super::hdfs::FsPermissionProto::default_instance())
    }

    fn get_perms_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::FsPermissionProto> {
        &self.perms
    }

    fn mut_perms_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::FsPermissionProto> {
        &mut self.perms
    }

    // repeated .hadoop.hdfs.AclEntryProto acls = 9;

    pub fn clear_acls(&mut self) {
        self.acls.clear();
    }

    // Param is passed by value, moved
    pub fn set_acls(&mut self, v: ::protobuf::RepeatedField<super::acl::AclEntryProto>) {
        self.acls = v;
    }

    // Mutable pointer to the field.
    pub fn mut_acls(&mut self) -> &mut ::protobuf::RepeatedField<super::acl::AclEntryProto> {
        &mut self.acls
    }

    // Take field
    pub fn take_acls(&mut self) -> ::protobuf::RepeatedField<super::acl::AclEntryProto> {
        ::std::mem::replace(&mut self.acls, ::protobuf::RepeatedField::new())
    }

    pub fn get_acls(&self) -> &[super::acl::AclEntryProto] {
        &self.acls
    }

    fn get_acls_for_reflect(&self) -> &::protobuf::RepeatedField<super::acl::AclEntryProto> {
        &self.acls
    }

    fn mut_acls_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::acl::AclEntryProto> {
        &mut self.acls
    }

    // repeated .hadoop.hdfs.XAttrProto xAttrs = 10;

    pub fn clear_xAttrs(&mut self) {
        self.xAttrs.clear();
    }

    // Param is passed by value, moved
    pub fn set_xAttrs(&mut self, v: ::protobuf::RepeatedField<super::xattr::XAttrProto>) {
        self.xAttrs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_xAttrs(&mut self) -> &mut ::protobuf::RepeatedField<super::xattr::XAttrProto> {
        &mut self.xAttrs
    }

    // Take field
    pub fn take_xAttrs(&mut self) -> ::protobuf::RepeatedField<super::xattr::XAttrProto> {
        ::std::mem::replace(&mut self.xAttrs, ::protobuf::RepeatedField::new())
    }

    pub fn get_xAttrs(&self) -> &[super::xattr::XAttrProto] {
        &self.xAttrs
    }

    fn get_xAttrs_for_reflect(&self) -> &::protobuf::RepeatedField<super::xattr::XAttrProto> {
        &self.xAttrs
    }

    fn mut_xAttrs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::xattr::XAttrProto> {
        &mut self.xAttrs
    }

    // optional bool xAttrsRemoved = 11;

    pub fn clear_xAttrsRemoved(&mut self) {
        self.xAttrsRemoved = ::std::option::Option::None;
    }

    pub fn has_xAttrsRemoved(&self) -> bool {
        self.xAttrsRemoved.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xAttrsRemoved(&mut self, v: bool) {
        self.xAttrsRemoved = ::std::option::Option::Some(v);
    }

    pub fn get_xAttrsRemoved(&self) -> bool {
        self.xAttrsRemoved.unwrap_or(false)
    }

    fn get_xAttrsRemoved_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.xAttrsRemoved
    }

    fn mut_xAttrsRemoved_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.xAttrsRemoved
    }
}

impl ::protobuf::Message for MetadataUpdateEventProto {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        }
        if self.field_type.is_none() {
            return false;
        }
        for v in &self.perms {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.acls {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.mtime = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.atime = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.replication = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ownerName)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.groupName)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.perms)?;
                },
                9 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.acls)?;
                },
                10 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.xAttrs)?;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.xAttrsRemoved = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(v) = self.mtime {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.atime {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.replication {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.ownerName.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(ref v) = self.groupName.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        if let Some(ref v) = self.perms.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.acls {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.xAttrs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.xAttrsRemoved {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.field_type {
            os.write_enum(2, v.value())?;
        }
        if let Some(v) = self.mtime {
            os.write_int64(3, v)?;
        }
        if let Some(v) = self.atime {
            os.write_int64(4, v)?;
        }
        if let Some(v) = self.replication {
            os.write_int32(5, v)?;
        }
        if let Some(ref v) = self.ownerName.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(ref v) = self.groupName.as_ref() {
            os.write_string(7, &v)?;
        }
        if let Some(ref v) = self.perms.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.acls {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.xAttrs {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.xAttrsRemoved {
            os.write_bool(11, v)?;
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

impl ::protobuf::MessageStatic for MetadataUpdateEventProto {
    fn new() -> MetadataUpdateEventProto {
        MetadataUpdateEventProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<MetadataUpdateEventProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    MetadataUpdateEventProto::get_path_for_reflect,
                    MetadataUpdateEventProto::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MetadataUpdateType>>(
                    "type",
                    MetadataUpdateEventProto::get_field_type_for_reflect,
                    MetadataUpdateEventProto::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "mtime",
                    MetadataUpdateEventProto::get_mtime_for_reflect,
                    MetadataUpdateEventProto::mut_mtime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "atime",
                    MetadataUpdateEventProto::get_atime_for_reflect,
                    MetadataUpdateEventProto::mut_atime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "replication",
                    MetadataUpdateEventProto::get_replication_for_reflect,
                    MetadataUpdateEventProto::mut_replication_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ownerName",
                    MetadataUpdateEventProto::get_ownerName_for_reflect,
                    MetadataUpdateEventProto::mut_ownerName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "groupName",
                    MetadataUpdateEventProto::get_groupName_for_reflect,
                    MetadataUpdateEventProto::mut_groupName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::FsPermissionProto>>(
                    "perms",
                    MetadataUpdateEventProto::get_perms_for_reflect,
                    MetadataUpdateEventProto::mut_perms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::acl::AclEntryProto>>(
                    "acls",
                    MetadataUpdateEventProto::get_acls_for_reflect,
                    MetadataUpdateEventProto::mut_acls_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::xattr::XAttrProto>>(
                    "xAttrs",
                    MetadataUpdateEventProto::get_xAttrs_for_reflect,
                    MetadataUpdateEventProto::mut_xAttrs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "xAttrsRemoved",
                    MetadataUpdateEventProto::get_xAttrsRemoved_for_reflect,
                    MetadataUpdateEventProto::mut_xAttrsRemoved_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MetadataUpdateEventProto>(
                    "MetadataUpdateEventProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MetadataUpdateEventProto {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_field_type();
        self.clear_mtime();
        self.clear_atime();
        self.clear_replication();
        self.clear_ownerName();
        self.clear_groupName();
        self.clear_perms();
        self.clear_acls();
        self.clear_xAttrs();
        self.clear_xAttrsRemoved();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MetadataUpdateEventProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MetadataUpdateEventProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnlinkEventProto {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    timestamp: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnlinkEventProto {}

impl UnlinkEventProto {
    pub fn new() -> UnlinkEventProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnlinkEventProto {
        static mut instance: ::protobuf::lazy::Lazy<UnlinkEventProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnlinkEventProto,
        };
        unsafe {
            instance.get(UnlinkEventProto::new)
        }
    }

    // required string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        }
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }

    // required int64 timestamp = 2;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: i64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp.unwrap_or(0)
    }

    fn get_timestamp_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.timestamp
    }
}

impl ::protobuf::Message for UnlinkEventProto {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        }
        if self.timestamp.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.timestamp {
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

impl ::protobuf::MessageStatic for UnlinkEventProto {
    fn new() -> UnlinkEventProto {
        UnlinkEventProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnlinkEventProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    UnlinkEventProto::get_path_for_reflect,
                    UnlinkEventProto::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "timestamp",
                    UnlinkEventProto::get_timestamp_for_reflect,
                    UnlinkEventProto::mut_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnlinkEventProto>(
                    "UnlinkEventProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnlinkEventProto {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnlinkEventProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnlinkEventProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EventsListProto {
    // message fields
    events: ::protobuf::RepeatedField<EventProto>,
    firstTxid: ::std::option::Option<i64>,
    lastTxid: ::std::option::Option<i64>,
    syncTxid: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EventsListProto {}

impl EventsListProto {
    pub fn new() -> EventsListProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EventsListProto {
        static mut instance: ::protobuf::lazy::Lazy<EventsListProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EventsListProto,
        };
        unsafe {
            instance.get(EventsListProto::new)
        }
    }

    // repeated .hadoop.hdfs.EventProto events = 1;

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    // Param is passed by value, moved
    pub fn set_events(&mut self, v: ::protobuf::RepeatedField<EventProto>) {
        self.events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_events(&mut self) -> &mut ::protobuf::RepeatedField<EventProto> {
        &mut self.events
    }

    // Take field
    pub fn take_events(&mut self) -> ::protobuf::RepeatedField<EventProto> {
        ::std::mem::replace(&mut self.events, ::protobuf::RepeatedField::new())
    }

    pub fn get_events(&self) -> &[EventProto] {
        &self.events
    }

    fn get_events_for_reflect(&self) -> &::protobuf::RepeatedField<EventProto> {
        &self.events
    }

    fn mut_events_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<EventProto> {
        &mut self.events
    }

    // required int64 firstTxid = 2;

    pub fn clear_firstTxid(&mut self) {
        self.firstTxid = ::std::option::Option::None;
    }

    pub fn has_firstTxid(&self) -> bool {
        self.firstTxid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_firstTxid(&mut self, v: i64) {
        self.firstTxid = ::std::option::Option::Some(v);
    }

    pub fn get_firstTxid(&self) -> i64 {
        self.firstTxid.unwrap_or(0)
    }

    fn get_firstTxid_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.firstTxid
    }

    fn mut_firstTxid_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.firstTxid
    }

    // required int64 lastTxid = 3;

    pub fn clear_lastTxid(&mut self) {
        self.lastTxid = ::std::option::Option::None;
    }

    pub fn has_lastTxid(&self) -> bool {
        self.lastTxid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastTxid(&mut self, v: i64) {
        self.lastTxid = ::std::option::Option::Some(v);
    }

    pub fn get_lastTxid(&self) -> i64 {
        self.lastTxid.unwrap_or(0)
    }

    fn get_lastTxid_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.lastTxid
    }

    fn mut_lastTxid_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.lastTxid
    }

    // required int64 syncTxid = 4;

    pub fn clear_syncTxid(&mut self) {
        self.syncTxid = ::std::option::Option::None;
    }

    pub fn has_syncTxid(&self) -> bool {
        self.syncTxid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_syncTxid(&mut self, v: i64) {
        self.syncTxid = ::std::option::Option::Some(v);
    }

    pub fn get_syncTxid(&self) -> i64 {
        self.syncTxid.unwrap_or(0)
    }

    fn get_syncTxid_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.syncTxid
    }

    fn mut_syncTxid_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.syncTxid
    }
}

impl ::protobuf::Message for EventsListProto {
    fn is_initialized(&self) -> bool {
        if self.firstTxid.is_none() {
            return false;
        }
        if self.lastTxid.is_none() {
            return false;
        }
        if self.syncTxid.is_none() {
            return false;
        }
        for v in &self.events {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.events)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.firstTxid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.lastTxid = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.syncTxid = ::std::option::Option::Some(tmp);
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
        for value in &self.events {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.firstTxid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.lastTxid {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.syncTxid {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.events {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.firstTxid {
            os.write_int64(2, v)?;
        }
        if let Some(v) = self.lastTxid {
            os.write_int64(3, v)?;
        }
        if let Some(v) = self.syncTxid {
            os.write_int64(4, v)?;
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

impl ::protobuf::MessageStatic for EventsListProto {
    fn new() -> EventsListProto {
        EventsListProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<EventsListProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EventProto>>(
                    "events",
                    EventsListProto::get_events_for_reflect,
                    EventsListProto::mut_events_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "firstTxid",
                    EventsListProto::get_firstTxid_for_reflect,
                    EventsListProto::mut_firstTxid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "lastTxid",
                    EventsListProto::get_lastTxid_for_reflect,
                    EventsListProto::mut_lastTxid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "syncTxid",
                    EventsListProto::get_syncTxid_for_reflect,
                    EventsListProto::mut_syncTxid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EventsListProto>(
                    "EventsListProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EventsListProto {
    fn clear(&mut self) {
        self.clear_events();
        self.clear_firstTxid();
        self.clear_lastTxid();
        self.clear_syncTxid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EventsListProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EventsListProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EventType {
    EVENT_CREATE = 0,
    EVENT_CLOSE = 1,
    EVENT_APPEND = 2,
    EVENT_RENAME = 3,
    EVENT_METADATA = 4,
    EVENT_UNLINK = 5,
}

impl ::protobuf::ProtobufEnum for EventType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EventType> {
        match value {
            0 => ::std::option::Option::Some(EventType::EVENT_CREATE),
            1 => ::std::option::Option::Some(EventType::EVENT_CLOSE),
            2 => ::std::option::Option::Some(EventType::EVENT_APPEND),
            3 => ::std::option::Option::Some(EventType::EVENT_RENAME),
            4 => ::std::option::Option::Some(EventType::EVENT_METADATA),
            5 => ::std::option::Option::Some(EventType::EVENT_UNLINK),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EventType] = &[
            EventType::EVENT_CREATE,
            EventType::EVENT_CLOSE,
            EventType::EVENT_APPEND,
            EventType::EVENT_RENAME,
            EventType::EVENT_METADATA,
            EventType::EVENT_UNLINK,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EventType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EventType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EventType {
}

impl ::protobuf::reflect::ProtobufValue for EventType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum INodeType {
    I_TYPE_FILE = 0,
    I_TYPE_DIRECTORY = 1,
    I_TYPE_SYMLINK = 2,
}

impl ::protobuf::ProtobufEnum for INodeType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<INodeType> {
        match value {
            0 => ::std::option::Option::Some(INodeType::I_TYPE_FILE),
            1 => ::std::option::Option::Some(INodeType::I_TYPE_DIRECTORY),
            2 => ::std::option::Option::Some(INodeType::I_TYPE_SYMLINK),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [INodeType] = &[
            INodeType::I_TYPE_FILE,
            INodeType::I_TYPE_DIRECTORY,
            INodeType::I_TYPE_SYMLINK,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<INodeType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("INodeType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for INodeType {
}

impl ::protobuf::reflect::ProtobufValue for INodeType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MetadataUpdateType {
    META_TYPE_TIMES = 0,
    META_TYPE_REPLICATION = 1,
    META_TYPE_OWNER = 2,
    META_TYPE_PERMS = 3,
    META_TYPE_ACLS = 4,
    META_TYPE_XATTRS = 5,
}

impl ::protobuf::ProtobufEnum for MetadataUpdateType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MetadataUpdateType> {
        match value {
            0 => ::std::option::Option::Some(MetadataUpdateType::META_TYPE_TIMES),
            1 => ::std::option::Option::Some(MetadataUpdateType::META_TYPE_REPLICATION),
            2 => ::std::option::Option::Some(MetadataUpdateType::META_TYPE_OWNER),
            3 => ::std::option::Option::Some(MetadataUpdateType::META_TYPE_PERMS),
            4 => ::std::option::Option::Some(MetadataUpdateType::META_TYPE_ACLS),
            5 => ::std::option::Option::Some(MetadataUpdateType::META_TYPE_XATTRS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MetadataUpdateType] = &[
            MetadataUpdateType::META_TYPE_TIMES,
            MetadataUpdateType::META_TYPE_REPLICATION,
            MetadataUpdateType::META_TYPE_OWNER,
            MetadataUpdateType::META_TYPE_PERMS,
            MetadataUpdateType::META_TYPE_ACLS,
            MetadataUpdateType::META_TYPE_XATTRS,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<MetadataUpdateType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MetadataUpdateType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MetadataUpdateType {
}

impl ::protobuf::reflect::ProtobufValue for MetadataUpdateType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rinotify.proto\x12\x0bhadoop.hdfs\x1a\tacl.proto\x1a\x0bxattr.proto\
    \x1a\nhdfs.proto\"D\n\nEventProto\x12$\n\x04type\x18\x01\x20\x02(\x0e2\
    \x16.hadoop.hdfs.EventType\x12\x10\n\x08contents\x18\x02\x20\x02(\x0c\"\
    \xe9\x01\n\x10CreateEventProto\x12$\n\x04type\x18\x01\x20\x02(\x0e2\x16.\
    hadoop.hdfs.INodeType\x12\x0c\n\x04path\x18\x02\x20\x02(\t\x12\r\n\x05ct\
    ime\x18\x03\x20\x02(\x03\x12\x11\n\townerName\x18\x04\x20\x02(\t\x12\x11\
    \n\tgroupName\x18\x05\x20\x02(\t\x12-\n\x05perms\x18\x06\x20\x02(\x0b2\
    \x1e.hadoop.hdfs.FsPermissionProto\x12\x13\n\x0breplication\x18\x07\x20\
    \x01(\x05\x12\x15\n\rsymlinkTarget\x18\x08\x20\x01(\t\x12\x11\n\toverwri\
    te\x18\t\x20\x01(\x08\"D\n\x0fCloseEventProto\x12\x0c\n\x04path\x18\x01\
    \x20\x02(\t\x12\x10\n\x08fileSize\x18\x02\x20\x02(\x03\x12\x11\n\ttimest\
    amp\x18\x03\x20\x02(\x03\"\x20\n\x10AppendEventProto\x12\x0c\n\x04path\
    \x18\x01\x20\x02(\t\"H\n\x10RenameEventProto\x12\x0f\n\x07srcPath\x18\
    \x01\x20\x02(\t\x12\x10\n\x08destPath\x18\x02\x20\x02(\t\x12\x11\n\ttime\
    stamp\x18\x03\x20\x02(\x03\"\xc9\x02\n\x18MetadataUpdateEventProto\x12\
    \x0c\n\x04path\x18\x01\x20\x02(\t\x12-\n\x04type\x18\x02\x20\x02(\x0e2\
    \x1f.hadoop.hdfs.MetadataUpdateType\x12\r\n\x05mtime\x18\x03\x20\x01(\
    \x03\x12\r\n\x05atime\x18\x04\x20\x01(\x03\x12\x13\n\x0breplication\x18\
    \x05\x20\x01(\x05\x12\x11\n\townerName\x18\x06\x20\x01(\t\x12\x11\n\tgro\
    upName\x18\x07\x20\x01(\t\x12-\n\x05perms\x18\x08\x20\x01(\x0b2\x1e.hado\
    op.hdfs.FsPermissionProto\x12(\n\x04acls\x18\t\x20\x03(\x0b2\x1a.hadoop.\
    hdfs.AclEntryProto\x12'\n\x06xAttrs\x18\n\x20\x03(\x0b2\x17.hadoop.hdfs.\
    XAttrProto\x12\x15\n\rxAttrsRemoved\x18\x0b\x20\x01(\x08\"3\n\x10UnlinkE\
    ventProto\x12\x0c\n\x04path\x18\x01\x20\x02(\t\x12\x11\n\ttimestamp\x18\
    \x02\x20\x02(\x03\"q\n\x0fEventsListProto\x12'\n\x06events\x18\x01\x20\
    \x03(\x0b2\x17.hadoop.hdfs.EventProto\x12\x11\n\tfirstTxid\x18\x02\x20\
    \x02(\x03\x12\x10\n\x08lastTxid\x18\x03\x20\x02(\x03\x12\x10\n\x08syncTx\
    id\x18\x04\x20\x02(\x03*x\n\tEventType\x12\x10\n\x0cEVENT_CREATE\x10\0\
    \x12\x0f\n\x0bEVENT_CLOSE\x10\x01\x12\x10\n\x0cEVENT_APPEND\x10\x02\x12\
    \x10\n\x0cEVENT_RENAME\x10\x03\x12\x12\n\x0eEVENT_METADATA\x10\x04\x12\
    \x10\n\x0cEVENT_UNLINK\x10\x05*F\n\tINodeType\x12\x0f\n\x0bI_TYPE_FILE\
    \x10\0\x12\x14\n\x10I_TYPE_DIRECTORY\x10\x01\x12\x12\n\x0eI_TYPE_SYMLINK\
    \x10\x02*\x98\x01\n\x12MetadataUpdateType\x12\x13\n\x0fMETA_TYPE_TIMES\
    \x10\0\x12\x19\n\x15META_TYPE_REPLICATION\x10\x01\x12\x13\n\x0fMETA_TYPE\
    _OWNER\x10\x02\x12\x13\n\x0fMETA_TYPE_PERMS\x10\x03\x12\x12\n\x0eMETA_TY\
    PE_ACLS\x10\x04\x12\x14\n\x10META_TYPE_XATTRS\x10\x05B9\n%org.apache.had\
    oop.hdfs.protocol.protoB\rInotifyProtos\xa0\x01\x01\
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
