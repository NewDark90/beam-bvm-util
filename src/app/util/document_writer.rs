use core::ffi::CStr;

use crate::{app::safe::*, common::extensions::*};

pub struct DocumentWriter {}

impl ObjectWriter for DocumentWriter { }
impl ArrayWriter for DocumentWriter { }
impl StringWriter for DocumentWriter { }
impl NumU32Writer for DocumentWriter { }
impl NumU64Writer for DocumentWriter { }
impl ObjectPropWriter for DocumentWriter { }
impl ArrayPropWriter for DocumentWriter { }
impl BlobPropWriter for DocumentWriter { }
impl StringPropWriter for DocumentWriter { }
impl NumU32PropWriter for DocumentWriter { }
impl NumU64PropWriter for DocumentWriter { }

pub struct ObjectFuncs {}

impl ObjectPropWriter for ObjectFuncs { }
impl ArrayPropWriter for ObjectFuncs { }
impl BlobPropWriter for ObjectFuncs { }
impl StringPropWriter for ObjectFuncs { }
impl NumU32PropWriter for ObjectFuncs { }
impl NumU64PropWriter for ObjectFuncs { }

pub struct ArrayFuncs {}

impl ObjectWriter for ArrayFuncs { }
impl StringWriter for ArrayFuncs { }
impl NumU32Writer for ArrayFuncs { }
impl NumU64Writer for ArrayFuncs { }



pub trait ObjectWriter {
    fn object<T: Fn(ObjectFuncs) -> ()>(self: &Self, props_fn: T) -> &Self{
        doc_add_group(CStr::empty());
        props_fn(ObjectFuncs {});
        doc_close_group();
        self
    }
}

pub trait ArrayWriter {
    fn array<T: Fn(ArrayFuncs) -> ()>(self: &Self, props_fn: T) -> &Self {
        doc_add_array(CStr::empty());
        props_fn(ArrayFuncs {});
        doc_close_array();
        self
    }
}

pub trait StringWriter {
    fn string(self: &Self, value: &CStr) -> &Self {
        doc_add_text(CStr::empty(), value);
        self
    }
}

pub trait NumU32Writer {
    fn u32(self: &Self, value: u32) -> &Self {
        doc_add_num32(CStr::empty(), value);
        self
    }
}

pub trait NumU64Writer {
    fn u64(self: &Self, value: u64) -> &Self {
        doc_add_num64(CStr::empty(), value);
        self
    }
}

pub trait ObjectPropWriter {
    fn object_prop<T: Fn(ObjectFuncs) -> ()>(self: &Self, prop_name: &CStr, props_fn: T) -> &Self {
        doc_add_group(prop_name);
        props_fn(ObjectFuncs {});
        doc_close_group();
        self
    }
}

pub trait ArrayPropWriter {
    fn array_prop<T: Fn(ArrayFuncs) -> ()>(self: &Self, prop_name: &CStr, props_fn: T) -> &Self {
        doc_add_array(prop_name);
        props_fn(ArrayFuncs {});
        doc_close_array();
        self 
    }
}

pub trait BlobPropWriter {
    fn blob_prop<V>(self: &Self, prop_name: &CStr, value: &V, value_size: u32) -> &Self {
        doc_add_blob(prop_name, value, value_size);
        self
    }
}

pub trait StringPropWriter {
    fn string_prop(self: &Self, prop_name: &CStr, value: &CStr) -> &Self {
        doc_add_text(prop_name, value);
        self
    }
}

pub trait NumU32PropWriter {
    fn u32_prop(self: &Self, prop_name: &CStr, value: u32) -> &Self {
        doc_add_num32(prop_name, value);
        self
    }
}

pub trait NumU64PropWriter {
    fn u64_prop(self: &Self, prop_name: &CStr, value: u64) -> &Self {
        doc_add_num64(prop_name, value);
        self
    }
}
