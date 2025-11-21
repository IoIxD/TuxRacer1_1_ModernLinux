// Dude I spent a day and a half trying to debug this and I found out that the Mesa libraries were tranparently failing to load because of "missing zlib functions" for a version of it from 2005. I am going to cry.

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
use std::os::raw::{c_ulong, c_void};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn compressBound(sourceLen: c_ulong) -> c_ulong {
    panic!();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gzopen64(
    path: *const ::std::os::raw::c_char,
    mode: *const ::std::os::raw::c_char,
) -> *mut c_void {
    panic!();
}
