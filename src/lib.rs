//#![feature(link_llvm_intrinsics)] // :3
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(static_mut_refs)]
#![allow(unused_variables)]

//unsafe extern "C" {
//    #[link_name = "llvm.returnaddress"]
//    pub fn return_address(a: i32) -> *const u8;
//}

mod exports;

mod type_defs;

mod backend;
mod egl;

mod make_mesa_happy;

use core::panic;
use std::{
    ffi::{CStr, c_int},
    process::exit,
    sync::LazyLock,
};

use libc::strsignal;
use parking_lot::Mutex;

use crate::backend::{Window, wayland::WaylandWindow};

static mut WINDOW: Mutex<LazyLock<Box<dyn Window>>> =
    Mutex::new(LazyLock::new(|| Box::new(WaylandWindow::new())));

pub fn window<'a>() -> &'a Mutex<LazyLock<Box<dyn Window>>> {
    unsafe { &WINDOW }
}
unsafe extern "C" fn sigsegv_handler(signum: c_int) {
    let bt = std::backtrace::Backtrace::force_capture();
    panic!(
        "\n{}\nBacktrace:\n{}",
        CStr::from_ptr(strsignal(signum)).to_string_lossy(),
        // return_address(0),
        bt.to_string()
    );
}
