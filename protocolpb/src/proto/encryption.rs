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
pub struct CreateEncryptionZoneRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    keyName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateEncryptionZoneRequestProto {}

impl CreateEncryptionZoneRequestProto {
    pub fn new() -> CreateEncryptionZoneRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateEncryptionZoneRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<CreateEncryptionZoneRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateEncryptionZoneRequestProto,
        };
        unsafe {
            instance.get(CreateEncryptionZoneRequestProto::new)
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

    // optional string keyName = 2;

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

impl ::protobuf::Message for CreateEncryptionZoneRequestProto {
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
                2 => {
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
        if let Some(ref v) = self.src.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.keyName.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.src.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.keyName.as_ref() {
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

impl ::protobuf::MessageStatic for CreateEncryptionZoneRequestProto {
    fn new() -> CreateEncryptionZoneRequestProto {
        CreateEncryptionZoneRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateEncryptionZoneRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src",
                    CreateEncryptionZoneRequestProto::get_src_for_reflect,
                    CreateEncryptionZoneRequestProto::mut_src_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "keyName",
                    CreateEncryptionZoneRequestProto::get_keyName_for_reflect,
                    CreateEncryptionZoneRequestProto::mut_keyName_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateEncryptionZoneRequestProto>(
                    "CreateEncryptionZoneRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateEncryptionZoneRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.clear_keyName();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateEncryptionZoneRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateEncryptionZoneRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CreateEncryptionZoneResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CreateEncryptionZoneResponseProto {}

impl CreateEncryptionZoneResponseProto {
    pub fn new() -> CreateEncryptionZoneResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateEncryptionZoneResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<CreateEncryptionZoneResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateEncryptionZoneResponseProto,
        };
        unsafe {
            instance.get(CreateEncryptionZoneResponseProto::new)
        }
    }
}

impl ::protobuf::Message for CreateEncryptionZoneResponseProto {
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

impl ::protobuf::MessageStatic for CreateEncryptionZoneResponseProto {
    fn new() -> CreateEncryptionZoneResponseProto {
        CreateEncryptionZoneResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateEncryptionZoneResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CreateEncryptionZoneResponseProto>(
                    "CreateEncryptionZoneResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateEncryptionZoneResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateEncryptionZoneResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateEncryptionZoneResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListEncryptionZonesRequestProto {
    // message fields
    id: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListEncryptionZonesRequestProto {}

impl ListEncryptionZonesRequestProto {
    pub fn new() -> ListEncryptionZonesRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListEncryptionZonesRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ListEncryptionZonesRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListEncryptionZonesRequestProto,
        };
        unsafe {
            instance.get(ListEncryptionZonesRequestProto::new)
        }
    }

    // required int64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> i64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.id
    }
}

impl ::protobuf::Message for ListEncryptionZonesRequestProto {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
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
                    self.id = ::std::option::Option::Some(tmp);
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
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

impl ::protobuf::MessageStatic for ListEncryptionZonesRequestProto {
    fn new() -> ListEncryptionZonesRequestProto {
        ListEncryptionZonesRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListEncryptionZonesRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "id",
                    ListEncryptionZonesRequestProto::get_id_for_reflect,
                    ListEncryptionZonesRequestProto::mut_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListEncryptionZonesRequestProto>(
                    "ListEncryptionZonesRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListEncryptionZonesRequestProto {
    fn clear(&mut self) {
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListEncryptionZonesRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListEncryptionZonesRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EncryptionZoneProto {
    // message fields
    id: ::std::option::Option<i64>,
    path: ::protobuf::SingularField<::std::string::String>,
    suite: ::std::option::Option<super::hdfs::CipherSuiteProto>,
    cryptoProtocolVersion: ::std::option::Option<super::hdfs::CryptoProtocolVersionProto>,
    keyName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EncryptionZoneProto {}

impl EncryptionZoneProto {
    pub fn new() -> EncryptionZoneProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EncryptionZoneProto {
        static mut instance: ::protobuf::lazy::Lazy<EncryptionZoneProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EncryptionZoneProto,
        };
        unsafe {
            instance.get(EncryptionZoneProto::new)
        }
    }

    // required int64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> i64 {
        self.id.unwrap_or(0)
    }

    fn get_id_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.id
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

    // required .hadoop.hdfs.CipherSuiteProto suite = 3;

    pub fn clear_suite(&mut self) {
        self.suite = ::std::option::Option::None;
    }

    pub fn has_suite(&self) -> bool {
        self.suite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suite(&mut self, v: super::hdfs::CipherSuiteProto) {
        self.suite = ::std::option::Option::Some(v);
    }

    pub fn get_suite(&self) -> super::hdfs::CipherSuiteProto {
        self.suite.unwrap_or(super::hdfs::CipherSuiteProto::UNKNOWN)
    }

    fn get_suite_for_reflect(&self) -> &::std::option::Option<super::hdfs::CipherSuiteProto> {
        &self.suite
    }

    fn mut_suite_for_reflect(&mut self) -> &mut ::std::option::Option<super::hdfs::CipherSuiteProto> {
        &mut self.suite
    }

    // required .hadoop.hdfs.CryptoProtocolVersionProto cryptoProtocolVersion = 4;

    pub fn clear_cryptoProtocolVersion(&mut self) {
        self.cryptoProtocolVersion = ::std::option::Option::None;
    }

    pub fn has_cryptoProtocolVersion(&self) -> bool {
        self.cryptoProtocolVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cryptoProtocolVersion(&mut self, v: super::hdfs::CryptoProtocolVersionProto) {
        self.cryptoProtocolVersion = ::std::option::Option::Some(v);
    }

    pub fn get_cryptoProtocolVersion(&self) -> super::hdfs::CryptoProtocolVersionProto {
        self.cryptoProtocolVersion.unwrap_or(super::hdfs::CryptoProtocolVersionProto::UNKNOWN_PROTOCOL_VERSION)
    }

    fn get_cryptoProtocolVersion_for_reflect(&self) -> &::std::option::Option<super::hdfs::CryptoProtocolVersionProto> {
        &self.cryptoProtocolVersion
    }

    fn mut_cryptoProtocolVersion_for_reflect(&mut self) -> &mut ::std::option::Option<super::hdfs::CryptoProtocolVersionProto> {
        &mut self.cryptoProtocolVersion
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
}

impl ::protobuf::Message for EncryptionZoneProto {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        }
        if self.path.is_none() {
            return false;
        }
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
                    let tmp = is.read_int64()?;
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.suite = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.cryptoProtocolVersion = ::std::option::Option::Some(tmp);
                },
                5 => {
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
        if let Some(v) = self.id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.suite {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        if let Some(v) = self.cryptoProtocolVersion {
            my_size += ::protobuf::rt::enum_size(4, v);
        }
        if let Some(ref v) = self.keyName.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            os.write_int64(1, v)?;
        }
        if let Some(ref v) = self.path.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.suite {
            os.write_enum(3, v.value())?;
        }
        if let Some(v) = self.cryptoProtocolVersion {
            os.write_enum(4, v.value())?;
        }
        if let Some(ref v) = self.keyName.as_ref() {
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

impl ::protobuf::MessageStatic for EncryptionZoneProto {
    fn new() -> EncryptionZoneProto {
        EncryptionZoneProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<EncryptionZoneProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "id",
                    EncryptionZoneProto::get_id_for_reflect,
                    EncryptionZoneProto::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    EncryptionZoneProto::get_path_for_reflect,
                    EncryptionZoneProto::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::hdfs::CipherSuiteProto>>(
                    "suite",
                    EncryptionZoneProto::get_suite_for_reflect,
                    EncryptionZoneProto::mut_suite_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::hdfs::CryptoProtocolVersionProto>>(
                    "cryptoProtocolVersion",
                    EncryptionZoneProto::get_cryptoProtocolVersion_for_reflect,
                    EncryptionZoneProto::mut_cryptoProtocolVersion_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "keyName",
                    EncryptionZoneProto::get_keyName_for_reflect,
                    EncryptionZoneProto::mut_keyName_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EncryptionZoneProto>(
                    "EncryptionZoneProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EncryptionZoneProto {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_path();
        self.clear_suite();
        self.clear_cryptoProtocolVersion();
        self.clear_keyName();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EncryptionZoneProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EncryptionZoneProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ListEncryptionZonesResponseProto {
    // message fields
    zones: ::protobuf::RepeatedField<EncryptionZoneProto>,
    hasMore: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ListEncryptionZonesResponseProto {}

impl ListEncryptionZonesResponseProto {
    pub fn new() -> ListEncryptionZonesResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListEncryptionZonesResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ListEncryptionZonesResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListEncryptionZonesResponseProto,
        };
        unsafe {
            instance.get(ListEncryptionZonesResponseProto::new)
        }
    }

    // repeated .hadoop.hdfs.EncryptionZoneProto zones = 1;

    pub fn clear_zones(&mut self) {
        self.zones.clear();
    }

    // Param is passed by value, moved
    pub fn set_zones(&mut self, v: ::protobuf::RepeatedField<EncryptionZoneProto>) {
        self.zones = v;
    }

    // Mutable pointer to the field.
    pub fn mut_zones(&mut self) -> &mut ::protobuf::RepeatedField<EncryptionZoneProto> {
        &mut self.zones
    }

    // Take field
    pub fn take_zones(&mut self) -> ::protobuf::RepeatedField<EncryptionZoneProto> {
        ::std::mem::replace(&mut self.zones, ::protobuf::RepeatedField::new())
    }

    pub fn get_zones(&self) -> &[EncryptionZoneProto] {
        &self.zones
    }

    fn get_zones_for_reflect(&self) -> &::protobuf::RepeatedField<EncryptionZoneProto> {
        &self.zones
    }

    fn mut_zones_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<EncryptionZoneProto> {
        &mut self.zones
    }

    // required bool hasMore = 2;

    pub fn clear_hasMore(&mut self) {
        self.hasMore = ::std::option::Option::None;
    }

    pub fn has_hasMore(&self) -> bool {
        self.hasMore.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hasMore(&mut self, v: bool) {
        self.hasMore = ::std::option::Option::Some(v);
    }

    pub fn get_hasMore(&self) -> bool {
        self.hasMore.unwrap_or(false)
    }

    fn get_hasMore_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.hasMore
    }

    fn mut_hasMore_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.hasMore
    }
}

impl ::protobuf::Message for ListEncryptionZonesResponseProto {
    fn is_initialized(&self) -> bool {
        if self.hasMore.is_none() {
            return false;
        }
        for v in &self.zones {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.zones)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.hasMore = ::std::option::Option::Some(tmp);
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
        for value in &self.zones {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.hasMore {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.zones {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.hasMore {
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

impl ::protobuf::MessageStatic for ListEncryptionZonesResponseProto {
    fn new() -> ListEncryptionZonesResponseProto {
        ListEncryptionZonesResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListEncryptionZonesResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EncryptionZoneProto>>(
                    "zones",
                    ListEncryptionZonesResponseProto::get_zones_for_reflect,
                    ListEncryptionZonesResponseProto::mut_zones_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "hasMore",
                    ListEncryptionZonesResponseProto::get_hasMore_for_reflect,
                    ListEncryptionZonesResponseProto::mut_hasMore_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListEncryptionZonesResponseProto>(
                    "ListEncryptionZonesResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListEncryptionZonesResponseProto {
    fn clear(&mut self) {
        self.clear_zones();
        self.clear_hasMore();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ListEncryptionZonesResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ListEncryptionZonesResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetEZForPathRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetEZForPathRequestProto {}

impl GetEZForPathRequestProto {
    pub fn new() -> GetEZForPathRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetEZForPathRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetEZForPathRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetEZForPathRequestProto,
        };
        unsafe {
            instance.get(GetEZForPathRequestProto::new)
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

impl ::protobuf::Message for GetEZForPathRequestProto {
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

impl ::protobuf::MessageStatic for GetEZForPathRequestProto {
    fn new() -> GetEZForPathRequestProto {
        GetEZForPathRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetEZForPathRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "src",
                    GetEZForPathRequestProto::get_src_for_reflect,
                    GetEZForPathRequestProto::mut_src_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetEZForPathRequestProto>(
                    "GetEZForPathRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetEZForPathRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetEZForPathRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetEZForPathRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetEZForPathResponseProto {
    // message fields
    zone: ::protobuf::SingularPtrField<EncryptionZoneProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetEZForPathResponseProto {}

impl GetEZForPathResponseProto {
    pub fn new() -> GetEZForPathResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetEZForPathResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetEZForPathResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetEZForPathResponseProto,
        };
        unsafe {
            instance.get(GetEZForPathResponseProto::new)
        }
    }

    // optional .hadoop.hdfs.EncryptionZoneProto zone = 1;

    pub fn clear_zone(&mut self) {
        self.zone.clear();
    }

    pub fn has_zone(&self) -> bool {
        self.zone.is_some()
    }

    // Param is passed by value, moved
    pub fn set_zone(&mut self, v: EncryptionZoneProto) {
        self.zone = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_zone(&mut self) -> &mut EncryptionZoneProto {
        if self.zone.is_none() {
            self.zone.set_default();
        }
        self.zone.as_mut().unwrap()
    }

    // Take field
    pub fn take_zone(&mut self) -> EncryptionZoneProto {
        self.zone.take().unwrap_or_else(|| EncryptionZoneProto::new())
    }

    pub fn get_zone(&self) -> &EncryptionZoneProto {
        self.zone.as_ref().unwrap_or_else(|| EncryptionZoneProto::default_instance())
    }

    fn get_zone_for_reflect(&self) -> &::protobuf::SingularPtrField<EncryptionZoneProto> {
        &self.zone
    }

    fn mut_zone_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<EncryptionZoneProto> {
        &mut self.zone
    }
}

impl ::protobuf::Message for GetEZForPathResponseProto {
    fn is_initialized(&self) -> bool {
        for v in &self.zone {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.zone)?;
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
        if let Some(ref v) = self.zone.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.zone.as_ref() {
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

impl ::protobuf::MessageStatic for GetEZForPathResponseProto {
    fn new() -> GetEZForPathResponseProto {
        GetEZForPathResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetEZForPathResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EncryptionZoneProto>>(
                    "zone",
                    GetEZForPathResponseProto::get_zone_for_reflect,
                    GetEZForPathResponseProto::mut_zone_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetEZForPathResponseProto>(
                    "GetEZForPathResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetEZForPathResponseProto {
    fn clear(&mut self) {
        self.clear_zone();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetEZForPathResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetEZForPathResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10encryption.proto\x12\x0bhadoop.hdfs\x1a\nhdfs.proto\"@\n\x20Create\
    EncryptionZoneRequestProto\x12\x0b\n\x03src\x18\x01\x20\x02(\t\x12\x0f\n\
    \x07keyName\x18\x02\x20\x01(\t\"#\n!CreateEncryptionZoneResponseProto\"-\
    \n\x1fListEncryptionZonesRequestProto\x12\n\n\x02id\x18\x01\x20\x02(\x03\
    \"\xb6\x01\n\x13EncryptionZoneProto\x12\n\n\x02id\x18\x01\x20\x02(\x03\
    \x12\x0c\n\x04path\x18\x02\x20\x02(\t\x12,\n\x05suite\x18\x03\x20\x02(\
    \x0e2\x1d.hadoop.hdfs.CipherSuiteProto\x12F\n\x15cryptoProtocolVersion\
    \x18\x04\x20\x02(\x0e2'.hadoop.hdfs.CryptoProtocolVersionProto\x12\x0f\n\
    \x07keyName\x18\x05\x20\x02(\t\"d\n\x20ListEncryptionZonesResponseProto\
    \x12/\n\x05zones\x18\x01\x20\x03(\x0b2\x20.hadoop.hdfs.EncryptionZonePro\
    to\x12\x0f\n\x07hasMore\x18\x02\x20\x02(\x08\"'\n\x18GetEZForPathRequest\
    Proto\x12\x0b\n\x03src\x18\x01\x20\x02(\t\"K\n\x19GetEZForPathResponsePr\
    oto\x12.\n\x04zone\x18\x01\x20\x01(\x0b2\x20.hadoop.hdfs.EncryptionZoneP\
    rotoBA\n%org.apache.hadoop.hdfs.protocol.protoB\x15EncryptionZonesProtos\
    \xa0\x01\x01\
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
