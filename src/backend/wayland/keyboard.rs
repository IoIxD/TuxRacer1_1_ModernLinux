use wayland_client::{Dispatch, protocol::wl_keyboard::WlKeyboard};

use crate::backend::wayland::WaylandState;

impl Dispatch<WlKeyboard, ()> for WaylandState {
    fn event(
        state: &mut Self,
        proxy: &WlKeyboard,
        event: <WlKeyboard as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        // match event {}
    }
}
