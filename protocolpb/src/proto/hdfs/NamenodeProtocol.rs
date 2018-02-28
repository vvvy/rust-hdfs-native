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
pub struct GetBlocksRequestProto {
    // message fields
    datanode: ::protobuf::SingularPtrField<super::hdfs::DatanodeIDProto>,
    size: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetBlocksRequestProto {}

impl GetBlocksRequestProto {
    pub fn new() -> GetBlocksRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBlocksRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetBlocksRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBlocksRequestProto,
        };
        unsafe {
            instance.get(GetBlocksRequestProto::new)
        }
    }

    // required .hadoop.hdfs.DatanodeIDProto datanode = 1;

    pub fn clear_datanode(&mut self) {
        self.datanode.clear();
    }

    pub fn has_datanode(&self) -> bool {
        self.datanode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_datanode(&mut self, v: super::hdfs::DatanodeIDProto) {
        self.datanode = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_datanode(&mut self) -> &mut super::hdfs::DatanodeIDProto {
        if self.datanode.is_none() {
            self.datanode.set_default();
        }
        self.datanode.as_mut().unwrap()
    }

    // Take field
    pub fn take_datanode(&mut self) -> super::hdfs::DatanodeIDProto {
        self.datanode.take().unwrap_or_else(|| super::hdfs::DatanodeIDProto::new())
    }

    pub fn get_datanode(&self) -> &super::hdfs::DatanodeIDProto {
        self.datanode.as_ref().unwrap_or_else(|| super::hdfs::DatanodeIDProto::default_instance())
    }

    fn get_datanode_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::DatanodeIDProto> {
        &self.datanode
    }

    fn mut_datanode_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::DatanodeIDProto> {
        &mut self.datanode
    }

    // required uint64 size = 2;

    pub fn clear_size(&mut self) {
        self.size = ::std::option::Option::None;
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: u64) {
        self.size = ::std::option::Option::Some(v);
    }

    pub fn get_size(&self) -> u64 {
        self.size.unwrap_or(0)
    }

    fn get_size_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.size
    }
}

impl ::protobuf::Message for GetBlocksRequestProto {
    fn is_initialized(&self) -> bool {
        if self.datanode.is_none() {
            return false;
        }
        if self.size.is_none() {
            return false;
        }
        for v in &self.datanode {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.datanode)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.size = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.datanode.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.size {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.datanode.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.size {
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

impl ::protobuf::MessageStatic for GetBlocksRequestProto {
    fn new() -> GetBlocksRequestProto {
        GetBlocksRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBlocksRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeIDProto>>(
                    "datanode",
                    GetBlocksRequestProto::get_datanode_for_reflect,
                    GetBlocksRequestProto::mut_datanode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "size",
                    GetBlocksRequestProto::get_size_for_reflect,
                    GetBlocksRequestProto::mut_size_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetBlocksRequestProto>(
                    "GetBlocksRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBlocksRequestProto {
    fn clear(&mut self) {
        self.clear_datanode();
        self.clear_size();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetBlocksRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBlocksRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetBlocksResponseProto {
    // message fields
    blocks: ::protobuf::SingularPtrField<super::hdfs::BlocksWithLocationsProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetBlocksResponseProto {}

impl GetBlocksResponseProto {
    pub fn new() -> GetBlocksResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBlocksResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetBlocksResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBlocksResponseProto,
        };
        unsafe {
            instance.get(GetBlocksResponseProto::new)
        }
    }

    // required .hadoop.hdfs.BlocksWithLocationsProto blocks = 1;

    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    pub fn has_blocks(&self) -> bool {
        self.blocks.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blocks(&mut self, v: super::hdfs::BlocksWithLocationsProto) {
        self.blocks = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blocks(&mut self) -> &mut super::hdfs::BlocksWithLocationsProto {
        if self.blocks.is_none() {
            self.blocks.set_default();
        }
        self.blocks.as_mut().unwrap()
    }

    // Take field
    pub fn take_blocks(&mut self) -> super::hdfs::BlocksWithLocationsProto {
        self.blocks.take().unwrap_or_else(|| super::hdfs::BlocksWithLocationsProto::new())
    }

    pub fn get_blocks(&self) -> &super::hdfs::BlocksWithLocationsProto {
        self.blocks.as_ref().unwrap_or_else(|| super::hdfs::BlocksWithLocationsProto::default_instance())
    }

    fn get_blocks_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::BlocksWithLocationsProto> {
        &self.blocks
    }

    fn mut_blocks_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::BlocksWithLocationsProto> {
        &mut self.blocks
    }
}

impl ::protobuf::Message for GetBlocksResponseProto {
    fn is_initialized(&self) -> bool {
        if self.blocks.is_none() {
            return false;
        }
        for v in &self.blocks {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.blocks)?;
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
        if let Some(ref v) = self.blocks.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.blocks.as_ref() {
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

impl ::protobuf::MessageStatic for GetBlocksResponseProto {
    fn new() -> GetBlocksResponseProto {
        GetBlocksResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBlocksResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::BlocksWithLocationsProto>>(
                    "blocks",
                    GetBlocksResponseProto::get_blocks_for_reflect,
                    GetBlocksResponseProto::mut_blocks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetBlocksResponseProto>(
                    "GetBlocksResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBlocksResponseProto {
    fn clear(&mut self) {
        self.clear_blocks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetBlocksResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBlocksResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetBlockKeysRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetBlockKeysRequestProto {}

impl GetBlockKeysRequestProto {
    pub fn new() -> GetBlockKeysRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBlockKeysRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetBlockKeysRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBlockKeysRequestProto,
        };
        unsafe {
            instance.get(GetBlockKeysRequestProto::new)
        }
    }
}

impl ::protobuf::Message for GetBlockKeysRequestProto {
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

impl ::protobuf::MessageStatic for GetBlockKeysRequestProto {
    fn new() -> GetBlockKeysRequestProto {
        GetBlockKeysRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBlockKeysRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetBlockKeysRequestProto>(
                    "GetBlockKeysRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBlockKeysRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetBlockKeysRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBlockKeysRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetBlockKeysResponseProto {
    // message fields
    keys: ::protobuf::SingularPtrField<super::hdfs::ExportedBlockKeysProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetBlockKeysResponseProto {}

impl GetBlockKeysResponseProto {
    pub fn new() -> GetBlockKeysResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBlockKeysResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetBlockKeysResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBlockKeysResponseProto,
        };
        unsafe {
            instance.get(GetBlockKeysResponseProto::new)
        }
    }

    // optional .hadoop.hdfs.ExportedBlockKeysProto keys = 1;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    pub fn has_keys(&self) -> bool {
        self.keys.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: super::hdfs::ExportedBlockKeysProto) {
        self.keys = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_keys(&mut self) -> &mut super::hdfs::ExportedBlockKeysProto {
        if self.keys.is_none() {
            self.keys.set_default();
        }
        self.keys.as_mut().unwrap()
    }

    // Take field
    pub fn take_keys(&mut self) -> super::hdfs::ExportedBlockKeysProto {
        self.keys.take().unwrap_or_else(|| super::hdfs::ExportedBlockKeysProto::new())
    }

    pub fn get_keys(&self) -> &super::hdfs::ExportedBlockKeysProto {
        self.keys.as_ref().unwrap_or_else(|| super::hdfs::ExportedBlockKeysProto::default_instance())
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::ExportedBlockKeysProto> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::ExportedBlockKeysProto> {
        &mut self.keys
    }
}

impl ::protobuf::Message for GetBlockKeysResponseProto {
    fn is_initialized(&self) -> bool {
        for v in &self.keys {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.keys)?;
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
        if let Some(ref v) = self.keys.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.keys.as_ref() {
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

impl ::protobuf::MessageStatic for GetBlockKeysResponseProto {
    fn new() -> GetBlockKeysResponseProto {
        GetBlockKeysResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBlockKeysResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::ExportedBlockKeysProto>>(
                    "keys",
                    GetBlockKeysResponseProto::get_keys_for_reflect,
                    GetBlockKeysResponseProto::mut_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetBlockKeysResponseProto>(
                    "GetBlockKeysResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBlockKeysResponseProto {
    fn clear(&mut self) {
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetBlockKeysResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBlockKeysResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetTransactionIdRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetTransactionIdRequestProto {}

impl GetTransactionIdRequestProto {
    pub fn new() -> GetTransactionIdRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetTransactionIdRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetTransactionIdRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetTransactionIdRequestProto,
        };
        unsafe {
            instance.get(GetTransactionIdRequestProto::new)
        }
    }
}

impl ::protobuf::Message for GetTransactionIdRequestProto {
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

impl ::protobuf::MessageStatic for GetTransactionIdRequestProto {
    fn new() -> GetTransactionIdRequestProto {
        GetTransactionIdRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetTransactionIdRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetTransactionIdRequestProto>(
                    "GetTransactionIdRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetTransactionIdRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetTransactionIdRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetTransactionIdRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetTransactionIdResponseProto {
    // message fields
    txId: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetTransactionIdResponseProto {}

impl GetTransactionIdResponseProto {
    pub fn new() -> GetTransactionIdResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetTransactionIdResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetTransactionIdResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetTransactionIdResponseProto,
        };
        unsafe {
            instance.get(GetTransactionIdResponseProto::new)
        }
    }

    // required uint64 txId = 1;

    pub fn clear_txId(&mut self) {
        self.txId = ::std::option::Option::None;
    }

    pub fn has_txId(&self) -> bool {
        self.txId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txId(&mut self, v: u64) {
        self.txId = ::std::option::Option::Some(v);
    }

    pub fn get_txId(&self) -> u64 {
        self.txId.unwrap_or(0)
    }

    fn get_txId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.txId
    }

    fn mut_txId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.txId
    }
}

impl ::protobuf::Message for GetTransactionIdResponseProto {
    fn is_initialized(&self) -> bool {
        if self.txId.is_none() {
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
                    self.txId = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.txId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.txId {
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

impl ::protobuf::MessageStatic for GetTransactionIdResponseProto {
    fn new() -> GetTransactionIdResponseProto {
        GetTransactionIdResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetTransactionIdResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "txId",
                    GetTransactionIdResponseProto::get_txId_for_reflect,
                    GetTransactionIdResponseProto::mut_txId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetTransactionIdResponseProto>(
                    "GetTransactionIdResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetTransactionIdResponseProto {
    fn clear(&mut self) {
        self.clear_txId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetTransactionIdResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetTransactionIdResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RollEditLogRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RollEditLogRequestProto {}

impl RollEditLogRequestProto {
    pub fn new() -> RollEditLogRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RollEditLogRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RollEditLogRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RollEditLogRequestProto,
        };
        unsafe {
            instance.get(RollEditLogRequestProto::new)
        }
    }
}

impl ::protobuf::Message for RollEditLogRequestProto {
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

impl ::protobuf::MessageStatic for RollEditLogRequestProto {
    fn new() -> RollEditLogRequestProto {
        RollEditLogRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RollEditLogRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RollEditLogRequestProto>(
                    "RollEditLogRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RollEditLogRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RollEditLogRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RollEditLogRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RollEditLogResponseProto {
    // message fields
    signature: ::protobuf::SingularPtrField<super::hdfs::CheckpointSignatureProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RollEditLogResponseProto {}

impl RollEditLogResponseProto {
    pub fn new() -> RollEditLogResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RollEditLogResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RollEditLogResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RollEditLogResponseProto,
        };
        unsafe {
            instance.get(RollEditLogResponseProto::new)
        }
    }

    // required .hadoop.hdfs.CheckpointSignatureProto signature = 1;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: super::hdfs::CheckpointSignatureProto) {
        self.signature = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut super::hdfs::CheckpointSignatureProto {
        if self.signature.is_none() {
            self.signature.set_default();
        }
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> super::hdfs::CheckpointSignatureProto {
        self.signature.take().unwrap_or_else(|| super::hdfs::CheckpointSignatureProto::new())
    }

    pub fn get_signature(&self) -> &super::hdfs::CheckpointSignatureProto {
        self.signature.as_ref().unwrap_or_else(|| super::hdfs::CheckpointSignatureProto::default_instance())
    }

    fn get_signature_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::CheckpointSignatureProto> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::CheckpointSignatureProto> {
        &mut self.signature
    }
}

impl ::protobuf::Message for RollEditLogResponseProto {
    fn is_initialized(&self) -> bool {
        if self.signature.is_none() {
            return false;
        }
        for v in &self.signature {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.signature)?;
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
        if let Some(ref v) = self.signature.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.signature.as_ref() {
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

impl ::protobuf::MessageStatic for RollEditLogResponseProto {
    fn new() -> RollEditLogResponseProto {
        RollEditLogResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RollEditLogResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::CheckpointSignatureProto>>(
                    "signature",
                    RollEditLogResponseProto::get_signature_for_reflect,
                    RollEditLogResponseProto::mut_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RollEditLogResponseProto>(
                    "RollEditLogResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RollEditLogResponseProto {
    fn clear(&mut self) {
        self.clear_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RollEditLogResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RollEditLogResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetMostRecentCheckpointTxIdRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetMostRecentCheckpointTxIdRequestProto {}

impl GetMostRecentCheckpointTxIdRequestProto {
    pub fn new() -> GetMostRecentCheckpointTxIdRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetMostRecentCheckpointTxIdRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetMostRecentCheckpointTxIdRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetMostRecentCheckpointTxIdRequestProto,
        };
        unsafe {
            instance.get(GetMostRecentCheckpointTxIdRequestProto::new)
        }
    }
}

impl ::protobuf::Message for GetMostRecentCheckpointTxIdRequestProto {
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

impl ::protobuf::MessageStatic for GetMostRecentCheckpointTxIdRequestProto {
    fn new() -> GetMostRecentCheckpointTxIdRequestProto {
        GetMostRecentCheckpointTxIdRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetMostRecentCheckpointTxIdRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetMostRecentCheckpointTxIdRequestProto>(
                    "GetMostRecentCheckpointTxIdRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetMostRecentCheckpointTxIdRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetMostRecentCheckpointTxIdRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetMostRecentCheckpointTxIdRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetMostRecentCheckpointTxIdResponseProto {
    // message fields
    txId: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetMostRecentCheckpointTxIdResponseProto {}

impl GetMostRecentCheckpointTxIdResponseProto {
    pub fn new() -> GetMostRecentCheckpointTxIdResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetMostRecentCheckpointTxIdResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetMostRecentCheckpointTxIdResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetMostRecentCheckpointTxIdResponseProto,
        };
        unsafe {
            instance.get(GetMostRecentCheckpointTxIdResponseProto::new)
        }
    }

    // required uint64 txId = 1;

    pub fn clear_txId(&mut self) {
        self.txId = ::std::option::Option::None;
    }

    pub fn has_txId(&self) -> bool {
        self.txId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txId(&mut self, v: u64) {
        self.txId = ::std::option::Option::Some(v);
    }

    pub fn get_txId(&self) -> u64 {
        self.txId.unwrap_or(0)
    }

    fn get_txId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.txId
    }

    fn mut_txId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.txId
    }
}

impl ::protobuf::Message for GetMostRecentCheckpointTxIdResponseProto {
    fn is_initialized(&self) -> bool {
        if self.txId.is_none() {
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
                    self.txId = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.txId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.txId {
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

impl ::protobuf::MessageStatic for GetMostRecentCheckpointTxIdResponseProto {
    fn new() -> GetMostRecentCheckpointTxIdResponseProto {
        GetMostRecentCheckpointTxIdResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetMostRecentCheckpointTxIdResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "txId",
                    GetMostRecentCheckpointTxIdResponseProto::get_txId_for_reflect,
                    GetMostRecentCheckpointTxIdResponseProto::mut_txId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetMostRecentCheckpointTxIdResponseProto>(
                    "GetMostRecentCheckpointTxIdResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetMostRecentCheckpointTxIdResponseProto {
    fn clear(&mut self) {
        self.clear_txId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetMostRecentCheckpointTxIdResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetMostRecentCheckpointTxIdResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ErrorReportRequestProto {
    // message fields
    registration: ::protobuf::SingularPtrField<super::hdfs::NamenodeRegistrationProto>,
    errorCode: ::std::option::Option<u32>,
    msg: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ErrorReportRequestProto {}

impl ErrorReportRequestProto {
    pub fn new() -> ErrorReportRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ErrorReportRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ErrorReportRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ErrorReportRequestProto,
        };
        unsafe {
            instance.get(ErrorReportRequestProto::new)
        }
    }

    // required .hadoop.hdfs.NamenodeRegistrationProto registration = 1;

    pub fn clear_registration(&mut self) {
        self.registration.clear();
    }

    pub fn has_registration(&self) -> bool {
        self.registration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_registration(&mut self, v: super::hdfs::NamenodeRegistrationProto) {
        self.registration = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_registration(&mut self) -> &mut super::hdfs::NamenodeRegistrationProto {
        if self.registration.is_none() {
            self.registration.set_default();
        }
        self.registration.as_mut().unwrap()
    }

    // Take field
    pub fn take_registration(&mut self) -> super::hdfs::NamenodeRegistrationProto {
        self.registration.take().unwrap_or_else(|| super::hdfs::NamenodeRegistrationProto::new())
    }

    pub fn get_registration(&self) -> &super::hdfs::NamenodeRegistrationProto {
        self.registration.as_ref().unwrap_or_else(|| super::hdfs::NamenodeRegistrationProto::default_instance())
    }

    fn get_registration_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::NamenodeRegistrationProto> {
        &self.registration
    }

    fn mut_registration_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::NamenodeRegistrationProto> {
        &mut self.registration
    }

    // required uint32 errorCode = 2;

    pub fn clear_errorCode(&mut self) {
        self.errorCode = ::std::option::Option::None;
    }

    pub fn has_errorCode(&self) -> bool {
        self.errorCode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_errorCode(&mut self, v: u32) {
        self.errorCode = ::std::option::Option::Some(v);
    }

    pub fn get_errorCode(&self) -> u32 {
        self.errorCode.unwrap_or(0)
    }

    fn get_errorCode_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.errorCode
    }

    fn mut_errorCode_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.errorCode
    }

    // required string msg = 3;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        if self.msg.is_none() {
            self.msg.set_default();
        }
        self.msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        self.msg.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_msg(&self) -> &str {
        match self.msg.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_msg_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.msg
    }

    fn mut_msg_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.msg
    }
}

impl ::protobuf::Message for ErrorReportRequestProto {
    fn is_initialized(&self) -> bool {
        if self.registration.is_none() {
            return false;
        }
        if self.errorCode.is_none() {
            return false;
        }
        if self.msg.is_none() {
            return false;
        }
        for v in &self.registration {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.registration)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.errorCode = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.msg)?;
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
        if let Some(ref v) = self.registration.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.errorCode {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.msg.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.registration.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.errorCode {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.msg.as_ref() {
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

impl ::protobuf::MessageStatic for ErrorReportRequestProto {
    fn new() -> ErrorReportRequestProto {
        ErrorReportRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ErrorReportRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::NamenodeRegistrationProto>>(
                    "registration",
                    ErrorReportRequestProto::get_registration_for_reflect,
                    ErrorReportRequestProto::mut_registration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "errorCode",
                    ErrorReportRequestProto::get_errorCode_for_reflect,
                    ErrorReportRequestProto::mut_errorCode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg",
                    ErrorReportRequestProto::get_msg_for_reflect,
                    ErrorReportRequestProto::mut_msg_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ErrorReportRequestProto>(
                    "ErrorReportRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ErrorReportRequestProto {
    fn clear(&mut self) {
        self.clear_registration();
        self.clear_errorCode();
        self.clear_msg();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ErrorReportRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ErrorReportRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ErrorReportResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ErrorReportResponseProto {}

impl ErrorReportResponseProto {
    pub fn new() -> ErrorReportResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ErrorReportResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ErrorReportResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ErrorReportResponseProto,
        };
        unsafe {
            instance.get(ErrorReportResponseProto::new)
        }
    }
}

impl ::protobuf::Message for ErrorReportResponseProto {
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

impl ::protobuf::MessageStatic for ErrorReportResponseProto {
    fn new() -> ErrorReportResponseProto {
        ErrorReportResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ErrorReportResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ErrorReportResponseProto>(
                    "ErrorReportResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ErrorReportResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ErrorReportResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ErrorReportResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegisterRequestProto {
    // message fields
    registration: ::protobuf::SingularPtrField<super::hdfs::NamenodeRegistrationProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegisterRequestProto {}

impl RegisterRequestProto {
    pub fn new() -> RegisterRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegisterRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RegisterRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegisterRequestProto,
        };
        unsafe {
            instance.get(RegisterRequestProto::new)
        }
    }

    // required .hadoop.hdfs.NamenodeRegistrationProto registration = 1;

    pub fn clear_registration(&mut self) {
        self.registration.clear();
    }

    pub fn has_registration(&self) -> bool {
        self.registration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_registration(&mut self, v: super::hdfs::NamenodeRegistrationProto) {
        self.registration = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_registration(&mut self) -> &mut super::hdfs::NamenodeRegistrationProto {
        if self.registration.is_none() {
            self.registration.set_default();
        }
        self.registration.as_mut().unwrap()
    }

    // Take field
    pub fn take_registration(&mut self) -> super::hdfs::NamenodeRegistrationProto {
        self.registration.take().unwrap_or_else(|| super::hdfs::NamenodeRegistrationProto::new())
    }

    pub fn get_registration(&self) -> &super::hdfs::NamenodeRegistrationProto {
        self.registration.as_ref().unwrap_or_else(|| super::hdfs::NamenodeRegistrationProto::default_instance())
    }

    fn get_registration_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::NamenodeRegistrationProto> {
        &self.registration
    }

    fn mut_registration_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::NamenodeRegistrationProto> {
        &mut self.registration
    }
}

impl ::protobuf::Message for RegisterRequestProto {
    fn is_initialized(&self) -> bool {
        if self.registration.is_none() {
            return false;
        }
        for v in &self.registration {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.registration)?;
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
        if let Some(ref v) = self.registration.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.registration.as_ref() {
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

impl ::protobuf::MessageStatic for RegisterRequestProto {
    fn new() -> RegisterRequestProto {
        RegisterRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegisterRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::NamenodeRegistrationProto>>(
                    "registration",
                    RegisterRequestProto::get_registration_for_reflect,
                    RegisterRequestProto::mut_registration_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegisterRequestProto>(
                    "RegisterRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegisterRequestProto {
    fn clear(&mut self) {
        self.clear_registration();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegisterRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegisterRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegisterResponseProto {
    // message fields
    registration: ::protobuf::SingularPtrField<super::hdfs::NamenodeRegistrationProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegisterResponseProto {}

impl RegisterResponseProto {
    pub fn new() -> RegisterResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegisterResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RegisterResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegisterResponseProto,
        };
        unsafe {
            instance.get(RegisterResponseProto::new)
        }
    }

    // required .hadoop.hdfs.NamenodeRegistrationProto registration = 1;

    pub fn clear_registration(&mut self) {
        self.registration.clear();
    }

    pub fn has_registration(&self) -> bool {
        self.registration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_registration(&mut self, v: super::hdfs::NamenodeRegistrationProto) {
        self.registration = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_registration(&mut self) -> &mut super::hdfs::NamenodeRegistrationProto {
        if self.registration.is_none() {
            self.registration.set_default();
        }
        self.registration.as_mut().unwrap()
    }

    // Take field
    pub fn take_registration(&mut self) -> super::hdfs::NamenodeRegistrationProto {
        self.registration.take().unwrap_or_else(|| super::hdfs::NamenodeRegistrationProto::new())
    }

    pub fn get_registration(&self) -> &super::hdfs::NamenodeRegistrationProto {
        self.registration.as_ref().unwrap_or_else(|| super::hdfs::NamenodeRegistrationProto::default_instance())
    }

    fn get_registration_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::NamenodeRegistrationProto> {
        &self.registration
    }

    fn mut_registration_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::NamenodeRegistrationProto> {
        &mut self.registration
    }
}

impl ::protobuf::Message for RegisterResponseProto {
    fn is_initialized(&self) -> bool {
        if self.registration.is_none() {
            return false;
        }
        for v in &self.registration {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.registration)?;
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
        if let Some(ref v) = self.registration.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.registration.as_ref() {
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

impl ::protobuf::MessageStatic for RegisterResponseProto {
    fn new() -> RegisterResponseProto {
        RegisterResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegisterResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::NamenodeRegistrationProto>>(
                    "registration",
                    RegisterResponseProto::get_registration_for_reflect,
                    RegisterResponseProto::mut_registration_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegisterResponseProto>(
                    "RegisterResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegisterResponseProto {
    fn clear(&mut self) {
        self.clear_registration();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegisterResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegisterResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StartCheckpointRequestProto {
    // message fields
    registration: ::protobuf::SingularPtrField<super::hdfs::NamenodeRegistrationProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StartCheckpointRequestProto {}

impl StartCheckpointRequestProto {
    pub fn new() -> StartCheckpointRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StartCheckpointRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<StartCheckpointRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StartCheckpointRequestProto,
        };
        unsafe {
            instance.get(StartCheckpointRequestProto::new)
        }
    }

    // required .hadoop.hdfs.NamenodeRegistrationProto registration = 1;

    pub fn clear_registration(&mut self) {
        self.registration.clear();
    }

    pub fn has_registration(&self) -> bool {
        self.registration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_registration(&mut self, v: super::hdfs::NamenodeRegistrationProto) {
        self.registration = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_registration(&mut self) -> &mut super::hdfs::NamenodeRegistrationProto {
        if self.registration.is_none() {
            self.registration.set_default();
        }
        self.registration.as_mut().unwrap()
    }

    // Take field
    pub fn take_registration(&mut self) -> super::hdfs::NamenodeRegistrationProto {
        self.registration.take().unwrap_or_else(|| super::hdfs::NamenodeRegistrationProto::new())
    }

    pub fn get_registration(&self) -> &super::hdfs::NamenodeRegistrationProto {
        self.registration.as_ref().unwrap_or_else(|| super::hdfs::NamenodeRegistrationProto::default_instance())
    }

    fn get_registration_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::NamenodeRegistrationProto> {
        &self.registration
    }

    fn mut_registration_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::NamenodeRegistrationProto> {
        &mut self.registration
    }
}

impl ::protobuf::Message for StartCheckpointRequestProto {
    fn is_initialized(&self) -> bool {
        if self.registration.is_none() {
            return false;
        }
        for v in &self.registration {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.registration)?;
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
        if let Some(ref v) = self.registration.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.registration.as_ref() {
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

impl ::protobuf::MessageStatic for StartCheckpointRequestProto {
    fn new() -> StartCheckpointRequestProto {
        StartCheckpointRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StartCheckpointRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::NamenodeRegistrationProto>>(
                    "registration",
                    StartCheckpointRequestProto::get_registration_for_reflect,
                    StartCheckpointRequestProto::mut_registration_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StartCheckpointRequestProto>(
                    "StartCheckpointRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StartCheckpointRequestProto {
    fn clear(&mut self) {
        self.clear_registration();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StartCheckpointRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartCheckpointRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StartCheckpointResponseProto {
    // message fields
    command: ::protobuf::SingularPtrField<super::hdfs::NamenodeCommandProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StartCheckpointResponseProto {}

impl StartCheckpointResponseProto {
    pub fn new() -> StartCheckpointResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StartCheckpointResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<StartCheckpointResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StartCheckpointResponseProto,
        };
        unsafe {
            instance.get(StartCheckpointResponseProto::new)
        }
    }

    // required .hadoop.hdfs.NamenodeCommandProto command = 1;

    pub fn clear_command(&mut self) {
        self.command.clear();
    }

    pub fn has_command(&self) -> bool {
        self.command.is_some()
    }

    // Param is passed by value, moved
    pub fn set_command(&mut self, v: super::hdfs::NamenodeCommandProto) {
        self.command = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_command(&mut self) -> &mut super::hdfs::NamenodeCommandProto {
        if self.command.is_none() {
            self.command.set_default();
        }
        self.command.as_mut().unwrap()
    }

    // Take field
    pub fn take_command(&mut self) -> super::hdfs::NamenodeCommandProto {
        self.command.take().unwrap_or_else(|| super::hdfs::NamenodeCommandProto::new())
    }

    pub fn get_command(&self) -> &super::hdfs::NamenodeCommandProto {
        self.command.as_ref().unwrap_or_else(|| super::hdfs::NamenodeCommandProto::default_instance())
    }

    fn get_command_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::NamenodeCommandProto> {
        &self.command
    }

    fn mut_command_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::NamenodeCommandProto> {
        &mut self.command
    }
}

impl ::protobuf::Message for StartCheckpointResponseProto {
    fn is_initialized(&self) -> bool {
        if self.command.is_none() {
            return false;
        }
        for v in &self.command {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.command)?;
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
        if let Some(ref v) = self.command.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.command.as_ref() {
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

impl ::protobuf::MessageStatic for StartCheckpointResponseProto {
    fn new() -> StartCheckpointResponseProto {
        StartCheckpointResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StartCheckpointResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::NamenodeCommandProto>>(
                    "command",
                    StartCheckpointResponseProto::get_command_for_reflect,
                    StartCheckpointResponseProto::mut_command_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StartCheckpointResponseProto>(
                    "StartCheckpointResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StartCheckpointResponseProto {
    fn clear(&mut self) {
        self.clear_command();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StartCheckpointResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartCheckpointResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EndCheckpointRequestProto {
    // message fields
    registration: ::protobuf::SingularPtrField<super::hdfs::NamenodeRegistrationProto>,
    signature: ::protobuf::SingularPtrField<super::hdfs::CheckpointSignatureProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EndCheckpointRequestProto {}

impl EndCheckpointRequestProto {
    pub fn new() -> EndCheckpointRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EndCheckpointRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<EndCheckpointRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EndCheckpointRequestProto,
        };
        unsafe {
            instance.get(EndCheckpointRequestProto::new)
        }
    }

    // required .hadoop.hdfs.NamenodeRegistrationProto registration = 1;

    pub fn clear_registration(&mut self) {
        self.registration.clear();
    }

    pub fn has_registration(&self) -> bool {
        self.registration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_registration(&mut self, v: super::hdfs::NamenodeRegistrationProto) {
        self.registration = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_registration(&mut self) -> &mut super::hdfs::NamenodeRegistrationProto {
        if self.registration.is_none() {
            self.registration.set_default();
        }
        self.registration.as_mut().unwrap()
    }

    // Take field
    pub fn take_registration(&mut self) -> super::hdfs::NamenodeRegistrationProto {
        self.registration.take().unwrap_or_else(|| super::hdfs::NamenodeRegistrationProto::new())
    }

    pub fn get_registration(&self) -> &super::hdfs::NamenodeRegistrationProto {
        self.registration.as_ref().unwrap_or_else(|| super::hdfs::NamenodeRegistrationProto::default_instance())
    }

    fn get_registration_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::NamenodeRegistrationProto> {
        &self.registration
    }

    fn mut_registration_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::NamenodeRegistrationProto> {
        &mut self.registration
    }

    // required .hadoop.hdfs.CheckpointSignatureProto signature = 2;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: super::hdfs::CheckpointSignatureProto) {
        self.signature = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut super::hdfs::CheckpointSignatureProto {
        if self.signature.is_none() {
            self.signature.set_default();
        }
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> super::hdfs::CheckpointSignatureProto {
        self.signature.take().unwrap_or_else(|| super::hdfs::CheckpointSignatureProto::new())
    }

    pub fn get_signature(&self) -> &super::hdfs::CheckpointSignatureProto {
        self.signature.as_ref().unwrap_or_else(|| super::hdfs::CheckpointSignatureProto::default_instance())
    }

    fn get_signature_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::CheckpointSignatureProto> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::CheckpointSignatureProto> {
        &mut self.signature
    }
}

impl ::protobuf::Message for EndCheckpointRequestProto {
    fn is_initialized(&self) -> bool {
        if self.registration.is_none() {
            return false;
        }
        if self.signature.is_none() {
            return false;
        }
        for v in &self.registration {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.signature {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.registration)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.signature)?;
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
        if let Some(ref v) = self.registration.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.signature.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.registration.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.signature.as_ref() {
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

impl ::protobuf::MessageStatic for EndCheckpointRequestProto {
    fn new() -> EndCheckpointRequestProto {
        EndCheckpointRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<EndCheckpointRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::NamenodeRegistrationProto>>(
                    "registration",
                    EndCheckpointRequestProto::get_registration_for_reflect,
                    EndCheckpointRequestProto::mut_registration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::CheckpointSignatureProto>>(
                    "signature",
                    EndCheckpointRequestProto::get_signature_for_reflect,
                    EndCheckpointRequestProto::mut_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EndCheckpointRequestProto>(
                    "EndCheckpointRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EndCheckpointRequestProto {
    fn clear(&mut self) {
        self.clear_registration();
        self.clear_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EndCheckpointRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EndCheckpointRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EndCheckpointResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EndCheckpointResponseProto {}

impl EndCheckpointResponseProto {
    pub fn new() -> EndCheckpointResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EndCheckpointResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<EndCheckpointResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EndCheckpointResponseProto,
        };
        unsafe {
            instance.get(EndCheckpointResponseProto::new)
        }
    }
}

impl ::protobuf::Message for EndCheckpointResponseProto {
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

impl ::protobuf::MessageStatic for EndCheckpointResponseProto {
    fn new() -> EndCheckpointResponseProto {
        EndCheckpointResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<EndCheckpointResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<EndCheckpointResponseProto>(
                    "EndCheckpointResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EndCheckpointResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EndCheckpointResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EndCheckpointResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetEditLogManifestRequestProto {
    // message fields
    sinceTxId: ::std::option::Option<u64>,
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

    // required uint64 sinceTxId = 1;

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
}

impl ::protobuf::Message for GetEditLogManifestRequestProto {
    fn is_initialized(&self) -> bool {
        if self.sinceTxId.is_none() {
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
                    self.sinceTxId = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.sinceTxId {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sinceTxId {
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
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "sinceTxId",
                    GetEditLogManifestRequestProto::get_sinceTxId_for_reflect,
                    GetEditLogManifestRequestProto::mut_sinceTxId_for_reflect,
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
        self.clear_sinceTxId();
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
}

impl ::protobuf::Message for GetEditLogManifestResponseProto {
    fn is_initialized(&self) -> bool {
        if self.manifest.is_none() {
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

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16NamenodeProtocol.proto\x12\x14hadoop.hdfs.namenode\x1a\nhdfs.proto\
    \"U\n\x15GetBlocksRequestProto\x12.\n\x08datanode\x18\x01\x20\x02(\x0b2\
    \x1c.hadoop.hdfs.DatanodeIDProto\x12\x0c\n\x04size\x18\x02\x20\x02(\x04\
    \"O\n\x16GetBlocksResponseProto\x125\n\x06blocks\x18\x01\x20\x02(\x0b2%.\
    hadoop.hdfs.BlocksWithLocationsProto\"\x1a\n\x18GetBlockKeysRequestProto\
    \"N\n\x19GetBlockKeysResponseProto\x121\n\x04keys\x18\x01\x20\x01(\x0b2#\
    .hadoop.hdfs.ExportedBlockKeysProto\"\x1e\n\x1cGetTransactionIdRequestPr\
    oto\"-\n\x1dGetTransactionIdResponseProto\x12\x0c\n\x04txId\x18\x01\x20\
    \x02(\x04\"\x19\n\x17RollEditLogRequestProto\"T\n\x18RollEditLogResponse\
    Proto\x128\n\tsignature\x18\x01\x20\x02(\x0b2%.hadoop.hdfs.CheckpointSig\
    natureProto\")\n'GetMostRecentCheckpointTxIdRequestProto\"8\n(GetMostRec\
    entCheckpointTxIdResponseProto\x12\x0c\n\x04txId\x18\x01\x20\x02(\x04\"w\
    \n\x17ErrorReportRequestProto\x12<\n\x0cregistration\x18\x01\x20\x02(\
    \x0b2&.hadoop.hdfs.NamenodeRegistrationProto\x12\x11\n\terrorCode\x18\
    \x02\x20\x02(\r\x12\x0b\n\x03msg\x18\x03\x20\x02(\t\"\x1a\n\x18ErrorRepo\
    rtResponseProto\"T\n\x14RegisterRequestProto\x12<\n\x0cregistration\x18\
    \x01\x20\x02(\x0b2&.hadoop.hdfs.NamenodeRegistrationProto\"U\n\x15Regist\
    erResponseProto\x12<\n\x0cregistration\x18\x01\x20\x02(\x0b2&.hadoop.hdf\
    s.NamenodeRegistrationProto\"[\n\x1bStartCheckpointRequestProto\x12<\n\
    \x0cregistration\x18\x01\x20\x02(\x0b2&.hadoop.hdfs.NamenodeRegistration\
    Proto\"R\n\x1cStartCheckpointResponseProto\x122\n\x07command\x18\x01\x20\
    \x02(\x0b2!.hadoop.hdfs.NamenodeCommandProto\"\x93\x01\n\x19EndCheckpoin\
    tRequestProto\x12<\n\x0cregistration\x18\x01\x20\x02(\x0b2&.hadoop.hdfs.\
    NamenodeRegistrationProto\x128\n\tsignature\x18\x02\x20\x02(\x0b2%.hadoo\
    p.hdfs.CheckpointSignatureProto\"\x1c\n\x1aEndCheckpointResponseProto\"3\
    \n\x1eGetEditLogManifestRequestProto\x12\x11\n\tsinceTxId\x18\x01\x20\
    \x02(\x04\"\\\n\x1fGetEditLogManifestResponseProto\x129\n\x08manifest\
    \x18\x01\x20\x02(\x0b2'.hadoop.hdfs.RemoteEditLogManifestProto2\xab\n\n\
    \x17NamenodeProtocolService\x12f\n\tgetBlocks\x12+.hadoop.hdfs.namenode.\
    GetBlocksRequestProto\x1a,.hadoop.hdfs.namenode.GetBlocksResponseProto\
    \x12o\n\x0cgetBlockKeys\x12..hadoop.hdfs.namenode.GetBlockKeysRequestPro\
    to\x1a/.hadoop.hdfs.namenode.GetBlockKeysResponseProto\x12{\n\x10getTran\
    sactionId\x122.hadoop.hdfs.namenode.GetTransactionIdRequestProto\x1a3.ha\
    doop.hdfs.namenode.GetTransactionIdResponseProto\x12\x9c\x01\n\x1bgetMos\
    tRecentCheckpointTxId\x12=.hadoop.hdfs.namenode.GetMostRecentCheckpointT\
    xIdRequestProto\x1a>.hadoop.hdfs.namenode.GetMostRecentCheckpointTxIdRes\
    ponseProto\x12l\n\x0brollEditLog\x12-.hadoop.hdfs.namenode.RollEditLogRe\
    questProto\x1a..hadoop.hdfs.namenode.RollEditLogResponseProto\x12U\n\x0e\
    versionRequest\x12\x20.hadoop.hdfs.VersionRequestProto\x1a!.hadoop.hdfs.\
    VersionResponseProto\x12l\n\x0berrorReport\x12-.hadoop.hdfs.namenode.Err\
    orReportRequestProto\x1a..hadoop.hdfs.namenode.ErrorReportResponseProto\
    \x12v\n\x1bregisterSubordinateNamenode\x12*.hadoop.hdfs.namenode.Registe\
    rRequestProto\x1a+.hadoop.hdfs.namenode.RegisterResponseProto\x12x\n\x0f\
    startCheckpoint\x121.hadoop.hdfs.namenode.StartCheckpointRequestProto\
    \x1a2.hadoop.hdfs.namenode.StartCheckpointResponseProto\x12r\n\rendCheck\
    point\x12/.hadoop.hdfs.namenode.EndCheckpointRequestProto\x1a0.hadoop.hd\
    fs.namenode.EndCheckpointResponseProto\x12\x81\x01\n\x12getEditLogManife\
    st\x124.hadoop.hdfs.namenode.GetEditLogManifestRequestProto\x1a5.hadoop.\
    hdfs.namenode.GetEditLogManifestResponseProtoBE\n%org.apache.hadoop.hdfs\
    .protocol.protoB\x16NamenodeProtocolProtos\xa0\x01\x01\x88\x01\x01\
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
