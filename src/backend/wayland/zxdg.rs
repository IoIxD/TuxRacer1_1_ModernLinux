use wayland_client::Dispatch;
use wayland_protocols::xdg::decoration::zv1::client::{
    zxdg_decoration_manager_v1::ZxdgDecorationManagerV1,
    zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1,
};

use crate::backend::wayland::WaylandState;

impl Dispatch<ZxdgDecorationManagerV1, ()> for WaylandState {
    fn event(
        state: &mut Self,
        proxy: &ZxdgDecorationManagerV1,
        event: <ZxdgDecorationManagerV1 as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        println!("dec manager");
    }
}
impl Dispatch<ZxdgToplevelDecorationV1, ()> for WaylandState {
    fn event(
        state: &mut Self,
        proxy: &ZxdgToplevelDecorationV1,
        event: <ZxdgToplevelDecorationV1 as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        println!("decoration");
    }
}
