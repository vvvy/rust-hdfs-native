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
pub struct JournalIdProto {
    // message fields
    identifier: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JournalIdProto {}

impl JournalIdProto {
    pub fn new() -> JournalIdProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JournalIdProto {
        static mut instance: ::protobuf::lazy::Lazy<JournalIdProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JournalIdProto,
        };
        unsafe {
            instance.get(JournalIdProto::new)
        }
    }

    // required string identifier = 1;

    pub fn clear_identifier(&mut self) {
        self.identifier.clear();
    }

    pub fn has_identifier(&self) -> bool {
        self.identifier.is_some()
    }

    // Param is passed by value, moved
    pub fn set_identifier(&mut self, v: ::std::string::String) {
        self.identifier = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_identifier(&mut self) -> &mut ::std::string::String {
        if self.identifier.is_none() {
            self.identifier.set_default();
        }
        self.identifier.as_mut().unwrap()
    }

    // Take field
    pub fn take_identifier(&mut self) -> ::std::string::String {
        self.identifier.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_identifier(&self) -> &str {
        match self.identifier.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_identifier_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.identifier
    }

    fn mut_identifier_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.identifier
    }
}

impl ::protobuf::Message for JournalIdProto {
    fn is_initialized(&self) -> bool {
        if self.identifier.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.identifier)?;
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
        if let Some(ref v) = self.identifier.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.identifier.as_ref() {
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

impl ::protobuf::MessageStatic for JournalIdProto {
    fn new() -> JournalIdProto {
        JournalIdProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<JournalIdProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "identifier",
                    JournalIdProto::get_identifier_for_reflect,
                    JournalIdProto::mut_identifier_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JournalIdProto>(
                    "JournalIdProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JournalIdProto {
    fn clear(&mut self) {
        self.clear_identifier();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JournalIdProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JournalIdProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestInfoProto {
    // message fields
    journalId: ::protobuf::SingularPtrField<JournalIdProto>,
    epoch: ::std::option::Option<u64>,
    ipcSerialNumber: ::std::option::Option<u64>,
    committedTxId: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestInfoProto {}

impl RequestInfoProto {
    pub fn new() -> RequestInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<RequestInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestInfoProto,
        };
        unsafe {
            instance.get(RequestInfoProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.JournalIdProto journalId = 1;

    pub fn clear_journalId(&mut self) {
        self.journalId.clear();
    }

    pub fn has_journalId(&self) -> bool {
        self.journalId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_journalId(&mut self, v: JournalIdProto) {
        self.journalId = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_journalId(&mut self) -> &mut JournalIdProto {
        if self.journalId.is_none() {
            self.journalId.set_default();
        }
        self.journalId.as_mut().unwrap()
    }

    // Take field
    pub fn take_journalId(&mut self) -> JournalIdProto {
        self.journalId.take().unwrap_or_else(|| JournalIdProto::new())
    }

    pub fn get_journalId(&self) -> &JournalIdProto {
        self.journalId.as_ref().unwrap_or_else(|| JournalIdProto::default_instance())
    }

    fn get_journalId_for_reflect(&self) -> &::protobuf::SingularPtrField<JournalIdProto> {
        &self.journalId
    }

    fn mut_journalId_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JournalIdProto> {
        &mut self.journalId
    }

    // required uint64 epoch = 2;

    pub fn clear_epoch(&mut self) {
        self.epoch = ::std::option::Option::None;
    }

    pub fn has_epoch(&self) -> bool {
        self.epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_epoch(&mut self, v: u64) {
        self.epoch = ::std::option::Option::Some(v);
    }

    pub fn get_epoch(&self) -> u64 {
        self.epoch.unwrap_or(0)
    }

    fn get_epoch_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.epoch
    }

    fn mut_epoch_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.epoch
    }

    // required uint64 ipcSerialNumber = 3;

    pub fn clear_ipcSerialNumber(&mut self) {
        self.ipcSerialNumber = ::std::option::Option::None;
    }

    pub fn has_ipcSerialNumber(&self) -> bool {
        self.ipcSerialNumber.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ipcSerialNumber(&mut self, v: u64) {
        self.ipcSerialNumber = ::std::option::Option::Some(v);
    }

    pub fn get_ipcSerialNumber(&self) -> u64 {
        self.ipcSerialNumber.unwrap_or(0)
    }

    fn get_ipcSerialNumber_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.ipcSerialNumber
    }

    fn mut_ipcSerialNumber_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.ipcSerialNumber
    }

    // optional uint64 committedTxId = 4;

    pub fn clear_committedTxId(&mut self) {
        self.committedTxId = ::std::option::Option::None;
    }

    pub fn has_committedTxId(&self) -> bool {
        self.committedTxId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_committedTxId(&mut self, v: u64) {
        self.committedTxId = ::std::option::Option::Some(v);
    }

    pub fn get_committedTxId(&self) -> u64 {
        self.committedTxId.unwrap_or(0)
    }

    fn get_committedTxId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.committedTxId
    }

    fn mut_committedTxId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.committedTxId
    }
}

impl ::protobuf::Message for RequestInfoProto {
    fn is_initialized(&self) -> bool {
        if self.journalId.is_none() {
            return false;
        }
        if self.epoch.is_none() {
            return false;
        }
        if self.ipcSerialNumber.is_none() {
            return false;
        }
        for v in &self.journalId {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.journalId)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.epoch = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.ipcSerialNumber = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.committedTxId = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.journalId.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.epoch {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ipcSerialNumber {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.committedTxId {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.journalId.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.epoch {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.ipcSerialNumber {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.committedTxId {
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

impl ::protobuf::MessageStatic for RequestInfoProto {
    fn new() -> RequestInfoProto {
        RequestInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JournalIdProto>>(
                    "journalId",
                    RequestInfoProto::get_journalId_for_reflect,
                    RequestInfoProto::mut_journalId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "epoch",
                    RequestInfoProto::get_epoch_for_reflect,
                    RequestInfoProto::mut_epoch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ipcSerialNumber",
                    RequestInfoProto::get_ipcSerialNumber_for_reflect,
                    RequestInfoProto::mut_ipcSerialNumber_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "committedTxId",
                    RequestInfoProto::get_committedTxId_for_reflect,
                    RequestInfoProto::mut_committedTxId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestInfoProto>(
                    "RequestInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestInfoProto {
    fn clear(&mut self) {
        self.clear_journalId();
        self.clear_epoch();
        self.clear_ipcSerialNumber();
        self.clear_committedTxId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SegmentStateProto {
    // message fields
    startTxId: ::std::option::Option<u64>,
    endTxId: ::std::option::Option<u64>,
    isInProgress: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SegmentStateProto {}

impl SegmentStateProto {
    pub fn new() -> SegmentStateProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SegmentStateProto {
        static mut instance: ::protobuf::lazy::Lazy<SegmentStateProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SegmentStateProto,
        };
        unsafe {
            instance.get(SegmentStateProto::new)
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

    // required bool isInProgress = 3;

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

impl ::protobuf::Message for SegmentStateProto {
    fn is_initialized(&self) -> bool {
        if self.startTxId.is_none() {
            return false;
        }
        if self.endTxId.is_none() {
            return false;
        }
        if self.isInProgress.is_none() {
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

impl ::protobuf::MessageStatic for SegmentStateProto {
    fn new() -> SegmentStateProto {
        SegmentStateProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SegmentStateProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "startTxId",
                    SegmentStateProto::get_startTxId_for_reflect,
                    SegmentStateProto::mut_startTxId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "endTxId",
                    SegmentStateProto::get_endTxId_for_reflect,
                    SegmentStateProto::mut_endTxId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "isInProgress",
                    SegmentStateProto::get_isInProgress_for_reflect,
                    SegmentStateProto::mut_isInProgress_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SegmentStateProto>(
                    "SegmentStateProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SegmentStateProto {
    fn clear(&mut self) {
        self.clear_startTxId();
        self.clear_endTxId();
        self.clear_isInProgress();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SegmentStateProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SegmentStateProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PersistedRecoveryPaxosData {
    // message fields
    segmentState: ::protobuf::SingularPtrField<SegmentStateProto>,
    acceptedInEpoch: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PersistedRecoveryPaxosData {}

impl PersistedRecoveryPaxosData {
    pub fn new() -> PersistedRecoveryPaxosData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PersistedRecoveryPaxosData {
        static mut instance: ::protobuf::lazy::Lazy<PersistedRecoveryPaxosData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PersistedRecoveryPaxosData,
        };
        unsafe {
            instance.get(PersistedRecoveryPaxosData::new)
        }
    }

    // required .hadoop.hdfs.qjournal.SegmentStateProto segmentState = 1;

    pub fn clear_segmentState(&mut self) {
        self.segmentState.clear();
    }

    pub fn has_segmentState(&self) -> bool {
        self.segmentState.is_some()
    }

    // Param is passed by value, moved
    pub fn set_segmentState(&mut self, v: SegmentStateProto) {
        self.segmentState = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_segmentState(&mut self) -> &mut SegmentStateProto {
        if self.segmentState.is_none() {
            self.segmentState.set_default();
        }
        self.segmentState.as_mut().unwrap()
    }

    // Take field
    pub fn take_segmentState(&mut self) -> SegmentStateProto {
        self.segmentState.take().unwrap_or_else(|| SegmentStateProto::new())
    }

    pub fn get_segmentState(&self) -> &SegmentStateProto {
        self.segmentState.as_ref().unwrap_or_else(|| SegmentStateProto::default_instance())
    }

    fn get_segmentState_for_reflect(&self) -> &::protobuf::SingularPtrField<SegmentStateProto> {
        &self.segmentState
    }

    fn mut_segmentState_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SegmentStateProto> {
        &mut self.segmentState
    }

    // required uint64 acceptedInEpoch = 2;

    pub fn clear_acceptedInEpoch(&mut self) {
        self.acceptedInEpoch = ::std::option::Option::None;
    }

    pub fn has_acceptedInEpoch(&self) -> bool {
        self.acceptedInEpoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_acceptedInEpoch(&mut self, v: u64) {
        self.acceptedInEpoch = ::std::option::Option::Some(v);
    }

    pub fn get_acceptedInEpoch(&self) -> u64 {
        self.acceptedInEpoch.unwrap_or(0)
    }

    fn get_acceptedInEpoch_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.acceptedInEpoch
    }

    fn mut_acceptedInEpoch_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.acceptedInEpoch
    }
}

impl ::protobuf::Message for PersistedRecoveryPaxosData {
    fn is_initialized(&self) -> bool {
        if self.segmentState.is_none() {
            return false;
        }
        if self.acceptedInEpoch.is_none() {
            return false;
        }
        for v in &self.segmentState {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.segmentState)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.acceptedInEpoch = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.segmentState.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.acceptedInEpoch {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.segmentState.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.acceptedInEpoch {
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

impl ::protobuf::MessageStatic for PersistedRecoveryPaxosData {
    fn new() -> PersistedRecoveryPaxosData {
        PersistedRecoveryPaxosData::new()
    }

    fn descriptor_static(_: ::std::option::Option<PersistedRecoveryPaxosData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SegmentStateProto>>(
                    "segmentState",
                    PersistedRecoveryPaxosData::get_segmentState_for_reflect,
                    PersistedRecoveryPaxosData::mut_segmentState_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "acceptedInEpoch",
                    PersistedRecoveryPaxosData::get_acceptedInEpoch_for_reflect,
                    PersistedRecoveryPaxosData::mut_acceptedInEpoch_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PersistedRecoveryPaxosData>(
                    "PersistedRecoveryPaxosData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PersistedRecoveryPaxosData {
    fn clear(&mut self) {
        self.clear_segmentState();
        self.clear_acceptedInEpoch();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PersistedRecoveryPaxosData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PersistedRecoveryPaxosData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JournalRequestProto {
    // message fields
    reqInfo: ::protobuf::SingularPtrField<RequestInfoProto>,
    firstTxnId: ::std::option::Option<u64>,
    numTxns: ::std::option::Option<u32>,
    records: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    segmentTxnId: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JournalRequestProto {}

impl JournalRequestProto {
    pub fn new() -> JournalRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JournalRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<JournalRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JournalRequestProto,
        };
        unsafe {
            instance.get(JournalRequestProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.RequestInfoProto reqInfo = 1;

    pub fn clear_reqInfo(&mut self) {
        self.reqInfo.clear();
    }

    pub fn has_reqInfo(&self) -> bool {
        self.reqInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reqInfo(&mut self, v: RequestInfoProto) {
        self.reqInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reqInfo(&mut self) -> &mut RequestInfoProto {
        if self.reqInfo.is_none() {
            self.reqInfo.set_default();
        }
        self.reqInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_reqInfo(&mut self) -> RequestInfoProto {
        self.reqInfo.take().unwrap_or_else(|| RequestInfoProto::new())
    }

    pub fn get_reqInfo(&self) -> &RequestInfoProto {
        self.reqInfo.as_ref().unwrap_or_else(|| RequestInfoProto::default_instance())
    }

    fn get_reqInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<RequestInfoProto> {
        &self.reqInfo
    }

    fn mut_reqInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RequestInfoProto> {
        &mut self.reqInfo
    }

    // required uint64 firstTxnId = 2;

    pub fn clear_firstTxnId(&mut self) {
        self.firstTxnId = ::std::option::Option::None;
    }

    pub fn has_firstTxnId(&self) -> bool {
        self.firstTxnId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_firstTxnId(&mut self, v: u64) {
        self.firstTxnId = ::std::option::Option::Some(v);
    }

    pub fn get_firstTxnId(&self) -> u64 {
        self.firstTxnId.unwrap_or(0)
    }

    fn get_firstTxnId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.firstTxnId
    }

    fn mut_firstTxnId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.firstTxnId
    }

    // required uint32 numTxns = 3;

    pub fn clear_numTxns(&mut self) {
        self.numTxns = ::std::option::Option::None;
    }

    pub fn has_numTxns(&self) -> bool {
        self.numTxns.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numTxns(&mut self, v: u32) {
        self.numTxns = ::std::option::Option::Some(v);
    }

    pub fn get_numTxns(&self) -> u32 {
        self.numTxns.unwrap_or(0)
    }

    fn get_numTxns_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.numTxns
    }

    fn mut_numTxns_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.numTxns
    }

    // required bytes records = 4;

    pub fn clear_records(&mut self) {
        self.records.clear();
    }

    pub fn has_records(&self) -> bool {
        self.records.is_some()
    }

    // Param is passed by value, moved
    pub fn set_records(&mut self, v: ::std::vec::Vec<u8>) {
        self.records = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_records(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.records.is_none() {
            self.records.set_default();
        }
        self.records.as_mut().unwrap()
    }

    // Take field
    pub fn take_records(&mut self) -> ::std::vec::Vec<u8> {
        self.records.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_records(&self) -> &[u8] {
        match self.records.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_records_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.records
    }

    fn mut_records_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.records
    }

    // required uint64 segmentTxnId = 5;

    pub fn clear_segmentTxnId(&mut self) {
        self.segmentTxnId = ::std::option::Option::None;
    }

    pub fn has_segmentTxnId(&self) -> bool {
        self.segmentTxnId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_segmentTxnId(&mut self, v: u64) {
        self.segmentTxnId = ::std::option::Option::Some(v);
    }

    pub fn get_segmentTxnId(&self) -> u64 {
        self.segmentTxnId.unwrap_or(0)
    }

    fn get_segmentTxnId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.segmentTxnId
    }

    fn mut_segmentTxnId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.segmentTxnId
    }
}

impl ::protobuf::Message for JournalRequestProto {
    fn is_initialized(&self) -> bool {
        if self.reqInfo.is_none() {
            return false;
        }
        if self.firstTxnId.is_none() {
            return false;
        }
        if self.numTxns.is_none() {
            return false;
        }
        if self.records.is_none() {
            return false;
        }
        if self.segmentTxnId.is_none() {
            return false;
        }
        for v in &self.reqInfo {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.reqInfo)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.firstTxnId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.numTxns = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.records)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.segmentTxnId = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.reqInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.firstTxnId {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numTxns {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.records.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        if let Some(v) = self.segmentTxnId {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.reqInfo.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.firstTxnId {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.numTxns {
            os.write_uint32(3, v)?;
        }
        if let Some(ref v) = self.records.as_ref() {
            os.write_bytes(4, &v)?;
        }
        if let Some(v) = self.segmentTxnId {
            os.write_uint64(5, v)?;
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

impl ::protobuf::MessageStatic for JournalRequestProto {
    fn new() -> JournalRequestProto {
        JournalRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<JournalRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RequestInfoProto>>(
                    "reqInfo",
                    JournalRequestProto::get_reqInfo_for_reflect,
                    JournalRequestProto::mut_reqInfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "firstTxnId",
                    JournalRequestProto::get_firstTxnId_for_reflect,
                    JournalRequestProto::mut_firstTxnId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "numTxns",
                    JournalRequestProto::get_numTxns_for_reflect,
                    JournalRequestProto::mut_numTxns_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "records",
                    JournalRequestProto::get_records_for_reflect,
                    JournalRequestProto::mut_records_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "segmentTxnId",
                    JournalRequestProto::get_segmentTxnId_for_reflect,
                    JournalRequestProto::mut_segmentTxnId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JournalRequestProto>(
                    "JournalRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JournalRequestProto {
    fn clear(&mut self) {
        self.clear_reqInfo();
        self.clear_firstTxnId();
        self.clear_numTxns();
        self.clear_records();
        self.clear_segmentTxnId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JournalRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JournalRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JournalResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JournalResponseProto {}

impl JournalResponseProto {
    pub fn new() -> JournalResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JournalResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<JournalResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JournalResponseProto,
        };
        unsafe {
            instance.get(JournalResponseProto::new)
        }
    }
}

impl ::protobuf::Message for JournalResponseProto {
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

impl ::protobuf::MessageStatic for JournalResponseProto {
    fn new() -> JournalResponseProto {
        JournalResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<JournalResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<JournalResponseProto>(
                    "JournalResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JournalResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JournalResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JournalResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HeartbeatRequestProto {
    // message fields
    reqInfo: ::protobuf::SingularPtrField<RequestInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HeartbeatRequestProto {}

impl HeartbeatRequestProto {
    pub fn new() -> HeartbeatRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HeartbeatRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<HeartbeatRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HeartbeatRequestProto,
        };
        unsafe {
            instance.get(HeartbeatRequestProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.RequestInfoProto reqInfo = 1;

    pub fn clear_reqInfo(&mut self) {
        self.reqInfo.clear();
    }

    pub fn has_reqInfo(&self) -> bool {
        self.reqInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reqInfo(&mut self, v: RequestInfoProto) {
        self.reqInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reqInfo(&mut self) -> &mut RequestInfoProto {
        if self.reqInfo.is_none() {
            self.reqInfo.set_default();
        }
        self.reqInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_reqInfo(&mut self) -> RequestInfoProto {
        self.reqInfo.take().unwrap_or_else(|| RequestInfoProto::new())
    }

    pub fn get_reqInfo(&self) -> &RequestInfoProto {
        self.reqInfo.as_ref().unwrap_or_else(|| RequestInfoProto::default_instance())
    }

    fn get_reqInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<RequestInfoProto> {
        &self.reqInfo
    }

    fn mut_reqInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RequestInfoProto> {
        &mut self.reqInfo
    }
}

impl ::protobuf::Message for HeartbeatRequestProto {
    fn is_initialized(&self) -> bool {
        if self.reqInfo.is_none() {
            return false;
        }
        for v in &self.reqInfo {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.reqInfo)?;
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
        if let Some(ref v) = self.reqInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.reqInfo.as_ref() {
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

impl ::protobuf::MessageStatic for HeartbeatRequestProto {
    fn new() -> HeartbeatRequestProto {
        HeartbeatRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<HeartbeatRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RequestInfoProto>>(
                    "reqInfo",
                    HeartbeatRequestProto::get_reqInfo_for_reflect,
                    HeartbeatRequestProto::mut_reqInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HeartbeatRequestProto>(
                    "HeartbeatRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HeartbeatRequestProto {
    fn clear(&mut self) {
        self.clear_reqInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HeartbeatRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HeartbeatRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HeartbeatResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HeartbeatResponseProto {}

impl HeartbeatResponseProto {
    pub fn new() -> HeartbeatResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HeartbeatResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<HeartbeatResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HeartbeatResponseProto,
        };
        unsafe {
            instance.get(HeartbeatResponseProto::new)
        }
    }
}

impl ::protobuf::Message for HeartbeatResponseProto {
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

impl ::protobuf::MessageStatic for HeartbeatResponseProto {
    fn new() -> HeartbeatResponseProto {
        HeartbeatResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<HeartbeatResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<HeartbeatResponseProto>(
                    "HeartbeatResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HeartbeatResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HeartbeatResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HeartbeatResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StartLogSegmentRequestProto {
    // message fields
    reqInfo: ::protobuf::SingularPtrField<RequestInfoProto>,
    txid: ::std::option::Option<u64>,
    layoutVersion: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StartLogSegmentRequestProto {}

impl StartLogSegmentRequestProto {
    pub fn new() -> StartLogSegmentRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StartLogSegmentRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<StartLogSegmentRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StartLogSegmentRequestProto,
        };
        unsafe {
            instance.get(StartLogSegmentRequestProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.RequestInfoProto reqInfo = 1;

    pub fn clear_reqInfo(&mut self) {
        self.reqInfo.clear();
    }

    pub fn has_reqInfo(&self) -> bool {
        self.reqInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reqInfo(&mut self, v: RequestInfoProto) {
        self.reqInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reqInfo(&mut self) -> &mut RequestInfoProto {
        if self.reqInfo.is_none() {
            self.reqInfo.set_default();
        }
        self.reqInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_reqInfo(&mut self) -> RequestInfoProto {
        self.reqInfo.take().unwrap_or_else(|| RequestInfoProto::new())
    }

    pub fn get_reqInfo(&self) -> &RequestInfoProto {
        self.reqInfo.as_ref().unwrap_or_else(|| RequestInfoProto::default_instance())
    }

    fn get_reqInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<RequestInfoProto> {
        &self.reqInfo
    }

    fn mut_reqInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RequestInfoProto> {
        &mut self.reqInfo
    }

    // required uint64 txid = 2;

    pub fn clear_txid(&mut self) {
        self.txid = ::std::option::Option::None;
    }

    pub fn has_txid(&self) -> bool {
        self.txid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txid(&mut self, v: u64) {
        self.txid = ::std::option::Option::Some(v);
    }

    pub fn get_txid(&self) -> u64 {
        self.txid.unwrap_or(0)
    }

    fn get_txid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.txid
    }

    fn mut_txid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.txid
    }

    // optional sint32 layoutVersion = 3;

    pub fn clear_layoutVersion(&mut self) {
        self.layoutVersion = ::std::option::Option::None;
    }

    pub fn has_layoutVersion(&self) -> bool {
        self.layoutVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_layoutVersion(&mut self, v: i32) {
        self.layoutVersion = ::std::option::Option::Some(v);
    }

    pub fn get_layoutVersion(&self) -> i32 {
        self.layoutVersion.unwrap_or(0)
    }

    fn get_layoutVersion_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.layoutVersion
    }

    fn mut_layoutVersion_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.layoutVersion
    }
}

impl ::protobuf::Message for StartLogSegmentRequestProto {
    fn is_initialized(&self) -> bool {
        if self.reqInfo.is_none() {
            return false;
        }
        if self.txid.is_none() {
            return false;
        }
        for v in &self.reqInfo {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.reqInfo)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.txid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.layoutVersion = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.reqInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.txid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.layoutVersion {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.reqInfo.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.txid {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.layoutVersion {
            os.write_sint32(3, v)?;
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

impl ::protobuf::MessageStatic for StartLogSegmentRequestProto {
    fn new() -> StartLogSegmentRequestProto {
        StartLogSegmentRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StartLogSegmentRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RequestInfoProto>>(
                    "reqInfo",
                    StartLogSegmentRequestProto::get_reqInfo_for_reflect,
                    StartLogSegmentRequestProto::mut_reqInfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "txid",
                    StartLogSegmentRequestProto::get_txid_for_reflect,
                    StartLogSegmentRequestProto::mut_txid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "layoutVersion",
                    StartLogSegmentRequestProto::get_layoutVersion_for_reflect,
                    StartLogSegmentRequestProto::mut_layoutVersion_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StartLogSegmentRequestProto>(
                    "StartLogSegmentRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StartLogSegmentRequestProto {
    fn clear(&mut self) {
        self.clear_reqInfo();
        self.clear_txid();
        self.clear_layoutVersion();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StartLogSegmentRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartLogSegmentRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StartLogSegmentResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StartLogSegmentResponseProto {}

impl StartLogSegmentResponseProto {
    pub fn new() -> StartLogSegmentResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StartLogSegmentResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<StartLogSegmentResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StartLogSegmentResponseProto,
        };
        unsafe {
            instance.get(StartLogSegmentResponseProto::new)
        }
    }
}

impl ::protobuf::Message for StartLogSegmentResponseProto {
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

impl ::protobuf::MessageStatic for StartLogSegmentResponseProto {
    fn new() -> StartLogSegmentResponseProto {
        StartLogSegmentResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StartLogSegmentResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<StartLogSegmentResponseProto>(
                    "StartLogSegmentResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StartLogSegmentResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StartLogSegmentResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartLogSegmentResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FinalizeLogSegmentRequestProto {
    // message fields
    reqInfo: ::protobuf::SingularPtrField<RequestInfoProto>,
    startTxId: ::std::option::Option<u64>,
    endTxId: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FinalizeLogSegmentRequestProto {}

impl FinalizeLogSegmentRequestProto {
    pub fn new() -> FinalizeLogSegmentRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FinalizeLogSegmentRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<FinalizeLogSegmentRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FinalizeLogSegmentRequestProto,
        };
        unsafe {
            instance.get(FinalizeLogSegmentRequestProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.RequestInfoProto reqInfo = 1;

    pub fn clear_reqInfo(&mut self) {
        self.reqInfo.clear();
    }

    pub fn has_reqInfo(&self) -> bool {
        self.reqInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reqInfo(&mut self, v: RequestInfoProto) {
        self.reqInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reqInfo(&mut self) -> &mut RequestInfoProto {
        if self.reqInfo.is_none() {
            self.reqInfo.set_default();
        }
        self.reqInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_reqInfo(&mut self) -> RequestInfoProto {
        self.reqInfo.take().unwrap_or_else(|| RequestInfoProto::new())
    }

    pub fn get_reqInfo(&self) -> &RequestInfoProto {
        self.reqInfo.as_ref().unwrap_or_else(|| RequestInfoProto::default_instance())
    }

    fn get_reqInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<RequestInfoProto> {
        &self.reqInfo
    }

    fn mut_reqInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RequestInfoProto> {
        &mut self.reqInfo
    }

    // required uint64 startTxId = 2;

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

    // required uint64 endTxId = 3;

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
}

impl ::protobuf::Message for FinalizeLogSegmentRequestProto {
    fn is_initialized(&self) -> bool {
        if self.reqInfo.is_none() {
            return false;
        }
        if self.startTxId.is_none() {
            return false;
        }
        if self.endTxId.is_none() {
            return false;
        }
        for v in &self.reqInfo {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.reqInfo)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.startTxId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.endTxId = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.reqInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.startTxId {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.endTxId {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.reqInfo.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.startTxId {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.endTxId {
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

impl ::protobuf::MessageStatic for FinalizeLogSegmentRequestProto {
    fn new() -> FinalizeLogSegmentRequestProto {
        FinalizeLogSegmentRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FinalizeLogSegmentRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RequestInfoProto>>(
                    "reqInfo",
                    FinalizeLogSegmentRequestProto::get_reqInfo_for_reflect,
                    FinalizeLogSegmentRequestProto::mut_reqInfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "startTxId",
                    FinalizeLogSegmentRequestProto::get_startTxId_for_reflect,
                    FinalizeLogSegmentRequestProto::mut_startTxId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "endTxId",
                    FinalizeLogSegmentRequestProto::get_endTxId_for_reflect,
                    FinalizeLogSegmentRequestProto::mut_endTxId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FinalizeLogSegmentRequestProto>(
                    "FinalizeLogSegmentRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FinalizeLogSegmentRequestProto {
    fn clear(&mut self) {
        self.clear_reqInfo();
        self.clear_startTxId();
        self.clear_endTxId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FinalizeLogSegmentRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FinalizeLogSegmentRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FinalizeLogSegmentResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FinalizeLogSegmentResponseProto {}

impl FinalizeLogSegmentResponseProto {
    pub fn new() -> FinalizeLogSegmentResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FinalizeLogSegmentResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<FinalizeLogSegmentResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FinalizeLogSegmentResponseProto,
        };
        unsafe {
            instance.get(FinalizeLogSegmentResponseProto::new)
        }
    }
}

impl ::protobuf::Message for FinalizeLogSegmentResponseProto {
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

impl ::protobuf::MessageStatic for FinalizeLogSegmentResponseProto {
    fn new() -> FinalizeLogSegmentResponseProto {
        FinalizeLogSegmentResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FinalizeLogSegmentResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<FinalizeLogSegmentResponseProto>(
                    "FinalizeLogSegmentResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FinalizeLogSegmentResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FinalizeLogSegmentResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FinalizeLogSegmentResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PurgeLogsRequestProto {
    // message fields
    reqInfo: ::protobuf::SingularPtrField<RequestInfoProto>,
    minTxIdToKeep: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PurgeLogsRequestProto {}

impl PurgeLogsRequestProto {
    pub fn new() -> PurgeLogsRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PurgeLogsRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<PurgeLogsRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PurgeLogsRequestProto,
        };
        unsafe {
            instance.get(PurgeLogsRequestProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.RequestInfoProto reqInfo = 1;

    pub fn clear_reqInfo(&mut self) {
        self.reqInfo.clear();
    }

    pub fn has_reqInfo(&self) -> bool {
        self.reqInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reqInfo(&mut self, v: RequestInfoProto) {
        self.reqInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reqInfo(&mut self) -> &mut RequestInfoProto {
        if self.reqInfo.is_none() {
            self.reqInfo.set_default();
        }
        self.reqInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_reqInfo(&mut self) -> RequestInfoProto {
        self.reqInfo.take().unwrap_or_else(|| RequestInfoProto::new())
    }

    pub fn get_reqInfo(&self) -> &RequestInfoProto {
        self.reqInfo.as_ref().unwrap_or_else(|| RequestInfoProto::default_instance())
    }

    fn get_reqInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<RequestInfoProto> {
        &self.reqInfo
    }

    fn mut_reqInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RequestInfoProto> {
        &mut self.reqInfo
    }

    // required uint64 minTxIdToKeep = 2;

    pub fn clear_minTxIdToKeep(&mut self) {
        self.minTxIdToKeep = ::std::option::Option::None;
    }

    pub fn has_minTxIdToKeep(&self) -> bool {
        self.minTxIdToKeep.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minTxIdToKeep(&mut self, v: u64) {
        self.minTxIdToKeep = ::std::option::Option::Some(v);
    }

    pub fn get_minTxIdToKeep(&self) -> u64 {
        self.minTxIdToKeep.unwrap_or(0)
    }

    fn get_minTxIdToKeep_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.minTxIdToKeep
    }

    fn mut_minTxIdToKeep_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.minTxIdToKeep
    }
}

impl ::protobuf::Message for PurgeLogsRequestProto {
    fn is_initialized(&self) -> bool {
        if self.reqInfo.is_none() {
            return false;
        }
        if self.minTxIdToKeep.is_none() {
            return false;
        }
        for v in &self.reqInfo {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.reqInfo)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.minTxIdToKeep = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.reqInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.minTxIdToKeep {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.reqInfo.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.minTxIdToKeep {
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

impl ::protobuf::MessageStatic for PurgeLogsRequestProto {
    fn new() -> PurgeLogsRequestProto {
        PurgeLogsRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<PurgeLogsRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RequestInfoProto>>(
                    "reqInfo",
                    PurgeLogsRequestProto::get_reqInfo_for_reflect,
                    PurgeLogsRequestProto::mut_reqInfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "minTxIdToKeep",
                    PurgeLogsRequestProto::get_minTxIdToKeep_for_reflect,
                    PurgeLogsRequestProto::mut_minTxIdToKeep_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PurgeLogsRequestProto>(
                    "PurgeLogsRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PurgeLogsRequestProto {
    fn clear(&mut self) {
        self.clear_reqInfo();
        self.clear_minTxIdToKeep();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PurgeLogsRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PurgeLogsRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PurgeLogsResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PurgeLogsResponseProto {}

impl PurgeLogsResponseProto {
    pub fn new() -> PurgeLogsResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PurgeLogsResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<PurgeLogsResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PurgeLogsResponseProto,
        };
        unsafe {
            instance.get(PurgeLogsResponseProto::new)
        }
    }
}

impl ::protobuf::Message for PurgeLogsResponseProto {
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

impl ::protobuf::MessageStatic for PurgeLogsResponseProto {
    fn new() -> PurgeLogsResponseProto {
        PurgeLogsResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<PurgeLogsResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<PurgeLogsResponseProto>(
                    "PurgeLogsResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PurgeLogsResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PurgeLogsResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PurgeLogsResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IsFormattedRequestProto {
    // message fields
    jid: ::protobuf::SingularPtrField<JournalIdProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IsFormattedRequestProto {}

impl IsFormattedRequestProto {
    pub fn new() -> IsFormattedRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IsFormattedRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<IsFormattedRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IsFormattedRequestProto,
        };
        unsafe {
            instance.get(IsFormattedRequestProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.JournalIdProto jid = 1;

    pub fn clear_jid(&mut self) {
        self.jid.clear();
    }

    pub fn has_jid(&self) -> bool {
        self.jid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jid(&mut self, v: JournalIdProto) {
        self.jid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_jid(&mut self) -> &mut JournalIdProto {
        if self.jid.is_none() {
            self.jid.set_default();
        }
        self.jid.as_mut().unwrap()
    }

    // Take field
    pub fn take_jid(&mut self) -> JournalIdProto {
        self.jid.take().unwrap_or_else(|| JournalIdProto::new())
    }

    pub fn get_jid(&self) -> &JournalIdProto {
        self.jid.as_ref().unwrap_or_else(|| JournalIdProto::default_instance())
    }

    fn get_jid_for_reflect(&self) -> &::protobuf::SingularPtrField<JournalIdProto> {
        &self.jid
    }

    fn mut_jid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JournalIdProto> {
        &mut self.jid
    }
}

impl ::protobuf::Message for IsFormattedRequestProto {
    fn is_initialized(&self) -> bool {
        if self.jid.is_none() {
            return false;
        }
        for v in &self.jid {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.jid)?;
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
        if let Some(ref v) = self.jid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.jid.as_ref() {
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

impl ::protobuf::MessageStatic for IsFormattedRequestProto {
    fn new() -> IsFormattedRequestProto {
        IsFormattedRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<IsFormattedRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JournalIdProto>>(
                    "jid",
                    IsFormattedRequestProto::get_jid_for_reflect,
                    IsFormattedRequestProto::mut_jid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IsFormattedRequestProto>(
                    "IsFormattedRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IsFormattedRequestProto {
    fn clear(&mut self) {
        self.clear_jid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IsFormattedRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IsFormattedRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IsFormattedResponseProto {
    // message fields
    isFormatted: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IsFormattedResponseProto {}

impl IsFormattedResponseProto {
    pub fn new() -> IsFormattedResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IsFormattedResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<IsFormattedResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IsFormattedResponseProto,
        };
        unsafe {
            instance.get(IsFormattedResponseProto::new)
        }
    }

    // required bool isFormatted = 1;

    pub fn clear_isFormatted(&mut self) {
        self.isFormatted = ::std::option::Option::None;
    }

    pub fn has_isFormatted(&self) -> bool {
        self.isFormatted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_isFormatted(&mut self, v: bool) {
        self.isFormatted = ::std::option::Option::Some(v);
    }

    pub fn get_isFormatted(&self) -> bool {
        self.isFormatted.unwrap_or(false)
    }

    fn get_isFormatted_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.isFormatted
    }

    fn mut_isFormatted_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.isFormatted
    }
}

impl ::protobuf::Message for IsFormattedResponseProto {
    fn is_initialized(&self) -> bool {
        if self.isFormatted.is_none() {
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
                    let tmp = is.read_bool()?;
                    self.isFormatted = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.isFormatted {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.isFormatted {
            os.write_bool(1, v)?;
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

impl ::protobuf::MessageStatic for IsFormattedResponseProto {
    fn new() -> IsFormattedResponseProto {
        IsFormattedResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<IsFormattedResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "isFormatted",
                    IsFormattedResponseProto::get_isFormatted_for_reflect,
                    IsFormattedResponseProto::mut_isFormatted_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IsFormattedResponseProto>(
                    "IsFormattedResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IsFormattedResponseProto {
    fn clear(&mut self) {
        self.clear_isFormatted();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IsFormattedResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IsFormattedResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DiscardSegmentsRequestProto {
    // message fields
    jid: ::protobuf::SingularPtrField<JournalIdProto>,
    startTxId: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DiscardSegmentsRequestProto {}

impl DiscardSegmentsRequestProto {
    pub fn new() -> DiscardSegmentsRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DiscardSegmentsRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<DiscardSegmentsRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DiscardSegmentsRequestProto,
        };
        unsafe {
            instance.get(DiscardSegmentsRequestProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.JournalIdProto jid = 1;

    pub fn clear_jid(&mut self) {
        self.jid.clear();
    }

    pub fn has_jid(&self) -> bool {
        self.jid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jid(&mut self, v: JournalIdProto) {
        self.jid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_jid(&mut self) -> &mut JournalIdProto {
        if self.jid.is_none() {
            self.jid.set_default();
        }
        self.jid.as_mut().unwrap()
    }

    // Take field
    pub fn take_jid(&mut self) -> JournalIdProto {
        self.jid.take().unwrap_or_else(|| JournalIdProto::new())
    }

    pub fn get_jid(&self) -> &JournalIdProto {
        self.jid.as_ref().unwrap_or_else(|| JournalIdProto::default_instance())
    }

    fn get_jid_for_reflect(&self) -> &::protobuf::SingularPtrField<JournalIdProto> {
        &self.jid
    }

    fn mut_jid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JournalIdProto> {
        &mut self.jid
    }

    // required uint64 startTxId = 2;

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
}

impl ::protobuf::Message for DiscardSegmentsRequestProto {
    fn is_initialized(&self) -> bool {
        if self.jid.is_none() {
            return false;
        }
        if self.startTxId.is_none() {
            return false;
        }
        for v in &self.jid {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.jid)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.startTxId = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.jid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.startTxId {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.jid.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.startTxId {
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

impl ::protobuf::MessageStatic for DiscardSegmentsRequestProto {
    fn new() -> DiscardSegmentsRequestProto {
        DiscardSegmentsRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DiscardSegmentsRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JournalIdProto>>(
                    "jid",
                    DiscardSegmentsRequestProto::get_jid_for_reflect,
                    DiscardSegmentsRequestProto::mut_jid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "startTxId",
                    DiscardSegmentsRequestProto::get_startTxId_for_reflect,
                    DiscardSegmentsRequestProto::mut_startTxId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DiscardSegmentsRequestProto>(
                    "DiscardSegmentsRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DiscardSegmentsRequestProto {
    fn clear(&mut self) {
        self.clear_jid();
        self.clear_startTxId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DiscardSegmentsRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DiscardSegmentsRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DiscardSegmentsResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DiscardSegmentsResponseProto {}

impl DiscardSegmentsResponseProto {
    pub fn new() -> DiscardSegmentsResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DiscardSegmentsResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<DiscardSegmentsResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DiscardSegmentsResponseProto,
        };
        unsafe {
            instance.get(DiscardSegmentsResponseProto::new)
        }
    }
}

impl ::protobuf::Message for DiscardSegmentsResponseProto {
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

impl ::protobuf::MessageStatic for DiscardSegmentsResponseProto {
    fn new() -> DiscardSegmentsResponseProto {
        DiscardSegmentsResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DiscardSegmentsResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DiscardSegmentsResponseProto>(
                    "DiscardSegmentsResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DiscardSegmentsResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DiscardSegmentsResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DiscardSegmentsResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetJournalCTimeRequestProto {
    // message fields
    jid: ::protobuf::SingularPtrField<JournalIdProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetJournalCTimeRequestProto {}

impl GetJournalCTimeRequestProto {
    pub fn new() -> GetJournalCTimeRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetJournalCTimeRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetJournalCTimeRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetJournalCTimeRequestProto,
        };
        unsafe {
            instance.get(GetJournalCTimeRequestProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.JournalIdProto jid = 1;

    pub fn clear_jid(&mut self) {
        self.jid.clear();
    }

    pub fn has_jid(&self) -> bool {
        self.jid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jid(&mut self, v: JournalIdProto) {
        self.jid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_jid(&mut self) -> &mut JournalIdProto {
        if self.jid.is_none() {
            self.jid.set_default();
        }
        self.jid.as_mut().unwrap()
    }

    // Take field
    pub fn take_jid(&mut self) -> JournalIdProto {
        self.jid.take().unwrap_or_else(|| JournalIdProto::new())
    }

    pub fn get_jid(&self) -> &JournalIdProto {
        self.jid.as_ref().unwrap_or_else(|| JournalIdProto::default_instance())
    }

    fn get_jid_for_reflect(&self) -> &::protobuf::SingularPtrField<JournalIdProto> {
        &self.jid
    }

    fn mut_jid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JournalIdProto> {
        &mut self.jid
    }
}

impl ::protobuf::Message for GetJournalCTimeRequestProto {
    fn is_initialized(&self) -> bool {
        if self.jid.is_none() {
            return false;
        }
        for v in &self.jid {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.jid)?;
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
        if let Some(ref v) = self.jid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.jid.as_ref() {
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

impl ::protobuf::MessageStatic for GetJournalCTimeRequestProto {
    fn new() -> GetJournalCTimeRequestProto {
        GetJournalCTimeRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetJournalCTimeRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JournalIdProto>>(
                    "jid",
                    GetJournalCTimeRequestProto::get_jid_for_reflect,
                    GetJournalCTimeRequestProto::mut_jid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetJournalCTimeRequestProto>(
                    "GetJournalCTimeRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetJournalCTimeRequestProto {
    fn clear(&mut self) {
        self.clear_jid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetJournalCTimeRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetJournalCTimeRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetJournalCTimeResponseProto {
    // message fields
    resultCTime: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetJournalCTimeResponseProto {}

impl GetJournalCTimeResponseProto {
    pub fn new() -> GetJournalCTimeResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetJournalCTimeResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetJournalCTimeResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetJournalCTimeResponseProto,
        };
        unsafe {
            instance.get(GetJournalCTimeResponseProto::new)
        }
    }

    // required int64 resultCTime = 1;

    pub fn clear_resultCTime(&mut self) {
        self.resultCTime = ::std::option::Option::None;
    }

    pub fn has_resultCTime(&self) -> bool {
        self.resultCTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resultCTime(&mut self, v: i64) {
        self.resultCTime = ::std::option::Option::Some(v);
    }

    pub fn get_resultCTime(&self) -> i64 {
        self.resultCTime.unwrap_or(0)
    }

    fn get_resultCTime_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.resultCTime
    }

    fn mut_resultCTime_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.resultCTime
    }
}

impl ::protobuf::Message for GetJournalCTimeResponseProto {
    fn is_initialized(&self) -> bool {
        if self.resultCTime.is_none() {
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
                    self.resultCTime = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.resultCTime {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.resultCTime {
            os.write_int64(1, v)?;
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

impl ::protobuf::MessageStatic for GetJournalCTimeResponseProto {
    fn new() -> GetJournalCTimeResponseProto {
        GetJournalCTimeResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetJournalCTimeResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "resultCTime",
                    GetJournalCTimeResponseProto::get_resultCTime_for_reflect,
                    GetJournalCTimeResponseProto::mut_resultCTime_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetJournalCTimeResponseProto>(
                    "GetJournalCTimeResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetJournalCTimeResponseProto {
    fn clear(&mut self) {
        self.clear_resultCTime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetJournalCTimeResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetJournalCTimeResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DoPreUpgradeRequestProto {
    // message fields
    jid: ::protobuf::SingularPtrField<JournalIdProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DoPreUpgradeRequestProto {}

impl DoPreUpgradeRequestProto {
    pub fn new() -> DoPreUpgradeRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DoPreUpgradeRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<DoPreUpgradeRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DoPreUpgradeRequestProto,
        };
        unsafe {
            instance.get(DoPreUpgradeRequestProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.JournalIdProto jid = 1;

    pub fn clear_jid(&mut self) {
        self.jid.clear();
    }

    pub fn has_jid(&self) -> bool {
        self.jid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jid(&mut self, v: JournalIdProto) {
        self.jid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_jid(&mut self) -> &mut JournalIdProto {
        if self.jid.is_none() {
            self.jid.set_default();
        }
        self.jid.as_mut().unwrap()
    }

    // Take field
    pub fn take_jid(&mut self) -> JournalIdProto {
        self.jid.take().unwrap_or_else(|| JournalIdProto::new())
    }

    pub fn get_jid(&self) -> &JournalIdProto {
        self.jid.as_ref().unwrap_or_else(|| JournalIdProto::default_instance())
    }

    fn get_jid_for_reflect(&self) -> &::protobuf::SingularPtrField<JournalIdProto> {
        &self.jid
    }

    fn mut_jid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JournalIdProto> {
        &mut self.jid
    }
}

impl ::protobuf::Message for DoPreUpgradeRequestProto {
    fn is_initialized(&self) -> bool {
        if self.jid.is_none() {
            return false;
        }
        for v in &self.jid {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.jid)?;
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
        if let Some(ref v) = self.jid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.jid.as_ref() {
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

impl ::protobuf::MessageStatic for DoPreUpgradeRequestProto {
    fn new() -> DoPreUpgradeRequestProto {
        DoPreUpgradeRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DoPreUpgradeRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JournalIdProto>>(
                    "jid",
                    DoPreUpgradeRequestProto::get_jid_for_reflect,
                    DoPreUpgradeRequestProto::mut_jid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DoPreUpgradeRequestProto>(
                    "DoPreUpgradeRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DoPreUpgradeRequestProto {
    fn clear(&mut self) {
        self.clear_jid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DoPreUpgradeRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DoPreUpgradeRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DoPreUpgradeResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DoPreUpgradeResponseProto {}

impl DoPreUpgradeResponseProto {
    pub fn new() -> DoPreUpgradeResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DoPreUpgradeResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<DoPreUpgradeResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DoPreUpgradeResponseProto,
        };
        unsafe {
            instance.get(DoPreUpgradeResponseProto::new)
        }
    }
}

impl ::protobuf::Message for DoPreUpgradeResponseProto {
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

impl ::protobuf::MessageStatic for DoPreUpgradeResponseProto {
    fn new() -> DoPreUpgradeResponseProto {
        DoPreUpgradeResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DoPreUpgradeResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DoPreUpgradeResponseProto>(
                    "DoPreUpgradeResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DoPreUpgradeResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DoPreUpgradeResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DoPreUpgradeResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DoUpgradeRequestProto {
    // message fields
    jid: ::protobuf::SingularPtrField<JournalIdProto>,
    sInfo: ::protobuf::SingularPtrField<super::hdfs::StorageInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DoUpgradeRequestProto {}

impl DoUpgradeRequestProto {
    pub fn new() -> DoUpgradeRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DoUpgradeRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<DoUpgradeRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DoUpgradeRequestProto,
        };
        unsafe {
            instance.get(DoUpgradeRequestProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.JournalIdProto jid = 1;

    pub fn clear_jid(&mut self) {
        self.jid.clear();
    }

    pub fn has_jid(&self) -> bool {
        self.jid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jid(&mut self, v: JournalIdProto) {
        self.jid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_jid(&mut self) -> &mut JournalIdProto {
        if self.jid.is_none() {
            self.jid.set_default();
        }
        self.jid.as_mut().unwrap()
    }

    // Take field
    pub fn take_jid(&mut self) -> JournalIdProto {
        self.jid.take().unwrap_or_else(|| JournalIdProto::new())
    }

    pub fn get_jid(&self) -> &JournalIdProto {
        self.jid.as_ref().unwrap_or_else(|| JournalIdProto::default_instance())
    }

    fn get_jid_for_reflect(&self) -> &::protobuf::SingularPtrField<JournalIdProto> {
        &self.jid
    }

    fn mut_jid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JournalIdProto> {
        &mut self.jid
    }

    // required .hadoop.hdfs.StorageInfoProto sInfo = 2;

    pub fn clear_sInfo(&mut self) {
        self.sInfo.clear();
    }

    pub fn has_sInfo(&self) -> bool {
        self.sInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sInfo(&mut self, v: super::hdfs::StorageInfoProto) {
        self.sInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sInfo(&mut self) -> &mut super::hdfs::StorageInfoProto {
        if self.sInfo.is_none() {
            self.sInfo.set_default();
        }
        self.sInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_sInfo(&mut self) -> super::hdfs::StorageInfoProto {
        self.sInfo.take().unwrap_or_else(|| super::hdfs::StorageInfoProto::new())
    }

    pub fn get_sInfo(&self) -> &super::hdfs::StorageInfoProto {
        self.sInfo.as_ref().unwrap_or_else(|| super::hdfs::StorageInfoProto::default_instance())
    }

    fn get_sInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::StorageInfoProto> {
        &self.sInfo
    }

    fn mut_sInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::StorageInfoProto> {
        &mut self.sInfo
    }
}

impl ::protobuf::Message for DoUpgradeRequestProto {
    fn is_initialized(&self) -> bool {
        if self.jid.is_none() {
            return false;
        }
        if self.sInfo.is_none() {
            return false;
        }
        for v in &self.jid {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.sInfo {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.jid)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.sInfo)?;
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
        if let Some(ref v) = self.jid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.sInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.jid.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.sInfo.as_ref() {
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

impl ::protobuf::MessageStatic for DoUpgradeRequestProto {
    fn new() -> DoUpgradeRequestProto {
        DoUpgradeRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DoUpgradeRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JournalIdProto>>(
                    "jid",
                    DoUpgradeRequestProto::get_jid_for_reflect,
                    DoUpgradeRequestProto::mut_jid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::StorageInfoProto>>(
                    "sInfo",
                    DoUpgradeRequestProto::get_sInfo_for_reflect,
                    DoUpgradeRequestProto::mut_sInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DoUpgradeRequestProto>(
                    "DoUpgradeRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DoUpgradeRequestProto {
    fn clear(&mut self) {
        self.clear_jid();
        self.clear_sInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DoUpgradeRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DoUpgradeRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DoUpgradeResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DoUpgradeResponseProto {}

impl DoUpgradeResponseProto {
    pub fn new() -> DoUpgradeResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DoUpgradeResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<DoUpgradeResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DoUpgradeResponseProto,
        };
        unsafe {
            instance.get(DoUpgradeResponseProto::new)
        }
    }
}

impl ::protobuf::Message for DoUpgradeResponseProto {
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

impl ::protobuf::MessageStatic for DoUpgradeResponseProto {
    fn new() -> DoUpgradeResponseProto {
        DoUpgradeResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DoUpgradeResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DoUpgradeResponseProto>(
                    "DoUpgradeResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DoUpgradeResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DoUpgradeResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DoUpgradeResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DoFinalizeRequestProto {
    // message fields
    jid: ::protobuf::SingularPtrField<JournalIdProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DoFinalizeRequestProto {}

impl DoFinalizeRequestProto {
    pub fn new() -> DoFinalizeRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DoFinalizeRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<DoFinalizeRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DoFinalizeRequestProto,
        };
        unsafe {
            instance.get(DoFinalizeRequestProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.JournalIdProto jid = 1;

    pub fn clear_jid(&mut self) {
        self.jid.clear();
    }

    pub fn has_jid(&self) -> bool {
        self.jid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jid(&mut self, v: JournalIdProto) {
        self.jid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_jid(&mut self) -> &mut JournalIdProto {
        if self.jid.is_none() {
            self.jid.set_default();
        }
        self.jid.as_mut().unwrap()
    }

    // Take field
    pub fn take_jid(&mut self) -> JournalIdProto {
        self.jid.take().unwrap_or_else(|| JournalIdProto::new())
    }

    pub fn get_jid(&self) -> &JournalIdProto {
        self.jid.as_ref().unwrap_or_else(|| JournalIdProto::default_instance())
    }

    fn get_jid_for_reflect(&self) -> &::protobuf::SingularPtrField<JournalIdProto> {
        &self.jid
    }

    fn mut_jid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JournalIdProto> {
        &mut self.jid
    }
}

impl ::protobuf::Message for DoFinalizeRequestProto {
    fn is_initialized(&self) -> bool {
        if self.jid.is_none() {
            return false;
        }
        for v in &self.jid {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.jid)?;
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
        if let Some(ref v) = self.jid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.jid.as_ref() {
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

impl ::protobuf::MessageStatic for DoFinalizeRequestProto {
    fn new() -> DoFinalizeRequestProto {
        DoFinalizeRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DoFinalizeRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JournalIdProto>>(
                    "jid",
                    DoFinalizeRequestProto::get_jid_for_reflect,
                    DoFinalizeRequestProto::mut_jid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DoFinalizeRequestProto>(
                    "DoFinalizeRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DoFinalizeRequestProto {
    fn clear(&mut self) {
        self.clear_jid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DoFinalizeRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DoFinalizeRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DoFinalizeResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DoFinalizeResponseProto {}

impl DoFinalizeResponseProto {
    pub fn new() -> DoFinalizeResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DoFinalizeResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<DoFinalizeResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DoFinalizeResponseProto,
        };
        unsafe {
            instance.get(DoFinalizeResponseProto::new)
        }
    }
}

impl ::protobuf::Message for DoFinalizeResponseProto {
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

impl ::protobuf::MessageStatic for DoFinalizeResponseProto {
    fn new() -> DoFinalizeResponseProto {
        DoFinalizeResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DoFinalizeResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DoFinalizeResponseProto>(
                    "DoFinalizeResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DoFinalizeResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DoFinalizeResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DoFinalizeResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CanRollBackRequestProto {
    // message fields
    jid: ::protobuf::SingularPtrField<JournalIdProto>,
    storage: ::protobuf::SingularPtrField<super::hdfs::StorageInfoProto>,
    prevStorage: ::protobuf::SingularPtrField<super::hdfs::StorageInfoProto>,
    targetLayoutVersion: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CanRollBackRequestProto {}

impl CanRollBackRequestProto {
    pub fn new() -> CanRollBackRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CanRollBackRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<CanRollBackRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CanRollBackRequestProto,
        };
        unsafe {
            instance.get(CanRollBackRequestProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.JournalIdProto jid = 1;

    pub fn clear_jid(&mut self) {
        self.jid.clear();
    }

    pub fn has_jid(&self) -> bool {
        self.jid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jid(&mut self, v: JournalIdProto) {
        self.jid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_jid(&mut self) -> &mut JournalIdProto {
        if self.jid.is_none() {
            self.jid.set_default();
        }
        self.jid.as_mut().unwrap()
    }

    // Take field
    pub fn take_jid(&mut self) -> JournalIdProto {
        self.jid.take().unwrap_or_else(|| JournalIdProto::new())
    }

    pub fn get_jid(&self) -> &JournalIdProto {
        self.jid.as_ref().unwrap_or_else(|| JournalIdProto::default_instance())
    }

    fn get_jid_for_reflect(&self) -> &::protobuf::SingularPtrField<JournalIdProto> {
        &self.jid
    }

    fn mut_jid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JournalIdProto> {
        &mut self.jid
    }

    // required .hadoop.hdfs.StorageInfoProto storage = 2;

    pub fn clear_storage(&mut self) {
        self.storage.clear();
    }

    pub fn has_storage(&self) -> bool {
        self.storage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storage(&mut self, v: super::hdfs::StorageInfoProto) {
        self.storage = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storage(&mut self) -> &mut super::hdfs::StorageInfoProto {
        if self.storage.is_none() {
            self.storage.set_default();
        }
        self.storage.as_mut().unwrap()
    }

    // Take field
    pub fn take_storage(&mut self) -> super::hdfs::StorageInfoProto {
        self.storage.take().unwrap_or_else(|| super::hdfs::StorageInfoProto::new())
    }

    pub fn get_storage(&self) -> &super::hdfs::StorageInfoProto {
        self.storage.as_ref().unwrap_or_else(|| super::hdfs::StorageInfoProto::default_instance())
    }

    fn get_storage_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::StorageInfoProto> {
        &self.storage
    }

    fn mut_storage_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::StorageInfoProto> {
        &mut self.storage
    }

    // required .hadoop.hdfs.StorageInfoProto prevStorage = 3;

    pub fn clear_prevStorage(&mut self) {
        self.prevStorage.clear();
    }

    pub fn has_prevStorage(&self) -> bool {
        self.prevStorage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prevStorage(&mut self, v: super::hdfs::StorageInfoProto) {
        self.prevStorage = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prevStorage(&mut self) -> &mut super::hdfs::StorageInfoProto {
        if self.prevStorage.is_none() {
            self.prevStorage.set_default();
        }
        self.prevStorage.as_mut().unwrap()
    }

    // Take field
    pub fn take_prevStorage(&mut self) -> super::hdfs::StorageInfoProto {
        self.prevStorage.take().unwrap_or_else(|| super::hdfs::StorageInfoProto::new())
    }

    pub fn get_prevStorage(&self) -> &super::hdfs::StorageInfoProto {
        self.prevStorage.as_ref().unwrap_or_else(|| super::hdfs::StorageInfoProto::default_instance())
    }

    fn get_prevStorage_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::StorageInfoProto> {
        &self.prevStorage
    }

    fn mut_prevStorage_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::StorageInfoProto> {
        &mut self.prevStorage
    }

    // required int32 targetLayoutVersion = 4;

    pub fn clear_targetLayoutVersion(&mut self) {
        self.targetLayoutVersion = ::std::option::Option::None;
    }

    pub fn has_targetLayoutVersion(&self) -> bool {
        self.targetLayoutVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_targetLayoutVersion(&mut self, v: i32) {
        self.targetLayoutVersion = ::std::option::Option::Some(v);
    }

    pub fn get_targetLayoutVersion(&self) -> i32 {
        self.targetLayoutVersion.unwrap_or(0)
    }

    fn get_targetLayoutVersion_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.targetLayoutVersion
    }

    fn mut_targetLayoutVersion_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.targetLayoutVersion
    }
}

impl ::protobuf::Message for CanRollBackRequestProto {
    fn is_initialized(&self) -> bool {
        if self.jid.is_none() {
            return false;
        }
        if self.storage.is_none() {
            return false;
        }
        if self.prevStorage.is_none() {
            return false;
        }
        if self.targetLayoutVersion.is_none() {
            return false;
        }
        for v in &self.jid {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.storage {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.prevStorage {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.jid)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.storage)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.prevStorage)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.targetLayoutVersion = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.jid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.storage.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.prevStorage.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.targetLayoutVersion {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.jid.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.storage.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.prevStorage.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.targetLayoutVersion {
            os.write_int32(4, v)?;
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

impl ::protobuf::MessageStatic for CanRollBackRequestProto {
    fn new() -> CanRollBackRequestProto {
        CanRollBackRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CanRollBackRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JournalIdProto>>(
                    "jid",
                    CanRollBackRequestProto::get_jid_for_reflect,
                    CanRollBackRequestProto::mut_jid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::StorageInfoProto>>(
                    "storage",
                    CanRollBackRequestProto::get_storage_for_reflect,
                    CanRollBackRequestProto::mut_storage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::StorageInfoProto>>(
                    "prevStorage",
                    CanRollBackRequestProto::get_prevStorage_for_reflect,
                    CanRollBackRequestProto::mut_prevStorage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "targetLayoutVersion",
                    CanRollBackRequestProto::get_targetLayoutVersion_for_reflect,
                    CanRollBackRequestProto::mut_targetLayoutVersion_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CanRollBackRequestProto>(
                    "CanRollBackRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CanRollBackRequestProto {
    fn clear(&mut self) {
        self.clear_jid();
        self.clear_storage();
        self.clear_prevStorage();
        self.clear_targetLayoutVersion();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CanRollBackRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CanRollBackRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CanRollBackResponseProto {
    // message fields
    canRollBack: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CanRollBackResponseProto {}

impl CanRollBackResponseProto {
    pub fn new() -> CanRollBackResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CanRollBackResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<CanRollBackResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CanRollBackResponseProto,
        };
        unsafe {
            instance.get(CanRollBackResponseProto::new)
        }
    }

    // required bool canRollBack = 1;

    pub fn clear_canRollBack(&mut self) {
        self.canRollBack = ::std::option::Option::None;
    }

    pub fn has_canRollBack(&self) -> bool {
        self.canRollBack.is_some()
    }

    // Param is passed by value, moved
    pub fn set_canRollBack(&mut self, v: bool) {
        self.canRollBack = ::std::option::Option::Some(v);
    }

    pub fn get_canRollBack(&self) -> bool {
        self.canRollBack.unwrap_or(false)
    }

    fn get_canRollBack_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.canRollBack
    }

    fn mut_canRollBack_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.canRollBack
    }
}

impl ::protobuf::Message for CanRollBackResponseProto {
    fn is_initialized(&self) -> bool {
        if self.canRollBack.is_none() {
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
                    let tmp = is.read_bool()?;
                    self.canRollBack = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.canRollBack {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.canRollBack {
            os.write_bool(1, v)?;
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

impl ::protobuf::MessageStatic for CanRollBackResponseProto {
    fn new() -> CanRollBackResponseProto {
        CanRollBackResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CanRollBackResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "canRollBack",
                    CanRollBackResponseProto::get_canRollBack_for_reflect,
                    CanRollBackResponseProto::mut_canRollBack_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CanRollBackResponseProto>(
                    "CanRollBackResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CanRollBackResponseProto {
    fn clear(&mut self) {
        self.clear_canRollBack();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CanRollBackResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CanRollBackResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DoRollbackRequestProto {
    // message fields
    jid: ::protobuf::SingularPtrField<JournalIdProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DoRollbackRequestProto {}

impl DoRollbackRequestProto {
    pub fn new() -> DoRollbackRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DoRollbackRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<DoRollbackRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DoRollbackRequestProto,
        };
        unsafe {
            instance.get(DoRollbackRequestProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.JournalIdProto jid = 1;

    pub fn clear_jid(&mut self) {
        self.jid.clear();
    }

    pub fn has_jid(&self) -> bool {
        self.jid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jid(&mut self, v: JournalIdProto) {
        self.jid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_jid(&mut self) -> &mut JournalIdProto {
        if self.jid.is_none() {
            self.jid.set_default();
        }
        self.jid.as_mut().unwrap()
    }

    // Take field
    pub fn take_jid(&mut self) -> JournalIdProto {
        self.jid.take().unwrap_or_else(|| JournalIdProto::new())
    }

    pub fn get_jid(&self) -> &JournalIdProto {
        self.jid.as_ref().unwrap_or_else(|| JournalIdProto::default_instance())
    }

    fn get_jid_for_reflect(&self) -> &::protobuf::SingularPtrField<JournalIdProto> {
        &self.jid
    }

    fn mut_jid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JournalIdProto> {
        &mut self.jid
    }
}

impl ::protobuf::Message for DoRollbackRequestProto {
    fn is_initialized(&self) -> bool {
        if self.jid.is_none() {
            return false;
        }
        for v in &self.jid {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.jid)?;
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
        if let Some(ref v) = self.jid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.jid.as_ref() {
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

impl ::protobuf::MessageStatic for DoRollbackRequestProto {
    fn new() -> DoRollbackRequestProto {
        DoRollbackRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DoRollbackRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JournalIdProto>>(
                    "jid",
                    DoRollbackRequestProto::get_jid_for_reflect,
                    DoRollbackRequestProto::mut_jid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DoRollbackRequestProto>(
                    "DoRollbackRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DoRollbackRequestProto {
    fn clear(&mut self) {
        self.clear_jid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DoRollbackRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DoRollbackRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DoRollbackResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DoRollbackResponseProto {}

impl DoRollbackResponseProto {
    pub fn new() -> DoRollbackResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DoRollbackResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<DoRollbackResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DoRollbackResponseProto,
        };
        unsafe {
            instance.get(DoRollbackResponseProto::new)
        }
    }
}

impl ::protobuf::Message for DoRollbackResponseProto {
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

impl ::protobuf::MessageStatic for DoRollbackResponseProto {
    fn new() -> DoRollbackResponseProto {
        DoRollbackResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DoRollbackResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DoRollbackResponseProto>(
                    "DoRollbackResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DoRollbackResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DoRollbackResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DoRollbackResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetJournalStateRequestProto {
    // message fields
    jid: ::protobuf::SingularPtrField<JournalIdProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetJournalStateRequestProto {}

impl GetJournalStateRequestProto {
    pub fn new() -> GetJournalStateRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetJournalStateRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetJournalStateRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetJournalStateRequestProto,
        };
        unsafe {
            instance.get(GetJournalStateRequestProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.JournalIdProto jid = 1;

    pub fn clear_jid(&mut self) {
        self.jid.clear();
    }

    pub fn has_jid(&self) -> bool {
        self.jid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jid(&mut self, v: JournalIdProto) {
        self.jid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_jid(&mut self) -> &mut JournalIdProto {
        if self.jid.is_none() {
            self.jid.set_default();
        }
        self.jid.as_mut().unwrap()
    }

    // Take field
    pub fn take_jid(&mut self) -> JournalIdProto {
        self.jid.take().unwrap_or_else(|| JournalIdProto::new())
    }

    pub fn get_jid(&self) -> &JournalIdProto {
        self.jid.as_ref().unwrap_or_else(|| JournalIdProto::default_instance())
    }

    fn get_jid_for_reflect(&self) -> &::protobuf::SingularPtrField<JournalIdProto> {
        &self.jid
    }

    fn mut_jid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JournalIdProto> {
        &mut self.jid
    }
}

impl ::protobuf::Message for GetJournalStateRequestProto {
    fn is_initialized(&self) -> bool {
        if self.jid.is_none() {
            return false;
        }
        for v in &self.jid {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.jid)?;
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
        if let Some(ref v) = self.jid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.jid.as_ref() {
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

impl ::protobuf::MessageStatic for GetJournalStateRequestProto {
    fn new() -> GetJournalStateRequestProto {
        GetJournalStateRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetJournalStateRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JournalIdProto>>(
                    "jid",
                    GetJournalStateRequestProto::get_jid_for_reflect,
                    GetJournalStateRequestProto::mut_jid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetJournalStateRequestProto>(
                    "GetJournalStateRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetJournalStateRequestProto {
    fn clear(&mut self) {
        self.clear_jid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetJournalStateRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetJournalStateRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetJournalStateResponseProto {
    // message fields
    lastPromisedEpoch: ::std::option::Option<u64>,
    httpPort: ::std::option::Option<u32>,
    fromURL: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetJournalStateResponseProto {}

impl GetJournalStateResponseProto {
    pub fn new() -> GetJournalStateResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetJournalStateResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetJournalStateResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetJournalStateResponseProto,
        };
        unsafe {
            instance.get(GetJournalStateResponseProto::new)
        }
    }

    // required uint64 lastPromisedEpoch = 1;

    pub fn clear_lastPromisedEpoch(&mut self) {
        self.lastPromisedEpoch = ::std::option::Option::None;
    }

    pub fn has_lastPromisedEpoch(&self) -> bool {
        self.lastPromisedEpoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastPromisedEpoch(&mut self, v: u64) {
        self.lastPromisedEpoch = ::std::option::Option::Some(v);
    }

    pub fn get_lastPromisedEpoch(&self) -> u64 {
        self.lastPromisedEpoch.unwrap_or(0)
    }

    fn get_lastPromisedEpoch_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lastPromisedEpoch
    }

    fn mut_lastPromisedEpoch_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lastPromisedEpoch
    }

    // required uint32 httpPort = 2;

    pub fn clear_httpPort(&mut self) {
        self.httpPort = ::std::option::Option::None;
    }

    pub fn has_httpPort(&self) -> bool {
        self.httpPort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_httpPort(&mut self, v: u32) {
        self.httpPort = ::std::option::Option::Some(v);
    }

    pub fn get_httpPort(&self) -> u32 {
        self.httpPort.unwrap_or(0)
    }

    fn get_httpPort_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.httpPort
    }

    fn mut_httpPort_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.httpPort
    }

    // optional string fromURL = 3;

    pub fn clear_fromURL(&mut self) {
        self.fromURL.clear();
    }

    pub fn has_fromURL(&self) -> bool {
        self.fromURL.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fromURL(&mut self, v: ::std::string::String) {
        self.fromURL = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fromURL(&mut self) -> &mut ::std::string::String {
        if self.fromURL.is_none() {
            self.fromURL.set_default();
        }
        self.fromURL.as_mut().unwrap()
    }

    // Take field
    pub fn take_fromURL(&mut self) -> ::std::string::String {
        self.fromURL.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_fromURL(&self) -> &str {
        match self.fromURL.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_fromURL_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.fromURL
    }

    fn mut_fromURL_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.fromURL
    }
}

impl ::protobuf::Message for GetJournalStateResponseProto {
    fn is_initialized(&self) -> bool {
        if self.lastPromisedEpoch.is_none() {
            return false;
        }
        if self.httpPort.is_none() {
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
                    self.lastPromisedEpoch = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.httpPort = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.fromURL)?;
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
        if let Some(v) = self.lastPromisedEpoch {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.httpPort {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.fromURL.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.lastPromisedEpoch {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.httpPort {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.fromURL.as_ref() {
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

impl ::protobuf::MessageStatic for GetJournalStateResponseProto {
    fn new() -> GetJournalStateResponseProto {
        GetJournalStateResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetJournalStateResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lastPromisedEpoch",
                    GetJournalStateResponseProto::get_lastPromisedEpoch_for_reflect,
                    GetJournalStateResponseProto::mut_lastPromisedEpoch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "httpPort",
                    GetJournalStateResponseProto::get_httpPort_for_reflect,
                    GetJournalStateResponseProto::mut_httpPort_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "fromURL",
                    GetJournalStateResponseProto::get_fromURL_for_reflect,
                    GetJournalStateResponseProto::mut_fromURL_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetJournalStateResponseProto>(
                    "GetJournalStateResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetJournalStateResponseProto {
    fn clear(&mut self) {
        self.clear_lastPromisedEpoch();
        self.clear_httpPort();
        self.clear_fromURL();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetJournalStateResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetJournalStateResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FormatRequestProto {
    // message fields
    jid: ::protobuf::SingularPtrField<JournalIdProto>,
    nsInfo: ::protobuf::SingularPtrField<super::hdfs::NamespaceInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FormatRequestProto {}

impl FormatRequestProto {
    pub fn new() -> FormatRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FormatRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<FormatRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FormatRequestProto,
        };
        unsafe {
            instance.get(FormatRequestProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.JournalIdProto jid = 1;

    pub fn clear_jid(&mut self) {
        self.jid.clear();
    }

    pub fn has_jid(&self) -> bool {
        self.jid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jid(&mut self, v: JournalIdProto) {
        self.jid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_jid(&mut self) -> &mut JournalIdProto {
        if self.jid.is_none() {
            self.jid.set_default();
        }
        self.jid.as_mut().unwrap()
    }

    // Take field
    pub fn take_jid(&mut self) -> JournalIdProto {
        self.jid.take().unwrap_or_else(|| JournalIdProto::new())
    }

    pub fn get_jid(&self) -> &JournalIdProto {
        self.jid.as_ref().unwrap_or_else(|| JournalIdProto::default_instance())
    }

    fn get_jid_for_reflect(&self) -> &::protobuf::SingularPtrField<JournalIdProto> {
        &self.jid
    }

    fn mut_jid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JournalIdProto> {
        &mut self.jid
    }

    // required .hadoop.hdfs.NamespaceInfoProto nsInfo = 2;

    pub fn clear_nsInfo(&mut self) {
        self.nsInfo.clear();
    }

    pub fn has_nsInfo(&self) -> bool {
        self.nsInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nsInfo(&mut self, v: super::hdfs::NamespaceInfoProto) {
        self.nsInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nsInfo(&mut self) -> &mut super::hdfs::NamespaceInfoProto {
        if self.nsInfo.is_none() {
            self.nsInfo.set_default();
        }
        self.nsInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_nsInfo(&mut self) -> super::hdfs::NamespaceInfoProto {
        self.nsInfo.take().unwrap_or_else(|| super::hdfs::NamespaceInfoProto::new())
    }

    pub fn get_nsInfo(&self) -> &super::hdfs::NamespaceInfoProto {
        self.nsInfo.as_ref().unwrap_or_else(|| super::hdfs::NamespaceInfoProto::default_instance())
    }

    fn get_nsInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::NamespaceInfoProto> {
        &self.nsInfo
    }

    fn mut_nsInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::NamespaceInfoProto> {
        &mut self.nsInfo
    }
}

impl ::protobuf::Message for FormatRequestProto {
    fn is_initialized(&self) -> bool {
        if self.jid.is_none() {
            return false;
        }
        if self.nsInfo.is_none() {
            return false;
        }
        for v in &self.jid {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.nsInfo {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.jid)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.nsInfo)?;
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
        if let Some(ref v) = self.jid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.nsInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.jid.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.nsInfo.as_ref() {
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

impl ::protobuf::MessageStatic for FormatRequestProto {
    fn new() -> FormatRequestProto {
        FormatRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FormatRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JournalIdProto>>(
                    "jid",
                    FormatRequestProto::get_jid_for_reflect,
                    FormatRequestProto::mut_jid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::NamespaceInfoProto>>(
                    "nsInfo",
                    FormatRequestProto::get_nsInfo_for_reflect,
                    FormatRequestProto::mut_nsInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FormatRequestProto>(
                    "FormatRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FormatRequestProto {
    fn clear(&mut self) {
        self.clear_jid();
        self.clear_nsInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FormatRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FormatRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FormatResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FormatResponseProto {}

impl FormatResponseProto {
    pub fn new() -> FormatResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FormatResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<FormatResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FormatResponseProto,
        };
        unsafe {
            instance.get(FormatResponseProto::new)
        }
    }
}

impl ::protobuf::Message for FormatResponseProto {
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

impl ::protobuf::MessageStatic for FormatResponseProto {
    fn new() -> FormatResponseProto {
        FormatResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FormatResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<FormatResponseProto>(
                    "FormatResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FormatResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FormatResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FormatResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NewEpochRequestProto {
    // message fields
    jid: ::protobuf::SingularPtrField<JournalIdProto>,
    nsInfo: ::protobuf::SingularPtrField<super::hdfs::NamespaceInfoProto>,
    epoch: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NewEpochRequestProto {}

impl NewEpochRequestProto {
    pub fn new() -> NewEpochRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NewEpochRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<NewEpochRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NewEpochRequestProto,
        };
        unsafe {
            instance.get(NewEpochRequestProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.JournalIdProto jid = 1;

    pub fn clear_jid(&mut self) {
        self.jid.clear();
    }

    pub fn has_jid(&self) -> bool {
        self.jid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jid(&mut self, v: JournalIdProto) {
        self.jid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_jid(&mut self) -> &mut JournalIdProto {
        if self.jid.is_none() {
            self.jid.set_default();
        }
        self.jid.as_mut().unwrap()
    }

    // Take field
    pub fn take_jid(&mut self) -> JournalIdProto {
        self.jid.take().unwrap_or_else(|| JournalIdProto::new())
    }

    pub fn get_jid(&self) -> &JournalIdProto {
        self.jid.as_ref().unwrap_or_else(|| JournalIdProto::default_instance())
    }

    fn get_jid_for_reflect(&self) -> &::protobuf::SingularPtrField<JournalIdProto> {
        &self.jid
    }

    fn mut_jid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JournalIdProto> {
        &mut self.jid
    }

    // required .hadoop.hdfs.NamespaceInfoProto nsInfo = 2;

    pub fn clear_nsInfo(&mut self) {
        self.nsInfo.clear();
    }

    pub fn has_nsInfo(&self) -> bool {
        self.nsInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nsInfo(&mut self, v: super::hdfs::NamespaceInfoProto) {
        self.nsInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nsInfo(&mut self) -> &mut super::hdfs::NamespaceInfoProto {
        if self.nsInfo.is_none() {
            self.nsInfo.set_default();
        }
        self.nsInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_nsInfo(&mut self) -> super::hdfs::NamespaceInfoProto {
        self.nsInfo.take().unwrap_or_else(|| super::hdfs::NamespaceInfoProto::new())
    }

    pub fn get_nsInfo(&self) -> &super::hdfs::NamespaceInfoProto {
        self.nsInfo.as_ref().unwrap_or_else(|| super::hdfs::NamespaceInfoProto::default_instance())
    }

    fn get_nsInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::NamespaceInfoProto> {
        &self.nsInfo
    }

    fn mut_nsInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::NamespaceInfoProto> {
        &mut self.nsInfo
    }

    // required uint64 epoch = 3;

    pub fn clear_epoch(&mut self) {
        self.epoch = ::std::option::Option::None;
    }

    pub fn has_epoch(&self) -> bool {
        self.epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_epoch(&mut self, v: u64) {
        self.epoch = ::std::option::Option::Some(v);
    }

    pub fn get_epoch(&self) -> u64 {
        self.epoch.unwrap_or(0)
    }

    fn get_epoch_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.epoch
    }

    fn mut_epoch_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.epoch
    }
}

impl ::protobuf::Message for NewEpochRequestProto {
    fn is_initialized(&self) -> bool {
        if self.jid.is_none() {
            return false;
        }
        if self.nsInfo.is_none() {
            return false;
        }
        if self.epoch.is_none() {
            return false;
        }
        for v in &self.jid {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.nsInfo {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.jid)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.nsInfo)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.epoch = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.jid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.nsInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.epoch {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.jid.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.nsInfo.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.epoch {
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

impl ::protobuf::MessageStatic for NewEpochRequestProto {
    fn new() -> NewEpochRequestProto {
        NewEpochRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<NewEpochRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JournalIdProto>>(
                    "jid",
                    NewEpochRequestProto::get_jid_for_reflect,
                    NewEpochRequestProto::mut_jid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::NamespaceInfoProto>>(
                    "nsInfo",
                    NewEpochRequestProto::get_nsInfo_for_reflect,
                    NewEpochRequestProto::mut_nsInfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "epoch",
                    NewEpochRequestProto::get_epoch_for_reflect,
                    NewEpochRequestProto::mut_epoch_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NewEpochRequestProto>(
                    "NewEpochRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NewEpochRequestProto {
    fn clear(&mut self) {
        self.clear_jid();
        self.clear_nsInfo();
        self.clear_epoch();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NewEpochRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NewEpochRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NewEpochResponseProto {
    // message fields
    lastSegmentTxId: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NewEpochResponseProto {}

impl NewEpochResponseProto {
    pub fn new() -> NewEpochResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NewEpochResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<NewEpochResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NewEpochResponseProto,
        };
        unsafe {
            instance.get(NewEpochResponseProto::new)
        }
    }

    // optional uint64 lastSegmentTxId = 1;

    pub fn clear_lastSegmentTxId(&mut self) {
        self.lastSegmentTxId = ::std::option::Option::None;
    }

    pub fn has_lastSegmentTxId(&self) -> bool {
        self.lastSegmentTxId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastSegmentTxId(&mut self, v: u64) {
        self.lastSegmentTxId = ::std::option::Option::Some(v);
    }

    pub fn get_lastSegmentTxId(&self) -> u64 {
        self.lastSegmentTxId.unwrap_or(0)
    }

    fn get_lastSegmentTxId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lastSegmentTxId
    }

    fn mut_lastSegmentTxId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lastSegmentTxId
    }
}

impl ::protobuf::Message for NewEpochResponseProto {
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
                    self.lastSegmentTxId = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.lastSegmentTxId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.lastSegmentTxId {
            os.write_uint64(1, v)?;
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

impl ::protobuf::MessageStatic for NewEpochResponseProto {
    fn new() -> NewEpochResponseProto {
        NewEpochResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<NewEpochResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lastSegmentTxId",
                    NewEpochResponseProto::get_lastSegmentTxId_for_reflect,
                    NewEpochResponseProto::mut_lastSegmentTxId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NewEpochResponseProto>(
                    "NewEpochResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NewEpochResponseProto {
    fn clear(&mut self) {
        self.clear_lastSegmentTxId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NewEpochResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NewEpochResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetEditLogManifestRequestProto {
    // message fields
    jid: ::protobuf::SingularPtrField<JournalIdProto>,
    sinceTxId: ::std::option::Option<u64>,
    inProgressOk: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetEditLogManifestRequestProto {}

impl GetEditLogManifestRequestProto {
    pub fn new() -> GetEditLogManifestRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetEditLogManifestRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetEditLogManifestRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetEditLogManifestRequestProto,
        };
        unsafe {
            instance.get(GetEditLogManifestRequestProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.JournalIdProto jid = 1;

    pub fn clear_jid(&mut self) {
        self.jid.clear();
    }

    pub fn has_jid(&self) -> bool {
        self.jid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jid(&mut self, v: JournalIdProto) {
        self.jid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_jid(&mut self) -> &mut JournalIdProto {
        if self.jid.is_none() {
            self.jid.set_default();
        }
        self.jid.as_mut().unwrap()
    }

    // Take field
    pub fn take_jid(&mut self) -> JournalIdProto {
        self.jid.take().unwrap_or_else(|| JournalIdProto::new())
    }

    pub fn get_jid(&self) -> &JournalIdProto {
        self.jid.as_ref().unwrap_or_else(|| JournalIdProto::default_instance())
    }

    fn get_jid_for_reflect(&self) -> &::protobuf::SingularPtrField<JournalIdProto> {
        &self.jid
    }

    fn mut_jid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JournalIdProto> {
        &mut self.jid
    }

    // required uint64 sinceTxId = 2;

    pub fn clear_sinceTxId(&mut self) {
        self.sinceTxId = ::std::option::Option::None;
    }

    pub fn has_sinceTxId(&self) -> bool {
        self.sinceTxId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sinceTxId(&mut self, v: u64) {
        self.sinceTxId = ::std::option::Option::Some(v);
    }

    pub fn get_sinceTxId(&self) -> u64 {
        self.sinceTxId.unwrap_or(0)
    }

    fn get_sinceTxId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.sinceTxId
    }

    fn mut_sinceTxId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.sinceTxId
    }

    // optional bool inProgressOk = 4;

    pub fn clear_inProgressOk(&mut self) {
        self.inProgressOk = ::std::option::Option::None;
    }

    pub fn has_inProgressOk(&self) -> bool {
        self.inProgressOk.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inProgressOk(&mut self, v: bool) {
        self.inProgressOk = ::std::option::Option::Some(v);
    }

    pub fn get_inProgressOk(&self) -> bool {
        self.inProgressOk.unwrap_or(false)
    }

    fn get_inProgressOk_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.inProgressOk
    }

    fn mut_inProgressOk_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.inProgressOk
    }
}

impl ::protobuf::Message for GetEditLogManifestRequestProto {
    fn is_initialized(&self) -> bool {
        if self.jid.is_none() {
            return false;
        }
        if self.sinceTxId.is_none() {
            return false;
        }
        for v in &self.jid {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.jid)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.sinceTxId = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.inProgressOk = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.jid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.sinceTxId {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.inProgressOk {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.jid.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.sinceTxId {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.inProgressOk {
            os.write_bool(4, v)?;
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

impl ::protobuf::MessageStatic for GetEditLogManifestRequestProto {
    fn new() -> GetEditLogManifestRequestProto {
        GetEditLogManifestRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetEditLogManifestRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JournalIdProto>>(
                    "jid",
                    GetEditLogManifestRequestProto::get_jid_for_reflect,
                    GetEditLogManifestRequestProto::mut_jid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "sinceTxId",
                    GetEditLogManifestRequestProto::get_sinceTxId_for_reflect,
                    GetEditLogManifestRequestProto::mut_sinceTxId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "inProgressOk",
                    GetEditLogManifestRequestProto::get_inProgressOk_for_reflect,
                    GetEditLogManifestRequestProto::mut_inProgressOk_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetEditLogManifestRequestProto>(
                    "GetEditLogManifestRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetEditLogManifestRequestProto {
    fn clear(&mut self) {
        self.clear_jid();
        self.clear_sinceTxId();
        self.clear_inProgressOk();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetEditLogManifestRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetEditLogManifestRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetEditLogManifestResponseProto {
    // message fields
    manifest: ::protobuf::SingularPtrField<super::hdfs::RemoteEditLogManifestProto>,
    httpPort: ::std::option::Option<u32>,
    fromURL: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetEditLogManifestResponseProto {}

impl GetEditLogManifestResponseProto {
    pub fn new() -> GetEditLogManifestResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetEditLogManifestResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetEditLogManifestResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetEditLogManifestResponseProto,
        };
        unsafe {
            instance.get(GetEditLogManifestResponseProto::new)
        }
    }

    // required .hadoop.hdfs.RemoteEditLogManifestProto manifest = 1;

    pub fn clear_manifest(&mut self) {
        self.manifest.clear();
    }

    pub fn has_manifest(&self) -> bool {
        self.manifest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_manifest(&mut self, v: super::hdfs::RemoteEditLogManifestProto) {
        self.manifest = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_manifest(&mut self) -> &mut super::hdfs::RemoteEditLogManifestProto {
        if self.manifest.is_none() {
            self.manifest.set_default();
        }
        self.manifest.as_mut().unwrap()
    }

    // Take field
    pub fn take_manifest(&mut self) -> super::hdfs::RemoteEditLogManifestProto {
        self.manifest.take().unwrap_or_else(|| super::hdfs::RemoteEditLogManifestProto::new())
    }

    pub fn get_manifest(&self) -> &super::hdfs::RemoteEditLogManifestProto {
        self.manifest.as_ref().unwrap_or_else(|| super::hdfs::RemoteEditLogManifestProto::default_instance())
    }

    fn get_manifest_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::RemoteEditLogManifestProto> {
        &self.manifest
    }

    fn mut_manifest_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::RemoteEditLogManifestProto> {
        &mut self.manifest
    }

    // required uint32 httpPort = 2;

    pub fn clear_httpPort(&mut self) {
        self.httpPort = ::std::option::Option::None;
    }

    pub fn has_httpPort(&self) -> bool {
        self.httpPort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_httpPort(&mut self, v: u32) {
        self.httpPort = ::std::option::Option::Some(v);
    }

    pub fn get_httpPort(&self) -> u32 {
        self.httpPort.unwrap_or(0)
    }

    fn get_httpPort_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.httpPort
    }

    fn mut_httpPort_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.httpPort
    }

    // optional string fromURL = 3;

    pub fn clear_fromURL(&mut self) {
        self.fromURL.clear();
    }

    pub fn has_fromURL(&self) -> bool {
        self.fromURL.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fromURL(&mut self, v: ::std::string::String) {
        self.fromURL = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fromURL(&mut self) -> &mut ::std::string::String {
        if self.fromURL.is_none() {
            self.fromURL.set_default();
        }
        self.fromURL.as_mut().unwrap()
    }

    // Take field
    pub fn take_fromURL(&mut self) -> ::std::string::String {
        self.fromURL.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_fromURL(&self) -> &str {
        match self.fromURL.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_fromURL_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.fromURL
    }

    fn mut_fromURL_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.fromURL
    }
}

impl ::protobuf::Message for GetEditLogManifestResponseProto {
    fn is_initialized(&self) -> bool {
        if self.manifest.is_none() {
            return false;
        }
        if self.httpPort.is_none() {
            return false;
        }
        for v in &self.manifest {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.manifest)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.httpPort = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.fromURL)?;
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
        if let Some(ref v) = self.manifest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.httpPort {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.fromURL.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.manifest.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.httpPort {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.fromURL.as_ref() {
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

impl ::protobuf::MessageStatic for GetEditLogManifestResponseProto {
    fn new() -> GetEditLogManifestResponseProto {
        GetEditLogManifestResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetEditLogManifestResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::RemoteEditLogManifestProto>>(
                    "manifest",
                    GetEditLogManifestResponseProto::get_manifest_for_reflect,
                    GetEditLogManifestResponseProto::mut_manifest_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "httpPort",
                    GetEditLogManifestResponseProto::get_httpPort_for_reflect,
                    GetEditLogManifestResponseProto::mut_httpPort_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "fromURL",
                    GetEditLogManifestResponseProto::get_fromURL_for_reflect,
                    GetEditLogManifestResponseProto::mut_fromURL_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetEditLogManifestResponseProto>(
                    "GetEditLogManifestResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetEditLogManifestResponseProto {
    fn clear(&mut self) {
        self.clear_manifest();
        self.clear_httpPort();
        self.clear_fromURL();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetEditLogManifestResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetEditLogManifestResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PrepareRecoveryRequestProto {
    // message fields
    reqInfo: ::protobuf::SingularPtrField<RequestInfoProto>,
    segmentTxId: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PrepareRecoveryRequestProto {}

impl PrepareRecoveryRequestProto {
    pub fn new() -> PrepareRecoveryRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PrepareRecoveryRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<PrepareRecoveryRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PrepareRecoveryRequestProto,
        };
        unsafe {
            instance.get(PrepareRecoveryRequestProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.RequestInfoProto reqInfo = 1;

    pub fn clear_reqInfo(&mut self) {
        self.reqInfo.clear();
    }

    pub fn has_reqInfo(&self) -> bool {
        self.reqInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reqInfo(&mut self, v: RequestInfoProto) {
        self.reqInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reqInfo(&mut self) -> &mut RequestInfoProto {
        if self.reqInfo.is_none() {
            self.reqInfo.set_default();
        }
        self.reqInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_reqInfo(&mut self) -> RequestInfoProto {
        self.reqInfo.take().unwrap_or_else(|| RequestInfoProto::new())
    }

    pub fn get_reqInfo(&self) -> &RequestInfoProto {
        self.reqInfo.as_ref().unwrap_or_else(|| RequestInfoProto::default_instance())
    }

    fn get_reqInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<RequestInfoProto> {
        &self.reqInfo
    }

    fn mut_reqInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RequestInfoProto> {
        &mut self.reqInfo
    }

    // required uint64 segmentTxId = 2;

    pub fn clear_segmentTxId(&mut self) {
        self.segmentTxId = ::std::option::Option::None;
    }

    pub fn has_segmentTxId(&self) -> bool {
        self.segmentTxId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_segmentTxId(&mut self, v: u64) {
        self.segmentTxId = ::std::option::Option::Some(v);
    }

    pub fn get_segmentTxId(&self) -> u64 {
        self.segmentTxId.unwrap_or(0)
    }

    fn get_segmentTxId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.segmentTxId
    }

    fn mut_segmentTxId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.segmentTxId
    }
}

impl ::protobuf::Message for PrepareRecoveryRequestProto {
    fn is_initialized(&self) -> bool {
        if self.reqInfo.is_none() {
            return false;
        }
        if self.segmentTxId.is_none() {
            return false;
        }
        for v in &self.reqInfo {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.reqInfo)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.segmentTxId = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.reqInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.segmentTxId {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.reqInfo.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.segmentTxId {
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

impl ::protobuf::MessageStatic for PrepareRecoveryRequestProto {
    fn new() -> PrepareRecoveryRequestProto {
        PrepareRecoveryRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<PrepareRecoveryRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RequestInfoProto>>(
                    "reqInfo",
                    PrepareRecoveryRequestProto::get_reqInfo_for_reflect,
                    PrepareRecoveryRequestProto::mut_reqInfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "segmentTxId",
                    PrepareRecoveryRequestProto::get_segmentTxId_for_reflect,
                    PrepareRecoveryRequestProto::mut_segmentTxId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PrepareRecoveryRequestProto>(
                    "PrepareRecoveryRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PrepareRecoveryRequestProto {
    fn clear(&mut self) {
        self.clear_reqInfo();
        self.clear_segmentTxId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PrepareRecoveryRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PrepareRecoveryRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PrepareRecoveryResponseProto {
    // message fields
    segmentState: ::protobuf::SingularPtrField<SegmentStateProto>,
    acceptedInEpoch: ::std::option::Option<u64>,
    lastWriterEpoch: ::std::option::Option<u64>,
    lastCommittedTxId: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PrepareRecoveryResponseProto {}

impl PrepareRecoveryResponseProto {
    pub fn new() -> PrepareRecoveryResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PrepareRecoveryResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<PrepareRecoveryResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PrepareRecoveryResponseProto,
        };
        unsafe {
            instance.get(PrepareRecoveryResponseProto::new)
        }
    }

    // optional .hadoop.hdfs.qjournal.SegmentStateProto segmentState = 1;

    pub fn clear_segmentState(&mut self) {
        self.segmentState.clear();
    }

    pub fn has_segmentState(&self) -> bool {
        self.segmentState.is_some()
    }

    // Param is passed by value, moved
    pub fn set_segmentState(&mut self, v: SegmentStateProto) {
        self.segmentState = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_segmentState(&mut self) -> &mut SegmentStateProto {
        if self.segmentState.is_none() {
            self.segmentState.set_default();
        }
        self.segmentState.as_mut().unwrap()
    }

    // Take field
    pub fn take_segmentState(&mut self) -> SegmentStateProto {
        self.segmentState.take().unwrap_or_else(|| SegmentStateProto::new())
    }

    pub fn get_segmentState(&self) -> &SegmentStateProto {
        self.segmentState.as_ref().unwrap_or_else(|| SegmentStateProto::default_instance())
    }

    fn get_segmentState_for_reflect(&self) -> &::protobuf::SingularPtrField<SegmentStateProto> {
        &self.segmentState
    }

    fn mut_segmentState_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SegmentStateProto> {
        &mut self.segmentState
    }

    // optional uint64 acceptedInEpoch = 2;

    pub fn clear_acceptedInEpoch(&mut self) {
        self.acceptedInEpoch = ::std::option::Option::None;
    }

    pub fn has_acceptedInEpoch(&self) -> bool {
        self.acceptedInEpoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_acceptedInEpoch(&mut self, v: u64) {
        self.acceptedInEpoch = ::std::option::Option::Some(v);
    }

    pub fn get_acceptedInEpoch(&self) -> u64 {
        self.acceptedInEpoch.unwrap_or(0)
    }

    fn get_acceptedInEpoch_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.acceptedInEpoch
    }

    fn mut_acceptedInEpoch_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.acceptedInEpoch
    }

    // required uint64 lastWriterEpoch = 3;

    pub fn clear_lastWriterEpoch(&mut self) {
        self.lastWriterEpoch = ::std::option::Option::None;
    }

    pub fn has_lastWriterEpoch(&self) -> bool {
        self.lastWriterEpoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastWriterEpoch(&mut self, v: u64) {
        self.lastWriterEpoch = ::std::option::Option::Some(v);
    }

    pub fn get_lastWriterEpoch(&self) -> u64 {
        self.lastWriterEpoch.unwrap_or(0)
    }

    fn get_lastWriterEpoch_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lastWriterEpoch
    }

    fn mut_lastWriterEpoch_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lastWriterEpoch
    }

    // optional uint64 lastCommittedTxId = 4;

    pub fn clear_lastCommittedTxId(&mut self) {
        self.lastCommittedTxId = ::std::option::Option::None;
    }

    pub fn has_lastCommittedTxId(&self) -> bool {
        self.lastCommittedTxId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastCommittedTxId(&mut self, v: u64) {
        self.lastCommittedTxId = ::std::option::Option::Some(v);
    }

    pub fn get_lastCommittedTxId(&self) -> u64 {
        self.lastCommittedTxId.unwrap_or(0)
    }

    fn get_lastCommittedTxId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lastCommittedTxId
    }

    fn mut_lastCommittedTxId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lastCommittedTxId
    }
}

impl ::protobuf::Message for PrepareRecoveryResponseProto {
    fn is_initialized(&self) -> bool {
        if self.lastWriterEpoch.is_none() {
            return false;
        }
        for v in &self.segmentState {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.segmentState)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.acceptedInEpoch = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.lastWriterEpoch = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.lastCommittedTxId = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.segmentState.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.acceptedInEpoch {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.lastWriterEpoch {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.lastCommittedTxId {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.segmentState.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.acceptedInEpoch {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.lastWriterEpoch {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.lastCommittedTxId {
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

impl ::protobuf::MessageStatic for PrepareRecoveryResponseProto {
    fn new() -> PrepareRecoveryResponseProto {
        PrepareRecoveryResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<PrepareRecoveryResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SegmentStateProto>>(
                    "segmentState",
                    PrepareRecoveryResponseProto::get_segmentState_for_reflect,
                    PrepareRecoveryResponseProto::mut_segmentState_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "acceptedInEpoch",
                    PrepareRecoveryResponseProto::get_acceptedInEpoch_for_reflect,
                    PrepareRecoveryResponseProto::mut_acceptedInEpoch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lastWriterEpoch",
                    PrepareRecoveryResponseProto::get_lastWriterEpoch_for_reflect,
                    PrepareRecoveryResponseProto::mut_lastWriterEpoch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lastCommittedTxId",
                    PrepareRecoveryResponseProto::get_lastCommittedTxId_for_reflect,
                    PrepareRecoveryResponseProto::mut_lastCommittedTxId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PrepareRecoveryResponseProto>(
                    "PrepareRecoveryResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PrepareRecoveryResponseProto {
    fn clear(&mut self) {
        self.clear_segmentState();
        self.clear_acceptedInEpoch();
        self.clear_lastWriterEpoch();
        self.clear_lastCommittedTxId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PrepareRecoveryResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PrepareRecoveryResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AcceptRecoveryRequestProto {
    // message fields
    reqInfo: ::protobuf::SingularPtrField<RequestInfoProto>,
    stateToAccept: ::protobuf::SingularPtrField<SegmentStateProto>,
    fromURL: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AcceptRecoveryRequestProto {}

impl AcceptRecoveryRequestProto {
    pub fn new() -> AcceptRecoveryRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AcceptRecoveryRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<AcceptRecoveryRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AcceptRecoveryRequestProto,
        };
        unsafe {
            instance.get(AcceptRecoveryRequestProto::new)
        }
    }

    // required .hadoop.hdfs.qjournal.RequestInfoProto reqInfo = 1;

    pub fn clear_reqInfo(&mut self) {
        self.reqInfo.clear();
    }

    pub fn has_reqInfo(&self) -> bool {
        self.reqInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reqInfo(&mut self, v: RequestInfoProto) {
        self.reqInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reqInfo(&mut self) -> &mut RequestInfoProto {
        if self.reqInfo.is_none() {
            self.reqInfo.set_default();
        }
        self.reqInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_reqInfo(&mut self) -> RequestInfoProto {
        self.reqInfo.take().unwrap_or_else(|| RequestInfoProto::new())
    }

    pub fn get_reqInfo(&self) -> &RequestInfoProto {
        self.reqInfo.as_ref().unwrap_or_else(|| RequestInfoProto::default_instance())
    }

    fn get_reqInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<RequestInfoProto> {
        &self.reqInfo
    }

    fn mut_reqInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RequestInfoProto> {
        &mut self.reqInfo
    }

    // required .hadoop.hdfs.qjournal.SegmentStateProto stateToAccept = 2;

    pub fn clear_stateToAccept(&mut self) {
        self.stateToAccept.clear();
    }

    pub fn has_stateToAccept(&self) -> bool {
        self.stateToAccept.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stateToAccept(&mut self, v: SegmentStateProto) {
        self.stateToAccept = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stateToAccept(&mut self) -> &mut SegmentStateProto {
        if self.stateToAccept.is_none() {
            self.stateToAccept.set_default();
        }
        self.stateToAccept.as_mut().unwrap()
    }

    // Take field
    pub fn take_stateToAccept(&mut self) -> SegmentStateProto {
        self.stateToAccept.take().unwrap_or_else(|| SegmentStateProto::new())
    }

    pub fn get_stateToAccept(&self) -> &SegmentStateProto {
        self.stateToAccept.as_ref().unwrap_or_else(|| SegmentStateProto::default_instance())
    }

    fn get_stateToAccept_for_reflect(&self) -> &::protobuf::SingularPtrField<SegmentStateProto> {
        &self.stateToAccept
    }

    fn mut_stateToAccept_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SegmentStateProto> {
        &mut self.stateToAccept
    }

    // required string fromURL = 3;

    pub fn clear_fromURL(&mut self) {
        self.fromURL.clear();
    }

    pub fn has_fromURL(&self) -> bool {
        self.fromURL.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fromURL(&mut self, v: ::std::string::String) {
        self.fromURL = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fromURL(&mut self) -> &mut ::std::string::String {
        if self.fromURL.is_none() {
            self.fromURL.set_default();
        }
        self.fromURL.as_mut().unwrap()
    }

    // Take field
    pub fn take_fromURL(&mut self) -> ::std::string::String {
        self.fromURL.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_fromURL(&self) -> &str {
        match self.fromURL.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_fromURL_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.fromURL
    }

    fn mut_fromURL_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.fromURL
    }
}

impl ::protobuf::Message for AcceptRecoveryRequestProto {
    fn is_initialized(&self) -> bool {
        if self.reqInfo.is_none() {
            return false;
        }
        if self.stateToAccept.is_none() {
            return false;
        }
        if self.fromURL.is_none() {
            return false;
        }
        for v in &self.reqInfo {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.stateToAccept {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.reqInfo)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stateToAccept)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.fromURL)?;
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
        if let Some(ref v) = self.reqInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.stateToAccept.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.fromURL.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.reqInfo.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.stateToAccept.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.fromURL.as_ref() {
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

impl ::protobuf::MessageStatic for AcceptRecoveryRequestProto {
    fn new() -> AcceptRecoveryRequestProto {
        AcceptRecoveryRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<AcceptRecoveryRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RequestInfoProto>>(
                    "reqInfo",
                    AcceptRecoveryRequestProto::get_reqInfo_for_reflect,
                    AcceptRecoveryRequestProto::mut_reqInfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SegmentStateProto>>(
                    "stateToAccept",
                    AcceptRecoveryRequestProto::get_stateToAccept_for_reflect,
                    AcceptRecoveryRequestProto::mut_stateToAccept_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "fromURL",
                    AcceptRecoveryRequestProto::get_fromURL_for_reflect,
                    AcceptRecoveryRequestProto::mut_fromURL_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AcceptRecoveryRequestProto>(
                    "AcceptRecoveryRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AcceptRecoveryRequestProto {
    fn clear(&mut self) {
        self.clear_reqInfo();
        self.clear_stateToAccept();
        self.clear_fromURL();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AcceptRecoveryRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AcceptRecoveryRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AcceptRecoveryResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AcceptRecoveryResponseProto {}

impl AcceptRecoveryResponseProto {
    pub fn new() -> AcceptRecoveryResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AcceptRecoveryResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<AcceptRecoveryResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AcceptRecoveryResponseProto,
        };
        unsafe {
            instance.get(AcceptRecoveryResponseProto::new)
        }
    }
}

impl ::protobuf::Message for AcceptRecoveryResponseProto {
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

impl ::protobuf::MessageStatic for AcceptRecoveryResponseProto {
    fn new() -> AcceptRecoveryResponseProto {
        AcceptRecoveryResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<AcceptRecoveryResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<AcceptRecoveryResponseProto>(
                    "AcceptRecoveryResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AcceptRecoveryResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AcceptRecoveryResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AcceptRecoveryResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16QJournalProtocol.proto\x12\x14hadoop.hdfs.qjournal\x1a\nhdfs.proto\
    \"$\n\x0eJournalIdProto\x12\x12\n\nidentifier\x18\x01\x20\x02(\t\"\x8a\
    \x01\n\x10RequestInfoProto\x127\n\tjournalId\x18\x01\x20\x02(\x0b2$.hado\
    op.hdfs.qjournal.JournalIdProto\x12\r\n\x05epoch\x18\x02\x20\x02(\x04\
    \x12\x17\n\x0fipcSerialNumber\x18\x03\x20\x02(\x04\x12\x15\n\rcommittedT\
    xId\x18\x04\x20\x01(\x04\"M\n\x11SegmentStateProto\x12\x11\n\tstartTxId\
    \x18\x01\x20\x02(\x04\x12\x0f\n\x07endTxId\x18\x02\x20\x02(\x04\x12\x14\
    \n\x0cisInProgress\x18\x03\x20\x02(\x08\"t\n\x1aPersistedRecoveryPaxosDa\
    ta\x12=\n\x0csegmentState\x18\x01\x20\x02(\x0b2'.hadoop.hdfs.qjournal.Se\
    gmentStateProto\x12\x17\n\x0facceptedInEpoch\x18\x02\x20\x02(\x04\"\x9a\
    \x01\n\x13JournalRequestProto\x127\n\x07reqInfo\x18\x01\x20\x02(\x0b2&.h\
    adoop.hdfs.qjournal.RequestInfoProto\x12\x12\n\nfirstTxnId\x18\x02\x20\
    \x02(\x04\x12\x0f\n\x07numTxns\x18\x03\x20\x02(\r\x12\x0f\n\x07records\
    \x18\x04\x20\x02(\x0c\x12\x14\n\x0csegmentTxnId\x18\x05\x20\x02(\x04\"\
    \x16\n\x14JournalResponseProto\"P\n\x15HeartbeatRequestProto\x127\n\x07r\
    eqInfo\x18\x01\x20\x02(\x0b2&.hadoop.hdfs.qjournal.RequestInfoProto\"\
    \x18\n\x16HeartbeatResponseProto\"{\n\x1bStartLogSegmentRequestProto\x12\
    7\n\x07reqInfo\x18\x01\x20\x02(\x0b2&.hadoop.hdfs.qjournal.RequestInfoPr\
    oto\x12\x0c\n\x04txid\x18\x02\x20\x02(\x04\x12\x15\n\rlayoutVersion\x18\
    \x03\x20\x01(\x11\"\x1e\n\x1cStartLogSegmentResponseProto\"}\n\x1eFinali\
    zeLogSegmentRequestProto\x127\n\x07reqInfo\x18\x01\x20\x02(\x0b2&.hadoop\
    .hdfs.qjournal.RequestInfoProto\x12\x11\n\tstartTxId\x18\x02\x20\x02(\
    \x04\x12\x0f\n\x07endTxId\x18\x03\x20\x02(\x04\"!\n\x1fFinalizeLogSegmen\
    tResponseProto\"g\n\x15PurgeLogsRequestProto\x127\n\x07reqInfo\x18\x01\
    \x20\x02(\x0b2&.hadoop.hdfs.qjournal.RequestInfoProto\x12\x15\n\rminTxId\
    ToKeep\x18\x02\x20\x02(\x04\"\x18\n\x16PurgeLogsResponseProto\"L\n\x17Is\
    FormattedRequestProto\x121\n\x03jid\x18\x01\x20\x02(\x0b2$.hadoop.hdfs.q\
    journal.JournalIdProto\"/\n\x18IsFormattedResponseProto\x12\x13\n\x0bisF\
    ormatted\x18\x01\x20\x02(\x08\"c\n\x1bDiscardSegmentsRequestProto\x121\n\
    \x03jid\x18\x01\x20\x02(\x0b2$.hadoop.hdfs.qjournal.JournalIdProto\x12\
    \x11\n\tstartTxId\x18\x02\x20\x02(\x04\"\x1e\n\x1cDiscardSegmentsRespons\
    eProto\"P\n\x1bGetJournalCTimeRequestProto\x121\n\x03jid\x18\x01\x20\x02\
    (\x0b2$.hadoop.hdfs.qjournal.JournalIdProto\"3\n\x1cGetJournalCTimeRespo\
    nseProto\x12\x13\n\x0bresultCTime\x18\x01\x20\x02(\x03\"M\n\x18DoPreUpgr\
    adeRequestProto\x121\n\x03jid\x18\x01\x20\x02(\x0b2$.hadoop.hdfs.qjourna\
    l.JournalIdProto\"\x1b\n\x19DoPreUpgradeResponseProto\"x\n\x15DoUpgradeR\
    equestProto\x121\n\x03jid\x18\x01\x20\x02(\x0b2$.hadoop.hdfs.qjournal.Jo\
    urnalIdProto\x12,\n\x05sInfo\x18\x02\x20\x02(\x0b2\x1d.hadoop.hdfs.Stora\
    geInfoProto\"\x18\n\x16DoUpgradeResponseProto\"K\n\x16DoFinalizeRequestP\
    roto\x121\n\x03jid\x18\x01\x20\x02(\x0b2$.hadoop.hdfs.qjournal.JournalId\
    Proto\"\x19\n\x17DoFinalizeResponseProto\"\xcd\x01\n\x17CanRollBackReque\
    stProto\x121\n\x03jid\x18\x01\x20\x02(\x0b2$.hadoop.hdfs.qjournal.Journa\
    lIdProto\x12.\n\x07storage\x18\x02\x20\x02(\x0b2\x1d.hadoop.hdfs.Storage\
    InfoProto\x122\n\x0bprevStorage\x18\x03\x20\x02(\x0b2\x1d.hadoop.hdfs.St\
    orageInfoProto\x12\x1b\n\x13targetLayoutVersion\x18\x04\x20\x02(\x05\"/\
    \n\x18CanRollBackResponseProto\x12\x13\n\x0bcanRollBack\x18\x01\x20\x02(\
    \x08\"K\n\x16DoRollbackRequestProto\x121\n\x03jid\x18\x01\x20\x02(\x0b2$\
    .hadoop.hdfs.qjournal.JournalIdProto\"\x19\n\x17DoRollbackResponseProto\
    \"P\n\x1bGetJournalStateRequestProto\x121\n\x03jid\x18\x01\x20\x02(\x0b2\
    $.hadoop.hdfs.qjournal.JournalIdProto\"\\\n\x1cGetJournalStateResponsePr\
    oto\x12\x19\n\x11lastPromisedEpoch\x18\x01\x20\x02(\x04\x12\x10\n\x08htt\
    pPort\x18\x02\x20\x02(\r\x12\x0f\n\x07fromURL\x18\x03\x20\x01(\t\"x\n\
    \x12FormatRequestProto\x121\n\x03jid\x18\x01\x20\x02(\x0b2$.hadoop.hdfs.\
    qjournal.JournalIdProto\x12/\n\x06nsInfo\x18\x02\x20\x02(\x0b2\x1f.hadoo\
    p.hdfs.NamespaceInfoProto\"\x15\n\x13FormatResponseProto\"\x89\x01\n\x14\
    NewEpochRequestProto\x121\n\x03jid\x18\x01\x20\x02(\x0b2$.hadoop.hdfs.qj\
    ournal.JournalIdProto\x12/\n\x06nsInfo\x18\x02\x20\x02(\x0b2\x1f.hadoop.\
    hdfs.NamespaceInfoProto\x12\r\n\x05epoch\x18\x03\x20\x02(\x04\"0\n\x15Ne\
    wEpochResponseProto\x12\x17\n\x0flastSegmentTxId\x18\x01\x20\x01(\x04\"\
    \x83\x01\n\x1eGetEditLogManifestRequestProto\x121\n\x03jid\x18\x01\x20\
    \x02(\x0b2$.hadoop.hdfs.qjournal.JournalIdProto\x12\x11\n\tsinceTxId\x18\
    \x02\x20\x02(\x04\x12\x1b\n\x0cinProgressOk\x18\x04\x20\x01(\x08:\x05fal\
    se\"\x7f\n\x1fGetEditLogManifestResponseProto\x129\n\x08manifest\x18\x01\
    \x20\x02(\x0b2'.hadoop.hdfs.RemoteEditLogManifestProto\x12\x10\n\x08http\
    Port\x18\x02\x20\x02(\r\x12\x0f\n\x07fromURL\x18\x03\x20\x01(\t\"k\n\x1b\
    PrepareRecoveryRequestProto\x127\n\x07reqInfo\x18\x01\x20\x02(\x0b2&.had\
    oop.hdfs.qjournal.RequestInfoProto\x12\x13\n\x0bsegmentTxId\x18\x02\x20\
    \x02(\x04\"\xaa\x01\n\x1cPrepareRecoveryResponseProto\x12=\n\x0csegmentS\
    tate\x18\x01\x20\x01(\x0b2'.hadoop.hdfs.qjournal.SegmentStateProto\x12\
    \x17\n\x0facceptedInEpoch\x18\x02\x20\x01(\x04\x12\x17\n\x0flastWriterEp\
    och\x18\x03\x20\x02(\x04\x12\x19\n\x11lastCommittedTxId\x18\x04\x20\x01(\
    \x04\"\xa6\x01\n\x1aAcceptRecoveryRequestProto\x127\n\x07reqInfo\x18\x01\
    \x20\x02(\x0b2&.hadoop.hdfs.qjournal.RequestInfoProto\x12>\n\rstateToAcc\
    ept\x18\x02\x20\x02(\x0b2'.hadoop.hdfs.qjournal.SegmentStateProto\x12\
    \x0f\n\x07fromURL\x18\x03\x20\x02(\t\"\x1d\n\x1bAcceptRecoveryResponsePr\
    oto2\xfb\x10\n\x17QJournalProtocolService\x12l\n\x0bisFormatted\x12-.had\
    oop.hdfs.qjournal.IsFormattedRequestProto\x1a..hadoop.hdfs.qjournal.IsFo\
    rmattedResponseProto\x12x\n\x0fdiscardSegments\x121.hadoop.hdfs.qjournal\
    .DiscardSegmentsRequestProto\x1a2.hadoop.hdfs.qjournal.DiscardSegmentsRe\
    sponseProto\x12x\n\x0fgetJournalCTime\x121.hadoop.hdfs.qjournal.GetJourn\
    alCTimeRequestProto\x1a2.hadoop.hdfs.qjournal.GetJournalCTimeResponsePro\
    to\x12o\n\x0cdoPreUpgrade\x12..hadoop.hdfs.qjournal.DoPreUpgradeRequestP\
    roto\x1a/.hadoop.hdfs.qjournal.DoPreUpgradeResponseProto\x12f\n\tdoUpgra\
    de\x12+.hadoop.hdfs.qjournal.DoUpgradeRequestProto\x1a,.hadoop.hdfs.qjou\
    rnal.DoUpgradeResponseProto\x12i\n\ndoFinalize\x12,.hadoop.hdfs.qjournal\
    .DoFinalizeRequestProto\x1a-.hadoop.hdfs.qjournal.DoFinalizeResponseProt\
    o\x12l\n\x0bcanRollBack\x12-.hadoop.hdfs.qjournal.CanRollBackRequestProt\
    o\x1a..hadoop.hdfs.qjournal.CanRollBackResponseProto\x12i\n\ndoRollback\
    \x12,.hadoop.hdfs.qjournal.DoRollbackRequestProto\x1a-.hadoop.hdfs.qjour\
    nal.DoRollbackResponseProto\x12x\n\x0fgetJournalState\x121.hadoop.hdfs.q\
    journal.GetJournalStateRequestProto\x1a2.hadoop.hdfs.qjournal.GetJournal\
    StateResponseProto\x12c\n\x08newEpoch\x12*.hadoop.hdfs.qjournal.NewEpoch\
    RequestProto\x1a+.hadoop.hdfs.qjournal.NewEpochResponseProto\x12]\n\x06f\
    ormat\x12(.hadoop.hdfs.qjournal.FormatRequestProto\x1a).hadoop.hdfs.qjou\
    rnal.FormatResponseProto\x12`\n\x07journal\x12).hadoop.hdfs.qjournal.Jou\
    rnalRequestProto\x1a*.hadoop.hdfs.qjournal.JournalResponseProto\x12f\n\t\
    heartbeat\x12+.hadoop.hdfs.qjournal.HeartbeatRequestProto\x1a,.hadoop.hd\
    fs.qjournal.HeartbeatResponseProto\x12x\n\x0fstartLogSegment\x121.hadoop\
    .hdfs.qjournal.StartLogSegmentRequestProto\x1a2.hadoop.hdfs.qjournal.Sta\
    rtLogSegmentResponseProto\x12\x81\x01\n\x12finalizeLogSegment\x124.hadoo\
    p.hdfs.qjournal.FinalizeLogSegmentRequestProto\x1a5.hadoop.hdfs.qjournal\
    .FinalizeLogSegmentResponseProto\x12f\n\tpurgeLogs\x12+.hadoop.hdfs.qjou\
    rnal.PurgeLogsRequestProto\x1a,.hadoop.hdfs.qjournal.PurgeLogsResponsePr\
    oto\x12\x81\x01\n\x12getEditLogManifest\x124.hadoop.hdfs.qjournal.GetEdi\
    tLogManifestRequestProto\x1a5.hadoop.hdfs.qjournal.GetEditLogManifestRes\
    ponseProto\x12x\n\x0fprepareRecovery\x121.hadoop.hdfs.qjournal.PrepareRe\
    coveryRequestProto\x1a2.hadoop.hdfs.qjournal.PrepareRecoveryResponseProt\
    o\x12u\n\x0eacceptRecovery\x120.hadoop.hdfs.qjournal.AcceptRecoveryReque\
    stProto\x1a1.hadoop.hdfs.qjournal.AcceptRecoveryResponseProtoBH\n(org.ap\
    ache.hadoop.hdfs.qjournal.protocolB\x16QJournalProtocolProtos\xa0\x01\
    \x01\x88\x01\x01\
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
