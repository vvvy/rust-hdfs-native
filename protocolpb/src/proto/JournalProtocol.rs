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
pub struct JournalInfoProto {
    // message fields
    clusterID: ::protobuf::SingularField<::std::string::String>,
    layoutVersion: ::std::option::Option<u32>,
    namespaceID: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JournalInfoProto {}

impl JournalInfoProto {
    pub fn new() -> JournalInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JournalInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<JournalInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JournalInfoProto,
        };
        unsafe {
            instance.get(JournalInfoProto::new)
        }
    }

    // required string clusterID = 1;

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

    // optional uint32 layoutVersion = 2;

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

    // optional uint32 namespaceID = 3;

    pub fn clear_namespaceID(&mut self) {
        self.namespaceID = ::std::option::Option::None;
    }

    pub fn has_namespaceID(&self) -> bool {
        self.namespaceID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_namespaceID(&mut self, v: u32) {
        self.namespaceID = ::std::option::Option::Some(v);
    }

    pub fn get_namespaceID(&self) -> u32 {
        self.namespaceID.unwrap_or(0)
    }

    fn get_namespaceID_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.namespaceID
    }

    fn mut_namespaceID_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.namespaceID
    }
}

impl ::protobuf::Message for JournalInfoProto {
    fn is_initialized(&self) -> bool {
        if self.clusterID.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.clusterID)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.layoutVersion = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.namespaceID = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.clusterID.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.layoutVersion {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.namespaceID {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.clusterID.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.layoutVersion {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.namespaceID {
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

impl ::protobuf::MessageStatic for JournalInfoProto {
    fn new() -> JournalInfoProto {
        JournalInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<JournalInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "clusterID",
                    JournalInfoProto::get_clusterID_for_reflect,
                    JournalInfoProto::mut_clusterID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "layoutVersion",
                    JournalInfoProto::get_layoutVersion_for_reflect,
                    JournalInfoProto::mut_layoutVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "namespaceID",
                    JournalInfoProto::get_namespaceID_for_reflect,
                    JournalInfoProto::mut_namespaceID_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JournalInfoProto>(
                    "JournalInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JournalInfoProto {
    fn clear(&mut self) {
        self.clear_clusterID();
        self.clear_layoutVersion();
        self.clear_namespaceID();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JournalInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JournalInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JournalRequestProto {
    // message fields
    journalInfo: ::protobuf::SingularPtrField<JournalInfoProto>,
    firstTxnId: ::std::option::Option<u64>,
    numTxns: ::std::option::Option<u32>,
    records: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    epoch: ::std::option::Option<u64>,
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

    // required .hadoop.hdfs.JournalInfoProto journalInfo = 1;

    pub fn clear_journalInfo(&mut self) {
        self.journalInfo.clear();
    }

    pub fn has_journalInfo(&self) -> bool {
        self.journalInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_journalInfo(&mut self, v: JournalInfoProto) {
        self.journalInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_journalInfo(&mut self) -> &mut JournalInfoProto {
        if self.journalInfo.is_none() {
            self.journalInfo.set_default();
        }
        self.journalInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_journalInfo(&mut self) -> JournalInfoProto {
        self.journalInfo.take().unwrap_or_else(|| JournalInfoProto::new())
    }

    pub fn get_journalInfo(&self) -> &JournalInfoProto {
        self.journalInfo.as_ref().unwrap_or_else(|| JournalInfoProto::default_instance())
    }

    fn get_journalInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<JournalInfoProto> {
        &self.journalInfo
    }

    fn mut_journalInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JournalInfoProto> {
        &mut self.journalInfo
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

    // required uint64 epoch = 5;

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

impl ::protobuf::Message for JournalRequestProto {
    fn is_initialized(&self) -> bool {
        if self.journalInfo.is_none() {
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
        if self.epoch.is_none() {
            return false;
        }
        for v in &self.journalInfo {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.journalInfo)?;
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
        if let Some(ref v) = self.journalInfo.as_ref() {
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
        if let Some(v) = self.epoch {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.journalInfo.as_ref() {
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
        if let Some(v) = self.epoch {
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JournalInfoProto>>(
                    "journalInfo",
                    JournalRequestProto::get_journalInfo_for_reflect,
                    JournalRequestProto::mut_journalInfo_for_reflect,
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
                    "epoch",
                    JournalRequestProto::get_epoch_for_reflect,
                    JournalRequestProto::mut_epoch_for_reflect,
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
        self.clear_journalInfo();
        self.clear_firstTxnId();
        self.clear_numTxns();
        self.clear_records();
        self.clear_epoch();
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
pub struct StartLogSegmentRequestProto {
    // message fields
    journalInfo: ::protobuf::SingularPtrField<JournalInfoProto>,
    txid: ::std::option::Option<u64>,
    epoch: ::std::option::Option<u64>,
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

    // required .hadoop.hdfs.JournalInfoProto journalInfo = 1;

    pub fn clear_journalInfo(&mut self) {
        self.journalInfo.clear();
    }

    pub fn has_journalInfo(&self) -> bool {
        self.journalInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_journalInfo(&mut self, v: JournalInfoProto) {
        self.journalInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_journalInfo(&mut self) -> &mut JournalInfoProto {
        if self.journalInfo.is_none() {
            self.journalInfo.set_default();
        }
        self.journalInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_journalInfo(&mut self) -> JournalInfoProto {
        self.journalInfo.take().unwrap_or_else(|| JournalInfoProto::new())
    }

    pub fn get_journalInfo(&self) -> &JournalInfoProto {
        self.journalInfo.as_ref().unwrap_or_else(|| JournalInfoProto::default_instance())
    }

    fn get_journalInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<JournalInfoProto> {
        &self.journalInfo
    }

    fn mut_journalInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JournalInfoProto> {
        &mut self.journalInfo
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

impl ::protobuf::Message for StartLogSegmentRequestProto {
    fn is_initialized(&self) -> bool {
        if self.journalInfo.is_none() {
            return false;
        }
        if self.txid.is_none() {
            return false;
        }
        if self.epoch.is_none() {
            return false;
        }
        for v in &self.journalInfo {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.journalInfo)?;
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
        if let Some(ref v) = self.journalInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.txid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.epoch {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.journalInfo.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.txid {
            os.write_uint64(2, v)?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JournalInfoProto>>(
                    "journalInfo",
                    StartLogSegmentRequestProto::get_journalInfo_for_reflect,
                    StartLogSegmentRequestProto::mut_journalInfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "txid",
                    StartLogSegmentRequestProto::get_txid_for_reflect,
                    StartLogSegmentRequestProto::mut_txid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "epoch",
                    StartLogSegmentRequestProto::get_epoch_for_reflect,
                    StartLogSegmentRequestProto::mut_epoch_for_reflect,
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
        self.clear_journalInfo();
        self.clear_txid();
        self.clear_epoch();
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
pub struct FenceRequestProto {
    // message fields
    journalInfo: ::protobuf::SingularPtrField<JournalInfoProto>,
    epoch: ::std::option::Option<u64>,
    fencerInfo: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FenceRequestProto {}

impl FenceRequestProto {
    pub fn new() -> FenceRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FenceRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<FenceRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FenceRequestProto,
        };
        unsafe {
            instance.get(FenceRequestProto::new)
        }
    }

    // required .hadoop.hdfs.JournalInfoProto journalInfo = 1;

    pub fn clear_journalInfo(&mut self) {
        self.journalInfo.clear();
    }

    pub fn has_journalInfo(&self) -> bool {
        self.journalInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_journalInfo(&mut self, v: JournalInfoProto) {
        self.journalInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_journalInfo(&mut self) -> &mut JournalInfoProto {
        if self.journalInfo.is_none() {
            self.journalInfo.set_default();
        }
        self.journalInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_journalInfo(&mut self) -> JournalInfoProto {
        self.journalInfo.take().unwrap_or_else(|| JournalInfoProto::new())
    }

    pub fn get_journalInfo(&self) -> &JournalInfoProto {
        self.journalInfo.as_ref().unwrap_or_else(|| JournalInfoProto::default_instance())
    }

    fn get_journalInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<JournalInfoProto> {
        &self.journalInfo
    }

    fn mut_journalInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JournalInfoProto> {
        &mut self.journalInfo
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

    // optional string fencerInfo = 3;

    pub fn clear_fencerInfo(&mut self) {
        self.fencerInfo.clear();
    }

    pub fn has_fencerInfo(&self) -> bool {
        self.fencerInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fencerInfo(&mut self, v: ::std::string::String) {
        self.fencerInfo = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fencerInfo(&mut self) -> &mut ::std::string::String {
        if self.fencerInfo.is_none() {
            self.fencerInfo.set_default();
        }
        self.fencerInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_fencerInfo(&mut self) -> ::std::string::String {
        self.fencerInfo.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_fencerInfo(&self) -> &str {
        match self.fencerInfo.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_fencerInfo_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.fencerInfo
    }

    fn mut_fencerInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.fencerInfo
    }
}

impl ::protobuf::Message for FenceRequestProto {
    fn is_initialized(&self) -> bool {
        if self.journalInfo.is_none() {
            return false;
        }
        if self.epoch.is_none() {
            return false;
        }
        for v in &self.journalInfo {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.journalInfo)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.epoch = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.fencerInfo)?;
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
        if let Some(ref v) = self.journalInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.epoch {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.fencerInfo.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.journalInfo.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.epoch {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.fencerInfo.as_ref() {
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

impl ::protobuf::MessageStatic for FenceRequestProto {
    fn new() -> FenceRequestProto {
        FenceRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FenceRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JournalInfoProto>>(
                    "journalInfo",
                    FenceRequestProto::get_journalInfo_for_reflect,
                    FenceRequestProto::mut_journalInfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "epoch",
                    FenceRequestProto::get_epoch_for_reflect,
                    FenceRequestProto::mut_epoch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "fencerInfo",
                    FenceRequestProto::get_fencerInfo_for_reflect,
                    FenceRequestProto::mut_fencerInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FenceRequestProto>(
                    "FenceRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FenceRequestProto {
    fn clear(&mut self) {
        self.clear_journalInfo();
        self.clear_epoch();
        self.clear_fencerInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FenceRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FenceRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FenceResponseProto {
    // message fields
    previousEpoch: ::std::option::Option<u64>,
    lastTransactionId: ::std::option::Option<u64>,
    inSync: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FenceResponseProto {}

impl FenceResponseProto {
    pub fn new() -> FenceResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FenceResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<FenceResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FenceResponseProto,
        };
        unsafe {
            instance.get(FenceResponseProto::new)
        }
    }

    // optional uint64 previousEpoch = 1;

    pub fn clear_previousEpoch(&mut self) {
        self.previousEpoch = ::std::option::Option::None;
    }

    pub fn has_previousEpoch(&self) -> bool {
        self.previousEpoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_previousEpoch(&mut self, v: u64) {
        self.previousEpoch = ::std::option::Option::Some(v);
    }

    pub fn get_previousEpoch(&self) -> u64 {
        self.previousEpoch.unwrap_or(0)
    }

    fn get_previousEpoch_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.previousEpoch
    }

    fn mut_previousEpoch_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.previousEpoch
    }

    // optional uint64 lastTransactionId = 2;

    pub fn clear_lastTransactionId(&mut self) {
        self.lastTransactionId = ::std::option::Option::None;
    }

    pub fn has_lastTransactionId(&self) -> bool {
        self.lastTransactionId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastTransactionId(&mut self, v: u64) {
        self.lastTransactionId = ::std::option::Option::Some(v);
    }

    pub fn get_lastTransactionId(&self) -> u64 {
        self.lastTransactionId.unwrap_or(0)
    }

    fn get_lastTransactionId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.lastTransactionId
    }

    fn mut_lastTransactionId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.lastTransactionId
    }

    // optional bool inSync = 3;

    pub fn clear_inSync(&mut self) {
        self.inSync = ::std::option::Option::None;
    }

    pub fn has_inSync(&self) -> bool {
        self.inSync.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inSync(&mut self, v: bool) {
        self.inSync = ::std::option::Option::Some(v);
    }

    pub fn get_inSync(&self) -> bool {
        self.inSync.unwrap_or(false)
    }

    fn get_inSync_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.inSync
    }

    fn mut_inSync_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.inSync
    }
}

impl ::protobuf::Message for FenceResponseProto {
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
                    self.previousEpoch = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.lastTransactionId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.inSync = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.previousEpoch {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.lastTransactionId {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.inSync {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.previousEpoch {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.lastTransactionId {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.inSync {
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

impl ::protobuf::MessageStatic for FenceResponseProto {
    fn new() -> FenceResponseProto {
        FenceResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FenceResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "previousEpoch",
                    FenceResponseProto::get_previousEpoch_for_reflect,
                    FenceResponseProto::mut_previousEpoch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "lastTransactionId",
                    FenceResponseProto::get_lastTransactionId_for_reflect,
                    FenceResponseProto::mut_lastTransactionId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "inSync",
                    FenceResponseProto::get_inSync_for_reflect,
                    FenceResponseProto::mut_inSync_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FenceResponseProto>(
                    "FenceResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FenceResponseProto {
    fn clear(&mut self) {
        self.clear_previousEpoch();
        self.clear_lastTransactionId();
        self.clear_inSync();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FenceResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FenceResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15JournalProtocol.proto\x12\x0bhadoop.hdfs\x1a\nhdfs.proto\"Q\n\x10J\
    ournalInfoProto\x12\x11\n\tclusterID\x18\x01\x20\x02(\t\x12\x15\n\rlayou\
    tVersion\x18\x02\x20\x01(\r\x12\x13\n\x0bnamespaceID\x18\x03\x20\x01(\r\
    \"\x8e\x01\n\x13JournalRequestProto\x122\n\x0bjournalInfo\x18\x01\x20\
    \x02(\x0b2\x1d.hadoop.hdfs.JournalInfoProto\x12\x12\n\nfirstTxnId\x18\
    \x02\x20\x02(\x04\x12\x0f\n\x07numTxns\x18\x03\x20\x02(\r\x12\x0f\n\x07r\
    ecords\x18\x04\x20\x02(\x0c\x12\r\n\x05epoch\x18\x05\x20\x02(\x04\"\x16\
    \n\x14JournalResponseProto\"n\n\x1bStartLogSegmentRequestProto\x122\n\
    \x0bjournalInfo\x18\x01\x20\x02(\x0b2\x1d.hadoop.hdfs.JournalInfoProto\
    \x12\x0c\n\x04txid\x18\x02\x20\x02(\x04\x12\r\n\x05epoch\x18\x03\x20\x02\
    (\x04\"\x1e\n\x1cStartLogSegmentResponseProto\"j\n\x11FenceRequestProto\
    \x122\n\x0bjournalInfo\x18\x01\x20\x02(\x0b2\x1d.hadoop.hdfs.JournalInfo\
    Proto\x12\r\n\x05epoch\x18\x02\x20\x02(\x04\x12\x12\n\nfencerInfo\x18\
    \x03\x20\x01(\t\"V\n\x12FenceResponseProto\x12\x15\n\rpreviousEpoch\x18\
    \x01\x20\x01(\x04\x12\x19\n\x11lastTransactionId\x18\x02\x20\x01(\x04\
    \x12\x0e\n\x06inSync\x18\x03\x20\x01(\x082\x9a\x02\n\x16JournalProtocolS\
    ervice\x12N\n\x07journal\x12\x20.hadoop.hdfs.JournalRequestProto\x1a!.ha\
    doop.hdfs.JournalResponseProto\x12f\n\x0fstartLogSegment\x12(.hadoop.hdf\
    s.StartLogSegmentRequestProto\x1a).hadoop.hdfs.StartLogSegmentResponsePr\
    oto\x12H\n\x05fence\x12\x1e.hadoop.hdfs.FenceRequestProto\x1a\x1f.hadoop\
    .hdfs.FenceResponseProtoBD\n%org.apache.hadoop.hdfs.protocol.protoB\x15J\
    ournalProtocolProtos\xa0\x01\x01\x88\x01\x01\
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
