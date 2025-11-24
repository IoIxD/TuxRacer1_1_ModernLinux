use wayland_client::{Dispatch, protocol::wl_pointer::WlPointer};

use crate::backend::wayland::WaylandState;
use wayland_client::protocol::wl_pointer::Event;

impl Dispatch<WlPointer, ()> for WaylandState {
    fn event(
        state: &mut Self,
        proxy: &WlPointer,
        event: <WlPointer as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        if state.pointer.is_none() {
            state.pointer = Some(proxy.clone());
        }
        match event {
            Event::Enter {
                serial,
                surface,
                surface_x,
                surface_y,
            } => {
                state.last_pointer_x = surface_x;
                state.last_pointer_y = surface_y;
                state.pointer_serial = serial;

                proxy.set_cursor(serial, None, 0, 0);
            }
            Event::Leave { serial, surface } => {
                proxy.set_cursor(serial, Some(&surface), 0, 0);
            }
            Event::Motion {
                time,
                surface_x,
                surface_y,
            } => {
                state.pointer_events.push(event);
            }
            Event::Button {
                serial,
                time,
                button,
                state: _,
            } => {
                state.pointer_events.push(event);
            }

            _ => {}
        }
    }
}
