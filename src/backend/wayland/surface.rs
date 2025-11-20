
use wayland_client::{
    Connection, Dispatch, QueueHandle,
    protocol::wl_surface::{self, WlSurface},
};

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
