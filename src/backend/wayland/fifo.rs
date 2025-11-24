use wayland_client::delegate_noop;
use wayland_protocols::wp::fifo::v1::client::{
    wp_fifo_manager_v1::WpFifoManagerV1, wp_fifo_v1::WpFifoV1,
};

use crate::backend::wayland::WaylandState;

impl wayland_client::Dispatch<WpFifoManagerV1, ()> for WaylandState {
    fn event(
        state: &mut Self,
        manager: &WpFifoManagerV1,
        event: <WpFifoManagerV1 as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        while let None = state.compositor_surface {}
    }
}
delegate_noop!(WaylandState: ignore WpFifoV1);
