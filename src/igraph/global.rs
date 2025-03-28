use std::ffi::{CStr};
use std::os::raw::{c_char};
use crate::igraph::ffi::*;

pub fn version_string() -> String {
    unsafe {
        // Also get numeric components
        let mut version_ptr: *const c_char = std::ptr::null();
        let mut major = 0;
        let mut minor = 0;
        let mut patch = 0;
        igraph_version(&mut version_ptr, &mut major, &mut minor, &mut patch);


        let version_str = CStr::from_ptr(version_ptr)
            .to_string_lossy()
            .into_owned();
        format!("({}: {}.{}.{})", version_str, major, minor, patch)
    }
}


pub fn enable_attr_table() -> &'static igraph_attribute_table_t {
    unsafe {
        let attr = &igraph_cattribute_table;
        igraph_set_attribute_table(attr);
        attr
    }
}