use wayland_client::{Dispatch, protocol::wl_compositor::WlCompositor};

use crate::backend::wayland::WaylandState;

impl Dispatch<WlCompositor, ()> for WaylandState {
    fn event(
        state: &mut Self,
        proxy: &WlCompositor,
        event: <WlCompositor as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
    }
}
