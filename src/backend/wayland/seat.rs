use wayland_client::{
    Connection, Dispatch, QueueHandle,
    protocol::wl_seat::WlSeat,
};

use crate::backend::wayland::WaylandState;

impl Dispatch<WlSeat, ()> for WaylandState {
    fn event(
        state: &mut Self,
        proxy: &WlSeat,
        event: <WlSeat as wayland_client::Proxy>::Event,
        data: &(),
        conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        let _ = event;
    }
}
