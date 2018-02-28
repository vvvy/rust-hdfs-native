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
pub struct InitReplicaRecoveryRequestProto {
    // message fields
    block: ::protobuf::SingularPtrField<super::hdfs::RecoveringBlockProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InitReplicaRecoveryRequestProto {}

impl InitReplicaRecoveryRequestProto {
    pub fn new() -> InitReplicaRecoveryRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InitReplicaRecoveryRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<InitReplicaRecoveryRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InitReplicaRecoveryRequestProto,
        };
        unsafe {
            instance.get(InitReplicaRecoveryRequestProto::new)
        }
    }

    // required .hadoop.hdfs.RecoveringBlockProto block = 1;

    pub fn clear_block(&mut self) {
        self.block.clear();
    }

    pub fn has_block(&self) -> bool {
        self.block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block(&mut self, v: super::hdfs::RecoveringBlockProto) {
        self.block = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block(&mut self) -> &mut super::hdfs::RecoveringBlockProto {
        if self.block.is_none() {
            self.block.set_default();
        }
        self.block.as_mut().unwrap()
    }

    // Take field
    pub fn take_block(&mut self) -> super::hdfs::RecoveringBlockProto {
        self.block.take().unwrap_or_else(|| super::hdfs::RecoveringBlockProto::new())
    }

    pub fn get_block(&self) -> &super::hdfs::RecoveringBlockProto {
        self.block.as_ref().unwrap_or_else(|| super::hdfs::RecoveringBlockProto::default_instance())
    }

    fn get_block_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::RecoveringBlockProto> {
        &self.block
    }

    fn mut_block_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::RecoveringBlockProto> {
        &mut self.block
    }
}

impl ::protobuf::Message for InitReplicaRecoveryRequestProto {
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

impl ::protobuf::MessageStatic for InitReplicaRecoveryRequestProto {
    fn new() -> InitReplicaRecoveryRequestProto {
        InitReplicaRecoveryRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<InitReplicaRecoveryRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::RecoveringBlockProto>>(
                    "block",
                    InitReplicaRecoveryRequestProto::get_block_for_reflect,
                    InitReplicaRecoveryRequestProto::mut_block_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InitReplicaRecoveryRequestProto>(
                    "InitReplicaRecoveryRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InitReplicaRecoveryRequestProto {
    fn clear(&mut self) {
        self.clear_block();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InitReplicaRecoveryRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InitReplicaRecoveryRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InitReplicaRecoveryResponseProto {
    // message fields
    replicaFound: ::std::option::Option<bool>,
    state: ::std::option::Option<super::hdfs::ReplicaStateProto>,
    block: ::protobuf::SingularPtrField<super::hdfs::BlockProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InitReplicaRecoveryResponseProto {}

impl InitReplicaRecoveryResponseProto {
    pub fn new() -> InitReplicaRecoveryResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InitReplicaRecoveryResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<InitReplicaRecoveryResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InitReplicaRecoveryResponseProto,
        };
        unsafe {
            instance.get(InitReplicaRecoveryResponseProto::new)
        }
    }

    // required bool replicaFound = 1;

    pub fn clear_replicaFound(&mut self) {
        self.replicaFound = ::std::option::Option::None;
    }

    pub fn has_replicaFound(&self) -> bool {
        self.replicaFound.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replicaFound(&mut self, v: bool) {
        self.replicaFound = ::std::option::Option::Some(v);
    }

    pub fn get_replicaFound(&self) -> bool {
        self.replicaFound.unwrap_or(false)
    }

    fn get_replicaFound_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.replicaFound
    }

    fn mut_replicaFound_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.replicaFound
    }

    // optional .hadoop.hdfs.ReplicaStateProto state = 2;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: super::hdfs::ReplicaStateProto) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> super::hdfs::ReplicaStateProto {
        self.state.unwrap_or(super::hdfs::ReplicaStateProto::FINALIZED)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<super::hdfs::ReplicaStateProto> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<super::hdfs::ReplicaStateProto> {
        &mut self.state
    }

    // optional .hadoop.hdfs.BlockProto block = 3;

    pub fn clear_block(&mut self) {
        self.block.clear();
    }

    pub fn has_block(&self) -> bool {
        self.block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block(&mut self, v: super::hdfs::BlockProto) {
        self.block = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block(&mut self) -> &mut super::hdfs::BlockProto {
        if self.block.is_none() {
            self.block.set_default();
        }
        self.block.as_mut().unwrap()
    }

    // Take field
    pub fn take_block(&mut self) -> super::hdfs::BlockProto {
        self.block.take().unwrap_or_else(|| super::hdfs::BlockProto::new())
    }

    pub fn get_block(&self) -> &super::hdfs::BlockProto {
        self.block.as_ref().unwrap_or_else(|| super::hdfs::BlockProto::default_instance())
    }

    fn get_block_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::BlockProto> {
        &self.block
    }

    fn mut_block_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::BlockProto> {
        &mut self.block
    }
}

impl ::protobuf::Message for InitReplicaRecoveryResponseProto {
    fn is_initialized(&self) -> bool {
        if self.replicaFound.is_none() {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.replicaFound = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.state = ::std::option::Option::Some(tmp);
                },
                3 => {
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
        if let Some(v) = self.replicaFound {
            my_size += 2;
        }
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(ref v) = self.block.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.replicaFound {
            os.write_bool(1, v)?;
        }
        if let Some(v) = self.state {
            os.write_enum(2, v.value())?;
        }
        if let Some(ref v) = self.block.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for InitReplicaRecoveryResponseProto {
    fn new() -> InitReplicaRecoveryResponseProto {
        InitReplicaRecoveryResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<InitReplicaRecoveryResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "replicaFound",
                    InitReplicaRecoveryResponseProto::get_replicaFound_for_reflect,
                    InitReplicaRecoveryResponseProto::mut_replicaFound_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::hdfs::ReplicaStateProto>>(
                    "state",
                    InitReplicaRecoveryResponseProto::get_state_for_reflect,
                    InitReplicaRecoveryResponseProto::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::BlockProto>>(
                    "block",
                    InitReplicaRecoveryResponseProto::get_block_for_reflect,
                    InitReplicaRecoveryResponseProto::mut_block_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InitReplicaRecoveryResponseProto>(
                    "InitReplicaRecoveryResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InitReplicaRecoveryResponseProto {
    fn clear(&mut self) {
        self.clear_replicaFound();
        self.clear_state();
        self.clear_block();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InitReplicaRecoveryResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InitReplicaRecoveryResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UpdateReplicaUnderRecoveryRequestProto {
    // message fields
    block: ::protobuf::SingularPtrField<super::hdfs::ExtendedBlockProto>,
    recoveryId: ::std::option::Option<u64>,
    newLength: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpdateReplicaUnderRecoveryRequestProto {}

impl UpdateReplicaUnderRecoveryRequestProto {
    pub fn new() -> UpdateReplicaUnderRecoveryRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpdateReplicaUnderRecoveryRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<UpdateReplicaUnderRecoveryRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpdateReplicaUnderRecoveryRequestProto,
        };
        unsafe {
            instance.get(UpdateReplicaUnderRecoveryRequestProto::new)
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

    // required uint64 recoveryId = 2;

    pub fn clear_recoveryId(&mut self) {
        self.recoveryId = ::std::option::Option::None;
    }

    pub fn has_recoveryId(&self) -> bool {
        self.recoveryId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_recoveryId(&mut self, v: u64) {
        self.recoveryId = ::std::option::Option::Some(v);
    }

    pub fn get_recoveryId(&self) -> u64 {
        self.recoveryId.unwrap_or(0)
    }

    fn get_recoveryId_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.recoveryId
    }

    fn mut_recoveryId_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.recoveryId
    }

    // required uint64 newLength = 3;

    pub fn clear_newLength(&mut self) {
        self.newLength = ::std::option::Option::None;
    }

    pub fn has_newLength(&self) -> bool {
        self.newLength.is_some()
    }

    // Param is passed by value, moved
    pub fn set_newLength(&mut self, v: u64) {
        self.newLength = ::std::option::Option::Some(v);
    }

    pub fn get_newLength(&self) -> u64 {
        self.newLength.unwrap_or(0)
    }

    fn get_newLength_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.newLength
    }

    fn mut_newLength_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.newLength
    }
}

impl ::protobuf::Message for UpdateReplicaUnderRecoveryRequestProto {
    fn is_initialized(&self) -> bool {
        if self.block.is_none() {
            return false;
        }
        if self.recoveryId.is_none() {
            return false;
        }
        if self.newLength.is_none() {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.recoveryId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.newLength = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.recoveryId {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.newLength {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.recoveryId {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.newLength {
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

impl ::protobuf::MessageStatic for UpdateReplicaUnderRecoveryRequestProto {
    fn new() -> UpdateReplicaUnderRecoveryRequestProto {
        UpdateReplicaUnderRecoveryRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpdateReplicaUnderRecoveryRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::ExtendedBlockProto>>(
                    "block",
                    UpdateReplicaUnderRecoveryRequestProto::get_block_for_reflect,
                    UpdateReplicaUnderRecoveryRequestProto::mut_block_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "recoveryId",
                    UpdateReplicaUnderRecoveryRequestProto::get_recoveryId_for_reflect,
                    UpdateReplicaUnderRecoveryRequestProto::mut_recoveryId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "newLength",
                    UpdateReplicaUnderRecoveryRequestProto::get_newLength_for_reflect,
                    UpdateReplicaUnderRecoveryRequestProto::mut_newLength_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpdateReplicaUnderRecoveryRequestProto>(
                    "UpdateReplicaUnderRecoveryRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpdateReplicaUnderRecoveryRequestProto {
    fn clear(&mut self) {
        self.clear_block();
        self.clear_recoveryId();
        self.clear_newLength();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UpdateReplicaUnderRecoveryRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateReplicaUnderRecoveryRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UpdateReplicaUnderRecoveryResponseProto {
    // message fields
    storageUuid: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpdateReplicaUnderRecoveryResponseProto {}

impl UpdateReplicaUnderRecoveryResponseProto {
    pub fn new() -> UpdateReplicaUnderRecoveryResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpdateReplicaUnderRecoveryResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<UpdateReplicaUnderRecoveryResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpdateReplicaUnderRecoveryResponseProto,
        };
        unsafe {
            instance.get(UpdateReplicaUnderRecoveryResponseProto::new)
        }
    }

    // optional string storageUuid = 1;

    pub fn clear_storageUuid(&mut self) {
        self.storageUuid.clear();
    }

    pub fn has_storageUuid(&self) -> bool {
        self.storageUuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageUuid(&mut self, v: ::std::string::String) {
        self.storageUuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storageUuid(&mut self) -> &mut ::std::string::String {
        if self.storageUuid.is_none() {
            self.storageUuid.set_default();
        }
        self.storageUuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_storageUuid(&mut self) -> ::std::string::String {
        self.storageUuid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_storageUuid(&self) -> &str {
        match self.storageUuid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_storageUuid_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.storageUuid
    }

    fn mut_storageUuid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.storageUuid
    }
}

impl ::protobuf::Message for UpdateReplicaUnderRecoveryResponseProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.storageUuid)?;
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
        if let Some(ref v) = self.storageUuid.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.storageUuid.as_ref() {
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

impl ::protobuf::MessageStatic for UpdateReplicaUnderRecoveryResponseProto {
    fn new() -> UpdateReplicaUnderRecoveryResponseProto {
        UpdateReplicaUnderRecoveryResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpdateReplicaUnderRecoveryResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "storageUuid",
                    UpdateReplicaUnderRecoveryResponseProto::get_storageUuid_for_reflect,
                    UpdateReplicaUnderRecoveryResponseProto::mut_storageUuid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpdateReplicaUnderRecoveryResponseProto>(
                    "UpdateReplicaUnderRecoveryResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpdateReplicaUnderRecoveryResponseProto {
    fn clear(&mut self) {
        self.clear_storageUuid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UpdateReplicaUnderRecoveryResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateReplicaUnderRecoveryResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bInterDatanodeProtocol.proto\x12\x0bhadoop.hdfs\x1a\nhdfs.proto\"S\
    \n\x1fInitReplicaRecoveryRequestProto\x120\n\x05block\x18\x01\x20\x02(\
    \x0b2!.hadoop.hdfs.RecoveringBlockProto\"\x8f\x01\n\x20InitReplicaRecove\
    ryResponseProto\x12\x14\n\x0creplicaFound\x18\x01\x20\x02(\x08\x12-\n\
    \x05state\x18\x02\x20\x01(\x0e2\x1e.hadoop.hdfs.ReplicaStateProto\x12&\n\
    \x05block\x18\x03\x20\x01(\x0b2\x17.hadoop.hdfs.BlockProto\"\x7f\n&Updat\
    eReplicaUnderRecoveryRequestProto\x12.\n\x05block\x18\x01\x20\x02(\x0b2\
    \x1f.hadoop.hdfs.ExtendedBlockProto\x12\x12\n\nrecoveryId\x18\x02\x20\
    \x02(\x04\x12\x11\n\tnewLength\x18\x03\x20\x02(\x04\">\n'UpdateReplicaUn\
    derRecoveryResponseProto\x12\x13\n\x0bstorageUuid\x18\x01\x20\x01(\t2\
    \x9c\x02\n\x1cInterDatanodeProtocolService\x12r\n\x13initReplicaRecovery\
    \x12,.hadoop.hdfs.InitReplicaRecoveryRequestProto\x1a-.hadoop.hdfs.InitR\
    eplicaRecoveryResponseProto\x12\x87\x01\n\x1aupdateReplicaUnderRecovery\
    \x123.hadoop.hdfs.UpdateReplicaUnderRecoveryRequestProto\x1a4.hadoop.hdf\
    s.UpdateReplicaUnderRecoveryResponseProtoBJ\n%org.apache.hadoop.hdfs.pro\
    tocol.protoB\x1bInterDatanodeProtocolProtos\xa0\x01\x01\x88\x01\x01\
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
