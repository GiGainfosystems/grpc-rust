// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct HelloRequest {
    // message fields
    pub name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HelloRequest {}

impl HelloRequest {
    pub fn new() -> HelloRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HelloRequest {
        static mut instance: ::protobuf::lazy::Lazy<HelloRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HelloRequest,
        };
        unsafe {
            instance.get(HelloRequest::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }
}

impl ::protobuf::Message for HelloRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.name != ::std::string::String::new() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.name != ::std::string::String::new() {
            try!(os.write_string(1, &self.name));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<HelloRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for HelloRequest {
    fn new() -> HelloRequest {
        HelloRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<HelloRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    HelloRequest::get_name_for_reflect,
                    HelloRequest::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HelloRequest>(
                    "HelloRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HelloRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for HelloRequest {
    fn eq(&self, other: &HelloRequest) -> bool {
        self.name == other.name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for HelloRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HelloRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,Default)]
pub struct HelloReply {
    // message fields
    pub message: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HelloReply {}

impl HelloReply {
    pub fn new() -> HelloReply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HelloReply {
        static mut instance: ::protobuf::lazy::Lazy<HelloReply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HelloReply,
        };
        unsafe {
            instance.get(HelloReply::new)
        }
    }

    // string message = 1;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    fn get_message_for_reflect(&self) -> &::std::string::String {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }
}

impl ::protobuf::Message for HelloReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.message));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.message != ::std::string::String::new() {
            my_size += ::protobuf::rt::string_size(1, &self.message);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.message != ::std::string::String::new() {
            try!(os.write_string(1, &self.message));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<HelloReply>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for HelloReply {
    fn new() -> HelloReply {
        HelloReply::new()
    }

    fn descriptor_static(_: ::std::option::Option<HelloReply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    HelloReply::get_message_for_reflect,
                    HelloReply::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HelloReply>(
                    "HelloReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HelloReply {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for HelloReply {
    fn eq(&self, other: &HelloReply) -> bool {
        self.message == other.message &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for HelloReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HelloReply {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x10, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x0a, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x22, 0x22,
    0x0a, 0x0c, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12,
    0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x22, 0x26, 0x0a, 0x0a, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x52, 0x65, 0x70, 0x6c, 0x79,
    0x12, 0x18, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x32, 0x49, 0x0a, 0x07, 0x47, 0x72,
    0x65, 0x65, 0x74, 0x65, 0x72, 0x12, 0x3e, 0x0a, 0x08, 0x53, 0x61, 0x79, 0x48, 0x65, 0x6c, 0x6c,
    0x6f, 0x12, 0x18, 0x2e, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x2e, 0x48,
    0x65, 0x6c, 0x6c, 0x6f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x16, 0x2e, 0x68, 0x65,
    0x6c, 0x6c, 0x6f, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x2e, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x52, 0x65,
    0x70, 0x6c, 0x79, 0x22, 0x00, 0x42, 0x36, 0x0a, 0x1b, 0x69, 0x6f, 0x2e, 0x67, 0x72, 0x70, 0x63,
    0x2e, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x73, 0x2e, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x77,
    0x6f, 0x72, 0x6c, 0x64, 0x42, 0x0f, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x57, 0x6f, 0x72, 0x6c, 0x64,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x48, 0x4c, 0x57, 0x4a, 0xeb, 0x11,
    0x0a, 0x06, 0x12, 0x04, 0x1d, 0x00, 0x34, 0x01, 0x0a, 0xe7, 0x0b, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x1d, 0x00, 0x12, 0x32, 0xdc, 0x0b, 0x20, 0x43, 0x6f, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74,
    0x20, 0x32, 0x30, 0x31, 0x35, 0x2c, 0x20, 0x47, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x20, 0x49, 0x6e,
    0x63, 0x2e, 0x0a, 0x20, 0x41, 0x6c, 0x6c, 0x20, 0x72, 0x69, 0x67, 0x68, 0x74, 0x73, 0x20, 0x72,
    0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x20, 0x52, 0x65, 0x64, 0x69, 0x73,
    0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x75, 0x73,
    0x65, 0x20, 0x69, 0x6e, 0x20, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x62, 0x69, 0x6e, 0x61, 0x72, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x73, 0x2c, 0x20, 0x77, 0x69,
    0x74, 0x68, 0x20, 0x6f, 0x72, 0x20, 0x77, 0x69, 0x74, 0x68, 0x6f, 0x75, 0x74, 0x0a, 0x20, 0x6d,
    0x6f, 0x64, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2c, 0x20, 0x61, 0x72, 0x65,
    0x20, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69,
    0x64, 0x65, 0x64, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x6f, 0x6c,
    0x6c, 0x6f, 0x77, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x20, 0x61, 0x72, 0x65, 0x0a, 0x20, 0x6d, 0x65, 0x74, 0x3a, 0x0a, 0x0a, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x2a, 0x20, 0x52, 0x65, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x63, 0x6f,
    0x64, 0x65, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x72, 0x65, 0x74, 0x61, 0x69, 0x6e, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x61, 0x62, 0x6f, 0x76, 0x65, 0x20, 0x63, 0x6f, 0x70, 0x79, 0x72, 0x69, 0x67,
    0x68, 0x74, 0x0a, 0x20, 0x6e, 0x6f, 0x74, 0x69, 0x63, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73,
    0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x6f, 0x6c, 0x6c,
    0x6f, 0x77, 0x69, 0x6e, 0x67, 0x20, 0x64, 0x69, 0x73, 0x63, 0x6c, 0x61, 0x69, 0x6d, 0x65, 0x72,
    0x2e, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x2a, 0x20, 0x52, 0x65, 0x64, 0x69, 0x73, 0x74, 0x72,
    0x69, 0x62, 0x75, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x62, 0x69, 0x6e, 0x61,
    0x72, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x72, 0x65, 0x70,
    0x72, 0x6f, 0x64, 0x75, 0x63, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x62, 0x6f, 0x76, 0x65,
    0x0a, 0x20, 0x63, 0x6f, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20, 0x6e, 0x6f, 0x74, 0x69,
    0x63, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66,
    0x20, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x69, 0x6e, 0x67, 0x20, 0x64, 0x69,
    0x73, 0x63, 0x6c, 0x61, 0x69, 0x6d, 0x65, 0x72, 0x0a, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x64, 0x6f, 0x63, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61,
    0x6e, 0x64, 0x2f, 0x6f, 0x72, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x6d, 0x61, 0x74, 0x65,
    0x72, 0x69, 0x61, 0x6c, 0x73, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x64, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62,
    0x75, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x2a, 0x20, 0x4e, 0x65,
    0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6f,
    0x66, 0x20, 0x47, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x20, 0x49, 0x6e, 0x63, 0x2e, 0x20, 0x6e, 0x6f,
    0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x69,
    0x74, 0x73, 0x0a, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x6f, 0x72, 0x73,
    0x20, 0x6d, 0x61, 0x79, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20,
    0x65, 0x6e, 0x64, 0x6f, 0x72, 0x73, 0x65, 0x20, 0x6f, 0x72, 0x20, 0x70, 0x72, 0x6f, 0x6d, 0x6f,
    0x74, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x74, 0x73, 0x20, 0x64, 0x65, 0x72, 0x69,
    0x76, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x0a, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x73,
    0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x20, 0x77, 0x69, 0x74, 0x68, 0x6f, 0x75, 0x74, 0x20,
    0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x70, 0x72, 0x69, 0x6f, 0x72, 0x20, 0x77,
    0x72, 0x69, 0x74, 0x74, 0x65, 0x6e, 0x20, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f,
    0x6e, 0x2e, 0x0a, 0x0a, 0x20, 0x54, 0x48, 0x49, 0x53, 0x20, 0x53, 0x4f, 0x46, 0x54, 0x57, 0x41,
    0x52, 0x45, 0x20, 0x49, 0x53, 0x20, 0x50, 0x52, 0x4f, 0x56, 0x49, 0x44, 0x45, 0x44, 0x20, 0x42,
    0x59, 0x20, 0x54, 0x48, 0x45, 0x20, 0x43, 0x4f, 0x50, 0x59, 0x52, 0x49, 0x47, 0x48, 0x54, 0x20,
    0x48, 0x4f, 0x4c, 0x44, 0x45, 0x52, 0x53, 0x20, 0x41, 0x4e, 0x44, 0x20, 0x43, 0x4f, 0x4e, 0x54,
    0x52, 0x49, 0x42, 0x55, 0x54, 0x4f, 0x52, 0x53, 0x0a, 0x20, 0x22, 0x41, 0x53, 0x20, 0x49, 0x53,
    0x22, 0x20, 0x41, 0x4e, 0x44, 0x20, 0x41, 0x4e, 0x59, 0x20, 0x45, 0x58, 0x50, 0x52, 0x45, 0x53,
    0x53, 0x20, 0x4f, 0x52, 0x20, 0x49, 0x4d, 0x50, 0x4c, 0x49, 0x45, 0x44, 0x20, 0x57, 0x41, 0x52,
    0x52, 0x41, 0x4e, 0x54, 0x49, 0x45, 0x53, 0x2c, 0x20, 0x49, 0x4e, 0x43, 0x4c, 0x55, 0x44, 0x49,
    0x4e, 0x47, 0x2c, 0x20, 0x42, 0x55, 0x54, 0x20, 0x4e, 0x4f, 0x54, 0x0a, 0x20, 0x4c, 0x49, 0x4d,
    0x49, 0x54, 0x45, 0x44, 0x20, 0x54, 0x4f, 0x2c, 0x20, 0x54, 0x48, 0x45, 0x20, 0x49, 0x4d, 0x50,
    0x4c, 0x49, 0x45, 0x44, 0x20, 0x57, 0x41, 0x52, 0x52, 0x41, 0x4e, 0x54, 0x49, 0x45, 0x53, 0x20,
    0x4f, 0x46, 0x20, 0x4d, 0x45, 0x52, 0x43, 0x48, 0x41, 0x4e, 0x54, 0x41, 0x42, 0x49, 0x4c, 0x49,
    0x54, 0x59, 0x20, 0x41, 0x4e, 0x44, 0x20, 0x46, 0x49, 0x54, 0x4e, 0x45, 0x53, 0x53, 0x20, 0x46,
    0x4f, 0x52, 0x0a, 0x20, 0x41, 0x20, 0x50, 0x41, 0x52, 0x54, 0x49, 0x43, 0x55, 0x4c, 0x41, 0x52,
    0x20, 0x50, 0x55, 0x52, 0x50, 0x4f, 0x53, 0x45, 0x20, 0x41, 0x52, 0x45, 0x20, 0x44, 0x49, 0x53,
    0x43, 0x4c, 0x41, 0x49, 0x4d, 0x45, 0x44, 0x2e, 0x20, 0x49, 0x4e, 0x20, 0x4e, 0x4f, 0x20, 0x45,
    0x56, 0x45, 0x4e, 0x54, 0x20, 0x53, 0x48, 0x41, 0x4c, 0x4c, 0x20, 0x54, 0x48, 0x45, 0x20, 0x43,
    0x4f, 0x50, 0x59, 0x52, 0x49, 0x47, 0x48, 0x54, 0x0a, 0x20, 0x4f, 0x57, 0x4e, 0x45, 0x52, 0x20,
    0x4f, 0x52, 0x20, 0x43, 0x4f, 0x4e, 0x54, 0x52, 0x49, 0x42, 0x55, 0x54, 0x4f, 0x52, 0x53, 0x20,
    0x42, 0x45, 0x20, 0x4c, 0x49, 0x41, 0x42, 0x4c, 0x45, 0x20, 0x46, 0x4f, 0x52, 0x20, 0x41, 0x4e,
    0x59, 0x20, 0x44, 0x49, 0x52, 0x45, 0x43, 0x54, 0x2c, 0x20, 0x49, 0x4e, 0x44, 0x49, 0x52, 0x45,
    0x43, 0x54, 0x2c, 0x20, 0x49, 0x4e, 0x43, 0x49, 0x44, 0x45, 0x4e, 0x54, 0x41, 0x4c, 0x2c, 0x0a,
    0x20, 0x53, 0x50, 0x45, 0x43, 0x49, 0x41, 0x4c, 0x2c, 0x20, 0x45, 0x58, 0x45, 0x4d, 0x50, 0x4c,
    0x41, 0x52, 0x59, 0x2c, 0x20, 0x4f, 0x52, 0x20, 0x43, 0x4f, 0x4e, 0x53, 0x45, 0x51, 0x55, 0x45,
    0x4e, 0x54, 0x49, 0x41, 0x4c, 0x20, 0x44, 0x41, 0x4d, 0x41, 0x47, 0x45, 0x53, 0x20, 0x28, 0x49,
    0x4e, 0x43, 0x4c, 0x55, 0x44, 0x49, 0x4e, 0x47, 0x2c, 0x20, 0x42, 0x55, 0x54, 0x20, 0x4e, 0x4f,
    0x54, 0x0a, 0x20, 0x4c, 0x49, 0x4d, 0x49, 0x54, 0x45, 0x44, 0x20, 0x54, 0x4f, 0x2c, 0x20, 0x50,
    0x52, 0x4f, 0x43, 0x55, 0x52, 0x45, 0x4d, 0x45, 0x4e, 0x54, 0x20, 0x4f, 0x46, 0x20, 0x53, 0x55,
    0x42, 0x53, 0x54, 0x49, 0x54, 0x55, 0x54, 0x45, 0x20, 0x47, 0x4f, 0x4f, 0x44, 0x53, 0x20, 0x4f,
    0x52, 0x20, 0x53, 0x45, 0x52, 0x56, 0x49, 0x43, 0x45, 0x53, 0x3b, 0x20, 0x4c, 0x4f, 0x53, 0x53,
    0x20, 0x4f, 0x46, 0x20, 0x55, 0x53, 0x45, 0x2c, 0x0a, 0x20, 0x44, 0x41, 0x54, 0x41, 0x2c, 0x20,
    0x4f, 0x52, 0x20, 0x50, 0x52, 0x4f, 0x46, 0x49, 0x54, 0x53, 0x3b, 0x20, 0x4f, 0x52, 0x20, 0x42,
    0x55, 0x53, 0x49, 0x4e, 0x45, 0x53, 0x53, 0x20, 0x49, 0x4e, 0x54, 0x45, 0x52, 0x52, 0x55, 0x50,
    0x54, 0x49, 0x4f, 0x4e, 0x29, 0x20, 0x48, 0x4f, 0x57, 0x45, 0x56, 0x45, 0x52, 0x20, 0x43, 0x41,
    0x55, 0x53, 0x45, 0x44, 0x20, 0x41, 0x4e, 0x44, 0x20, 0x4f, 0x4e, 0x20, 0x41, 0x4e, 0x59, 0x0a,
    0x20, 0x54, 0x48, 0x45, 0x4f, 0x52, 0x59, 0x20, 0x4f, 0x46, 0x20, 0x4c, 0x49, 0x41, 0x42, 0x49,
    0x4c, 0x49, 0x54, 0x59, 0x2c, 0x20, 0x57, 0x48, 0x45, 0x54, 0x48, 0x45, 0x52, 0x20, 0x49, 0x4e,
    0x20, 0x43, 0x4f, 0x4e, 0x54, 0x52, 0x41, 0x43, 0x54, 0x2c, 0x20, 0x53, 0x54, 0x52, 0x49, 0x43,
    0x54, 0x20, 0x4c, 0x49, 0x41, 0x42, 0x49, 0x4c, 0x49, 0x54, 0x59, 0x2c, 0x20, 0x4f, 0x52, 0x20,
    0x54, 0x4f, 0x52, 0x54, 0x0a, 0x20, 0x28, 0x49, 0x4e, 0x43, 0x4c, 0x55, 0x44, 0x49, 0x4e, 0x47,
    0x20, 0x4e, 0x45, 0x47, 0x4c, 0x49, 0x47, 0x45, 0x4e, 0x43, 0x45, 0x20, 0x4f, 0x52, 0x20, 0x4f,
    0x54, 0x48, 0x45, 0x52, 0x57, 0x49, 0x53, 0x45, 0x29, 0x20, 0x41, 0x52, 0x49, 0x53, 0x49, 0x4e,
    0x47, 0x20, 0x49, 0x4e, 0x20, 0x41, 0x4e, 0x59, 0x20, 0x57, 0x41, 0x59, 0x20, 0x4f, 0x55, 0x54,
    0x20, 0x4f, 0x46, 0x20, 0x54, 0x48, 0x45, 0x20, 0x55, 0x53, 0x45, 0x0a, 0x20, 0x4f, 0x46, 0x20,
    0x54, 0x48, 0x49, 0x53, 0x20, 0x53, 0x4f, 0x46, 0x54, 0x57, 0x41, 0x52, 0x45, 0x2c, 0x20, 0x45,
    0x56, 0x45, 0x4e, 0x20, 0x49, 0x46, 0x20, 0x41, 0x44, 0x56, 0x49, 0x53, 0x45, 0x44, 0x20, 0x4f,
    0x46, 0x20, 0x54, 0x48, 0x45, 0x20, 0x50, 0x4f, 0x53, 0x53, 0x49, 0x42, 0x49, 0x4c, 0x49, 0x54,
    0x59, 0x20, 0x4f, 0x46, 0x20, 0x53, 0x55, 0x43, 0x48, 0x20, 0x44, 0x41, 0x4d, 0x41, 0x47, 0x45,
    0x2e, 0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1f, 0x00, 0x22, 0x0a, 0x0b, 0x0a, 0x04,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1f, 0x00, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x1f, 0x07, 0x1a, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x1f, 0x07, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x1f, 0x07, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12,
    0x03, 0x1f, 0x1d, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x20, 0x00, 0x34, 0x0a, 0x0b,
    0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x20, 0x00, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x08,
    0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x20, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x20, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x20, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01,
    0x07, 0x12, 0x03, 0x20, 0x16, 0x33, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x21, 0x00, 0x30,
    0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x02, 0x12, 0x03, 0x21, 0x00, 0x30, 0x0a, 0x0c, 0x0a,
    0x05, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x12, 0x03, 0x21, 0x07, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x08,
    0xe7, 0x07, 0x02, 0x02, 0x00, 0x12, 0x03, 0x21, 0x07, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7,
    0x07, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x07, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7,
    0x07, 0x02, 0x07, 0x12, 0x03, 0x21, 0x1e, 0x2f, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x22,
    0x00, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x03, 0x12, 0x03, 0x22, 0x00, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x03, 0x02, 0x12, 0x03, 0x22, 0x07, 0x18, 0x0a, 0x0d, 0x0a,
    0x06, 0x08, 0xe7, 0x07, 0x03, 0x02, 0x00, 0x12, 0x03, 0x22, 0x07, 0x18, 0x0a, 0x0e, 0x0a, 0x07,
    0x08, 0xe7, 0x07, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22, 0x07, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x03, 0x07, 0x12, 0x03, 0x22, 0x1b, 0x20, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x24, 0x08, 0x12, 0x0a, 0x2e, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x27, 0x00, 0x2a, 0x01,
    0x1a, 0x22, 0x20, 0x54, 0x68, 0x65, 0x20, 0x67, 0x72, 0x65, 0x65, 0x74, 0x69, 0x6e, 0x67, 0x20,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x69, 0x74, 0x69,
    0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x27, 0x08, 0x0f,
    0x0a, 0x1f, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x29, 0x02, 0x35, 0x1a, 0x12, 0x20,
    0x53, 0x65, 0x6e, 0x64, 0x73, 0x20, 0x61, 0x20, 0x67, 0x72, 0x65, 0x65, 0x74, 0x69, 0x6e, 0x67,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x06, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x29, 0x10, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x27, 0x31, 0x0a, 0x3d, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x2d, 0x00, 0x2f, 0x01, 0x1a, 0x31, 0x20, 0x54, 0x68, 0x65, 0x20, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x63, 0x6f,
    0x6e, 0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x73, 0x65,
    0x72, 0x27, 0x73, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x2d, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x2e, 0x02, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x2e, 0x02,
    0x2d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2e, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x09, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2e, 0x10, 0x11, 0x0a, 0x3b, 0x0a, 0x02,
    0x04, 0x01, 0x12, 0x04, 0x32, 0x00, 0x34, 0x01, 0x1a, 0x2f, 0x20, 0x54, 0x68, 0x65, 0x20, 0x72,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20,
    0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67,
    0x72, 0x65, 0x65, 0x74, 0x69, 0x6e, 0x67, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01,
    0x12, 0x03, 0x32, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x33,
    0x02, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x33, 0x02, 0x32,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x33, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x33, 0x09, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x33, 0x13, 0x14, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33,
];

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
