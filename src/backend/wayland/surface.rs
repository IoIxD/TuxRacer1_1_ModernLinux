use std::os::raw::c_void;

use wayland_client::{
    Connection, Dispatch, Proxy, QueueHandle,
    protocol::wl_surface::{self, WlSurface},
};
use wayland_egl::WlEglSurface;

use crate::backend::wayland::WaylandState;

impl Dispatch<WlSurface, ()> for WaylandState {
    fn event(
        state: &mut Self,
        surface: &WlSurface,
        event: wl_surface::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        panic!("surface dispatch");
    }
}
