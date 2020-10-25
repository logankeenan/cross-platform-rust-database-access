use std::os::raw::{c_char};
use std::ffi::{CString, CStr};
use rust_core::database_test;

#[no_mangle]
pub extern fn call_database(to: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(to) };

    let database_path = c_str.to_str().unwrap().to_string();
    let database_result = database_test(database_path);

    CString::new(database_result).unwrap().into_raw()
}

#[no_mangle]
pub extern fn call_database_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}