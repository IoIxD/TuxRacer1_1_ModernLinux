use wayland_client::{
    Connection, Dispatch, QueueHandle, WEnum,
    protocol::wl_seat::{self, WlSeat},
};

use crate::backend::wayland::WaylandState;

impl Dispatch<WlSeat, ()> for WaylandState {
    fn event(
        state: &mut Self,
        seat: &WlSeat,
        event: <WlSeat as wayland_client::Proxy>::Event,
        data: &(),
        conn: &Connection,
        qhandle: &QueueHandle<Self>,
    ) {
        let _ = event;
        if let wl_seat::Event::Capabilities {
            capabilities: WEnum::Value(capabilities),
        } = event
        {
            if capabilities.contains(wl_seat::Capability::Keyboard) {
                seat.get_keyboard(qhandle, ());
            }
            if capabilities.contains(wl_seat::Capability::Pointer) {
                seat.get_pointer(qhandle, ());
            }
        }
    }
}
