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
pub struct HAStateChangeRequestInfoProto {
    // message fields
    reqSource: ::std::option::Option<HARequestSource>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HAStateChangeRequestInfoProto {}

impl HAStateChangeRequestInfoProto {
    pub fn new() -> HAStateChangeRequestInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HAStateChangeRequestInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<HAStateChangeRequestInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HAStateChangeRequestInfoProto,
        };
        unsafe {
            instance.get(HAStateChangeRequestInfoProto::new)
        }
    }

    // required .hadoop.common.HARequestSource reqSource = 1;

    pub fn clear_reqSource(&mut self) {
        self.reqSource = ::std::option::Option::None;
    }

    pub fn has_reqSource(&self) -> bool {
        self.reqSource.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reqSource(&mut self, v: HARequestSource) {
        self.reqSource = ::std::option::Option::Some(v);
    }

    pub fn get_reqSource(&self) -> HARequestSource {
        self.reqSource.unwrap_or(HARequestSource::REQUEST_BY_USER)
    }

    fn get_reqSource_for_reflect(&self) -> &::std::option::Option<HARequestSource> {
        &self.reqSource
    }

    fn mut_reqSource_for_reflect(&mut self) -> &mut ::std::option::Option<HARequestSource> {
        &mut self.reqSource
    }
}

impl ::protobuf::Message for HAStateChangeRequestInfoProto {
    fn is_initialized(&self) -> bool {
        if self.reqSource.is_none() {
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
                    self.reqSource = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.reqSource {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.reqSource {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for HAStateChangeRequestInfoProto {
    fn new() -> HAStateChangeRequestInfoProto {
        HAStateChangeRequestInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<HAStateChangeRequestInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<HARequestSource>>(
                    "reqSource",
                    HAStateChangeRequestInfoProto::get_reqSource_for_reflect,
                    HAStateChangeRequestInfoProto::mut_reqSource_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HAStateChangeRequestInfoProto>(
                    "HAStateChangeRequestInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HAStateChangeRequestInfoProto {
    fn clear(&mut self) {
        self.clear_reqSource();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HAStateChangeRequestInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HAStateChangeRequestInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MonitorHealthRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MonitorHealthRequestProto {}

impl MonitorHealthRequestProto {
    pub fn new() -> MonitorHealthRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MonitorHealthRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<MonitorHealthRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MonitorHealthRequestProto,
        };
        unsafe {
            instance.get(MonitorHealthRequestProto::new)
        }
    }
}

impl ::protobuf::Message for MonitorHealthRequestProto {
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

impl ::protobuf::MessageStatic for MonitorHealthRequestProto {
    fn new() -> MonitorHealthRequestProto {
        MonitorHealthRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<MonitorHealthRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<MonitorHealthRequestProto>(
                    "MonitorHealthRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MonitorHealthRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MonitorHealthRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MonitorHealthRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MonitorHealthResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MonitorHealthResponseProto {}

impl MonitorHealthResponseProto {
    pub fn new() -> MonitorHealthResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MonitorHealthResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<MonitorHealthResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MonitorHealthResponseProto,
        };
        unsafe {
            instance.get(MonitorHealthResponseProto::new)
        }
    }
}

impl ::protobuf::Message for MonitorHealthResponseProto {
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

impl ::protobuf::MessageStatic for MonitorHealthResponseProto {
    fn new() -> MonitorHealthResponseProto {
        MonitorHealthResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<MonitorHealthResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<MonitorHealthResponseProto>(
                    "MonitorHealthResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MonitorHealthResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MonitorHealthResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MonitorHealthResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TransitionToActiveRequestProto {
    // message fields
    reqInfo: ::protobuf::SingularPtrField<HAStateChangeRequestInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransitionToActiveRequestProto {}

impl TransitionToActiveRequestProto {
    pub fn new() -> TransitionToActiveRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransitionToActiveRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<TransitionToActiveRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransitionToActiveRequestProto,
        };
        unsafe {
            instance.get(TransitionToActiveRequestProto::new)
        }
    }

    // required .hadoop.common.HAStateChangeRequestInfoProto reqInfo = 1;

    pub fn clear_reqInfo(&mut self) {
        self.reqInfo.clear();
    }

    pub fn has_reqInfo(&self) -> bool {
        self.reqInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reqInfo(&mut self, v: HAStateChangeRequestInfoProto) {
        self.reqInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reqInfo(&mut self) -> &mut HAStateChangeRequestInfoProto {
        if self.reqInfo.is_none() {
            self.reqInfo.set_default();
        }
        self.reqInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_reqInfo(&mut self) -> HAStateChangeRequestInfoProto {
        self.reqInfo.take().unwrap_or_else(|| HAStateChangeRequestInfoProto::new())
    }

    pub fn get_reqInfo(&self) -> &HAStateChangeRequestInfoProto {
        self.reqInfo.as_ref().unwrap_or_else(|| HAStateChangeRequestInfoProto::default_instance())
    }

    fn get_reqInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<HAStateChangeRequestInfoProto> {
        &self.reqInfo
    }

    fn mut_reqInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<HAStateChangeRequestInfoProto> {
        &mut self.reqInfo
    }
}

impl ::protobuf::Message for TransitionToActiveRequestProto {
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

impl ::protobuf::MessageStatic for TransitionToActiveRequestProto {
    fn new() -> TransitionToActiveRequestProto {
        TransitionToActiveRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransitionToActiveRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HAStateChangeRequestInfoProto>>(
                    "reqInfo",
                    TransitionToActiveRequestProto::get_reqInfo_for_reflect,
                    TransitionToActiveRequestProto::mut_reqInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransitionToActiveRequestProto>(
                    "TransitionToActiveRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransitionToActiveRequestProto {
    fn clear(&mut self) {
        self.clear_reqInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransitionToActiveRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransitionToActiveRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TransitionToActiveResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransitionToActiveResponseProto {}

impl TransitionToActiveResponseProto {
    pub fn new() -> TransitionToActiveResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransitionToActiveResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<TransitionToActiveResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransitionToActiveResponseProto,
        };
        unsafe {
            instance.get(TransitionToActiveResponseProto::new)
        }
    }
}

impl ::protobuf::Message for TransitionToActiveResponseProto {
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

impl ::protobuf::MessageStatic for TransitionToActiveResponseProto {
    fn new() -> TransitionToActiveResponseProto {
        TransitionToActiveResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransitionToActiveResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<TransitionToActiveResponseProto>(
                    "TransitionToActiveResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransitionToActiveResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransitionToActiveResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransitionToActiveResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TransitionToStandbyRequestProto {
    // message fields
    reqInfo: ::protobuf::SingularPtrField<HAStateChangeRequestInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransitionToStandbyRequestProto {}

impl TransitionToStandbyRequestProto {
    pub fn new() -> TransitionToStandbyRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransitionToStandbyRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<TransitionToStandbyRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransitionToStandbyRequestProto,
        };
        unsafe {
            instance.get(TransitionToStandbyRequestProto::new)
        }
    }

    // required .hadoop.common.HAStateChangeRequestInfoProto reqInfo = 1;

    pub fn clear_reqInfo(&mut self) {
        self.reqInfo.clear();
    }

    pub fn has_reqInfo(&self) -> bool {
        self.reqInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reqInfo(&mut self, v: HAStateChangeRequestInfoProto) {
        self.reqInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reqInfo(&mut self) -> &mut HAStateChangeRequestInfoProto {
        if self.reqInfo.is_none() {
            self.reqInfo.set_default();
        }
        self.reqInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_reqInfo(&mut self) -> HAStateChangeRequestInfoProto {
        self.reqInfo.take().unwrap_or_else(|| HAStateChangeRequestInfoProto::new())
    }

    pub fn get_reqInfo(&self) -> &HAStateChangeRequestInfoProto {
        self.reqInfo.as_ref().unwrap_or_else(|| HAStateChangeRequestInfoProto::default_instance())
    }

    fn get_reqInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<HAStateChangeRequestInfoProto> {
        &self.reqInfo
    }

    fn mut_reqInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<HAStateChangeRequestInfoProto> {
        &mut self.reqInfo
    }
}

impl ::protobuf::Message for TransitionToStandbyRequestProto {
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

impl ::protobuf::MessageStatic for TransitionToStandbyRequestProto {
    fn new() -> TransitionToStandbyRequestProto {
        TransitionToStandbyRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransitionToStandbyRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HAStateChangeRequestInfoProto>>(
                    "reqInfo",
                    TransitionToStandbyRequestProto::get_reqInfo_for_reflect,
                    TransitionToStandbyRequestProto::mut_reqInfo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransitionToStandbyRequestProto>(
                    "TransitionToStandbyRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransitionToStandbyRequestProto {
    fn clear(&mut self) {
        self.clear_reqInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransitionToStandbyRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransitionToStandbyRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TransitionToStandbyResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransitionToStandbyResponseProto {}

impl TransitionToStandbyResponseProto {
    pub fn new() -> TransitionToStandbyResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransitionToStandbyResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<TransitionToStandbyResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransitionToStandbyResponseProto,
        };
        unsafe {
            instance.get(TransitionToStandbyResponseProto::new)
        }
    }
}

impl ::protobuf::Message for TransitionToStandbyResponseProto {
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

impl ::protobuf::MessageStatic for TransitionToStandbyResponseProto {
    fn new() -> TransitionToStandbyResponseProto {
        TransitionToStandbyResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransitionToStandbyResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<TransitionToStandbyResponseProto>(
                    "TransitionToStandbyResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransitionToStandbyResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransitionToStandbyResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransitionToStandbyResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetServiceStatusRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetServiceStatusRequestProto {}

impl GetServiceStatusRequestProto {
    pub fn new() -> GetServiceStatusRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetServiceStatusRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetServiceStatusRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetServiceStatusRequestProto,
        };
        unsafe {
            instance.get(GetServiceStatusRequestProto::new)
        }
    }
}

impl ::protobuf::Message for GetServiceStatusRequestProto {
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

impl ::protobuf::MessageStatic for GetServiceStatusRequestProto {
    fn new() -> GetServiceStatusRequestProto {
        GetServiceStatusRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetServiceStatusRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetServiceStatusRequestProto>(
                    "GetServiceStatusRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetServiceStatusRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetServiceStatusRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetServiceStatusRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetServiceStatusResponseProto {
    // message fields
    state: ::std::option::Option<HAServiceStateProto>,
    readyToBecomeActive: ::std::option::Option<bool>,
    notReadyReason: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetServiceStatusResponseProto {}

impl GetServiceStatusResponseProto {
    pub fn new() -> GetServiceStatusResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetServiceStatusResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetServiceStatusResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetServiceStatusResponseProto,
        };
        unsafe {
            instance.get(GetServiceStatusResponseProto::new)
        }
    }

    // required .hadoop.common.HAServiceStateProto state = 1;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: HAServiceStateProto) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> HAServiceStateProto {
        self.state.unwrap_or(HAServiceStateProto::INITIALIZING)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<HAServiceStateProto> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<HAServiceStateProto> {
        &mut self.state
    }

    // optional bool readyToBecomeActive = 2;

    pub fn clear_readyToBecomeActive(&mut self) {
        self.readyToBecomeActive = ::std::option::Option::None;
    }

    pub fn has_readyToBecomeActive(&self) -> bool {
        self.readyToBecomeActive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_readyToBecomeActive(&mut self, v: bool) {
        self.readyToBecomeActive = ::std::option::Option::Some(v);
    }

    pub fn get_readyToBecomeActive(&self) -> bool {
        self.readyToBecomeActive.unwrap_or(false)
    }

    fn get_readyToBecomeActive_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.readyToBecomeActive
    }

    fn mut_readyToBecomeActive_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.readyToBecomeActive
    }

    // optional string notReadyReason = 3;

    pub fn clear_notReadyReason(&mut self) {
        self.notReadyReason.clear();
    }

    pub fn has_notReadyReason(&self) -> bool {
        self.notReadyReason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_notReadyReason(&mut self, v: ::std::string::String) {
        self.notReadyReason = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_notReadyReason(&mut self) -> &mut ::std::string::String {
        if self.notReadyReason.is_none() {
            self.notReadyReason.set_default();
        }
        self.notReadyReason.as_mut().unwrap()
    }

    // Take field
    pub fn take_notReadyReason(&mut self) -> ::std::string::String {
        self.notReadyReason.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_notReadyReason(&self) -> &str {
        match self.notReadyReason.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_notReadyReason_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.notReadyReason
    }

    fn mut_notReadyReason_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.notReadyReason
    }
}

impl ::protobuf::Message for GetServiceStatusResponseProto {
    fn is_initialized(&self) -> bool {
        if self.state.is_none() {
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
                    self.state = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.readyToBecomeActive = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.notReadyReason)?;
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
        if let Some(v) = self.state {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.readyToBecomeActive {
            my_size += 2;
        }
        if let Some(ref v) = self.notReadyReason.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.state {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.readyToBecomeActive {
            os.write_bool(2, v)?;
        }
        if let Some(ref v) = self.notReadyReason.as_ref() {
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

impl ::protobuf::MessageStatic for GetServiceStatusResponseProto {
    fn new() -> GetServiceStatusResponseProto {
        GetServiceStatusResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetServiceStatusResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<HAServiceStateProto>>(
                    "state",
                    GetServiceStatusResponseProto::get_state_for_reflect,
                    GetServiceStatusResponseProto::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "readyToBecomeActive",
                    GetServiceStatusResponseProto::get_readyToBecomeActive_for_reflect,
                    GetServiceStatusResponseProto::mut_readyToBecomeActive_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "notReadyReason",
                    GetServiceStatusResponseProto::get_notReadyReason_for_reflect,
                    GetServiceStatusResponseProto::mut_notReadyReason_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetServiceStatusResponseProto>(
                    "GetServiceStatusResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetServiceStatusResponseProto {
    fn clear(&mut self) {
        self.clear_state();
        self.clear_readyToBecomeActive();
        self.clear_notReadyReason();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetServiceStatusResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetServiceStatusResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum HAServiceStateProto {
    INITIALIZING = 0,
    ACTIVE = 1,
    STANDBY = 2,
}

impl ::protobuf::ProtobufEnum for HAServiceStateProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<HAServiceStateProto> {
        match value {
            0 => ::std::option::Option::Some(HAServiceStateProto::INITIALIZING),
            1 => ::std::option::Option::Some(HAServiceStateProto::ACTIVE),
            2 => ::std::option::Option::Some(HAServiceStateProto::STANDBY),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [HAServiceStateProto] = &[
            HAServiceStateProto::INITIALIZING,
            HAServiceStateProto::ACTIVE,
            HAServiceStateProto::STANDBY,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<HAServiceStateProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("HAServiceStateProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for HAServiceStateProto {
}

impl ::protobuf::reflect::ProtobufValue for HAServiceStateProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum HARequestSource {
    REQUEST_BY_USER = 0,
    REQUEST_BY_USER_FORCED = 1,
    REQUEST_BY_ZKFC = 2,
}

impl ::protobuf::ProtobufEnum for HARequestSource {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<HARequestSource> {
        match value {
            0 => ::std::option::Option::Some(HARequestSource::REQUEST_BY_USER),
            1 => ::std::option::Option::Some(HARequestSource::REQUEST_BY_USER_FORCED),
            2 => ::std::option::Option::Some(HARequestSource::REQUEST_BY_ZKFC),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [HARequestSource] = &[
            HARequestSource::REQUEST_BY_USER,
            HARequestSource::REQUEST_BY_USER_FORCED,
            HARequestSource::REQUEST_BY_ZKFC,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<HARequestSource>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("HARequestSource", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for HARequestSource {
}

impl ::protobuf::reflect::ProtobufValue for HARequestSource {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17HAServiceProtocol.proto\x12\rhadoop.common\"R\n\x1dHAStateChangeRe\
    questInfoProto\x121\n\treqSource\x18\x01\x20\x02(\x0e2\x1e.hadoop.common\
    .HARequestSource\"\x1b\n\x19MonitorHealthRequestProto\"\x1c\n\x1aMonitor\
    HealthResponseProto\"_\n\x1eTransitionToActiveRequestProto\x12=\n\x07req\
    Info\x18\x01\x20\x02(\x0b2,.hadoop.common.HAStateChangeRequestInfoProto\
    \"!\n\x1fTransitionToActiveResponseProto\"`\n\x1fTransitionToStandbyRequ\
    estProto\x12=\n\x07reqInfo\x18\x01\x20\x02(\x0b2,.hadoop.common.HAStateC\
    hangeRequestInfoProto\"\"\n\x20TransitionToStandbyResponseProto\"\x1e\n\
    \x1cGetServiceStatusRequestProto\"\x87\x01\n\x1dGetServiceStatusResponse\
    Proto\x121\n\x05state\x18\x01\x20\x02(\x0e2\".hadoop.common.HAServiceSta\
    teProto\x12\x1b\n\x13readyToBecomeActive\x18\x02\x20\x01(\x08\x12\x16\n\
    \x0enotReadyReason\x18\x03\x20\x01(\t*@\n\x13HAServiceStateProto\x12\x10\
    \n\x0cINITIALIZING\x10\0\x12\n\n\x06ACTIVE\x10\x01\x12\x0b\n\x07STANDBY\
    \x10\x02*W\n\x0fHARequestSource\x12\x13\n\x0fREQUEST_BY_USER\x10\0\x12\
    \x1a\n\x16REQUEST_BY_USER_FORCED\x10\x01\x12\x13\n\x0fREQUEST_BY_ZKFC\
    \x10\x022\xdc\x03\n\x18HAServiceProtocolService\x12d\n\rmonitorHealth\
    \x12(.hadoop.common.MonitorHealthRequestProto\x1a).hadoop.common.Monitor\
    HealthResponseProto\x12s\n\x12transitionToActive\x12-.hadoop.common.Tran\
    sitionToActiveRequestProto\x1a..hadoop.common.TransitionToActiveResponse\
    Proto\x12v\n\x13transitionToStandby\x12..hadoop.common.TransitionToStand\
    byRequestProto\x1a/.hadoop.common.TransitionToStandbyResponseProto\x12m\
    \n\x10getServiceStatus\x12+.hadoop.common.GetServiceStatusRequestProto\
    \x1a,.hadoop.common.GetServiceStatusResponseProtoB;\n\x1aorg.apache.hado\
    op.ha.protoB\x17HAServiceProtocolProtos\xa0\x01\x01\x88\x01\x01\
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
