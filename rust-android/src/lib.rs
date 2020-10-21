#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use std::ffi::{CString, CStr};
use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::{jstring};
use rust_core::database_test;

#[no_mangle]
pub unsafe extern fn Java_com_example_android_MainActivity_calldatabase(env: JNIEnv, _: JObject, j_recipient: JString) -> jstring {
    let database_path_c_string = CString::from(
        CStr::from_ptr(
            env.get_string(j_recipient).unwrap().as_ptr()
        )
    );
    let database_path = database_path_c_string.to_str().unwrap().to_string();

    let database_result = database_test(database_path);

    let output = env.new_string(database_result.to_owned()).unwrap();
    output.into_inner()
}