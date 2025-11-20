#![allow(static_mut_refs)]
#![allow(unused_variables)]

use std::sync::LazyLock;

use parking_lot::Mutex;

use crate::backend::{Window, wayland::WaylandWindow};

static mut WINDOW: Mutex<LazyLock<Box<dyn Window>>> =
    Mutex::new(LazyLock::new(|| Box::new(WaylandWindow::new())));

pub fn window<'a>() -> &'a Mutex<LazyLock<Box<dyn Window>>> {
    unsafe { &WINDOW }
}
