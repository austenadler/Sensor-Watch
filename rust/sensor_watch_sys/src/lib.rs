#![cfg_attr(not(target_arch = "wasm32"), no_std)]
#![allow(rustdoc::bare_urls)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
// We're importing from c, give me a break
#![allow(non_upper_case_globals)]
use core::ffi::{c_uint, c_void, CStr};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern "C" {
    pub fn malloc(size: usize) -> *mut c_void;
}
