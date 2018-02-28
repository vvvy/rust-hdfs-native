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
pub struct DatanodeRegistrationProto {
    // message fields
    datanodeID: ::protobuf::SingularPtrField<super::hdfs::DatanodeIDProto>,
    storageInfo: ::protobuf::SingularPtrField<super::hdfs::StorageInfoProto>,
    keys: ::protobuf::SingularPtrField<super::hdfs::ExportedBlockKeysProto>,
    softwareVersion: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DatanodeRegistrationProto {}

impl DatanodeRegistrationProto {
    pub fn new() -> DatanodeRegistrationProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DatanodeRegistrationProto {
        static mut instance: ::protobuf::lazy::Lazy<DatanodeRegistrationProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DatanodeRegistrationProto,
        };
        unsafe {
            instance.get(DatanodeRegistrationProto::new)
        }
    }

    // required .hadoop.hdfs.DatanodeIDProto datanodeID = 1;

    pub fn clear_datanodeID(&mut self) {
        self.datanodeID.clear();
    }

    pub fn has_datanodeID(&self) -> bool {
        self.datanodeID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_datanodeID(&mut self, v: super::hdfs::DatanodeIDProto) {
        self.datanodeID = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_datanodeID(&mut self) -> &mut super::hdfs::DatanodeIDProto {
        if self.datanodeID.is_none() {
            self.datanodeID.set_default();
        }
        self.datanodeID.as_mut().unwrap()
    }

    // Take field
    pub fn take_datanodeID(&mut self) -> super::hdfs::DatanodeIDProto {
        self.datanodeID.take().unwrap_or_else(|| super::hdfs::DatanodeIDProto::new())
    }

    pub fn get_datanodeID(&self) -> &super::hdfs::DatanodeIDProto {
        self.datanodeID.as_ref().unwrap_or_else(|| super::hdfs::DatanodeIDProto::default_instance())
    }

    fn get_datanodeID_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::DatanodeIDProto> {
        &self.datanodeID
    }

    fn mut_datanodeID_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::DatanodeIDProto> {
        &mut self.datanodeID
    }

    // required .hadoop.hdfs.StorageInfoProto storageInfo = 2;

    pub fn clear_storageInfo(&mut self) {
        self.storageInfo.clear();
    }

    pub fn has_storageInfo(&self) -> bool {
        self.storageInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageInfo(&mut self, v: super::hdfs::StorageInfoProto) {
        self.storageInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storageInfo(&mut self) -> &mut super::hdfs::StorageInfoProto {
        if self.storageInfo.is_none() {
            self.storageInfo.set_default();
        }
        self.storageInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_storageInfo(&mut self) -> super::hdfs::StorageInfoProto {
        self.storageInfo.take().unwrap_or_else(|| super::hdfs::StorageInfoProto::new())
    }

    pub fn get_storageInfo(&self) -> &super::hdfs::StorageInfoProto {
        self.storageInfo.as_ref().unwrap_or_else(|| super::hdfs::StorageInfoProto::default_instance())
    }

    fn get_storageInfo_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::StorageInfoProto> {
        &self.storageInfo
    }

    fn mut_storageInfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::StorageInfoProto> {
        &mut self.storageInfo
    }

    // required .hadoop.hdfs.ExportedBlockKeysProto keys = 3;

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

    // required string softwareVersion = 4;

    pub fn clear_softwareVersion(&mut self) {
        self.softwareVersion.clear();
    }

    pub fn has_softwareVersion(&self) -> bool {
        self.softwareVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_softwareVersion(&mut self, v: ::std::string::String) {
        self.softwareVersion = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_softwareVersion(&mut self) -> &mut ::std::string::String {
        if self.softwareVersion.is_none() {
            self.softwareVersion.set_default();
        }
        self.softwareVersion.as_mut().unwrap()
    }

    // Take field
    pub fn take_softwareVersion(&mut self) -> ::std::string::String {
        self.softwareVersion.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_softwareVersion(&self) -> &str {
        match self.softwareVersion.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_softwareVersion_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.softwareVersion
    }

    fn mut_softwareVersion_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.softwareVersion
    }
}

impl ::protobuf::Message for DatanodeRegistrationProto {
    fn is_initialized(&self) -> bool {
        if self.datanodeID.is_none() {
            return false;
        }
        if self.storageInfo.is_none() {
            return false;
        }
        if self.keys.is_none() {
            return false;
        }
        if self.softwareVersion.is_none() {
            return false;
        }
        for v in &self.datanodeID {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.storageInfo {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.datanodeID)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.storageInfo)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.keys)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.softwareVersion)?;
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
        if let Some(ref v) = self.datanodeID.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.storageInfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.keys.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.softwareVersion.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.datanodeID.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.storageInfo.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.keys.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.softwareVersion.as_ref() {
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

impl ::protobuf::MessageStatic for DatanodeRegistrationProto {
    fn new() -> DatanodeRegistrationProto {
        DatanodeRegistrationProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DatanodeRegistrationProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeIDProto>>(
                    "datanodeID",
                    DatanodeRegistrationProto::get_datanodeID_for_reflect,
                    DatanodeRegistrationProto::mut_datanodeID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::StorageInfoProto>>(
                    "storageInfo",
                    DatanodeRegistrationProto::get_storageInfo_for_reflect,
                    DatanodeRegistrationProto::mut_storageInfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::ExportedBlockKeysProto>>(
                    "keys",
                    DatanodeRegistrationProto::get_keys_for_reflect,
                    DatanodeRegistrationProto::mut_keys_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "softwareVersion",
                    DatanodeRegistrationProto::get_softwareVersion_for_reflect,
                    DatanodeRegistrationProto::mut_softwareVersion_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DatanodeRegistrationProto>(
                    "DatanodeRegistrationProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DatanodeRegistrationProto {
    fn clear(&mut self) {
        self.clear_datanodeID();
        self.clear_storageInfo();
        self.clear_keys();
        self.clear_softwareVersion();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DatanodeRegistrationProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DatanodeRegistrationProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DatanodeCommandProto {
    // message fields
    cmdType: ::std::option::Option<DatanodeCommandProto_Type>,
    balancerCmd: ::protobuf::SingularPtrField<BalancerBandwidthCommandProto>,
    blkCmd: ::protobuf::SingularPtrField<BlockCommandProto>,
    recoveryCmd: ::protobuf::SingularPtrField<BlockRecoveryCommandProto>,
    finalizeCmd: ::protobuf::SingularPtrField<FinalizeCommandProto>,
    keyUpdateCmd: ::protobuf::SingularPtrField<KeyUpdateCommandProto>,
    registerCmd: ::protobuf::SingularPtrField<RegisterCommandProto>,
    blkIdCmd: ::protobuf::SingularPtrField<BlockIdCommandProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DatanodeCommandProto {}

impl DatanodeCommandProto {
    pub fn new() -> DatanodeCommandProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DatanodeCommandProto {
        static mut instance: ::protobuf::lazy::Lazy<DatanodeCommandProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DatanodeCommandProto,
        };
        unsafe {
            instance.get(DatanodeCommandProto::new)
        }
    }

    // required .hadoop.hdfs.datanode.DatanodeCommandProto.Type cmdType = 1;

    pub fn clear_cmdType(&mut self) {
        self.cmdType = ::std::option::Option::None;
    }

    pub fn has_cmdType(&self) -> bool {
        self.cmdType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmdType(&mut self, v: DatanodeCommandProto_Type) {
        self.cmdType = ::std::option::Option::Some(v);
    }

    pub fn get_cmdType(&self) -> DatanodeCommandProto_Type {
        self.cmdType.unwrap_or(DatanodeCommandProto_Type::BalancerBandwidthCommand)
    }

    fn get_cmdType_for_reflect(&self) -> &::std::option::Option<DatanodeCommandProto_Type> {
        &self.cmdType
    }

    fn mut_cmdType_for_reflect(&mut self) -> &mut ::std::option::Option<DatanodeCommandProto_Type> {
        &mut self.cmdType
    }

    // optional .hadoop.hdfs.datanode.BalancerBandwidthCommandProto balancerCmd = 2;

    pub fn clear_balancerCmd(&mut self) {
        self.balancerCmd.clear();
    }

    pub fn has_balancerCmd(&self) -> bool {
        self.balancerCmd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_balancerCmd(&mut self, v: BalancerBandwidthCommandProto) {
        self.balancerCmd = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_balancerCmd(&mut self) -> &mut BalancerBandwidthCommandProto {
        if self.balancerCmd.is_none() {
            self.balancerCmd.set_default();
        }
        self.balancerCmd.as_mut().unwrap()
    }

    // Take field
    pub fn take_balancerCmd(&mut self) -> BalancerBandwidthCommandProto {
        self.balancerCmd.take().unwrap_or_else(|| BalancerBandwidthCommandProto::new())
    }

    pub fn get_balancerCmd(&self) -> &BalancerBandwidthCommandProto {
        self.balancerCmd.as_ref().unwrap_or_else(|| BalancerBandwidthCommandProto::default_instance())
    }

    fn get_balancerCmd_for_reflect(&self) -> &::protobuf::SingularPtrField<BalancerBandwidthCommandProto> {
        &self.balancerCmd
    }

    fn mut_balancerCmd_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BalancerBandwidthCommandProto> {
        &mut self.balancerCmd
    }

    // optional .hadoop.hdfs.datanode.BlockCommandProto blkCmd = 3;

    pub fn clear_blkCmd(&mut self) {
        self.blkCmd.clear();
    }

    pub fn has_blkCmd(&self) -> bool {
        self.blkCmd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blkCmd(&mut self, v: BlockCommandProto) {
        self.blkCmd = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blkCmd(&mut self) -> &mut BlockCommandProto {
        if self.blkCmd.is_none() {
            self.blkCmd.set_default();
        }
        self.blkCmd.as_mut().unwrap()
    }

    // Take field
    pub fn take_blkCmd(&mut self) -> BlockCommandProto {
        self.blkCmd.take().unwrap_or_else(|| BlockCommandProto::new())
    }

    pub fn get_blkCmd(&self) -> &BlockCommandProto {
        self.blkCmd.as_ref().unwrap_or_else(|| BlockCommandProto::default_instance())
    }

    fn get_blkCmd_for_reflect(&self) -> &::protobuf::SingularPtrField<BlockCommandProto> {
        &self.blkCmd
    }

    fn mut_blkCmd_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BlockCommandProto> {
        &mut self.blkCmd
    }

    // optional .hadoop.hdfs.datanode.BlockRecoveryCommandProto recoveryCmd = 4;

    pub fn clear_recoveryCmd(&mut self) {
        self.recoveryCmd.clear();
    }

    pub fn has_recoveryCmd(&self) -> bool {
        self.recoveryCmd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_recoveryCmd(&mut self, v: BlockRecoveryCommandProto) {
        self.recoveryCmd = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_recoveryCmd(&mut self) -> &mut BlockRecoveryCommandProto {
        if self.recoveryCmd.is_none() {
            self.recoveryCmd.set_default();
        }
        self.recoveryCmd.as_mut().unwrap()
    }

    // Take field
    pub fn take_recoveryCmd(&mut self) -> BlockRecoveryCommandProto {
        self.recoveryCmd.take().unwrap_or_else(|| BlockRecoveryCommandProto::new())
    }

    pub fn get_recoveryCmd(&self) -> &BlockRecoveryCommandProto {
        self.recoveryCmd.as_ref().unwrap_or_else(|| BlockRecoveryCommandProto::default_instance())
    }

    fn get_recoveryCmd_for_reflect(&self) -> &::protobuf::SingularPtrField<BlockRecoveryCommandProto> {
        &self.recoveryCmd
    }

    fn mut_recoveryCmd_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BlockRecoveryCommandProto> {
        &mut self.recoveryCmd
    }

    // optional .hadoop.hdfs.datanode.FinalizeCommandProto finalizeCmd = 5;

    pub fn clear_finalizeCmd(&mut self) {
        self.finalizeCmd.clear();
    }

    pub fn has_finalizeCmd(&self) -> bool {
        self.finalizeCmd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_finalizeCmd(&mut self, v: FinalizeCommandProto) {
        self.finalizeCmd = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_finalizeCmd(&mut self) -> &mut FinalizeCommandProto {
        if self.finalizeCmd.is_none() {
            self.finalizeCmd.set_default();
        }
        self.finalizeCmd.as_mut().unwrap()
    }

    // Take field
    pub fn take_finalizeCmd(&mut self) -> FinalizeCommandProto {
        self.finalizeCmd.take().unwrap_or_else(|| FinalizeCommandProto::new())
    }

    pub fn get_finalizeCmd(&self) -> &FinalizeCommandProto {
        self.finalizeCmd.as_ref().unwrap_or_else(|| FinalizeCommandProto::default_instance())
    }

    fn get_finalizeCmd_for_reflect(&self) -> &::protobuf::SingularPtrField<FinalizeCommandProto> {
        &self.finalizeCmd
    }

    fn mut_finalizeCmd_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FinalizeCommandProto> {
        &mut self.finalizeCmd
    }

    // optional .hadoop.hdfs.datanode.KeyUpdateCommandProto keyUpdateCmd = 6;

    pub fn clear_keyUpdateCmd(&mut self) {
        self.keyUpdateCmd.clear();
    }

    pub fn has_keyUpdateCmd(&self) -> bool {
        self.keyUpdateCmd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyUpdateCmd(&mut self, v: KeyUpdateCommandProto) {
        self.keyUpdateCmd = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_keyUpdateCmd(&mut self) -> &mut KeyUpdateCommandProto {
        if self.keyUpdateCmd.is_none() {
            self.keyUpdateCmd.set_default();
        }
        self.keyUpdateCmd.as_mut().unwrap()
    }

    // Take field
    pub fn take_keyUpdateCmd(&mut self) -> KeyUpdateCommandProto {
        self.keyUpdateCmd.take().unwrap_or_else(|| KeyUpdateCommandProto::new())
    }

    pub fn get_keyUpdateCmd(&self) -> &KeyUpdateCommandProto {
        self.keyUpdateCmd.as_ref().unwrap_or_else(|| KeyUpdateCommandProto::default_instance())
    }

    fn get_keyUpdateCmd_for_reflect(&self) -> &::protobuf::SingularPtrField<KeyUpdateCommandProto> {
        &self.keyUpdateCmd
    }

    fn mut_keyUpdateCmd_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<KeyUpdateCommandProto> {
        &mut self.keyUpdateCmd
    }

    // optional .hadoop.hdfs.datanode.RegisterCommandProto registerCmd = 7;

    pub fn clear_registerCmd(&mut self) {
        self.registerCmd.clear();
    }

    pub fn has_registerCmd(&self) -> bool {
        self.registerCmd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_registerCmd(&mut self, v: RegisterCommandProto) {
        self.registerCmd = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_registerCmd(&mut self) -> &mut RegisterCommandProto {
        if self.registerCmd.is_none() {
            self.registerCmd.set_default();
        }
        self.registerCmd.as_mut().unwrap()
    }

    // Take field
    pub fn take_registerCmd(&mut self) -> RegisterCommandProto {
        self.registerCmd.take().unwrap_or_else(|| RegisterCommandProto::new())
    }

    pub fn get_registerCmd(&self) -> &RegisterCommandProto {
        self.registerCmd.as_ref().unwrap_or_else(|| RegisterCommandProto::default_instance())
    }

    fn get_registerCmd_for_reflect(&self) -> &::protobuf::SingularPtrField<RegisterCommandProto> {
        &self.registerCmd
    }

    fn mut_registerCmd_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RegisterCommandProto> {
        &mut self.registerCmd
    }

    // optional .hadoop.hdfs.datanode.BlockIdCommandProto blkIdCmd = 8;

    pub fn clear_blkIdCmd(&mut self) {
        self.blkIdCmd.clear();
    }

    pub fn has_blkIdCmd(&self) -> bool {
        self.blkIdCmd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blkIdCmd(&mut self, v: BlockIdCommandProto) {
        self.blkIdCmd = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blkIdCmd(&mut self) -> &mut BlockIdCommandProto {
        if self.blkIdCmd.is_none() {
            self.blkIdCmd.set_default();
        }
        self.blkIdCmd.as_mut().unwrap()
    }

    // Take field
    pub fn take_blkIdCmd(&mut self) -> BlockIdCommandProto {
        self.blkIdCmd.take().unwrap_or_else(|| BlockIdCommandProto::new())
    }

    pub fn get_blkIdCmd(&self) -> &BlockIdCommandProto {
        self.blkIdCmd.as_ref().unwrap_or_else(|| BlockIdCommandProto::default_instance())
    }

    fn get_blkIdCmd_for_reflect(&self) -> &::protobuf::SingularPtrField<BlockIdCommandProto> {
        &self.blkIdCmd
    }

    fn mut_blkIdCmd_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BlockIdCommandProto> {
        &mut self.blkIdCmd
    }
}

impl ::protobuf::Message for DatanodeCommandProto {
    fn is_initialized(&self) -> bool {
        if self.cmdType.is_none() {
            return false;
        }
        for v in &self.balancerCmd {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.blkCmd {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.recoveryCmd {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.finalizeCmd {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.keyUpdateCmd {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.registerCmd {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.blkIdCmd {
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
                    self.cmdType = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.balancerCmd)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.blkCmd)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.recoveryCmd)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.finalizeCmd)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.keyUpdateCmd)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.registerCmd)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.blkIdCmd)?;
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
        if let Some(v) = self.cmdType {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.balancerCmd.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.blkCmd.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.recoveryCmd.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.finalizeCmd.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.keyUpdateCmd.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.registerCmd.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.blkIdCmd.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cmdType {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.balancerCmd.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.blkCmd.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.recoveryCmd.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.finalizeCmd.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.keyUpdateCmd.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.registerCmd.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.blkIdCmd.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for DatanodeCommandProto {
    fn new() -> DatanodeCommandProto {
        DatanodeCommandProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DatanodeCommandProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DatanodeCommandProto_Type>>(
                    "cmdType",
                    DatanodeCommandProto::get_cmdType_for_reflect,
                    DatanodeCommandProto::mut_cmdType_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BalancerBandwidthCommandProto>>(
                    "balancerCmd",
                    DatanodeCommandProto::get_balancerCmd_for_reflect,
                    DatanodeCommandProto::mut_balancerCmd_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockCommandProto>>(
                    "blkCmd",
                    DatanodeCommandProto::get_blkCmd_for_reflect,
                    DatanodeCommandProto::mut_blkCmd_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockRecoveryCommandProto>>(
                    "recoveryCmd",
                    DatanodeCommandProto::get_recoveryCmd_for_reflect,
                    DatanodeCommandProto::mut_recoveryCmd_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FinalizeCommandProto>>(
                    "finalizeCmd",
                    DatanodeCommandProto::get_finalizeCmd_for_reflect,
                    DatanodeCommandProto::mut_finalizeCmd_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyUpdateCommandProto>>(
                    "keyUpdateCmd",
                    DatanodeCommandProto::get_keyUpdateCmd_for_reflect,
                    DatanodeCommandProto::mut_keyUpdateCmd_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RegisterCommandProto>>(
                    "registerCmd",
                    DatanodeCommandProto::get_registerCmd_for_reflect,
                    DatanodeCommandProto::mut_registerCmd_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockIdCommandProto>>(
                    "blkIdCmd",
                    DatanodeCommandProto::get_blkIdCmd_for_reflect,
                    DatanodeCommandProto::mut_blkIdCmd_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DatanodeCommandProto>(
                    "DatanodeCommandProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DatanodeCommandProto {
    fn clear(&mut self) {
        self.clear_cmdType();
        self.clear_balancerCmd();
        self.clear_blkCmd();
        self.clear_recoveryCmd();
        self.clear_finalizeCmd();
        self.clear_keyUpdateCmd();
        self.clear_registerCmd();
        self.clear_blkIdCmd();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DatanodeCommandProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DatanodeCommandProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DatanodeCommandProto_Type {
    BalancerBandwidthCommand = 0,
    BlockCommand = 1,
    BlockRecoveryCommand = 2,
    FinalizeCommand = 3,
    KeyUpdateCommand = 4,
    RegisterCommand = 5,
    UnusedUpgradeCommand = 6,
    NullDatanodeCommand = 7,
    BlockIdCommand = 8,
}

impl ::protobuf::ProtobufEnum for DatanodeCommandProto_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DatanodeCommandProto_Type> {
        match value {
            0 => ::std::option::Option::Some(DatanodeCommandProto_Type::BalancerBandwidthCommand),
            1 => ::std::option::Option::Some(DatanodeCommandProto_Type::BlockCommand),
            2 => ::std::option::Option::Some(DatanodeCommandProto_Type::BlockRecoveryCommand),
            3 => ::std::option::Option::Some(DatanodeCommandProto_Type::FinalizeCommand),
            4 => ::std::option::Option::Some(DatanodeCommandProto_Type::KeyUpdateCommand),
            5 => ::std::option::Option::Some(DatanodeCommandProto_Type::RegisterCommand),
            6 => ::std::option::Option::Some(DatanodeCommandProto_Type::UnusedUpgradeCommand),
            7 => ::std::option::Option::Some(DatanodeCommandProto_Type::NullDatanodeCommand),
            8 => ::std::option::Option::Some(DatanodeCommandProto_Type::BlockIdCommand),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DatanodeCommandProto_Type] = &[
            DatanodeCommandProto_Type::BalancerBandwidthCommand,
            DatanodeCommandProto_Type::BlockCommand,
            DatanodeCommandProto_Type::BlockRecoveryCommand,
            DatanodeCommandProto_Type::FinalizeCommand,
            DatanodeCommandProto_Type::KeyUpdateCommand,
            DatanodeCommandProto_Type::RegisterCommand,
            DatanodeCommandProto_Type::UnusedUpgradeCommand,
            DatanodeCommandProto_Type::NullDatanodeCommand,
            DatanodeCommandProto_Type::BlockIdCommand,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DatanodeCommandProto_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DatanodeCommandProto_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DatanodeCommandProto_Type {
}

impl ::protobuf::reflect::ProtobufValue for DatanodeCommandProto_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BalancerBandwidthCommandProto {
    // message fields
    bandwidth: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BalancerBandwidthCommandProto {}

impl BalancerBandwidthCommandProto {
    pub fn new() -> BalancerBandwidthCommandProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BalancerBandwidthCommandProto {
        static mut instance: ::protobuf::lazy::Lazy<BalancerBandwidthCommandProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BalancerBandwidthCommandProto,
        };
        unsafe {
            instance.get(BalancerBandwidthCommandProto::new)
        }
    }

    // required uint64 bandwidth = 1;

    pub fn clear_bandwidth(&mut self) {
        self.bandwidth = ::std::option::Option::None;
    }

    pub fn has_bandwidth(&self) -> bool {
        self.bandwidth.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bandwidth(&mut self, v: u64) {
        self.bandwidth = ::std::option::Option::Some(v);
    }

    pub fn get_bandwidth(&self) -> u64 {
        self.bandwidth.unwrap_or(0)
    }

    fn get_bandwidth_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.bandwidth
    }

    fn mut_bandwidth_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.bandwidth
    }
}

impl ::protobuf::Message for BalancerBandwidthCommandProto {
    fn is_initialized(&self) -> bool {
        if self.bandwidth.is_none() {
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
                    self.bandwidth = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.bandwidth {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bandwidth {
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

impl ::protobuf::MessageStatic for BalancerBandwidthCommandProto {
    fn new() -> BalancerBandwidthCommandProto {
        BalancerBandwidthCommandProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BalancerBandwidthCommandProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "bandwidth",
                    BalancerBandwidthCommandProto::get_bandwidth_for_reflect,
                    BalancerBandwidthCommandProto::mut_bandwidth_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BalancerBandwidthCommandProto>(
                    "BalancerBandwidthCommandProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BalancerBandwidthCommandProto {
    fn clear(&mut self) {
        self.clear_bandwidth();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BalancerBandwidthCommandProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BalancerBandwidthCommandProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockCommandProto {
    // message fields
    action: ::std::option::Option<BlockCommandProto_Action>,
    blockPoolId: ::protobuf::SingularField<::std::string::String>,
    blocks: ::protobuf::RepeatedField<super::hdfs::BlockProto>,
    targets: ::protobuf::RepeatedField<super::hdfs::DatanodeInfosProto>,
    targetStorageUuids: ::protobuf::RepeatedField<super::hdfs::StorageUuidsProto>,
    targetStorageTypes: ::protobuf::RepeatedField<super::hdfs::StorageTypesProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockCommandProto {}

impl BlockCommandProto {
    pub fn new() -> BlockCommandProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockCommandProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockCommandProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockCommandProto,
        };
        unsafe {
            instance.get(BlockCommandProto::new)
        }
    }

    // required .hadoop.hdfs.datanode.BlockCommandProto.Action action = 1;

    pub fn clear_action(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_action(&self) -> bool {
        self.action.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action(&mut self, v: BlockCommandProto_Action) {
        self.action = ::std::option::Option::Some(v);
    }

    pub fn get_action(&self) -> BlockCommandProto_Action {
        self.action.unwrap_or(BlockCommandProto_Action::TRANSFER)
    }

    fn get_action_for_reflect(&self) -> &::std::option::Option<BlockCommandProto_Action> {
        &self.action
    }

    fn mut_action_for_reflect(&mut self) -> &mut ::std::option::Option<BlockCommandProto_Action> {
        &mut self.action
    }

    // required string blockPoolId = 2;

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

    // repeated .hadoop.hdfs.BlockProto blocks = 3;

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

    // repeated .hadoop.hdfs.DatanodeInfosProto targets = 4;

    pub fn clear_targets(&mut self) {
        self.targets.clear();
    }

    // Param is passed by value, moved
    pub fn set_targets(&mut self, v: ::protobuf::RepeatedField<super::hdfs::DatanodeInfosProto>) {
        self.targets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targets(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::DatanodeInfosProto> {
        &mut self.targets
    }

    // Take field
    pub fn take_targets(&mut self) -> ::protobuf::RepeatedField<super::hdfs::DatanodeInfosProto> {
        ::std::mem::replace(&mut self.targets, ::protobuf::RepeatedField::new())
    }

    pub fn get_targets(&self) -> &[super::hdfs::DatanodeInfosProto] {
        &self.targets
    }

    fn get_targets_for_reflect(&self) -> &::protobuf::RepeatedField<super::hdfs::DatanodeInfosProto> {
        &self.targets
    }

    fn mut_targets_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::DatanodeInfosProto> {
        &mut self.targets
    }

    // repeated .hadoop.hdfs.StorageUuidsProto targetStorageUuids = 5;

    pub fn clear_targetStorageUuids(&mut self) {
        self.targetStorageUuids.clear();
    }

    // Param is passed by value, moved
    pub fn set_targetStorageUuids(&mut self, v: ::protobuf::RepeatedField<super::hdfs::StorageUuidsProto>) {
        self.targetStorageUuids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targetStorageUuids(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::StorageUuidsProto> {
        &mut self.targetStorageUuids
    }

    // Take field
    pub fn take_targetStorageUuids(&mut self) -> ::protobuf::RepeatedField<super::hdfs::StorageUuidsProto> {
        ::std::mem::replace(&mut self.targetStorageUuids, ::protobuf::RepeatedField::new())
    }

    pub fn get_targetStorageUuids(&self) -> &[super::hdfs::StorageUuidsProto] {
        &self.targetStorageUuids
    }

    fn get_targetStorageUuids_for_reflect(&self) -> &::protobuf::RepeatedField<super::hdfs::StorageUuidsProto> {
        &self.targetStorageUuids
    }

    fn mut_targetStorageUuids_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::StorageUuidsProto> {
        &mut self.targetStorageUuids
    }

    // repeated .hadoop.hdfs.StorageTypesProto targetStorageTypes = 6;

    pub fn clear_targetStorageTypes(&mut self) {
        self.targetStorageTypes.clear();
    }

    // Param is passed by value, moved
    pub fn set_targetStorageTypes(&mut self, v: ::protobuf::RepeatedField<super::hdfs::StorageTypesProto>) {
        self.targetStorageTypes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targetStorageTypes(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::StorageTypesProto> {
        &mut self.targetStorageTypes
    }

    // Take field
    pub fn take_targetStorageTypes(&mut self) -> ::protobuf::RepeatedField<super::hdfs::StorageTypesProto> {
        ::std::mem::replace(&mut self.targetStorageTypes, ::protobuf::RepeatedField::new())
    }

    pub fn get_targetStorageTypes(&self) -> &[super::hdfs::StorageTypesProto] {
        &self.targetStorageTypes
    }

    fn get_targetStorageTypes_for_reflect(&self) -> &::protobuf::RepeatedField<super::hdfs::StorageTypesProto> {
        &self.targetStorageTypes
    }

    fn mut_targetStorageTypes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::StorageTypesProto> {
        &mut self.targetStorageTypes
    }
}

impl ::protobuf::Message for BlockCommandProto {
    fn is_initialized(&self) -> bool {
        if self.action.is_none() {
            return false;
        }
        if self.blockPoolId.is_none() {
            return false;
        }
        for v in &self.blocks {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.targets {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.targetStorageUuids {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.targetStorageTypes {
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
                    self.action = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.blockPoolId)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.blocks)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.targets)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.targetStorageUuids)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.targetStorageTypes)?;
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
        if let Some(v) = self.action {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.blockPoolId.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        for value in &self.blocks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.targets {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.targetStorageUuids {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.targetStorageTypes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.action {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.blockPoolId.as_ref() {
            os.write_string(2, &v)?;
        }
        for v in &self.blocks {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.targets {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.targetStorageUuids {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.targetStorageTypes {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for BlockCommandProto {
    fn new() -> BlockCommandProto {
        BlockCommandProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockCommandProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<BlockCommandProto_Action>>(
                    "action",
                    BlockCommandProto::get_action_for_reflect,
                    BlockCommandProto::mut_action_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "blockPoolId",
                    BlockCommandProto::get_blockPoolId_for_reflect,
                    BlockCommandProto::mut_blockPoolId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::BlockProto>>(
                    "blocks",
                    BlockCommandProto::get_blocks_for_reflect,
                    BlockCommandProto::mut_blocks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeInfosProto>>(
                    "targets",
                    BlockCommandProto::get_targets_for_reflect,
                    BlockCommandProto::mut_targets_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::StorageUuidsProto>>(
                    "targetStorageUuids",
                    BlockCommandProto::get_targetStorageUuids_for_reflect,
                    BlockCommandProto::mut_targetStorageUuids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::StorageTypesProto>>(
                    "targetStorageTypes",
                    BlockCommandProto::get_targetStorageTypes_for_reflect,
                    BlockCommandProto::mut_targetStorageTypes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockCommandProto>(
                    "BlockCommandProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockCommandProto {
    fn clear(&mut self) {
        self.clear_action();
        self.clear_blockPoolId();
        self.clear_blocks();
        self.clear_targets();
        self.clear_targetStorageUuids();
        self.clear_targetStorageTypes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockCommandProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockCommandProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum BlockCommandProto_Action {
    TRANSFER = 1,
    INVALIDATE = 2,
    SHUTDOWN = 3,
}

impl ::protobuf::ProtobufEnum for BlockCommandProto_Action {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<BlockCommandProto_Action> {
        match value {
            1 => ::std::option::Option::Some(BlockCommandProto_Action::TRANSFER),
            2 => ::std::option::Option::Some(BlockCommandProto_Action::INVALIDATE),
            3 => ::std::option::Option::Some(BlockCommandProto_Action::SHUTDOWN),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [BlockCommandProto_Action] = &[
            BlockCommandProto_Action::TRANSFER,
            BlockCommandProto_Action::INVALIDATE,
            BlockCommandProto_Action::SHUTDOWN,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<BlockCommandProto_Action>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("BlockCommandProto_Action", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for BlockCommandProto_Action {
}

impl ::protobuf::reflect::ProtobufValue for BlockCommandProto_Action {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockIdCommandProto {
    // message fields
    action: ::std::option::Option<BlockIdCommandProto_Action>,
    blockPoolId: ::protobuf::SingularField<::std::string::String>,
    blockIds: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockIdCommandProto {}

impl BlockIdCommandProto {
    pub fn new() -> BlockIdCommandProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockIdCommandProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockIdCommandProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockIdCommandProto,
        };
        unsafe {
            instance.get(BlockIdCommandProto::new)
        }
    }

    // required .hadoop.hdfs.datanode.BlockIdCommandProto.Action action = 1;

    pub fn clear_action(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_action(&self) -> bool {
        self.action.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action(&mut self, v: BlockIdCommandProto_Action) {
        self.action = ::std::option::Option::Some(v);
    }

    pub fn get_action(&self) -> BlockIdCommandProto_Action {
        self.action.unwrap_or(BlockIdCommandProto_Action::CACHE)
    }

    fn get_action_for_reflect(&self) -> &::std::option::Option<BlockIdCommandProto_Action> {
        &self.action
    }

    fn mut_action_for_reflect(&mut self) -> &mut ::std::option::Option<BlockIdCommandProto_Action> {
        &mut self.action
    }

    // required string blockPoolId = 2;

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

    // repeated uint64 blockIds = 3;

    pub fn clear_blockIds(&mut self) {
        self.blockIds.clear();
    }

    // Param is passed by value, moved
    pub fn set_blockIds(&mut self, v: ::std::vec::Vec<u64>) {
        self.blockIds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blockIds(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.blockIds
    }

    // Take field
    pub fn take_blockIds(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.blockIds, ::std::vec::Vec::new())
    }

    pub fn get_blockIds(&self) -> &[u64] {
        &self.blockIds
    }

    fn get_blockIds_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.blockIds
    }

    fn mut_blockIds_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.blockIds
    }
}

impl ::protobuf::Message for BlockIdCommandProto {
    fn is_initialized(&self) -> bool {
        if self.action.is_none() {
            return false;
        }
        if self.blockPoolId.is_none() {
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
                    self.action = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.blockPoolId)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.blockIds)?;
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
        if let Some(v) = self.action {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.blockPoolId.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if !self.blockIds.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(3, &self.blockIds);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.action {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.blockPoolId.as_ref() {
            os.write_string(2, &v)?;
        }
        if !self.blockIds.is_empty() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.blockIds))?;
            for v in &self.blockIds {
                os.write_uint64_no_tag(*v)?;
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

impl ::protobuf::MessageStatic for BlockIdCommandProto {
    fn new() -> BlockIdCommandProto {
        BlockIdCommandProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockIdCommandProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<BlockIdCommandProto_Action>>(
                    "action",
                    BlockIdCommandProto::get_action_for_reflect,
                    BlockIdCommandProto::mut_action_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "blockPoolId",
                    BlockIdCommandProto::get_blockPoolId_for_reflect,
                    BlockIdCommandProto::mut_blockPoolId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "blockIds",
                    BlockIdCommandProto::get_blockIds_for_reflect,
                    BlockIdCommandProto::mut_blockIds_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockIdCommandProto>(
                    "BlockIdCommandProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockIdCommandProto {
    fn clear(&mut self) {
        self.clear_action();
        self.clear_blockPoolId();
        self.clear_blockIds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockIdCommandProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockIdCommandProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum BlockIdCommandProto_Action {
    CACHE = 1,
    UNCACHE = 2,
}

impl ::protobuf::ProtobufEnum for BlockIdCommandProto_Action {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<BlockIdCommandProto_Action> {
        match value {
            1 => ::std::option::Option::Some(BlockIdCommandProto_Action::CACHE),
            2 => ::std::option::Option::Some(BlockIdCommandProto_Action::UNCACHE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [BlockIdCommandProto_Action] = &[
            BlockIdCommandProto_Action::CACHE,
            BlockIdCommandProto_Action::UNCACHE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<BlockIdCommandProto_Action>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("BlockIdCommandProto_Action", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for BlockIdCommandProto_Action {
}

impl ::protobuf::reflect::ProtobufValue for BlockIdCommandProto_Action {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockRecoveryCommandProto {
    // message fields
    blocks: ::protobuf::RepeatedField<super::hdfs::RecoveringBlockProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockRecoveryCommandProto {}

impl BlockRecoveryCommandProto {
    pub fn new() -> BlockRecoveryCommandProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockRecoveryCommandProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockRecoveryCommandProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockRecoveryCommandProto,
        };
        unsafe {
            instance.get(BlockRecoveryCommandProto::new)
        }
    }

    // repeated .hadoop.hdfs.RecoveringBlockProto blocks = 1;

    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    // Param is passed by value, moved
    pub fn set_blocks(&mut self, v: ::protobuf::RepeatedField<super::hdfs::RecoveringBlockProto>) {
        self.blocks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blocks(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::RecoveringBlockProto> {
        &mut self.blocks
    }

    // Take field
    pub fn take_blocks(&mut self) -> ::protobuf::RepeatedField<super::hdfs::RecoveringBlockProto> {
        ::std::mem::replace(&mut self.blocks, ::protobuf::RepeatedField::new())
    }

    pub fn get_blocks(&self) -> &[super::hdfs::RecoveringBlockProto] {
        &self.blocks
    }

    fn get_blocks_for_reflect(&self) -> &::protobuf::RepeatedField<super::hdfs::RecoveringBlockProto> {
        &self.blocks
    }

    fn mut_blocks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::RecoveringBlockProto> {
        &mut self.blocks
    }
}

impl ::protobuf::Message for BlockRecoveryCommandProto {
    fn is_initialized(&self) -> bool {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.blocks)?;
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
        for value in &self.blocks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.blocks {
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

impl ::protobuf::MessageStatic for BlockRecoveryCommandProto {
    fn new() -> BlockRecoveryCommandProto {
        BlockRecoveryCommandProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockRecoveryCommandProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::RecoveringBlockProto>>(
                    "blocks",
                    BlockRecoveryCommandProto::get_blocks_for_reflect,
                    BlockRecoveryCommandProto::mut_blocks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockRecoveryCommandProto>(
                    "BlockRecoveryCommandProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockRecoveryCommandProto {
    fn clear(&mut self) {
        self.clear_blocks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockRecoveryCommandProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockRecoveryCommandProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FinalizeCommandProto {
    // message fields
    blockPoolId: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FinalizeCommandProto {}

impl FinalizeCommandProto {
    pub fn new() -> FinalizeCommandProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FinalizeCommandProto {
        static mut instance: ::protobuf::lazy::Lazy<FinalizeCommandProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FinalizeCommandProto,
        };
        unsafe {
            instance.get(FinalizeCommandProto::new)
        }
    }

    // required string blockPoolId = 1;

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
}

impl ::protobuf::Message for FinalizeCommandProto {
    fn is_initialized(&self) -> bool {
        if self.blockPoolId.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.blockPoolId)?;
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
        if let Some(ref v) = self.blockPoolId.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.blockPoolId.as_ref() {
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

impl ::protobuf::MessageStatic for FinalizeCommandProto {
    fn new() -> FinalizeCommandProto {
        FinalizeCommandProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FinalizeCommandProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "blockPoolId",
                    FinalizeCommandProto::get_blockPoolId_for_reflect,
                    FinalizeCommandProto::mut_blockPoolId_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FinalizeCommandProto>(
                    "FinalizeCommandProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FinalizeCommandProto {
    fn clear(&mut self) {
        self.clear_blockPoolId();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FinalizeCommandProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FinalizeCommandProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct KeyUpdateCommandProto {
    // message fields
    keys: ::protobuf::SingularPtrField<super::hdfs::ExportedBlockKeysProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KeyUpdateCommandProto {}

impl KeyUpdateCommandProto {
    pub fn new() -> KeyUpdateCommandProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KeyUpdateCommandProto {
        static mut instance: ::protobuf::lazy::Lazy<KeyUpdateCommandProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KeyUpdateCommandProto,
        };
        unsafe {
            instance.get(KeyUpdateCommandProto::new)
        }
    }

    // required .hadoop.hdfs.ExportedBlockKeysProto keys = 1;

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

impl ::protobuf::Message for KeyUpdateCommandProto {
    fn is_initialized(&self) -> bool {
        if self.keys.is_none() {
            return false;
        }
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

impl ::protobuf::MessageStatic for KeyUpdateCommandProto {
    fn new() -> KeyUpdateCommandProto {
        KeyUpdateCommandProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<KeyUpdateCommandProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::ExportedBlockKeysProto>>(
                    "keys",
                    KeyUpdateCommandProto::get_keys_for_reflect,
                    KeyUpdateCommandProto::mut_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KeyUpdateCommandProto>(
                    "KeyUpdateCommandProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KeyUpdateCommandProto {
    fn clear(&mut self) {
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KeyUpdateCommandProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KeyUpdateCommandProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegisterCommandProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegisterCommandProto {}

impl RegisterCommandProto {
    pub fn new() -> RegisterCommandProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegisterCommandProto {
        static mut instance: ::protobuf::lazy::Lazy<RegisterCommandProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegisterCommandProto,
        };
        unsafe {
            instance.get(RegisterCommandProto::new)
        }
    }
}

impl ::protobuf::Message for RegisterCommandProto {
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

impl ::protobuf::MessageStatic for RegisterCommandProto {
    fn new() -> RegisterCommandProto {
        RegisterCommandProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegisterCommandProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RegisterCommandProto>(
                    "RegisterCommandProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegisterCommandProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegisterCommandProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegisterCommandProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegisterDatanodeRequestProto {
    // message fields
    registration: ::protobuf::SingularPtrField<DatanodeRegistrationProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegisterDatanodeRequestProto {}

impl RegisterDatanodeRequestProto {
    pub fn new() -> RegisterDatanodeRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegisterDatanodeRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RegisterDatanodeRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegisterDatanodeRequestProto,
        };
        unsafe {
            instance.get(RegisterDatanodeRequestProto::new)
        }
    }

    // required .hadoop.hdfs.datanode.DatanodeRegistrationProto registration = 1;

    pub fn clear_registration(&mut self) {
        self.registration.clear();
    }

    pub fn has_registration(&self) -> bool {
        self.registration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_registration(&mut self, v: DatanodeRegistrationProto) {
        self.registration = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_registration(&mut self) -> &mut DatanodeRegistrationProto {
        if self.registration.is_none() {
            self.registration.set_default();
        }
        self.registration.as_mut().unwrap()
    }

    // Take field
    pub fn take_registration(&mut self) -> DatanodeRegistrationProto {
        self.registration.take().unwrap_or_else(|| DatanodeRegistrationProto::new())
    }

    pub fn get_registration(&self) -> &DatanodeRegistrationProto {
        self.registration.as_ref().unwrap_or_else(|| DatanodeRegistrationProto::default_instance())
    }

    fn get_registration_for_reflect(&self) -> &::protobuf::SingularPtrField<DatanodeRegistrationProto> {
        &self.registration
    }

    fn mut_registration_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DatanodeRegistrationProto> {
        &mut self.registration
    }
}

impl ::protobuf::Message for RegisterDatanodeRequestProto {
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

impl ::protobuf::MessageStatic for RegisterDatanodeRequestProto {
    fn new() -> RegisterDatanodeRequestProto {
        RegisterDatanodeRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegisterDatanodeRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DatanodeRegistrationProto>>(
                    "registration",
                    RegisterDatanodeRequestProto::get_registration_for_reflect,
                    RegisterDatanodeRequestProto::mut_registration_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegisterDatanodeRequestProto>(
                    "RegisterDatanodeRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegisterDatanodeRequestProto {
    fn clear(&mut self) {
        self.clear_registration();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegisterDatanodeRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegisterDatanodeRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegisterDatanodeResponseProto {
    // message fields
    registration: ::protobuf::SingularPtrField<DatanodeRegistrationProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegisterDatanodeResponseProto {}

impl RegisterDatanodeResponseProto {
    pub fn new() -> RegisterDatanodeResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegisterDatanodeResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RegisterDatanodeResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegisterDatanodeResponseProto,
        };
        unsafe {
            instance.get(RegisterDatanodeResponseProto::new)
        }
    }

    // required .hadoop.hdfs.datanode.DatanodeRegistrationProto registration = 1;

    pub fn clear_registration(&mut self) {
        self.registration.clear();
    }

    pub fn has_registration(&self) -> bool {
        self.registration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_registration(&mut self, v: DatanodeRegistrationProto) {
        self.registration = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_registration(&mut self) -> &mut DatanodeRegistrationProto {
        if self.registration.is_none() {
            self.registration.set_default();
        }
        self.registration.as_mut().unwrap()
    }

    // Take field
    pub fn take_registration(&mut self) -> DatanodeRegistrationProto {
        self.registration.take().unwrap_or_else(|| DatanodeRegistrationProto::new())
    }

    pub fn get_registration(&self) -> &DatanodeRegistrationProto {
        self.registration.as_ref().unwrap_or_else(|| DatanodeRegistrationProto::default_instance())
    }

    fn get_registration_for_reflect(&self) -> &::protobuf::SingularPtrField<DatanodeRegistrationProto> {
        &self.registration
    }

    fn mut_registration_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DatanodeRegistrationProto> {
        &mut self.registration
    }
}

impl ::protobuf::Message for RegisterDatanodeResponseProto {
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

impl ::protobuf::MessageStatic for RegisterDatanodeResponseProto {
    fn new() -> RegisterDatanodeResponseProto {
        RegisterDatanodeResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegisterDatanodeResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DatanodeRegistrationProto>>(
                    "registration",
                    RegisterDatanodeResponseProto::get_registration_for_reflect,
                    RegisterDatanodeResponseProto::mut_registration_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegisterDatanodeResponseProto>(
                    "RegisterDatanodeResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegisterDatanodeResponseProto {
    fn clear(&mut self) {
        self.clear_registration();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegisterDatanodeResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegisterDatanodeResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HeartbeatRequestProto {
    // message fields
    registration: ::protobuf::SingularPtrField<DatanodeRegistrationProto>,
    reports: ::protobuf::RepeatedField<super::hdfs::StorageReportProto>,
    xmitsInProgress: ::std::option::Option<u32>,
    xceiverCount: ::std::option::Option<u32>,
    failedVolumes: ::std::option::Option<u32>,
    cacheCapacity: ::std::option::Option<u64>,
    cacheUsed: ::std::option::Option<u64>,
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

    // required .hadoop.hdfs.datanode.DatanodeRegistrationProto registration = 1;

    pub fn clear_registration(&mut self) {
        self.registration.clear();
    }

    pub fn has_registration(&self) -> bool {
        self.registration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_registration(&mut self, v: DatanodeRegistrationProto) {
        self.registration = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_registration(&mut self) -> &mut DatanodeRegistrationProto {
        if self.registration.is_none() {
            self.registration.set_default();
        }
        self.registration.as_mut().unwrap()
    }

    // Take field
    pub fn take_registration(&mut self) -> DatanodeRegistrationProto {
        self.registration.take().unwrap_or_else(|| DatanodeRegistrationProto::new())
    }

    pub fn get_registration(&self) -> &DatanodeRegistrationProto {
        self.registration.as_ref().unwrap_or_else(|| DatanodeRegistrationProto::default_instance())
    }

    fn get_registration_for_reflect(&self) -> &::protobuf::SingularPtrField<DatanodeRegistrationProto> {
        &self.registration
    }

    fn mut_registration_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DatanodeRegistrationProto> {
        &mut self.registration
    }

    // repeated .hadoop.hdfs.StorageReportProto reports = 2;

    pub fn clear_reports(&mut self) {
        self.reports.clear();
    }

    // Param is passed by value, moved
    pub fn set_reports(&mut self, v: ::protobuf::RepeatedField<super::hdfs::StorageReportProto>) {
        self.reports = v;
    }

    // Mutable pointer to the field.
    pub fn mut_reports(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::StorageReportProto> {
        &mut self.reports
    }

    // Take field
    pub fn take_reports(&mut self) -> ::protobuf::RepeatedField<super::hdfs::StorageReportProto> {
        ::std::mem::replace(&mut self.reports, ::protobuf::RepeatedField::new())
    }

    pub fn get_reports(&self) -> &[super::hdfs::StorageReportProto] {
        &self.reports
    }

    fn get_reports_for_reflect(&self) -> &::protobuf::RepeatedField<super::hdfs::StorageReportProto> {
        &self.reports
    }

    fn mut_reports_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::StorageReportProto> {
        &mut self.reports
    }

    // optional uint32 xmitsInProgress = 3;

    pub fn clear_xmitsInProgress(&mut self) {
        self.xmitsInProgress = ::std::option::Option::None;
    }

    pub fn has_xmitsInProgress(&self) -> bool {
        self.xmitsInProgress.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xmitsInProgress(&mut self, v: u32) {
        self.xmitsInProgress = ::std::option::Option::Some(v);
    }

    pub fn get_xmitsInProgress(&self) -> u32 {
        self.xmitsInProgress.unwrap_or(0u32)
    }

    fn get_xmitsInProgress_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.xmitsInProgress
    }

    fn mut_xmitsInProgress_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.xmitsInProgress
    }

    // optional uint32 xceiverCount = 4;

    pub fn clear_xceiverCount(&mut self) {
        self.xceiverCount = ::std::option::Option::None;
    }

    pub fn has_xceiverCount(&self) -> bool {
        self.xceiverCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xceiverCount(&mut self, v: u32) {
        self.xceiverCount = ::std::option::Option::Some(v);
    }

    pub fn get_xceiverCount(&self) -> u32 {
        self.xceiverCount.unwrap_or(0u32)
    }

    fn get_xceiverCount_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.xceiverCount
    }

    fn mut_xceiverCount_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.xceiverCount
    }

    // optional uint32 failedVolumes = 5;

    pub fn clear_failedVolumes(&mut self) {
        self.failedVolumes = ::std::option::Option::None;
    }

    pub fn has_failedVolumes(&self) -> bool {
        self.failedVolumes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_failedVolumes(&mut self, v: u32) {
        self.failedVolumes = ::std::option::Option::Some(v);
    }

    pub fn get_failedVolumes(&self) -> u32 {
        self.failedVolumes.unwrap_or(0u32)
    }

    fn get_failedVolumes_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.failedVolumes
    }

    fn mut_failedVolumes_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.failedVolumes
    }

    // optional uint64 cacheCapacity = 6;

    pub fn clear_cacheCapacity(&mut self) {
        self.cacheCapacity = ::std::option::Option::None;
    }

    pub fn has_cacheCapacity(&self) -> bool {
        self.cacheCapacity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cacheCapacity(&mut self, v: u64) {
        self.cacheCapacity = ::std::option::Option::Some(v);
    }

    pub fn get_cacheCapacity(&self) -> u64 {
        self.cacheCapacity.unwrap_or(0u64)
    }

    fn get_cacheCapacity_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.cacheCapacity
    }

    fn mut_cacheCapacity_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.cacheCapacity
    }

    // optional uint64 cacheUsed = 7;

    pub fn clear_cacheUsed(&mut self) {
        self.cacheUsed = ::std::option::Option::None;
    }

    pub fn has_cacheUsed(&self) -> bool {
        self.cacheUsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cacheUsed(&mut self, v: u64) {
        self.cacheUsed = ::std::option::Option::Some(v);
    }

    pub fn get_cacheUsed(&self) -> u64 {
        self.cacheUsed.unwrap_or(0u64)
    }

    fn get_cacheUsed_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.cacheUsed
    }

    fn mut_cacheUsed_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.cacheUsed
    }
}

impl ::protobuf::Message for HeartbeatRequestProto {
    fn is_initialized(&self) -> bool {
        if self.registration.is_none() {
            return false;
        }
        for v in &self.registration {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.reports {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.reports)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.xmitsInProgress = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.xceiverCount = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.failedVolumes = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.cacheCapacity = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.cacheUsed = ::std::option::Option::Some(tmp);
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
        for value in &self.reports {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.xmitsInProgress {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.xceiverCount {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.failedVolumes {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.cacheCapacity {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.cacheUsed {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
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
        for v in &self.reports {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.xmitsInProgress {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.xceiverCount {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.failedVolumes {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.cacheCapacity {
            os.write_uint64(6, v)?;
        }
        if let Some(v) = self.cacheUsed {
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DatanodeRegistrationProto>>(
                    "registration",
                    HeartbeatRequestProto::get_registration_for_reflect,
                    HeartbeatRequestProto::mut_registration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::StorageReportProto>>(
                    "reports",
                    HeartbeatRequestProto::get_reports_for_reflect,
                    HeartbeatRequestProto::mut_reports_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "xmitsInProgress",
                    HeartbeatRequestProto::get_xmitsInProgress_for_reflect,
                    HeartbeatRequestProto::mut_xmitsInProgress_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "xceiverCount",
                    HeartbeatRequestProto::get_xceiverCount_for_reflect,
                    HeartbeatRequestProto::mut_xceiverCount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "failedVolumes",
                    HeartbeatRequestProto::get_failedVolumes_for_reflect,
                    HeartbeatRequestProto::mut_failedVolumes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "cacheCapacity",
                    HeartbeatRequestProto::get_cacheCapacity_for_reflect,
                    HeartbeatRequestProto::mut_cacheCapacity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "cacheUsed",
                    HeartbeatRequestProto::get_cacheUsed_for_reflect,
                    HeartbeatRequestProto::mut_cacheUsed_for_reflect,
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
        self.clear_registration();
        self.clear_reports();
        self.clear_xmitsInProgress();
        self.clear_xceiverCount();
        self.clear_failedVolumes();
        self.clear_cacheCapacity();
        self.clear_cacheUsed();
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
pub struct NNHAStatusHeartbeatProto {
    // message fields
    state: ::std::option::Option<NNHAStatusHeartbeatProto_State>,
    txid: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NNHAStatusHeartbeatProto {}

impl NNHAStatusHeartbeatProto {
    pub fn new() -> NNHAStatusHeartbeatProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NNHAStatusHeartbeatProto {
        static mut instance: ::protobuf::lazy::Lazy<NNHAStatusHeartbeatProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NNHAStatusHeartbeatProto,
        };
        unsafe {
            instance.get(NNHAStatusHeartbeatProto::new)
        }
    }

    // required .hadoop.hdfs.datanode.NNHAStatusHeartbeatProto.State state = 1;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: NNHAStatusHeartbeatProto_State) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state(&self) -> NNHAStatusHeartbeatProto_State {
        self.state.unwrap_or(NNHAStatusHeartbeatProto_State::ACTIVE)
    }

    fn get_state_for_reflect(&self) -> &::std::option::Option<NNHAStatusHeartbeatProto_State> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::std::option::Option<NNHAStatusHeartbeatProto_State> {
        &mut self.state
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
}

impl ::protobuf::Message for NNHAStatusHeartbeatProto {
    fn is_initialized(&self) -> bool {
        if self.state.is_none() {
            return false;
        }
        if self.txid.is_none() {
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
                    let tmp = is.read_uint64()?;
                    self.txid = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.txid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.state {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.txid {
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

impl ::protobuf::MessageStatic for NNHAStatusHeartbeatProto {
    fn new() -> NNHAStatusHeartbeatProto {
        NNHAStatusHeartbeatProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<NNHAStatusHeartbeatProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<NNHAStatusHeartbeatProto_State>>(
                    "state",
                    NNHAStatusHeartbeatProto::get_state_for_reflect,
                    NNHAStatusHeartbeatProto::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "txid",
                    NNHAStatusHeartbeatProto::get_txid_for_reflect,
                    NNHAStatusHeartbeatProto::mut_txid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NNHAStatusHeartbeatProto>(
                    "NNHAStatusHeartbeatProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NNHAStatusHeartbeatProto {
    fn clear(&mut self) {
        self.clear_state();
        self.clear_txid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NNHAStatusHeartbeatProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NNHAStatusHeartbeatProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum NNHAStatusHeartbeatProto_State {
    ACTIVE = 0,
    STANDBY = 1,
}

impl ::protobuf::ProtobufEnum for NNHAStatusHeartbeatProto_State {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NNHAStatusHeartbeatProto_State> {
        match value {
            0 => ::std::option::Option::Some(NNHAStatusHeartbeatProto_State::ACTIVE),
            1 => ::std::option::Option::Some(NNHAStatusHeartbeatProto_State::STANDBY),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [NNHAStatusHeartbeatProto_State] = &[
            NNHAStatusHeartbeatProto_State::ACTIVE,
            NNHAStatusHeartbeatProto_State::STANDBY,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<NNHAStatusHeartbeatProto_State>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("NNHAStatusHeartbeatProto_State", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for NNHAStatusHeartbeatProto_State {
}

impl ::protobuf::reflect::ProtobufValue for NNHAStatusHeartbeatProto_State {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HeartbeatResponseProto {
    // message fields
    cmds: ::protobuf::RepeatedField<DatanodeCommandProto>,
    haStatus: ::protobuf::SingularPtrField<NNHAStatusHeartbeatProto>,
    rollingUpgradeStatus: ::protobuf::SingularPtrField<super::hdfs::RollingUpgradeStatusProto>,
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

    // repeated .hadoop.hdfs.datanode.DatanodeCommandProto cmds = 1;

    pub fn clear_cmds(&mut self) {
        self.cmds.clear();
    }

    // Param is passed by value, moved
    pub fn set_cmds(&mut self, v: ::protobuf::RepeatedField<DatanodeCommandProto>) {
        self.cmds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cmds(&mut self) -> &mut ::protobuf::RepeatedField<DatanodeCommandProto> {
        &mut self.cmds
    }

    // Take field
    pub fn take_cmds(&mut self) -> ::protobuf::RepeatedField<DatanodeCommandProto> {
        ::std::mem::replace(&mut self.cmds, ::protobuf::RepeatedField::new())
    }

    pub fn get_cmds(&self) -> &[DatanodeCommandProto] {
        &self.cmds
    }

    fn get_cmds_for_reflect(&self) -> &::protobuf::RepeatedField<DatanodeCommandProto> {
        &self.cmds
    }

    fn mut_cmds_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DatanodeCommandProto> {
        &mut self.cmds
    }

    // required .hadoop.hdfs.datanode.NNHAStatusHeartbeatProto haStatus = 2;

    pub fn clear_haStatus(&mut self) {
        self.haStatus.clear();
    }

    pub fn has_haStatus(&self) -> bool {
        self.haStatus.is_some()
    }

    // Param is passed by value, moved
    pub fn set_haStatus(&mut self, v: NNHAStatusHeartbeatProto) {
        self.haStatus = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_haStatus(&mut self) -> &mut NNHAStatusHeartbeatProto {
        if self.haStatus.is_none() {
            self.haStatus.set_default();
        }
        self.haStatus.as_mut().unwrap()
    }

    // Take field
    pub fn take_haStatus(&mut self) -> NNHAStatusHeartbeatProto {
        self.haStatus.take().unwrap_or_else(|| NNHAStatusHeartbeatProto::new())
    }

    pub fn get_haStatus(&self) -> &NNHAStatusHeartbeatProto {
        self.haStatus.as_ref().unwrap_or_else(|| NNHAStatusHeartbeatProto::default_instance())
    }

    fn get_haStatus_for_reflect(&self) -> &::protobuf::SingularPtrField<NNHAStatusHeartbeatProto> {
        &self.haStatus
    }

    fn mut_haStatus_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<NNHAStatusHeartbeatProto> {
        &mut self.haStatus
    }

    // optional .hadoop.hdfs.RollingUpgradeStatusProto rollingUpgradeStatus = 3;

    pub fn clear_rollingUpgradeStatus(&mut self) {
        self.rollingUpgradeStatus.clear();
    }

    pub fn has_rollingUpgradeStatus(&self) -> bool {
        self.rollingUpgradeStatus.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rollingUpgradeStatus(&mut self, v: super::hdfs::RollingUpgradeStatusProto) {
        self.rollingUpgradeStatus = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rollingUpgradeStatus(&mut self) -> &mut super::hdfs::RollingUpgradeStatusProto {
        if self.rollingUpgradeStatus.is_none() {
            self.rollingUpgradeStatus.set_default();
        }
        self.rollingUpgradeStatus.as_mut().unwrap()
    }

    // Take field
    pub fn take_rollingUpgradeStatus(&mut self) -> super::hdfs::RollingUpgradeStatusProto {
        self.rollingUpgradeStatus.take().unwrap_or_else(|| super::hdfs::RollingUpgradeStatusProto::new())
    }

    pub fn get_rollingUpgradeStatus(&self) -> &super::hdfs::RollingUpgradeStatusProto {
        self.rollingUpgradeStatus.as_ref().unwrap_or_else(|| super::hdfs::RollingUpgradeStatusProto::default_instance())
    }

    fn get_rollingUpgradeStatus_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::RollingUpgradeStatusProto> {
        &self.rollingUpgradeStatus
    }

    fn mut_rollingUpgradeStatus_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::RollingUpgradeStatusProto> {
        &mut self.rollingUpgradeStatus
    }
}

impl ::protobuf::Message for HeartbeatResponseProto {
    fn is_initialized(&self) -> bool {
        if self.haStatus.is_none() {
            return false;
        }
        for v in &self.cmds {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.haStatus {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.rollingUpgradeStatus {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cmds)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.haStatus)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rollingUpgradeStatus)?;
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
        for value in &self.cmds {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.haStatus.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.rollingUpgradeStatus.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.cmds {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.haStatus.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.rollingUpgradeStatus.as_ref() {
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
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DatanodeCommandProto>>(
                    "cmds",
                    HeartbeatResponseProto::get_cmds_for_reflect,
                    HeartbeatResponseProto::mut_cmds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<NNHAStatusHeartbeatProto>>(
                    "haStatus",
                    HeartbeatResponseProto::get_haStatus_for_reflect,
                    HeartbeatResponseProto::mut_haStatus_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::RollingUpgradeStatusProto>>(
                    "rollingUpgradeStatus",
                    HeartbeatResponseProto::get_rollingUpgradeStatus_for_reflect,
                    HeartbeatResponseProto::mut_rollingUpgradeStatus_for_reflect,
                ));
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
        self.clear_cmds();
        self.clear_haStatus();
        self.clear_rollingUpgradeStatus();
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
pub struct BlockReportRequestProto {
    // message fields
    registration: ::protobuf::SingularPtrField<DatanodeRegistrationProto>,
    blockPoolId: ::protobuf::SingularField<::std::string::String>,
    reports: ::protobuf::RepeatedField<StorageBlockReportProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockReportRequestProto {}

impl BlockReportRequestProto {
    pub fn new() -> BlockReportRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockReportRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockReportRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockReportRequestProto,
        };
        unsafe {
            instance.get(BlockReportRequestProto::new)
        }
    }

    // required .hadoop.hdfs.datanode.DatanodeRegistrationProto registration = 1;

    pub fn clear_registration(&mut self) {
        self.registration.clear();
    }

    pub fn has_registration(&self) -> bool {
        self.registration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_registration(&mut self, v: DatanodeRegistrationProto) {
        self.registration = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_registration(&mut self) -> &mut DatanodeRegistrationProto {
        if self.registration.is_none() {
            self.registration.set_default();
        }
        self.registration.as_mut().unwrap()
    }

    // Take field
    pub fn take_registration(&mut self) -> DatanodeRegistrationProto {
        self.registration.take().unwrap_or_else(|| DatanodeRegistrationProto::new())
    }

    pub fn get_registration(&self) -> &DatanodeRegistrationProto {
        self.registration.as_ref().unwrap_or_else(|| DatanodeRegistrationProto::default_instance())
    }

    fn get_registration_for_reflect(&self) -> &::protobuf::SingularPtrField<DatanodeRegistrationProto> {
        &self.registration
    }

    fn mut_registration_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DatanodeRegistrationProto> {
        &mut self.registration
    }

    // required string blockPoolId = 2;

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

    // repeated .hadoop.hdfs.datanode.StorageBlockReportProto reports = 3;

    pub fn clear_reports(&mut self) {
        self.reports.clear();
    }

    // Param is passed by value, moved
    pub fn set_reports(&mut self, v: ::protobuf::RepeatedField<StorageBlockReportProto>) {
        self.reports = v;
    }

    // Mutable pointer to the field.
    pub fn mut_reports(&mut self) -> &mut ::protobuf::RepeatedField<StorageBlockReportProto> {
        &mut self.reports
    }

    // Take field
    pub fn take_reports(&mut self) -> ::protobuf::RepeatedField<StorageBlockReportProto> {
        ::std::mem::replace(&mut self.reports, ::protobuf::RepeatedField::new())
    }

    pub fn get_reports(&self) -> &[StorageBlockReportProto] {
        &self.reports
    }

    fn get_reports_for_reflect(&self) -> &::protobuf::RepeatedField<StorageBlockReportProto> {
        &self.reports
    }

    fn mut_reports_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<StorageBlockReportProto> {
        &mut self.reports
    }
}

impl ::protobuf::Message for BlockReportRequestProto {
    fn is_initialized(&self) -> bool {
        if self.registration.is_none() {
            return false;
        }
        if self.blockPoolId.is_none() {
            return false;
        }
        for v in &self.registration {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.reports {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.blockPoolId)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.reports)?;
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
        if let Some(ref v) = self.blockPoolId.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        for value in &self.reports {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
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
        if let Some(ref v) = self.blockPoolId.as_ref() {
            os.write_string(2, &v)?;
        }
        for v in &self.reports {
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

impl ::protobuf::MessageStatic for BlockReportRequestProto {
    fn new() -> BlockReportRequestProto {
        BlockReportRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockReportRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DatanodeRegistrationProto>>(
                    "registration",
                    BlockReportRequestProto::get_registration_for_reflect,
                    BlockReportRequestProto::mut_registration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "blockPoolId",
                    BlockReportRequestProto::get_blockPoolId_for_reflect,
                    BlockReportRequestProto::mut_blockPoolId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StorageBlockReportProto>>(
                    "reports",
                    BlockReportRequestProto::get_reports_for_reflect,
                    BlockReportRequestProto::mut_reports_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockReportRequestProto>(
                    "BlockReportRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockReportRequestProto {
    fn clear(&mut self) {
        self.clear_registration();
        self.clear_blockPoolId();
        self.clear_reports();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockReportRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockReportRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StorageBlockReportProto {
    // message fields
    storage: ::protobuf::SingularPtrField<super::hdfs::DatanodeStorageProto>,
    blocks: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StorageBlockReportProto {}

impl StorageBlockReportProto {
    pub fn new() -> StorageBlockReportProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StorageBlockReportProto {
        static mut instance: ::protobuf::lazy::Lazy<StorageBlockReportProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StorageBlockReportProto,
        };
        unsafe {
            instance.get(StorageBlockReportProto::new)
        }
    }

    // required .hadoop.hdfs.DatanodeStorageProto storage = 1;

    pub fn clear_storage(&mut self) {
        self.storage.clear();
    }

    pub fn has_storage(&self) -> bool {
        self.storage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storage(&mut self, v: super::hdfs::DatanodeStorageProto) {
        self.storage = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storage(&mut self) -> &mut super::hdfs::DatanodeStorageProto {
        if self.storage.is_none() {
            self.storage.set_default();
        }
        self.storage.as_mut().unwrap()
    }

    // Take field
    pub fn take_storage(&mut self) -> super::hdfs::DatanodeStorageProto {
        self.storage.take().unwrap_or_else(|| super::hdfs::DatanodeStorageProto::new())
    }

    pub fn get_storage(&self) -> &super::hdfs::DatanodeStorageProto {
        self.storage.as_ref().unwrap_or_else(|| super::hdfs::DatanodeStorageProto::default_instance())
    }

    fn get_storage_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::DatanodeStorageProto> {
        &self.storage
    }

    fn mut_storage_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::DatanodeStorageProto> {
        &mut self.storage
    }

    // repeated uint64 blocks = 2;

    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    // Param is passed by value, moved
    pub fn set_blocks(&mut self, v: ::std::vec::Vec<u64>) {
        self.blocks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blocks(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.blocks
    }

    // Take field
    pub fn take_blocks(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.blocks, ::std::vec::Vec::new())
    }

    pub fn get_blocks(&self) -> &[u64] {
        &self.blocks
    }

    fn get_blocks_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.blocks
    }

    fn mut_blocks_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.blocks
    }
}

impl ::protobuf::Message for StorageBlockReportProto {
    fn is_initialized(&self) -> bool {
        if self.storage.is_none() {
            return false;
        }
        for v in &self.storage {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.storage)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.blocks)?;
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
        if let Some(ref v) = self.storage.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.blocks.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(2, &self.blocks);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.storage.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.blocks.is_empty() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.blocks))?;
            for v in &self.blocks {
                os.write_uint64_no_tag(*v)?;
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

impl ::protobuf::MessageStatic for StorageBlockReportProto {
    fn new() -> StorageBlockReportProto {
        StorageBlockReportProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StorageBlockReportProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeStorageProto>>(
                    "storage",
                    StorageBlockReportProto::get_storage_for_reflect,
                    StorageBlockReportProto::mut_storage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "blocks",
                    StorageBlockReportProto::get_blocks_for_reflect,
                    StorageBlockReportProto::mut_blocks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StorageBlockReportProto>(
                    "StorageBlockReportProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StorageBlockReportProto {
    fn clear(&mut self) {
        self.clear_storage();
        self.clear_blocks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StorageBlockReportProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StorageBlockReportProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockReportResponseProto {
    // message fields
    cmd: ::protobuf::SingularPtrField<DatanodeCommandProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockReportResponseProto {}

impl BlockReportResponseProto {
    pub fn new() -> BlockReportResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockReportResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockReportResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockReportResponseProto,
        };
        unsafe {
            instance.get(BlockReportResponseProto::new)
        }
    }

    // optional .hadoop.hdfs.datanode.DatanodeCommandProto cmd = 1;

    pub fn clear_cmd(&mut self) {
        self.cmd.clear();
    }

    pub fn has_cmd(&self) -> bool {
        self.cmd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd(&mut self, v: DatanodeCommandProto) {
        self.cmd = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd(&mut self) -> &mut DatanodeCommandProto {
        if self.cmd.is_none() {
            self.cmd.set_default();
        }
        self.cmd.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd(&mut self) -> DatanodeCommandProto {
        self.cmd.take().unwrap_or_else(|| DatanodeCommandProto::new())
    }

    pub fn get_cmd(&self) -> &DatanodeCommandProto {
        self.cmd.as_ref().unwrap_or_else(|| DatanodeCommandProto::default_instance())
    }

    fn get_cmd_for_reflect(&self) -> &::protobuf::SingularPtrField<DatanodeCommandProto> {
        &self.cmd
    }

    fn mut_cmd_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DatanodeCommandProto> {
        &mut self.cmd
    }
}

impl ::protobuf::Message for BlockReportResponseProto {
    fn is_initialized(&self) -> bool {
        for v in &self.cmd {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd)?;
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
        if let Some(ref v) = self.cmd.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.cmd.as_ref() {
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

impl ::protobuf::MessageStatic for BlockReportResponseProto {
    fn new() -> BlockReportResponseProto {
        BlockReportResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockReportResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DatanodeCommandProto>>(
                    "cmd",
                    BlockReportResponseProto::get_cmd_for_reflect,
                    BlockReportResponseProto::mut_cmd_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockReportResponseProto>(
                    "BlockReportResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockReportResponseProto {
    fn clear(&mut self) {
        self.clear_cmd();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockReportResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockReportResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CacheReportRequestProto {
    // message fields
    registration: ::protobuf::SingularPtrField<DatanodeRegistrationProto>,
    blockPoolId: ::protobuf::SingularField<::std::string::String>,
    blocks: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CacheReportRequestProto {}

impl CacheReportRequestProto {
    pub fn new() -> CacheReportRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CacheReportRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<CacheReportRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CacheReportRequestProto,
        };
        unsafe {
            instance.get(CacheReportRequestProto::new)
        }
    }

    // required .hadoop.hdfs.datanode.DatanodeRegistrationProto registration = 1;

    pub fn clear_registration(&mut self) {
        self.registration.clear();
    }

    pub fn has_registration(&self) -> bool {
        self.registration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_registration(&mut self, v: DatanodeRegistrationProto) {
        self.registration = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_registration(&mut self) -> &mut DatanodeRegistrationProto {
        if self.registration.is_none() {
            self.registration.set_default();
        }
        self.registration.as_mut().unwrap()
    }

    // Take field
    pub fn take_registration(&mut self) -> DatanodeRegistrationProto {
        self.registration.take().unwrap_or_else(|| DatanodeRegistrationProto::new())
    }

    pub fn get_registration(&self) -> &DatanodeRegistrationProto {
        self.registration.as_ref().unwrap_or_else(|| DatanodeRegistrationProto::default_instance())
    }

    fn get_registration_for_reflect(&self) -> &::protobuf::SingularPtrField<DatanodeRegistrationProto> {
        &self.registration
    }

    fn mut_registration_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DatanodeRegistrationProto> {
        &mut self.registration
    }

    // required string blockPoolId = 2;

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

    // repeated uint64 blocks = 3;

    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    // Param is passed by value, moved
    pub fn set_blocks(&mut self, v: ::std::vec::Vec<u64>) {
        self.blocks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blocks(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.blocks
    }

    // Take field
    pub fn take_blocks(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.blocks, ::std::vec::Vec::new())
    }

    pub fn get_blocks(&self) -> &[u64] {
        &self.blocks
    }

    fn get_blocks_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.blocks
    }

    fn mut_blocks_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.blocks
    }
}

impl ::protobuf::Message for CacheReportRequestProto {
    fn is_initialized(&self) -> bool {
        if self.registration.is_none() {
            return false;
        }
        if self.blockPoolId.is_none() {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.blockPoolId)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.blocks)?;
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
        if let Some(ref v) = self.blockPoolId.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if !self.blocks.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(3, &self.blocks);
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
        if let Some(ref v) = self.blockPoolId.as_ref() {
            os.write_string(2, &v)?;
        }
        if !self.blocks.is_empty() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.blocks))?;
            for v in &self.blocks {
                os.write_uint64_no_tag(*v)?;
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

impl ::protobuf::MessageStatic for CacheReportRequestProto {
    fn new() -> CacheReportRequestProto {
        CacheReportRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CacheReportRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DatanodeRegistrationProto>>(
                    "registration",
                    CacheReportRequestProto::get_registration_for_reflect,
                    CacheReportRequestProto::mut_registration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "blockPoolId",
                    CacheReportRequestProto::get_blockPoolId_for_reflect,
                    CacheReportRequestProto::mut_blockPoolId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "blocks",
                    CacheReportRequestProto::get_blocks_for_reflect,
                    CacheReportRequestProto::mut_blocks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CacheReportRequestProto>(
                    "CacheReportRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CacheReportRequestProto {
    fn clear(&mut self) {
        self.clear_registration();
        self.clear_blockPoolId();
        self.clear_blocks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CacheReportRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CacheReportRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CacheReportResponseProto {
    // message fields
    cmd: ::protobuf::SingularPtrField<DatanodeCommandProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CacheReportResponseProto {}

impl CacheReportResponseProto {
    pub fn new() -> CacheReportResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CacheReportResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<CacheReportResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CacheReportResponseProto,
        };
        unsafe {
            instance.get(CacheReportResponseProto::new)
        }
    }

    // optional .hadoop.hdfs.datanode.DatanodeCommandProto cmd = 1;

    pub fn clear_cmd(&mut self) {
        self.cmd.clear();
    }

    pub fn has_cmd(&self) -> bool {
        self.cmd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd(&mut self, v: DatanodeCommandProto) {
        self.cmd = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd(&mut self) -> &mut DatanodeCommandProto {
        if self.cmd.is_none() {
            self.cmd.set_default();
        }
        self.cmd.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd(&mut self) -> DatanodeCommandProto {
        self.cmd.take().unwrap_or_else(|| DatanodeCommandProto::new())
    }

    pub fn get_cmd(&self) -> &DatanodeCommandProto {
        self.cmd.as_ref().unwrap_or_else(|| DatanodeCommandProto::default_instance())
    }

    fn get_cmd_for_reflect(&self) -> &::protobuf::SingularPtrField<DatanodeCommandProto> {
        &self.cmd
    }

    fn mut_cmd_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DatanodeCommandProto> {
        &mut self.cmd
    }
}

impl ::protobuf::Message for CacheReportResponseProto {
    fn is_initialized(&self) -> bool {
        for v in &self.cmd {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd)?;
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
        if let Some(ref v) = self.cmd.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.cmd.as_ref() {
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

impl ::protobuf::MessageStatic for CacheReportResponseProto {
    fn new() -> CacheReportResponseProto {
        CacheReportResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CacheReportResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DatanodeCommandProto>>(
                    "cmd",
                    CacheReportResponseProto::get_cmd_for_reflect,
                    CacheReportResponseProto::mut_cmd_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CacheReportResponseProto>(
                    "CacheReportResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CacheReportResponseProto {
    fn clear(&mut self) {
        self.clear_cmd();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CacheReportResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CacheReportResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReceivedDeletedBlockInfoProto {
    // message fields
    block: ::protobuf::SingularPtrField<super::hdfs::BlockProto>,
    status: ::std::option::Option<ReceivedDeletedBlockInfoProto_BlockStatus>,
    deleteHint: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReceivedDeletedBlockInfoProto {}

impl ReceivedDeletedBlockInfoProto {
    pub fn new() -> ReceivedDeletedBlockInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReceivedDeletedBlockInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<ReceivedDeletedBlockInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReceivedDeletedBlockInfoProto,
        };
        unsafe {
            instance.get(ReceivedDeletedBlockInfoProto::new)
        }
    }

    // required .hadoop.hdfs.BlockProto block = 1;

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

    // required .hadoop.hdfs.datanode.ReceivedDeletedBlockInfoProto.BlockStatus status = 3;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: ReceivedDeletedBlockInfoProto_BlockStatus) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> ReceivedDeletedBlockInfoProto_BlockStatus {
        self.status.unwrap_or(ReceivedDeletedBlockInfoProto_BlockStatus::RECEIVING)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<ReceivedDeletedBlockInfoProto_BlockStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<ReceivedDeletedBlockInfoProto_BlockStatus> {
        &mut self.status
    }

    // optional string deleteHint = 2;

    pub fn clear_deleteHint(&mut self) {
        self.deleteHint.clear();
    }

    pub fn has_deleteHint(&self) -> bool {
        self.deleteHint.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deleteHint(&mut self, v: ::std::string::String) {
        self.deleteHint = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deleteHint(&mut self) -> &mut ::std::string::String {
        if self.deleteHint.is_none() {
            self.deleteHint.set_default();
        }
        self.deleteHint.as_mut().unwrap()
    }

    // Take field
    pub fn take_deleteHint(&mut self) -> ::std::string::String {
        self.deleteHint.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_deleteHint(&self) -> &str {
        match self.deleteHint.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_deleteHint_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.deleteHint
    }

    fn mut_deleteHint_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.deleteHint
    }
}

impl ::protobuf::Message for ReceivedDeletedBlockInfoProto {
    fn is_initialized(&self) -> bool {
        if self.block.is_none() {
            return false;
        }
        if self.status.is_none() {
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
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.deleteHint)?;
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
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        if let Some(ref v) = self.deleteHint.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
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
        if let Some(v) = self.status {
            os.write_enum(3, v.value())?;
        }
        if let Some(ref v) = self.deleteHint.as_ref() {
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

impl ::protobuf::MessageStatic for ReceivedDeletedBlockInfoProto {
    fn new() -> ReceivedDeletedBlockInfoProto {
        ReceivedDeletedBlockInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReceivedDeletedBlockInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::BlockProto>>(
                    "block",
                    ReceivedDeletedBlockInfoProto::get_block_for_reflect,
                    ReceivedDeletedBlockInfoProto::mut_block_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ReceivedDeletedBlockInfoProto_BlockStatus>>(
                    "status",
                    ReceivedDeletedBlockInfoProto::get_status_for_reflect,
                    ReceivedDeletedBlockInfoProto::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "deleteHint",
                    ReceivedDeletedBlockInfoProto::get_deleteHint_for_reflect,
                    ReceivedDeletedBlockInfoProto::mut_deleteHint_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReceivedDeletedBlockInfoProto>(
                    "ReceivedDeletedBlockInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReceivedDeletedBlockInfoProto {
    fn clear(&mut self) {
        self.clear_block();
        self.clear_status();
        self.clear_deleteHint();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReceivedDeletedBlockInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReceivedDeletedBlockInfoProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ReceivedDeletedBlockInfoProto_BlockStatus {
    RECEIVING = 1,
    RECEIVED = 2,
    DELETED = 3,
}

impl ::protobuf::ProtobufEnum for ReceivedDeletedBlockInfoProto_BlockStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReceivedDeletedBlockInfoProto_BlockStatus> {
        match value {
            1 => ::std::option::Option::Some(ReceivedDeletedBlockInfoProto_BlockStatus::RECEIVING),
            2 => ::std::option::Option::Some(ReceivedDeletedBlockInfoProto_BlockStatus::RECEIVED),
            3 => ::std::option::Option::Some(ReceivedDeletedBlockInfoProto_BlockStatus::DELETED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ReceivedDeletedBlockInfoProto_BlockStatus] = &[
            ReceivedDeletedBlockInfoProto_BlockStatus::RECEIVING,
            ReceivedDeletedBlockInfoProto_BlockStatus::RECEIVED,
            ReceivedDeletedBlockInfoProto_BlockStatus::DELETED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ReceivedDeletedBlockInfoProto_BlockStatus>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ReceivedDeletedBlockInfoProto_BlockStatus", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ReceivedDeletedBlockInfoProto_BlockStatus {
}

impl ::protobuf::reflect::ProtobufValue for ReceivedDeletedBlockInfoProto_BlockStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StorageReceivedDeletedBlocksProto {
    // message fields
    storageUuid: ::protobuf::SingularField<::std::string::String>,
    blocks: ::protobuf::RepeatedField<ReceivedDeletedBlockInfoProto>,
    storage: ::protobuf::SingularPtrField<super::hdfs::DatanodeStorageProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StorageReceivedDeletedBlocksProto {}

impl StorageReceivedDeletedBlocksProto {
    pub fn new() -> StorageReceivedDeletedBlocksProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StorageReceivedDeletedBlocksProto {
        static mut instance: ::protobuf::lazy::Lazy<StorageReceivedDeletedBlocksProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StorageReceivedDeletedBlocksProto,
        };
        unsafe {
            instance.get(StorageReceivedDeletedBlocksProto::new)
        }
    }

    // required string storageUuid = 1;

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

    // repeated .hadoop.hdfs.datanode.ReceivedDeletedBlockInfoProto blocks = 2;

    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    // Param is passed by value, moved
    pub fn set_blocks(&mut self, v: ::protobuf::RepeatedField<ReceivedDeletedBlockInfoProto>) {
        self.blocks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blocks(&mut self) -> &mut ::protobuf::RepeatedField<ReceivedDeletedBlockInfoProto> {
        &mut self.blocks
    }

    // Take field
    pub fn take_blocks(&mut self) -> ::protobuf::RepeatedField<ReceivedDeletedBlockInfoProto> {
        ::std::mem::replace(&mut self.blocks, ::protobuf::RepeatedField::new())
    }

    pub fn get_blocks(&self) -> &[ReceivedDeletedBlockInfoProto] {
        &self.blocks
    }

    fn get_blocks_for_reflect(&self) -> &::protobuf::RepeatedField<ReceivedDeletedBlockInfoProto> {
        &self.blocks
    }

    fn mut_blocks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ReceivedDeletedBlockInfoProto> {
        &mut self.blocks
    }

    // optional .hadoop.hdfs.DatanodeStorageProto storage = 3;

    pub fn clear_storage(&mut self) {
        self.storage.clear();
    }

    pub fn has_storage(&self) -> bool {
        self.storage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storage(&mut self, v: super::hdfs::DatanodeStorageProto) {
        self.storage = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storage(&mut self) -> &mut super::hdfs::DatanodeStorageProto {
        if self.storage.is_none() {
            self.storage.set_default();
        }
        self.storage.as_mut().unwrap()
    }

    // Take field
    pub fn take_storage(&mut self) -> super::hdfs::DatanodeStorageProto {
        self.storage.take().unwrap_or_else(|| super::hdfs::DatanodeStorageProto::new())
    }

    pub fn get_storage(&self) -> &super::hdfs::DatanodeStorageProto {
        self.storage.as_ref().unwrap_or_else(|| super::hdfs::DatanodeStorageProto::default_instance())
    }

    fn get_storage_for_reflect(&self) -> &::protobuf::SingularPtrField<super::hdfs::DatanodeStorageProto> {
        &self.storage
    }

    fn mut_storage_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::hdfs::DatanodeStorageProto> {
        &mut self.storage
    }
}

impl ::protobuf::Message for StorageReceivedDeletedBlocksProto {
    fn is_initialized(&self) -> bool {
        if self.storageUuid.is_none() {
            return false;
        }
        for v in &self.blocks {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.storage {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.storageUuid)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.blocks)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.storage)?;
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
        for value in &self.blocks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.storage.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.storageUuid.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.blocks {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.storage.as_ref() {
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

impl ::protobuf::MessageStatic for StorageReceivedDeletedBlocksProto {
    fn new() -> StorageReceivedDeletedBlocksProto {
        StorageReceivedDeletedBlocksProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StorageReceivedDeletedBlocksProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "storageUuid",
                    StorageReceivedDeletedBlocksProto::get_storageUuid_for_reflect,
                    StorageReceivedDeletedBlocksProto::mut_storageUuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ReceivedDeletedBlockInfoProto>>(
                    "blocks",
                    StorageReceivedDeletedBlocksProto::get_blocks_for_reflect,
                    StorageReceivedDeletedBlocksProto::mut_blocks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeStorageProto>>(
                    "storage",
                    StorageReceivedDeletedBlocksProto::get_storage_for_reflect,
                    StorageReceivedDeletedBlocksProto::mut_storage_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StorageReceivedDeletedBlocksProto>(
                    "StorageReceivedDeletedBlocksProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StorageReceivedDeletedBlocksProto {
    fn clear(&mut self) {
        self.clear_storageUuid();
        self.clear_blocks();
        self.clear_storage();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StorageReceivedDeletedBlocksProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StorageReceivedDeletedBlocksProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockReceivedAndDeletedRequestProto {
    // message fields
    registration: ::protobuf::SingularPtrField<DatanodeRegistrationProto>,
    blockPoolId: ::protobuf::SingularField<::std::string::String>,
    blocks: ::protobuf::RepeatedField<StorageReceivedDeletedBlocksProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockReceivedAndDeletedRequestProto {}

impl BlockReceivedAndDeletedRequestProto {
    pub fn new() -> BlockReceivedAndDeletedRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockReceivedAndDeletedRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockReceivedAndDeletedRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockReceivedAndDeletedRequestProto,
        };
        unsafe {
            instance.get(BlockReceivedAndDeletedRequestProto::new)
        }
    }

    // required .hadoop.hdfs.datanode.DatanodeRegistrationProto registration = 1;

    pub fn clear_registration(&mut self) {
        self.registration.clear();
    }

    pub fn has_registration(&self) -> bool {
        self.registration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_registration(&mut self, v: DatanodeRegistrationProto) {
        self.registration = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_registration(&mut self) -> &mut DatanodeRegistrationProto {
        if self.registration.is_none() {
            self.registration.set_default();
        }
        self.registration.as_mut().unwrap()
    }

    // Take field
    pub fn take_registration(&mut self) -> DatanodeRegistrationProto {
        self.registration.take().unwrap_or_else(|| DatanodeRegistrationProto::new())
    }

    pub fn get_registration(&self) -> &DatanodeRegistrationProto {
        self.registration.as_ref().unwrap_or_else(|| DatanodeRegistrationProto::default_instance())
    }

    fn get_registration_for_reflect(&self) -> &::protobuf::SingularPtrField<DatanodeRegistrationProto> {
        &self.registration
    }

    fn mut_registration_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DatanodeRegistrationProto> {
        &mut self.registration
    }

    // required string blockPoolId = 2;

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

    // repeated .hadoop.hdfs.datanode.StorageReceivedDeletedBlocksProto blocks = 3;

    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    // Param is passed by value, moved
    pub fn set_blocks(&mut self, v: ::protobuf::RepeatedField<StorageReceivedDeletedBlocksProto>) {
        self.blocks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blocks(&mut self) -> &mut ::protobuf::RepeatedField<StorageReceivedDeletedBlocksProto> {
        &mut self.blocks
    }

    // Take field
    pub fn take_blocks(&mut self) -> ::protobuf::RepeatedField<StorageReceivedDeletedBlocksProto> {
        ::std::mem::replace(&mut self.blocks, ::protobuf::RepeatedField::new())
    }

    pub fn get_blocks(&self) -> &[StorageReceivedDeletedBlocksProto] {
        &self.blocks
    }

    fn get_blocks_for_reflect(&self) -> &::protobuf::RepeatedField<StorageReceivedDeletedBlocksProto> {
        &self.blocks
    }

    fn mut_blocks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<StorageReceivedDeletedBlocksProto> {
        &mut self.blocks
    }
}

impl ::protobuf::Message for BlockReceivedAndDeletedRequestProto {
    fn is_initialized(&self) -> bool {
        if self.registration.is_none() {
            return false;
        }
        if self.blockPoolId.is_none() {
            return false;
        }
        for v in &self.registration {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.registration)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.blockPoolId)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.blocks)?;
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
        if let Some(ref v) = self.blockPoolId.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        for value in &self.blocks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
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
        if let Some(ref v) = self.blockPoolId.as_ref() {
            os.write_string(2, &v)?;
        }
        for v in &self.blocks {
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

impl ::protobuf::MessageStatic for BlockReceivedAndDeletedRequestProto {
    fn new() -> BlockReceivedAndDeletedRequestProto {
        BlockReceivedAndDeletedRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockReceivedAndDeletedRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DatanodeRegistrationProto>>(
                    "registration",
                    BlockReceivedAndDeletedRequestProto::get_registration_for_reflect,
                    BlockReceivedAndDeletedRequestProto::mut_registration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "blockPoolId",
                    BlockReceivedAndDeletedRequestProto::get_blockPoolId_for_reflect,
                    BlockReceivedAndDeletedRequestProto::mut_blockPoolId_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StorageReceivedDeletedBlocksProto>>(
                    "blocks",
                    BlockReceivedAndDeletedRequestProto::get_blocks_for_reflect,
                    BlockReceivedAndDeletedRequestProto::mut_blocks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockReceivedAndDeletedRequestProto>(
                    "BlockReceivedAndDeletedRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockReceivedAndDeletedRequestProto {
    fn clear(&mut self) {
        self.clear_registration();
        self.clear_blockPoolId();
        self.clear_blocks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockReceivedAndDeletedRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockReceivedAndDeletedRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockReceivedAndDeletedResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockReceivedAndDeletedResponseProto {}

impl BlockReceivedAndDeletedResponseProto {
    pub fn new() -> BlockReceivedAndDeletedResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockReceivedAndDeletedResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockReceivedAndDeletedResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockReceivedAndDeletedResponseProto,
        };
        unsafe {
            instance.get(BlockReceivedAndDeletedResponseProto::new)
        }
    }
}

impl ::protobuf::Message for BlockReceivedAndDeletedResponseProto {
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

impl ::protobuf::MessageStatic for BlockReceivedAndDeletedResponseProto {
    fn new() -> BlockReceivedAndDeletedResponseProto {
        BlockReceivedAndDeletedResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockReceivedAndDeletedResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<BlockReceivedAndDeletedResponseProto>(
                    "BlockReceivedAndDeletedResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockReceivedAndDeletedResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockReceivedAndDeletedResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockReceivedAndDeletedResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ErrorReportRequestProto {
    // message fields
    registartion: ::protobuf::SingularPtrField<DatanodeRegistrationProto>,
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

    // required .hadoop.hdfs.datanode.DatanodeRegistrationProto registartion = 1;

    pub fn clear_registartion(&mut self) {
        self.registartion.clear();
    }

    pub fn has_registartion(&self) -> bool {
        self.registartion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_registartion(&mut self, v: DatanodeRegistrationProto) {
        self.registartion = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_registartion(&mut self) -> &mut DatanodeRegistrationProto {
        if self.registartion.is_none() {
            self.registartion.set_default();
        }
        self.registartion.as_mut().unwrap()
    }

    // Take field
    pub fn take_registartion(&mut self) -> DatanodeRegistrationProto {
        self.registartion.take().unwrap_or_else(|| DatanodeRegistrationProto::new())
    }

    pub fn get_registartion(&self) -> &DatanodeRegistrationProto {
        self.registartion.as_ref().unwrap_or_else(|| DatanodeRegistrationProto::default_instance())
    }

    fn get_registartion_for_reflect(&self) -> &::protobuf::SingularPtrField<DatanodeRegistrationProto> {
        &self.registartion
    }

    fn mut_registartion_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DatanodeRegistrationProto> {
        &mut self.registartion
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
        if self.registartion.is_none() {
            return false;
        }
        if self.errorCode.is_none() {
            return false;
        }
        if self.msg.is_none() {
            return false;
        }
        for v in &self.registartion {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.registartion)?;
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
        if let Some(ref v) = self.registartion.as_ref() {
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
        if let Some(ref v) = self.registartion.as_ref() {
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DatanodeRegistrationProto>>(
                    "registartion",
                    ErrorReportRequestProto::get_registartion_for_reflect,
                    ErrorReportRequestProto::mut_registartion_for_reflect,
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
        self.clear_registartion();
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

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ErrorReportRequestProto_ErrorCode {
    NOTIFY = 0,
    DISK_ERROR = 1,
    INVALID_BLOCK = 2,
    FATAL_DISK_ERROR = 3,
}

impl ::protobuf::ProtobufEnum for ErrorReportRequestProto_ErrorCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ErrorReportRequestProto_ErrorCode> {
        match value {
            0 => ::std::option::Option::Some(ErrorReportRequestProto_ErrorCode::NOTIFY),
            1 => ::std::option::Option::Some(ErrorReportRequestProto_ErrorCode::DISK_ERROR),
            2 => ::std::option::Option::Some(ErrorReportRequestProto_ErrorCode::INVALID_BLOCK),
            3 => ::std::option::Option::Some(ErrorReportRequestProto_ErrorCode::FATAL_DISK_ERROR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ErrorReportRequestProto_ErrorCode] = &[
            ErrorReportRequestProto_ErrorCode::NOTIFY,
            ErrorReportRequestProto_ErrorCode::DISK_ERROR,
            ErrorReportRequestProto_ErrorCode::INVALID_BLOCK,
            ErrorReportRequestProto_ErrorCode::FATAL_DISK_ERROR,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ErrorReportRequestProto_ErrorCode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ErrorReportRequestProto_ErrorCode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ErrorReportRequestProto_ErrorCode {
}

impl ::protobuf::reflect::ProtobufValue for ErrorReportRequestProto_ErrorCode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
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
pub struct ReportBadBlocksRequestProto {
    // message fields
    blocks: ::protobuf::RepeatedField<super::hdfs::LocatedBlockProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReportBadBlocksRequestProto {}

impl ReportBadBlocksRequestProto {
    pub fn new() -> ReportBadBlocksRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReportBadBlocksRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ReportBadBlocksRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReportBadBlocksRequestProto,
        };
        unsafe {
            instance.get(ReportBadBlocksRequestProto::new)
        }
    }

    // repeated .hadoop.hdfs.LocatedBlockProto blocks = 1;

    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    // Param is passed by value, moved
    pub fn set_blocks(&mut self, v: ::protobuf::RepeatedField<super::hdfs::LocatedBlockProto>) {
        self.blocks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blocks(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::LocatedBlockProto> {
        &mut self.blocks
    }

    // Take field
    pub fn take_blocks(&mut self) -> ::protobuf::RepeatedField<super::hdfs::LocatedBlockProto> {
        ::std::mem::replace(&mut self.blocks, ::protobuf::RepeatedField::new())
    }

    pub fn get_blocks(&self) -> &[super::hdfs::LocatedBlockProto] {
        &self.blocks
    }

    fn get_blocks_for_reflect(&self) -> &::protobuf::RepeatedField<super::hdfs::LocatedBlockProto> {
        &self.blocks
    }

    fn mut_blocks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::LocatedBlockProto> {
        &mut self.blocks
    }
}

impl ::protobuf::Message for ReportBadBlocksRequestProto {
    fn is_initialized(&self) -> bool {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.blocks)?;
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
        for value in &self.blocks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.blocks {
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

impl ::protobuf::MessageStatic for ReportBadBlocksRequestProto {
    fn new() -> ReportBadBlocksRequestProto {
        ReportBadBlocksRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReportBadBlocksRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::LocatedBlockProto>>(
                    "blocks",
                    ReportBadBlocksRequestProto::get_blocks_for_reflect,
                    ReportBadBlocksRequestProto::mut_blocks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReportBadBlocksRequestProto>(
                    "ReportBadBlocksRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReportBadBlocksRequestProto {
    fn clear(&mut self) {
        self.clear_blocks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReportBadBlocksRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReportBadBlocksRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReportBadBlocksResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReportBadBlocksResponseProto {}

impl ReportBadBlocksResponseProto {
    pub fn new() -> ReportBadBlocksResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReportBadBlocksResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ReportBadBlocksResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReportBadBlocksResponseProto,
        };
        unsafe {
            instance.get(ReportBadBlocksResponseProto::new)
        }
    }
}

impl ::protobuf::Message for ReportBadBlocksResponseProto {
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

impl ::protobuf::MessageStatic for ReportBadBlocksResponseProto {
    fn new() -> ReportBadBlocksResponseProto {
        ReportBadBlocksResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReportBadBlocksResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ReportBadBlocksResponseProto>(
                    "ReportBadBlocksResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReportBadBlocksResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReportBadBlocksResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReportBadBlocksResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CommitBlockSynchronizationRequestProto {
    // message fields
    block: ::protobuf::SingularPtrField<super::hdfs::ExtendedBlockProto>,
    newGenStamp: ::std::option::Option<u64>,
    newLength: ::std::option::Option<u64>,
    closeFile: ::std::option::Option<bool>,
    deleteBlock: ::std::option::Option<bool>,
    newTaragets: ::protobuf::RepeatedField<super::hdfs::DatanodeIDProto>,
    newTargetStorages: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CommitBlockSynchronizationRequestProto {}

impl CommitBlockSynchronizationRequestProto {
    pub fn new() -> CommitBlockSynchronizationRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CommitBlockSynchronizationRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<CommitBlockSynchronizationRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CommitBlockSynchronizationRequestProto,
        };
        unsafe {
            instance.get(CommitBlockSynchronizationRequestProto::new)
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

    // required uint64 newGenStamp = 2;

    pub fn clear_newGenStamp(&mut self) {
        self.newGenStamp = ::std::option::Option::None;
    }

    pub fn has_newGenStamp(&self) -> bool {
        self.newGenStamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_newGenStamp(&mut self, v: u64) {
        self.newGenStamp = ::std::option::Option::Some(v);
    }

    pub fn get_newGenStamp(&self) -> u64 {
        self.newGenStamp.unwrap_or(0)
    }

    fn get_newGenStamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.newGenStamp
    }

    fn mut_newGenStamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.newGenStamp
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

    // required bool closeFile = 4;

    pub fn clear_closeFile(&mut self) {
        self.closeFile = ::std::option::Option::None;
    }

    pub fn has_closeFile(&self) -> bool {
        self.closeFile.is_some()
    }

    // Param is passed by value, moved
    pub fn set_closeFile(&mut self, v: bool) {
        self.closeFile = ::std::option::Option::Some(v);
    }

    pub fn get_closeFile(&self) -> bool {
        self.closeFile.unwrap_or(false)
    }

    fn get_closeFile_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.closeFile
    }

    fn mut_closeFile_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.closeFile
    }

    // required bool deleteBlock = 5;

    pub fn clear_deleteBlock(&mut self) {
        self.deleteBlock = ::std::option::Option::None;
    }

    pub fn has_deleteBlock(&self) -> bool {
        self.deleteBlock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deleteBlock(&mut self, v: bool) {
        self.deleteBlock = ::std::option::Option::Some(v);
    }

    pub fn get_deleteBlock(&self) -> bool {
        self.deleteBlock.unwrap_or(false)
    }

    fn get_deleteBlock_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.deleteBlock
    }

    fn mut_deleteBlock_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.deleteBlock
    }

    // repeated .hadoop.hdfs.DatanodeIDProto newTaragets = 6;

    pub fn clear_newTaragets(&mut self) {
        self.newTaragets.clear();
    }

    // Param is passed by value, moved
    pub fn set_newTaragets(&mut self, v: ::protobuf::RepeatedField<super::hdfs::DatanodeIDProto>) {
        self.newTaragets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_newTaragets(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::DatanodeIDProto> {
        &mut self.newTaragets
    }

    // Take field
    pub fn take_newTaragets(&mut self) -> ::protobuf::RepeatedField<super::hdfs::DatanodeIDProto> {
        ::std::mem::replace(&mut self.newTaragets, ::protobuf::RepeatedField::new())
    }

    pub fn get_newTaragets(&self) -> &[super::hdfs::DatanodeIDProto] {
        &self.newTaragets
    }

    fn get_newTaragets_for_reflect(&self) -> &::protobuf::RepeatedField<super::hdfs::DatanodeIDProto> {
        &self.newTaragets
    }

    fn mut_newTaragets_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::hdfs::DatanodeIDProto> {
        &mut self.newTaragets
    }

    // repeated string newTargetStorages = 7;

    pub fn clear_newTargetStorages(&mut self) {
        self.newTargetStorages.clear();
    }

    // Param is passed by value, moved
    pub fn set_newTargetStorages(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.newTargetStorages = v;
    }

    // Mutable pointer to the field.
    pub fn mut_newTargetStorages(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.newTargetStorages
    }

    // Take field
    pub fn take_newTargetStorages(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.newTargetStorages, ::protobuf::RepeatedField::new())
    }

    pub fn get_newTargetStorages(&self) -> &[::std::string::String] {
        &self.newTargetStorages
    }

    fn get_newTargetStorages_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.newTargetStorages
    }

    fn mut_newTargetStorages_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.newTargetStorages
    }
}

impl ::protobuf::Message for CommitBlockSynchronizationRequestProto {
    fn is_initialized(&self) -> bool {
        if self.block.is_none() {
            return false;
        }
        if self.newGenStamp.is_none() {
            return false;
        }
        if self.newLength.is_none() {
            return false;
        }
        if self.closeFile.is_none() {
            return false;
        }
        if self.deleteBlock.is_none() {
            return false;
        }
        for v in &self.block {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.newTaragets {
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
                    self.newGenStamp = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.newLength = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.closeFile = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.deleteBlock = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.newTaragets)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.newTargetStorages)?;
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
        if let Some(v) = self.newGenStamp {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.newLength {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.closeFile {
            my_size += 2;
        }
        if let Some(v) = self.deleteBlock {
            my_size += 2;
        }
        for value in &self.newTaragets {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.newTargetStorages {
            my_size += ::protobuf::rt::string_size(7, &value);
        };
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
        if let Some(v) = self.newGenStamp {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.newLength {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.closeFile {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.deleteBlock {
            os.write_bool(5, v)?;
        }
        for v in &self.newTaragets {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.newTargetStorages {
            os.write_string(7, &v)?;
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

impl ::protobuf::MessageStatic for CommitBlockSynchronizationRequestProto {
    fn new() -> CommitBlockSynchronizationRequestProto {
        CommitBlockSynchronizationRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CommitBlockSynchronizationRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::ExtendedBlockProto>>(
                    "block",
                    CommitBlockSynchronizationRequestProto::get_block_for_reflect,
                    CommitBlockSynchronizationRequestProto::mut_block_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "newGenStamp",
                    CommitBlockSynchronizationRequestProto::get_newGenStamp_for_reflect,
                    CommitBlockSynchronizationRequestProto::mut_newGenStamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "newLength",
                    CommitBlockSynchronizationRequestProto::get_newLength_for_reflect,
                    CommitBlockSynchronizationRequestProto::mut_newLength_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "closeFile",
                    CommitBlockSynchronizationRequestProto::get_closeFile_for_reflect,
                    CommitBlockSynchronizationRequestProto::mut_closeFile_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "deleteBlock",
                    CommitBlockSynchronizationRequestProto::get_deleteBlock_for_reflect,
                    CommitBlockSynchronizationRequestProto::mut_deleteBlock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::hdfs::DatanodeIDProto>>(
                    "newTaragets",
                    CommitBlockSynchronizationRequestProto::get_newTaragets_for_reflect,
                    CommitBlockSynchronizationRequestProto::mut_newTaragets_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "newTargetStorages",
                    CommitBlockSynchronizationRequestProto::get_newTargetStorages_for_reflect,
                    CommitBlockSynchronizationRequestProto::mut_newTargetStorages_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CommitBlockSynchronizationRequestProto>(
                    "CommitBlockSynchronizationRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CommitBlockSynchronizationRequestProto {
    fn clear(&mut self) {
        self.clear_block();
        self.clear_newGenStamp();
        self.clear_newLength();
        self.clear_closeFile();
        self.clear_deleteBlock();
        self.clear_newTaragets();
        self.clear_newTargetStorages();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CommitBlockSynchronizationRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CommitBlockSynchronizationRequestProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CommitBlockSynchronizationResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CommitBlockSynchronizationResponseProto {}

impl CommitBlockSynchronizationResponseProto {
    pub fn new() -> CommitBlockSynchronizationResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CommitBlockSynchronizationResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<CommitBlockSynchronizationResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CommitBlockSynchronizationResponseProto,
        };
        unsafe {
            instance.get(CommitBlockSynchronizationResponseProto::new)
        }
    }
}

impl ::protobuf::Message for CommitBlockSynchronizationResponseProto {
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

impl ::protobuf::MessageStatic for CommitBlockSynchronizationResponseProto {
    fn new() -> CommitBlockSynchronizationResponseProto {
        CommitBlockSynchronizationResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CommitBlockSynchronizationResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CommitBlockSynchronizationResponseProto>(
                    "CommitBlockSynchronizationResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CommitBlockSynchronizationResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CommitBlockSynchronizationResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CommitBlockSynchronizationResponseProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16DatanodeProtocol.proto\x12\x14hadoop.hdfs.datanode\x1a\nhdfs.proto\
    \"\xcd\x01\n\x19DatanodeRegistrationProto\x120\n\ndatanodeID\x18\x01\x20\
    \x02(\x0b2\x1c.hadoop.hdfs.DatanodeIDProto\x122\n\x0bstorageInfo\x18\x02\
    \x20\x02(\x0b2\x1d.hadoop.hdfs.StorageInfoProto\x121\n\x04keys\x18\x03\
    \x20\x02(\x0b2#.hadoop.hdfs.ExportedBlockKeysProto\x12\x17\n\x0fsoftware\
    Version\x18\x04\x20\x02(\t\"\xfd\x05\n\x14DatanodeCommandProto\x12@\n\
    \x07cmdType\x18\x01\x20\x02(\x0e2/.hadoop.hdfs.datanode.DatanodeCommandP\
    roto.Type\x12H\n\x0bbalancerCmd\x18\x02\x20\x01(\x0b23.hadoop.hdfs.datan\
    ode.BalancerBandwidthCommandProto\x127\n\x06blkCmd\x18\x03\x20\x01(\x0b2\
    '.hadoop.hdfs.datanode.BlockCommandProto\x12D\n\x0brecoveryCmd\x18\x04\
    \x20\x01(\x0b2/.hadoop.hdfs.datanode.BlockRecoveryCommandProto\x12?\n\
    \x0bfinalizeCmd\x18\x05\x20\x01(\x0b2*.hadoop.hdfs.datanode.FinalizeComm\
    andProto\x12A\n\x0ckeyUpdateCmd\x18\x06\x20\x01(\x0b2+.hadoop.hdfs.datan\
    ode.KeyUpdateCommandProto\x12?\n\x0bregisterCmd\x18\x07\x20\x01(\x0b2*.h\
    adoop.hdfs.datanode.RegisterCommandProto\x12;\n\x08blkIdCmd\x18\x08\x20\
    \x01(\x0b2).hadoop.hdfs.datanode.BlockIdCommandProto\"\xd7\x01\n\x04Type\
    \x12\x1c\n\x18BalancerBandwidthCommand\x10\0\x12\x10\n\x0cBlockCommand\
    \x10\x01\x12\x18\n\x14BlockRecoveryCommand\x10\x02\x12\x13\n\x0fFinalize\
    Command\x10\x03\x12\x14\n\x10KeyUpdateCommand\x10\x04\x12\x13\n\x0fRegis\
    terCommand\x10\x05\x12\x18\n\x14UnusedUpgradeCommand\x10\x06\x12\x17\n\
    \x13NullDatanodeCommand\x10\x07\x12\x12\n\x0eBlockIdCommand\x10\x08\"2\n\
    \x1dBalancerBandwidthCommandProto\x12\x11\n\tbandwidth\x18\x01\x20\x02(\
    \x04\"\xf1\x02\n\x11BlockCommandProto\x12>\n\x06action\x18\x01\x20\x02(\
    \x0e2..hadoop.hdfs.datanode.BlockCommandProto.Action\x12\x13\n\x0bblockP\
    oolId\x18\x02\x20\x02(\t\x12'\n\x06blocks\x18\x03\x20\x03(\x0b2\x17.hado\
    op.hdfs.BlockProto\x120\n\x07targets\x18\x04\x20\x03(\x0b2\x1f.hadoop.hd\
    fs.DatanodeInfosProto\x12:\n\x12targetStorageUuids\x18\x05\x20\x03(\x0b2\
    \x1e.hadoop.hdfs.StorageUuidsProto\x12:\n\x12targetStorageTypes\x18\x06\
    \x20\x03(\x0b2\x1e.hadoop.hdfs.StorageTypesProto\"4\n\x06Action\x12\x0c\
    \n\x08TRANSFER\x10\x01\x12\x0e\n\nINVALIDATE\x10\x02\x12\x0c\n\x08SHUTDO\
    WN\x10\x03\"\xa4\x01\n\x13BlockIdCommandProto\x12@\n\x06action\x18\x01\
    \x20\x02(\x0e20.hadoop.hdfs.datanode.BlockIdCommandProto.Action\x12\x13\
    \n\x0bblockPoolId\x18\x02\x20\x02(\t\x12\x14\n\x08blockIds\x18\x03\x20\
    \x03(\x04B\x02\x10\x01\"\x20\n\x06Action\x12\t\n\x05CACHE\x10\x01\x12\
    \x0b\n\x07UNCACHE\x10\x02\"N\n\x19BlockRecoveryCommandProto\x121\n\x06bl\
    ocks\x18\x01\x20\x03(\x0b2!.hadoop.hdfs.RecoveringBlockProto\"+\n\x14Fin\
    alizeCommandProto\x12\x13\n\x0bblockPoolId\x18\x01\x20\x02(\t\"J\n\x15Ke\
    yUpdateCommandProto\x121\n\x04keys\x18\x01\x20\x02(\x0b2#.hadoop.hdfs.Ex\
    portedBlockKeysProto\"\x16\n\x14RegisterCommandProto\"e\n\x1cRegisterDat\
    anodeRequestProto\x12E\n\x0cregistration\x18\x01\x20\x02(\x0b2/.hadoop.h\
    dfs.datanode.DatanodeRegistrationProto\"f\n\x1dRegisterDatanodeResponseP\
    roto\x12E\n\x0cregistration\x18\x01\x20\x02(\x0b2/.hadoop.hdfs.datanode.\
    DatanodeRegistrationProto\"\x8f\x02\n\x15HeartbeatRequestProto\x12E\n\
    \x0cregistration\x18\x01\x20\x02(\x0b2/.hadoop.hdfs.datanode.DatanodeReg\
    istrationProto\x120\n\x07reports\x18\x02\x20\x03(\x0b2\x1f.hadoop.hdfs.S\
    torageReportProto\x12\x1a\n\x0fxmitsInProgress\x18\x03\x20\x01(\r:\x010\
    \x12\x17\n\x0cxceiverCount\x18\x04\x20\x01(\r:\x010\x12\x18\n\rfailedVol\
    umes\x18\x05\x20\x01(\r:\x010\x12\x18\n\rcacheCapacity\x18\x06\x20\x01(\
    \x04:\x010\x12\x14\n\tcacheUsed\x18\x07\x20\x01(\x04:\x010\"\x8f\x01\n\
    \x18NNHAStatusHeartbeatProto\x12C\n\x05state\x18\x01\x20\x02(\x0e24.hado\
    op.hdfs.datanode.NNHAStatusHeartbeatProto.State\x12\x0c\n\x04txid\x18\
    \x02\x20\x02(\x04\"\x20\n\x05State\x12\n\n\x06ACTIVE\x10\0\x12\x0b\n\x07\
    STANDBY\x10\x01\"\xda\x01\n\x16HeartbeatResponseProto\x128\n\x04cmds\x18\
    \x01\x20\x03(\x0b2*.hadoop.hdfs.datanode.DatanodeCommandProto\x12@\n\x08\
    haStatus\x18\x02\x20\x02(\x0b2..hadoop.hdfs.datanode.NNHAStatusHeartbeat\
    Proto\x12D\n\x14rollingUpgradeStatus\x18\x03\x20\x01(\x0b2&.hadoop.hdfs.\
    RollingUpgradeStatusProto\"\xb5\x01\n\x17BlockReportRequestProto\x12E\n\
    \x0cregistration\x18\x01\x20\x02(\x0b2/.hadoop.hdfs.datanode.DatanodeReg\
    istrationProto\x12\x13\n\x0bblockPoolId\x18\x02\x20\x02(\t\x12>\n\x07rep\
    orts\x18\x03\x20\x03(\x0b2-.hadoop.hdfs.datanode.StorageBlockReportProto\
    \"a\n\x17StorageBlockReportProto\x122\n\x07storage\x18\x01\x20\x02(\x0b2\
    !.hadoop.hdfs.DatanodeStorageProto\x12\x12\n\x06blocks\x18\x02\x20\x03(\
    \x04B\x02\x10\x01\"S\n\x18BlockReportResponseProto\x127\n\x03cmd\x18\x01\
    \x20\x01(\x0b2*.hadoop.hdfs.datanode.DatanodeCommandProto\"\x89\x01\n\
    \x17CacheReportRequestProto\x12E\n\x0cregistration\x18\x01\x20\x02(\x0b2\
    /.hadoop.hdfs.datanode.DatanodeRegistrationProto\x12\x13\n\x0bblockPoolI\
    d\x18\x02\x20\x02(\t\x12\x12\n\x06blocks\x18\x03\x20\x03(\x04B\x02\x10\
    \x01\"S\n\x18CacheReportResponseProto\x127\n\x03cmd\x18\x01\x20\x01(\x0b\
    2*.hadoop.hdfs.datanode.DatanodeCommandProto\"\xe5\x01\n\x1dReceivedDele\
    tedBlockInfoProto\x12&\n\x05block\x18\x01\x20\x02(\x0b2\x17.hadoop.hdfs.\
    BlockProto\x12O\n\x06status\x18\x03\x20\x02(\x0e2?.hadoop.hdfs.datanode.\
    ReceivedDeletedBlockInfoProto.BlockStatus\x12\x12\n\ndeleteHint\x18\x02\
    \x20\x01(\t\"7\n\x0bBlockStatus\x12\r\n\tRECEIVING\x10\x01\x12\x0c\n\x08\
    RECEIVED\x10\x02\x12\x0b\n\x07DELETED\x10\x03\"\xb5\x01\n!StorageReceive\
    dDeletedBlocksProto\x12\x17\n\x0bstorageUuid\x18\x01\x20\x02(\tB\x02\x18\
    \x01\x12C\n\x06blocks\x18\x02\x20\x03(\x0b23.hadoop.hdfs.datanode.Receiv\
    edDeletedBlockInfoProto\x122\n\x07storage\x18\x03\x20\x01(\x0b2!.hadoop.\
    hdfs.DatanodeStorageProto\"\xca\x01\n#BlockReceivedAndDeletedRequestProt\
    o\x12E\n\x0cregistration\x18\x01\x20\x02(\x0b2/.hadoop.hdfs.datanode.Dat\
    anodeRegistrationProto\x12\x13\n\x0bblockPoolId\x18\x02\x20\x02(\t\x12G\
    \n\x06blocks\x18\x03\x20\x03(\x0b27.hadoop.hdfs.datanode.StorageReceived\
    DeletedBlocksProto\"&\n$BlockReceivedAndDeletedResponseProto\"\xd2\x01\n\
    \x17ErrorReportRequestProto\x12E\n\x0cregistartion\x18\x01\x20\x02(\x0b2\
    /.hadoop.hdfs.datanode.DatanodeRegistrationProto\x12\x11\n\terrorCode\
    \x18\x02\x20\x02(\r\x12\x0b\n\x03msg\x18\x03\x20\x02(\t\"P\n\tErrorCode\
    \x12\n\n\x06NOTIFY\x10\0\x12\x0e\n\nDISK_ERROR\x10\x01\x12\x11\n\rINVALI\
    D_BLOCK\x10\x02\x12\x14\n\x10FATAL_DISK_ERROR\x10\x03\"\x1a\n\x18ErrorRe\
    portResponseProto\"M\n\x1bReportBadBlocksRequestProto\x12.\n\x06blocks\
    \x18\x01\x20\x03(\x0b2\x1e.hadoop.hdfs.LocatedBlockProto\"\x1e\n\x1cRepo\
    rtBadBlocksResponseProto\"\xf6\x01\n&CommitBlockSynchronizationRequestPr\
    oto\x12.\n\x05block\x18\x01\x20\x02(\x0b2\x1f.hadoop.hdfs.ExtendedBlockP\
    roto\x12\x13\n\x0bnewGenStamp\x18\x02\x20\x02(\x04\x12\x11\n\tnewLength\
    \x18\x03\x20\x02(\x04\x12\x11\n\tcloseFile\x18\x04\x20\x02(\x08\x12\x13\
    \n\x0bdeleteBlock\x18\x05\x20\x02(\x08\x121\n\x0bnewTaragets\x18\x06\x20\
    \x03(\x0b2\x1c.hadoop.hdfs.DatanodeIDProto\x12\x19\n\x11newTargetStorage\
    s\x18\x07\x20\x03(\t\")\n'CommitBlockSynchronizationResponseProto2\xcc\
    \x08\n\x17DatanodeProtocolService\x12{\n\x10registerDatanode\x122.hadoop\
    .hdfs.datanode.RegisterDatanodeRequestProto\x1a3.hadoop.hdfs.datanode.Re\
    gisterDatanodeResponseProto\x12j\n\rsendHeartbeat\x12+.hadoop.hdfs.datan\
    ode.HeartbeatRequestProto\x1a,.hadoop.hdfs.datanode.HeartbeatResponsePro\
    to\x12l\n\x0bblockReport\x12-.hadoop.hdfs.datanode.BlockReportRequestPro\
    to\x1a..hadoop.hdfs.datanode.BlockReportResponseProto\x12l\n\x0bcacheRep\
    ort\x12-.hadoop.hdfs.datanode.CacheReportRequestProto\x1a..hadoop.hdfs.d\
    atanode.CacheReportResponseProto\x12\x90\x01\n\x17blockReceivedAndDelete\
    d\x129.hadoop.hdfs.datanode.BlockReceivedAndDeletedRequestProto\x1a:.had\
    oop.hdfs.datanode.BlockReceivedAndDeletedResponseProto\x12l\n\x0berrorRe\
    port\x12-.hadoop.hdfs.datanode.ErrorReportRequestProto\x1a..hadoop.hdfs.\
    datanode.ErrorReportResponseProto\x12U\n\x0eversionRequest\x12\x20.hadoo\
    p.hdfs.VersionRequestProto\x1a!.hadoop.hdfs.VersionResponseProto\x12x\n\
    \x0freportBadBlocks\x121.hadoop.hdfs.datanode.ReportBadBlocksRequestProt\
    o\x1a2.hadoop.hdfs.datanode.ReportBadBlocksResponseProto\x12\x99\x01\n\
    \x1acommitBlockSynchronization\x12<.hadoop.hdfs.datanode.CommitBlockSync\
    hronizationRequestProto\x1a=.hadoop.hdfs.datanode.CommitBlockSynchroniza\
    tionResponseProtoBE\n%org.apache.hadoop.hdfs.protocol.protoB\x16Datanode\
    ProtocolProtos\xa0\x01\x01\x88\x01\x01\
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
