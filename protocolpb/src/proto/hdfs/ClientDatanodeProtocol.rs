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
pub struct GetReplicaVisibleLengthRequestProto {
    // message fields
    block: ::protobuf::SingularPtrField<super::hdfs::ExtendedBlockProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetReplicaVisibleLengthRequestProto {}

impl GetReplicaVisibleLengthRequestProto {
    pub fn new() -> GetReplicaVisibleLengthRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetReplicaVisibleLengthRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetReplicaVisibleLengthRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetReplicaVisibleLengthRequestProto,
        };
        unsafe {
            instance.get(GetReplicaVisibleLengthRequestProto::new)
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
}

impl ::protobuf::Message for GetReplicaVisibleLengthRequestProto {
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

impl ::protobuf::MessageStatic for GetReplicaVisibleLengthRequestProto {
    fn new() -> GetReplicaVisibleLengthRequestProto {
        GetReplicaVisibleLengthRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetReplicaVisibleLengthRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::ExtendedBlockProto>>(
                    "block",
                    GetReplicaVisibleLengthRequestProto::get_block_for_reflect,
                    GetReplicaVisibleLengthRequestProto::mut_block_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetReplicaVisibleLengthRequestProto>(
                    "GetReplicaVisibleLengthRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetReplicaVisibleLengthRequestProto {
    fn clear(&mut self) {
        self.clear_block();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetReplicaVisibleLengthRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetReplicaVisibleLengthRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetReplicaVisibleLengthResponseProto {
    // message fields
    length: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetReplicaVisibleLengthResponseProto {}

impl GetReplicaVisibleLengthResponseProto {
    pub fn new() -> GetReplicaVisibleLengthResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetReplicaVisibleLengthResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetReplicaVisibleLengthResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetReplicaVisibleLengthResponseProto,
        };
        unsafe {
            instance.get(GetReplicaVisibleLengthResponseProto::new)
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
}

impl ::protobuf::Message for GetReplicaVisibleLengthResponseProto {
    fn is_initialized(&self) -> bool {
        if self.length.is_none() {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.length {
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

impl ::protobuf::MessageStatic for GetReplicaVisibleLengthResponseProto {
    fn new() -> GetReplicaVisibleLengthResponseProto {
        GetReplicaVisibleLengthResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetReplicaVisibleLengthResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "length",
                    GetReplicaVisibleLengthResponseProto::get_length_for_reflect,
                    GetReplicaVisibleLengthResponseProto::mut_length_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetReplicaVisibleLengthResponseProto>(
                    "GetReplicaVisibleLengthResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetReplicaVisibleLengthResponseProto {
    fn clear(&mut self) {
        self.clear_length();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetReplicaVisibleLengthResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetReplicaVisibleLengthResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RefreshNamenodesRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RefreshNamenodesRequestProto {}

impl RefreshNamenodesRequestProto {
    pub fn new() -> RefreshNamenodesRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RefreshNamenodesRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RefreshNamenodesRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RefreshNamenodesRequestProto,
        };
        unsafe {
            instance.get(RefreshNamenodesRequestProto::new)
        }
    }
}

impl ::protobuf::Message for RefreshNamenodesRequestProto {
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

impl ::protobuf::MessageStatic for RefreshNamenodesRequestProto {
    fn new() -> RefreshNamenodesRequestProto {
        RefreshNamenodesRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RefreshNamenodesRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RefreshNamenodesRequestProto>(
                    "RefreshNamenodesRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RefreshNamenodesRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RefreshNamenodesRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RefreshNamenodesRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RefreshNamenodesResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RefreshNamenodesResponseProto {}

impl RefreshNamenodesResponseProto {
    pub fn new() -> RefreshNamenodesResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RefreshNamenodesResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RefreshNamenodesResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RefreshNamenodesResponseProto,
        };
        unsafe {
            instance.get(RefreshNamenodesResponseProto::new)
        }
    }
}

impl ::protobuf::Message for RefreshNamenodesResponseProto {
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

impl ::protobuf::MessageStatic for RefreshNamenodesResponseProto {
    fn new() -> RefreshNamenodesResponseProto {
        RefreshNamenodesResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RefreshNamenodesResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RefreshNamenodesResponseProto>(
                    "RefreshNamenodesResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RefreshNamenodesResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RefreshNamenodesResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RefreshNamenodesResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteBlockPoolRequestProto {
    // message fields
    blockPool: ::protobuf::SingularField<::std::string::String>,
    force: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteBlockPoolRequestProto {}

impl DeleteBlockPoolRequestProto {
    pub fn new() -> DeleteBlockPoolRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteBlockPoolRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<DeleteBlockPoolRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteBlockPoolRequestProto,
        };
        unsafe {
            instance.get(DeleteBlockPoolRequestProto::new)
        }
    }

    // required string blockPool = 1;

    pub fn clear_blockPool(&mut self) {
        self.blockPool.clear();
    }

    pub fn has_blockPool(&self) -> bool {
        self.blockPool.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPool(&mut self, v: ::std::string::String) {
        self.blockPool = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockPool(&mut self) -> &mut ::std::string::String {
        if self.blockPool.is_none() {
            self.blockPool.set_default();
        }
        self.blockPool.as_mut().unwrap()
    }

    // Take field
    pub fn take_blockPool(&mut self) -> ::std::string::String {
        self.blockPool.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_blockPool(&self) -> &str {
        match self.blockPool.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_blockPool_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.blockPool
    }

    fn mut_blockPool_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.blockPool
    }

    // required bool force = 2;

    pub fn clear_force(&mut self) {
        self.force = ::std::option::Option::None;
    }

    pub fn has_force(&self) -> bool {
        self.force.is_some()
    }

    // Param is passed by value, moved
    pub fn set_force(&mut self, v: bool) {
        self.force = ::std::option::Option::Some(v);
    }

    pub fn get_force(&self) -> bool {
        self.force.unwrap_or(false)
    }

    fn get_force_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.force
    }

    fn mut_force_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.force
    }
}

impl ::protobuf::Message for DeleteBlockPoolRequestProto {
    fn is_initialized(&self) -> bool {
        if self.blockPool.is_none() {
            return false;
        }
        if self.force.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.blockPool)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.force = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.blockPool.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.force {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.blockPool.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.force {
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

impl ::protobuf::MessageStatic for DeleteBlockPoolRequestProto {
    fn new() -> DeleteBlockPoolRequestProto {
        DeleteBlockPoolRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteBlockPoolRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "blockPool",
                    DeleteBlockPoolRequestProto::get_blockPool_for_reflect,
                    DeleteBlockPoolRequestProto::mut_blockPool_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "force",
                    DeleteBlockPoolRequestProto::get_force_for_reflect,
                    DeleteBlockPoolRequestProto::mut_force_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteBlockPoolRequestProto>(
                    "DeleteBlockPoolRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteBlockPoolRequestProto {
    fn clear(&mut self) {
        self.clear_blockPool();
        self.clear_force();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteBlockPoolRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteBlockPoolRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteBlockPoolResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteBlockPoolResponseProto {}

impl DeleteBlockPoolResponseProto {
    pub fn new() -> DeleteBlockPoolResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteBlockPoolResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<DeleteBlockPoolResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteBlockPoolResponseProto,
        };
        unsafe {
            instance.get(DeleteBlockPoolResponseProto::new)
        }
    }
}

impl ::protobuf::Message for DeleteBlockPoolResponseProto {
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

impl ::protobuf::MessageStatic for DeleteBlockPoolResponseProto {
    fn new() -> DeleteBlockPoolResponseProto {
        DeleteBlockPoolResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteBlockPoolResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DeleteBlockPoolResponseProto>(
                    "DeleteBlockPoolResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteBlockPoolResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteBlockPoolResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteBlockPoolResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetBlockLocalPathInfoRequestProto {
    // message fields
    block: ::protobuf::SingularPtrField<super::hdfs::ExtendedBlockProto>,
    token: ::protobuf::SingularPtrField<super::Security::TokenProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetBlockLocalPathInfoRequestProto {}

impl GetBlockLocalPathInfoRequestProto {
    pub fn new() -> GetBlockLocalPathInfoRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBlockLocalPathInfoRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetBlockLocalPathInfoRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBlockLocalPathInfoRequestProto,
        };
        unsafe {
            instance.get(GetBlockLocalPathInfoRequestProto::new)
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

    // required .hadoop.common.TokenProto token = 2;

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
}

impl ::protobuf::Message for GetBlockLocalPathInfoRequestProto {
    fn is_initialized(&self) -> bool {
        if self.block.is_none() {
            return false;
        }
        if self.token.is_none() {
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

impl ::protobuf::MessageStatic for GetBlockLocalPathInfoRequestProto {
    fn new() -> GetBlockLocalPathInfoRequestProto {
        GetBlockLocalPathInfoRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBlockLocalPathInfoRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::ExtendedBlockProto>>(
                    "block",
                    GetBlockLocalPathInfoRequestProto::get_block_for_reflect,
                    GetBlockLocalPathInfoRequestProto::mut_block_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::Security::TokenProto>>(
                    "token",
                    GetBlockLocalPathInfoRequestProto::get_token_for_reflect,
                    GetBlockLocalPathInfoRequestProto::mut_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetBlockLocalPathInfoRequestProto>(
                    "GetBlockLocalPathInfoRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBlockLocalPathInfoRequestProto {
    fn clear(&mut self) {
        self.clear_block();
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetBlockLocalPathInfoRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBlockLocalPathInfoRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetBlockLocalPathInfoResponseProto {
    // message fields
    block: ::protobuf::SingularPtrField<super::hdfs::ExtendedBlockProto>,
    localPath: ::protobuf::SingularField<::std::string::String>,
    localMetaPath: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetBlockLocalPathInfoResponseProto {}

impl GetBlockLocalPathInfoResponseProto {
    pub fn new() -> GetBlockLocalPathInfoResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBlockLocalPathInfoResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetBlockLocalPathInfoResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBlockLocalPathInfoResponseProto,
        };
        unsafe {
            instance.get(GetBlockLocalPathInfoResponseProto::new)
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

    // required string localPath = 2;

    pub fn clear_localPath(&mut self) {
        self.localPath.clear();
    }

    pub fn has_localPath(&self) -> bool {
        self.localPath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_localPath(&mut self, v: ::std::string::String) {
        self.localPath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_localPath(&mut self) -> &mut ::std::string::String {
        if self.localPath.is_none() {
            self.localPath.set_default();
        }
        self.localPath.as_mut().unwrap()
    }

    // Take field
    pub fn take_localPath(&mut self) -> ::std::string::String {
        self.localPath.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_localPath(&self) -> &str {
        match self.localPath.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_localPath_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.localPath
    }

    fn mut_localPath_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.localPath
    }

    // required string localMetaPath = 3;

    pub fn clear_localMetaPath(&mut self) {
        self.localMetaPath.clear();
    }

    pub fn has_localMetaPath(&self) -> bool {
        self.localMetaPath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_localMetaPath(&mut self, v: ::std::string::String) {
        self.localMetaPath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_localMetaPath(&mut self) -> &mut ::std::string::String {
        if self.localMetaPath.is_none() {
            self.localMetaPath.set_default();
        }
        self.localMetaPath.as_mut().unwrap()
    }

    // Take field
    pub fn take_localMetaPath(&mut self) -> ::std::string::String {
        self.localMetaPath.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_localMetaPath(&self) -> &str {
        match self.localMetaPath.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_localMetaPath_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.localMetaPath
    }

    fn mut_localMetaPath_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.localMetaPath
    }
}

impl ::protobuf::Message for GetBlockLocalPathInfoResponseProto {
    fn is_initialized(&self) -> bool {
        if self.block.is_none() {
            return false;
        }
        if self.localPath.is_none() {
            return false;
        }
        if self.localMetaPath.is_none() {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.localPath)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.localMetaPath)?;
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
        if let Some(ref v) = self.localPath.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.localMetaPath.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
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
        if let Some(ref v) = self.localPath.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.localMetaPath.as_ref() {
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

impl ::protobuf::MessageStatic for GetBlockLocalPathInfoResponseProto {
    fn new() -> GetBlockLocalPathInfoResponseProto {
        GetBlockLocalPathInfoResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBlockLocalPathInfoResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::ExtendedBlockProto>>(
                    "block",
                    GetBlockLocalPathInfoResponseProto::get_block_for_reflect,
                    GetBlockLocalPathInfoResponseProto::mut_block_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "localPath",
                    GetBlockLocalPathInfoResponseProto::get_localPath_for_reflect,
                    GetBlockLocalPathInfoResponseProto::mut_localPath_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "localMetaPath",
                    GetBlockLocalPathInfoResponseProto::get_localMetaPath_for_reflect,
                    GetBlockLocalPathInfoResponseProto::mut_localMetaPath_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetBlockLocalPathInfoResponseProto>(
                    "GetBlockLocalPathInfoResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBlockLocalPathInfoResponseProto {
    fn clear(&mut self) {
        self.clear_block();
        self.clear_localPath();
        self.clear_localMetaPath();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetBlockLocalPathInfoResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBlockLocalPathInfoResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetHdfsBlockLocationsRequestProto {
    // message fields
    tokens: ::protobuf::RepeatedField<super::Security::TokenProto>,
    blockPoolId: ::protobuf::SingularField<::std::string::String>,
    blockIds: ::std::vec::Vec<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetHdfsBlockLocationsRequestProto {}

impl GetHdfsBlockLocationsRequestProto {
    pub fn new() -> GetHdfsBlockLocationsRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetHdfsBlockLocationsRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetHdfsBlockLocationsRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetHdfsBlockLocationsRequestProto,
        };
        unsafe {
            instance.get(GetHdfsBlockLocationsRequestProto::new)
        }
    }

    // repeated .hadoop.common.TokenProto tokens = 2;

    pub fn clear_tokens(&mut self) {
        self.tokens.clear();
    }

    // Param is passed by value, moved
    pub fn set_tokens(&mut self, v: ::protobuf::RepeatedField<super::Security::TokenProto>) {
        self.tokens = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tokens(&mut self) -> &mut ::protobuf::RepeatedField<super::Security::TokenProto> {
        &mut self.tokens
    }

    // Take field
    pub fn take_tokens(&mut self) -> ::protobuf::RepeatedField<super::Security::TokenProto> {
        ::std::mem::replace(&mut self.tokens, ::protobuf::RepeatedField::new())
    }

    pub fn get_tokens(&self) -> &[super::Security::TokenProto] {
        &self.tokens
    }

    fn get_tokens_for_reflect(&self) -> &::protobuf::RepeatedField<super::Security::TokenProto> {
        &self.tokens
    }

    fn mut_tokens_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::Security::TokenProto> {
        &mut self.tokens
    }

    // required string blockPoolId = 3;

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

    // repeated sfixed64 blockIds = 4;

    pub fn clear_blockIds(&mut self) {
        self.blockIds.clear();
    }

    // Param is passed by value, moved
    pub fn set_blockIds(&mut self, v: ::std::vec::Vec<i64>) {
        self.blockIds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blockIds(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.blockIds
    }

    // Take field
    pub fn take_blockIds(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.blockIds, ::std::vec::Vec::new())
    }

    pub fn get_blockIds(&self) -> &[i64] {
        &self.blockIds
    }

    fn get_blockIds_for_reflect(&self) -> &::std::vec::Vec<i64> {
        &self.blockIds
    }

    fn mut_blockIds_for_reflect(&mut self) -> &mut ::std::vec::Vec<i64> {
        &mut self.blockIds
    }
}

impl ::protobuf::Message for GetHdfsBlockLocationsRequestProto {
    fn is_initialized(&self) -> bool {
        if self.blockPoolId.is_none() {
            return false;
        }
        for v in &self.tokens {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tokens)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.blockPoolId)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_sfixed64_into(wire_type, is, &mut self.blockIds)?;
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
        for value in &self.tokens {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.blockPoolId.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if !self.blockIds.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.blockIds.len() as u32) + (self.blockIds.len() * 8) as u32;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.tokens {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.blockPoolId.as_ref() {
            os.write_string(3, &v)?;
        }
        if !self.blockIds.is_empty() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32((self.blockIds.len() * 8) as u32)?;
            for v in &self.blockIds {
                os.write_sfixed64_no_tag(*v)?;
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

impl ::protobuf::MessageStatic for GetHdfsBlockLocationsRequestProto {
    fn new() -> GetHdfsBlockLocationsRequestProto {
        GetHdfsBlockLocationsRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetHdfsBlockLocationsRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::Security::TokenProto>>(
                    "tokens",
                    GetHdfsBlockLocationsRequestProto::get_tokens_for_reflect,
                    GetHdfsBlockLocationsRequestProto::mut_tokens_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "blockPoolId",
                    GetHdfsBlockLocationsRequestProto::get_blockPoolId_for_reflect,
                    GetHdfsBlockLocationsRequestProto::mut_blockPoolId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeSfixed64>(
                    "blockIds",
                    GetHdfsBlockLocationsRequestProto::get_blockIds_for_reflect,
                    GetHdfsBlockLocationsRequestProto::mut_blockIds_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetHdfsBlockLocationsRequestProto>(
                    "GetHdfsBlockLocationsRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetHdfsBlockLocationsRequestProto {
    fn clear(&mut self) {
        self.clear_tokens();
        self.clear_blockPoolId();
        self.clear_blockIds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetHdfsBlockLocationsRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetHdfsBlockLocationsRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetHdfsBlockLocationsResponseProto {
    // message fields
    volumeIds: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    volumeIndexes: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetHdfsBlockLocationsResponseProto {}

impl GetHdfsBlockLocationsResponseProto {
    pub fn new() -> GetHdfsBlockLocationsResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetHdfsBlockLocationsResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetHdfsBlockLocationsResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetHdfsBlockLocationsResponseProto,
        };
        unsafe {
            instance.get(GetHdfsBlockLocationsResponseProto::new)
        }
    }

    // repeated bytes volumeIds = 1;

    pub fn clear_volumeIds(&mut self) {
        self.volumeIds.clear();
    }

    // Param is passed by value, moved
    pub fn set_volumeIds(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.volumeIds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_volumeIds(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.volumeIds
    }

    // Take field
    pub fn take_volumeIds(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.volumeIds, ::protobuf::RepeatedField::new())
    }

    pub fn get_volumeIds(&self) -> &[::std::vec::Vec<u8>] {
        &self.volumeIds
    }

    fn get_volumeIds_for_reflect(&self) -> &::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &self.volumeIds
    }

    fn mut_volumeIds_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.volumeIds
    }

    // repeated uint32 volumeIndexes = 2;

    pub fn clear_volumeIndexes(&mut self) {
        self.volumeIndexes.clear();
    }

    // Param is passed by value, moved
    pub fn set_volumeIndexes(&mut self, v: ::std::vec::Vec<u32>) {
        self.volumeIndexes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_volumeIndexes(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.volumeIndexes
    }

    // Take field
    pub fn take_volumeIndexes(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.volumeIndexes, ::std::vec::Vec::new())
    }

    pub fn get_volumeIndexes(&self) -> &[u32] {
        &self.volumeIndexes
    }

    fn get_volumeIndexes_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.volumeIndexes
    }

    fn mut_volumeIndexes_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.volumeIndexes
    }
}

impl ::protobuf::Message for GetHdfsBlockLocationsResponseProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.volumeIds)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.volumeIndexes)?;
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
        for value in &self.volumeIds {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        if !self.volumeIndexes.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(2, &self.volumeIndexes);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.volumeIds {
            os.write_bytes(1, &v)?;
        };
        if !self.volumeIndexes.is_empty() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.volumeIndexes))?;
            for v in &self.volumeIndexes {
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

impl ::protobuf::MessageStatic for GetHdfsBlockLocationsResponseProto {
    fn new() -> GetHdfsBlockLocationsResponseProto {
        GetHdfsBlockLocationsResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetHdfsBlockLocationsResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "volumeIds",
                    GetHdfsBlockLocationsResponseProto::get_volumeIds_for_reflect,
                    GetHdfsBlockLocationsResponseProto::mut_volumeIds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "volumeIndexes",
                    GetHdfsBlockLocationsResponseProto::get_volumeIndexes_for_reflect,
                    GetHdfsBlockLocationsResponseProto::mut_volumeIndexes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetHdfsBlockLocationsResponseProto>(
                    "GetHdfsBlockLocationsResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetHdfsBlockLocationsResponseProto {
    fn clear(&mut self) {
        self.clear_volumeIds();
        self.clear_volumeIndexes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetHdfsBlockLocationsResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetHdfsBlockLocationsResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ShutdownDatanodeRequestProto {
    // message fields
    forUpgrade: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ShutdownDatanodeRequestProto {}

impl ShutdownDatanodeRequestProto {
    pub fn new() -> ShutdownDatanodeRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShutdownDatanodeRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ShutdownDatanodeRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShutdownDatanodeRequestProto,
        };
        unsafe {
            instance.get(ShutdownDatanodeRequestProto::new)
        }
    }

    // required bool forUpgrade = 1;

    pub fn clear_forUpgrade(&mut self) {
        self.forUpgrade = ::std::option::Option::None;
    }

    pub fn has_forUpgrade(&self) -> bool {
        self.forUpgrade.is_some()
    }

    // Param is passed by value, moved
    pub fn set_forUpgrade(&mut self, v: bool) {
        self.forUpgrade = ::std::option::Option::Some(v);
    }

    pub fn get_forUpgrade(&self) -> bool {
        self.forUpgrade.unwrap_or(false)
    }

    fn get_forUpgrade_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.forUpgrade
    }

    fn mut_forUpgrade_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.forUpgrade
    }
}

impl ::protobuf::Message for ShutdownDatanodeRequestProto {
    fn is_initialized(&self) -> bool {
        if self.forUpgrade.is_none() {
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
                    self.forUpgrade = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.forUpgrade {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.forUpgrade {
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

impl ::protobuf::MessageStatic for ShutdownDatanodeRequestProto {
    fn new() -> ShutdownDatanodeRequestProto {
        ShutdownDatanodeRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShutdownDatanodeRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "forUpgrade",
                    ShutdownDatanodeRequestProto::get_forUpgrade_for_reflect,
                    ShutdownDatanodeRequestProto::mut_forUpgrade_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShutdownDatanodeRequestProto>(
                    "ShutdownDatanodeRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShutdownDatanodeRequestProto {
    fn clear(&mut self) {
        self.clear_forUpgrade();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ShutdownDatanodeRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShutdownDatanodeRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ShutdownDatanodeResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ShutdownDatanodeResponseProto {}

impl ShutdownDatanodeResponseProto {
    pub fn new() -> ShutdownDatanodeResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShutdownDatanodeResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ShutdownDatanodeResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShutdownDatanodeResponseProto,
        };
        unsafe {
            instance.get(ShutdownDatanodeResponseProto::new)
        }
    }
}

impl ::protobuf::Message for ShutdownDatanodeResponseProto {
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

impl ::protobuf::MessageStatic for ShutdownDatanodeResponseProto {
    fn new() -> ShutdownDatanodeResponseProto {
        ShutdownDatanodeResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShutdownDatanodeResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ShutdownDatanodeResponseProto>(
                    "ShutdownDatanodeResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShutdownDatanodeResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ShutdownDatanodeResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShutdownDatanodeResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetDatanodeInfoRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetDatanodeInfoRequestProto {}

impl GetDatanodeInfoRequestProto {
    pub fn new() -> GetDatanodeInfoRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetDatanodeInfoRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetDatanodeInfoRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetDatanodeInfoRequestProto,
        };
        unsafe {
            instance.get(GetDatanodeInfoRequestProto::new)
        }
    }
}

impl ::protobuf::Message for GetDatanodeInfoRequestProto {
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

impl ::protobuf::MessageStatic for GetDatanodeInfoRequestProto {
    fn new() -> GetDatanodeInfoRequestProto {
        GetDatanodeInfoRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetDatanodeInfoRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetDatanodeInfoRequestProto>(
                    "GetDatanodeInfoRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetDatanodeInfoRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetDatanodeInfoRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetDatanodeInfoRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetDatanodeInfoResponseProto {
    // message fields
    localInfo: ::protobuf::SingularPtrField<super::hdfs::DatanodeLocalInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetDatanodeInfoResponseProto {}

impl GetDatanodeInfoResponseProto {
    pub fn new() -> GetDatanodeInfoResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetDatanodeInfoResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetDatanodeInfoResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetDatanodeInfoResponseProto,
        };
        unsafe {
            instance.get(GetDatanodeInfoResponseProto::new)
        }
    }

    // required .hadoop.hdfs.DatanodeLocalInfoProto localInfo = 1;

    pub fn clear_localInfo(&mut self) {
        self.localInfo.clear();
    }

    pub fn has_localInfo(&self) -> bool {
        self.localInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_localInfo(&mut self, v: super::hdfs::DatanodeLocalInfoProto) {
        self.localInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_localInfo(&mut self) -> &mut super::hdfs::DatanodeLocalInfoProto {
        if self.localInfo.is_none() {
            self.localInfo.set_default();
        }
        self.localInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_localInfo(&mut self) -> super::hdfs::DatanodeLocalInfoProto {
        self.localInfo.take().unwrap_or_else(|| super::hdfs::DatanodeLocalInfoProto::new())
    }

    pub fn get_localInfo(&self) -> &super::hdfs::DatanodeLocalInfoProto {
        self.localInfo.as_ref().unwrap_or_else(|| super::hdfs::DatanodeLocalInfoProto::default_instance())
    }

    fn get_localInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::DatanodeLocalInfoProto> {
        &self.localInfo
    }

    fn mut_localInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::DatanodeLocalInfoProto> {
        &mut self.localInfo
    }
}

impl ::protobuf::Message for GetDatanodeInfoResponseProto {
    fn is_initialized(&self) -> bool {
        if self.localInfo.is_none() {
            return false;
        }
        for v in &self.localInfo {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.localInfo)?;
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
        if let Some(ref v) = self.localInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.localInfo.as_ref() {
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

impl ::protobuf::MessageStatic for GetDatanodeInfoResponseProto {
    fn new() -> GetDatanodeInfoResponseProto {
        GetDatanodeInfoResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetDatanodeInfoResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeLocalInfoProto>>(
                    "localInfo",
                    GetDatanodeInfoResponseProto::get_localInfo_for_reflect,
                    GetDatanodeInfoResponseProto::mut_localInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetDatanodeInfoResponseProto>(
                    "GetDatanodeInfoResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetDatanodeInfoResponseProto {
    fn clear(&mut self) {
        self.clear_localInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetDatanodeInfoResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetDatanodeInfoResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StartReconfigurationRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StartReconfigurationRequestProto {}

impl StartReconfigurationRequestProto {
    pub fn new() -> StartReconfigurationRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StartReconfigurationRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<StartReconfigurationRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StartReconfigurationRequestProto,
        };
        unsafe {
            instance.get(StartReconfigurationRequestProto::new)
        }
    }
}

impl ::protobuf::Message for StartReconfigurationRequestProto {
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

impl ::protobuf::MessageStatic for StartReconfigurationRequestProto {
    fn new() -> StartReconfigurationRequestProto {
        StartReconfigurationRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StartReconfigurationRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<StartReconfigurationRequestProto>(
                    "StartReconfigurationRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StartReconfigurationRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StartReconfigurationRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartReconfigurationRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StartReconfigurationResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StartReconfigurationResponseProto {}

impl StartReconfigurationResponseProto {
    pub fn new() -> StartReconfigurationResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StartReconfigurationResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<StartReconfigurationResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StartReconfigurationResponseProto,
        };
        unsafe {
            instance.get(StartReconfigurationResponseProto::new)
        }
    }
}

impl ::protobuf::Message for StartReconfigurationResponseProto {
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

impl ::protobuf::MessageStatic for StartReconfigurationResponseProto {
    fn new() -> StartReconfigurationResponseProto {
        StartReconfigurationResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StartReconfigurationResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<StartReconfigurationResponseProto>(
                    "StartReconfigurationResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StartReconfigurationResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StartReconfigurationResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartReconfigurationResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetReconfigurationStatusRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetReconfigurationStatusRequestProto {}

impl GetReconfigurationStatusRequestProto {
    pub fn new() -> GetReconfigurationStatusRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetReconfigurationStatusRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetReconfigurationStatusRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetReconfigurationStatusRequestProto,
        };
        unsafe {
            instance.get(GetReconfigurationStatusRequestProto::new)
        }
    }
}

impl ::protobuf::Message for GetReconfigurationStatusRequestProto {
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

impl ::protobuf::MessageStatic for GetReconfigurationStatusRequestProto {
    fn new() -> GetReconfigurationStatusRequestProto {
        GetReconfigurationStatusRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetReconfigurationStatusRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetReconfigurationStatusRequestProto>(
                    "GetReconfigurationStatusRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetReconfigurationStatusRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetReconfigurationStatusRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetReconfigurationStatusRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetReconfigurationStatusConfigChangeProto {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    oldValue: ::protobuf::SingularField<::std::string::String>,
    newValue: ::protobuf::SingularField<::std::string::String>,
    errorMessage: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetReconfigurationStatusConfigChangeProto {}

impl GetReconfigurationStatusConfigChangeProto {
    pub fn new() -> GetReconfigurationStatusConfigChangeProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetReconfigurationStatusConfigChangeProto {
        static mut instance: ::protobuf::lazy::Lazy<GetReconfigurationStatusConfigChangeProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetReconfigurationStatusConfigChangeProto,
        };
        unsafe {
            instance.get(GetReconfigurationStatusConfigChangeProto::new)
        }
    }

    // required string name = 1;

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

    // required string oldValue = 2;

    pub fn clear_oldValue(&mut self) {
        self.oldValue.clear();
    }

    pub fn has_oldValue(&self) -> bool {
        self.oldValue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_oldValue(&mut self, v: ::std::string::String) {
        self.oldValue = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_oldValue(&mut self) -> &mut ::std::string::String {
        if self.oldValue.is_none() {
            self.oldValue.set_default();
        }
        self.oldValue.as_mut().unwrap()
    }

    // Take field
    pub fn take_oldValue(&mut self) -> ::std::string::String {
        self.oldValue.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_oldValue(&self) -> &str {
        match self.oldValue.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_oldValue_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.oldValue
    }

    fn mut_oldValue_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.oldValue
    }

    // optional string newValue = 3;

    pub fn clear_newValue(&mut self) {
        self.newValue.clear();
    }

    pub fn has_newValue(&self) -> bool {
        self.newValue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_newValue(&mut self, v: ::std::string::String) {
        self.newValue = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_newValue(&mut self) -> &mut ::std::string::String {
        if self.newValue.is_none() {
            self.newValue.set_default();
        }
        self.newValue.as_mut().unwrap()
    }

    // Take field
    pub fn take_newValue(&mut self) -> ::std::string::String {
        self.newValue.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_newValue(&self) -> &str {
        match self.newValue.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_newValue_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.newValue
    }

    fn mut_newValue_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.newValue
    }

    // optional string errorMessage = 4;

    pub fn clear_errorMessage(&mut self) {
        self.errorMessage.clear();
    }

    pub fn has_errorMessage(&self) -> bool {
        self.errorMessage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_errorMessage(&mut self, v: ::std::string::String) {
        self.errorMessage = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_errorMessage(&mut self) -> &mut ::std::string::String {
        if self.errorMessage.is_none() {
            self.errorMessage.set_default();
        }
        self.errorMessage.as_mut().unwrap()
    }

    // Take field
    pub fn take_errorMessage(&mut self) -> ::std::string::String {
        self.errorMessage.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_errorMessage(&self) -> &str {
        match self.errorMessage.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_errorMessage_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.errorMessage
    }

    fn mut_errorMessage_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.errorMessage
    }
}

impl ::protobuf::Message for GetReconfigurationStatusConfigChangeProto {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        if self.oldValue.is_none() {
            return false;
        }
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.oldValue)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.newValue)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.errorMessage)?;
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
        if let Some(ref v) = self.oldValue.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.newValue.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.errorMessage.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.oldValue.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.newValue.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.errorMessage.as_ref() {
            os.write_string(4, &v)?;
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

impl ::protobuf::MessageStatic for GetReconfigurationStatusConfigChangeProto {
    fn new() -> GetReconfigurationStatusConfigChangeProto {
        GetReconfigurationStatusConfigChangeProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetReconfigurationStatusConfigChangeProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    GetReconfigurationStatusConfigChangeProto::get_name_for_reflect,
                    GetReconfigurationStatusConfigChangeProto::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "oldValue",
                    GetReconfigurationStatusConfigChangeProto::get_oldValue_for_reflect,
                    GetReconfigurationStatusConfigChangeProto::mut_oldValue_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "newValue",
                    GetReconfigurationStatusConfigChangeProto::get_newValue_for_reflect,
                    GetReconfigurationStatusConfigChangeProto::mut_newValue_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "errorMessage",
                    GetReconfigurationStatusConfigChangeProto::get_errorMessage_for_reflect,
                    GetReconfigurationStatusConfigChangeProto::mut_errorMessage_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetReconfigurationStatusConfigChangeProto>(
                    "GetReconfigurationStatusConfigChangeProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetReconfigurationStatusConfigChangeProto {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_oldValue();
        self.clear_newValue();
        self.clear_errorMessage();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetReconfigurationStatusConfigChangeProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetReconfigurationStatusConfigChangeProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetReconfigurationStatusResponseProto {
    // message fields
    startTime: ::std::option::Option<i64>,
    endTime: ::std::option::Option<i64>,
    changes: ::protobuf::RepeatedField<GetReconfigurationStatusConfigChangeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetReconfigurationStatusResponseProto {}

impl GetReconfigurationStatusResponseProto {
    pub fn new() -> GetReconfigurationStatusResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetReconfigurationStatusResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetReconfigurationStatusResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetReconfigurationStatusResponseProto,
        };
        unsafe {
            instance.get(GetReconfigurationStatusResponseProto::new)
        }
    }

    // required int64 startTime = 1;

    pub fn clear_startTime(&mut self) {
        self.startTime = ::std::option::Option::None;
    }

    pub fn has_startTime(&self) -> bool {
        self.startTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_startTime(&mut self, v: i64) {
        self.startTime = ::std::option::Option::Some(v);
    }

    pub fn get_startTime(&self) -> i64 {
        self.startTime.unwrap_or(0)
    }

    fn get_startTime_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.startTime
    }

    fn mut_startTime_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.startTime
    }

    // optional int64 endTime = 2;

    pub fn clear_endTime(&mut self) {
        self.endTime = ::std::option::Option::None;
    }

    pub fn has_endTime(&self) -> bool {
        self.endTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_endTime(&mut self, v: i64) {
        self.endTime = ::std::option::Option::Some(v);
    }

    pub fn get_endTime(&self) -> i64 {
        self.endTime.unwrap_or(0)
    }

    fn get_endTime_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.endTime
    }

    fn mut_endTime_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.endTime
    }

    // repeated .hadoop.hdfs.GetReconfigurationStatusConfigChangeProto changes = 3;

    pub fn clear_changes(&mut self) {
        self.changes.clear();
    }

    // Param is passed by value, moved
    pub fn set_changes(&mut self, v: ::protobuf::RepeatedField<GetReconfigurationStatusConfigChangeProto>) {
        self.changes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_changes(&mut self) -> &mut ::protobuf::RepeatedField<GetReconfigurationStatusConfigChangeProto> {
        &mut self.changes
    }

    // Take field
    pub fn take_changes(&mut self) -> ::protobuf::RepeatedField<GetReconfigurationStatusConfigChangeProto> {
        ::std::mem::replace(&mut self.changes, ::protobuf::RepeatedField::new())
    }

    pub fn get_changes(&self) -> &[GetReconfigurationStatusConfigChangeProto] {
        &self.changes
    }

    fn get_changes_for_reflect(&self) -> &::protobuf::RepeatedField<GetReconfigurationStatusConfigChangeProto> {
        &self.changes
    }

    fn mut_changes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<GetReconfigurationStatusConfigChangeProto> {
        &mut self.changes
    }
}

impl ::protobuf::Message for GetReconfigurationStatusResponseProto {
    fn is_initialized(&self) -> bool {
        if self.startTime.is_none() {
            return false;
        }
        for v in &self.changes {
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
                    let tmp = is.read_int64()?;
                    self.startTime = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.endTime = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.changes)?;
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
        if let Some(v) = self.startTime {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.endTime {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.changes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.startTime {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.endTime {
            os.write_int64(2, v)?;
        }
        for v in &self.changes {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for GetReconfigurationStatusResponseProto {
    fn new() -> GetReconfigurationStatusResponseProto {
        GetReconfigurationStatusResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetReconfigurationStatusResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "startTime",
                    GetReconfigurationStatusResponseProto::get_startTime_for_reflect,
                    GetReconfigurationStatusResponseProto::mut_startTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "endTime",
                    GetReconfigurationStatusResponseProto::get_endTime_for_reflect,
                    GetReconfigurationStatusResponseProto::mut_endTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GetReconfigurationStatusConfigChangeProto>>(
                    "changes",
                    GetReconfigurationStatusResponseProto::get_changes_for_reflect,
                    GetReconfigurationStatusResponseProto::mut_changes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetReconfigurationStatusResponseProto>(
                    "GetReconfigurationStatusResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetReconfigurationStatusResponseProto {
    fn clear(&mut self) {
        self.clear_startTime();
        self.clear_endTime();
        self.clear_changes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetReconfigurationStatusResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetReconfigurationStatusResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cClientDatanodeProtocol.proto\x12\x0bhadoop.hdfs\x1a\x0eSecurity.pr\
    oto\x1a\nhdfs.proto\"U\n#GetReplicaVisibleLengthRequestProto\x12.\n\x05b\
    lock\x18\x01\x20\x02(\x0b2\x1f.hadoop.hdfs.ExtendedBlockProto\"6\n$GetRe\
    plicaVisibleLengthResponseProto\x12\x0e\n\x06length\x18\x01\x20\x02(\x04\
    \"\x1e\n\x1cRefreshNamenodesRequestProto\"\x1f\n\x1dRefreshNamenodesResp\
    onseProto\"?\n\x1bDeleteBlockPoolRequestProto\x12\x11\n\tblockPool\x18\
    \x01\x20\x02(\t\x12\r\n\x05force\x18\x02\x20\x02(\x08\"\x1e\n\x1cDeleteB\
    lockPoolResponseProto\"}\n!GetBlockLocalPathInfoRequestProto\x12.\n\x05b\
    lock\x18\x01\x20\x02(\x0b2\x1f.hadoop.hdfs.ExtendedBlockProto\x12(\n\x05\
    token\x18\x02\x20\x02(\x0b2\x19.hadoop.common.TokenProto\"~\n\"GetBlockL\
    ocalPathInfoResponseProto\x12.\n\x05block\x18\x01\x20\x02(\x0b2\x1f.hado\
    op.hdfs.ExtendedBlockProto\x12\x11\n\tlocalPath\x18\x02\x20\x02(\t\x12\
    \x15\n\rlocalMetaPath\x18\x03\x20\x02(\t\"y\n!GetHdfsBlockLocationsReque\
    stProto\x12)\n\x06tokens\x18\x02\x20\x03(\x0b2\x19.hadoop.common.TokenPr\
    oto\x12\x13\n\x0bblockPoolId\x18\x03\x20\x02(\t\x12\x14\n\x08blockIds\
    \x18\x04\x20\x03(\x10B\x02\x10\x01\"R\n\"GetHdfsBlockLocationsResponsePr\
    oto\x12\x11\n\tvolumeIds\x18\x01\x20\x03(\x0c\x12\x19\n\rvolumeIndexes\
    \x18\x02\x20\x03(\rB\x02\x10\x01\"2\n\x1cShutdownDatanodeRequestProto\
    \x12\x12\n\nforUpgrade\x18\x01\x20\x02(\x08\"\x1f\n\x1dShutdownDatanodeR\
    esponseProto\"\x1d\n\x1bGetDatanodeInfoRequestProto\"V\n\x1cGetDatanodeI\
    nfoResponseProto\x126\n\tlocalInfo\x18\x01\x20\x02(\x0b2#.hadoop.hdfs.Da\
    tanodeLocalInfoProto\"\"\n\x20StartReconfigurationRequestProto\"#\n!Star\
    tReconfigurationResponseProto\"&\n$GetReconfigurationStatusRequestProto\
    \"s\n)GetReconfigurationStatusConfigChangeProto\x12\x0c\n\x04name\x18\
    \x01\x20\x02(\t\x12\x10\n\x08oldValue\x18\x02\x20\x02(\t\x12\x10\n\x08ne\
    wValue\x18\x03\x20\x01(\t\x12\x14\n\x0cerrorMessage\x18\x04\x20\x01(\t\"\
    \x94\x01\n%GetReconfigurationStatusResponseProto\x12\x11\n\tstartTime\
    \x18\x01\x20\x02(\x03\x12\x0f\n\x07endTime\x18\x02\x20\x01(\x03\x12G\n\
    \x07changes\x18\x03\x20\x03(\x0b26.hadoop.hdfs.GetReconfigurationStatusC\
    onfigChangeProto2\xb4\x08\n\x1dClientDatanodeProtocolService\x12~\n\x17g\
    etReplicaVisibleLength\x120.hadoop.hdfs.GetReplicaVisibleLengthRequestPr\
    oto\x1a1.hadoop.hdfs.GetReplicaVisibleLengthResponseProto\x12i\n\x10refr\
    eshNamenodes\x12).hadoop.hdfs.RefreshNamenodesRequestProto\x1a*.hadoop.h\
    dfs.RefreshNamenodesResponseProto\x12f\n\x0fdeleteBlockPool\x12(.hadoop.\
    hdfs.DeleteBlockPoolRequestProto\x1a).hadoop.hdfs.DeleteBlockPoolRespons\
    eProto\x12x\n\x15getBlockLocalPathInfo\x12..hadoop.hdfs.GetBlockLocalPat\
    hInfoRequestProto\x1a/.hadoop.hdfs.GetBlockLocalPathInfoResponseProto\
    \x12x\n\x15getHdfsBlockLocations\x12..hadoop.hdfs.GetHdfsBlockLocationsR\
    equestProto\x1a/.hadoop.hdfs.GetHdfsBlockLocationsResponseProto\x12i\n\
    \x10shutdownDatanode\x12).hadoop.hdfs.ShutdownDatanodeRequestProto\x1a*.\
    hadoop.hdfs.ShutdownDatanodeResponseProto\x12f\n\x0fgetDatanodeInfo\x12(\
    .hadoop.hdfs.GetDatanodeInfoRequestProto\x1a).hadoop.hdfs.GetDatanodeInf\
    oResponseProto\x12\x81\x01\n\x18getReconfigurationStatus\x121.hadoop.hdf\
    s.GetReconfigurationStatusRequestProto\x1a2.hadoop.hdfs.GetReconfigurati\
    onStatusResponseProto\x12u\n\x14startReconfiguration\x12-.hadoop.hdfs.St\
    artReconfigurationRequestProto\x1a..hadoop.hdfs.StartReconfigurationResp\
    onseProtoBK\n%org.apache.hadoop.hdfs.protocol.protoB\x1cClientDatanodePr\
    otocolProtos\xa0\x01\x01\x88\x01\x01\
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
