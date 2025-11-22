use std::{fs::File, os::fd::AsFd};

use std::io::Write;
use wayland_client::{
    Connection, Dispatch, Proxy, QueueHandle,
    protocol::{
        wl_compositor,
        wl_registry::{self},
        wl_seat,
        wl_shm::{self},
    },
};
use wayland_egl::WlEglSurface;
use wayland_protocols::xdg::{
    decoration::zv1::client::zxdg_decoration_manager_v1::ZxdgDecorationManagerV1,
    shell::client::xdg_wm_base,
};

use crate::backend::wayland::WaylandState;

impl Dispatch<wl_registry::WlRegistry, ()> for WaylandState {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name, interface, ..
        } = event
        {
            match &interface[..] {
                "wl_compositor" => {
                    let compositor =
                        registry.bind::<wl_compositor::WlCompositor, _, _>(name, 1, qh, ());
                    let surface = compositor.create_surface(qh, ());

                    let region = compositor.create_region(qh, ());
                    region.add(0, 0, 640, 480);
                    surface.set_opaque_region(Some(&region));

                    let egl_surface = WlEglSurface::new(surface.id(), 640, 480).unwrap();
                    assert!(!egl_surface.ptr().is_null());

                    state.compositor = Some(compositor);
                    state.compositor_surface = Some(surface);
                    state.egl_surface = Some(egl_surface);

                    if state.wm_base.is_some() && state.xdg_surface.is_none() {
                        state.init_xdg_surface(qh);
                    }
                }
                "wl_shm" => {
                    let shm = registry.bind::<wl_shm::WlShm, _, _>(name, 1, qh, ());

                    let (init_w, init_h) = (320, 240);

                    let mut file = tempfile::tempfile().unwrap();
                    draw(&mut file, (init_w, init_h));
                    let pool = shm.create_pool(file.as_fd(), (init_w * init_h * 4) as i32, qh, ());
                    let buffer = pool.create_buffer(
                        0,
                        init_w as i32,
                        init_h as i32,
                        (init_w * 4) as i32,
                        wl_shm::Format::Argb8888,
                        qh,
                        (),
                    );
                    state.buffer = Some(buffer.clone());
                }
                "wl_seat" => {
                    registry.bind::<wl_seat::WlSeat, _, _>(name, 1, qh, ());
                }
                "xdg_wm_base" => {
                    let wm_base = registry.bind::<xdg_wm_base::XdgWmBase, _, _>(name, 1, qh, ());
                    state.wm_base = Some(wm_base);

                    if state.compositor_surface.is_some() && state.xdg_surface.is_none() {
                        state.init_xdg_surface(qh);
                    }
                }
                "zxdg_decoration_manager_v1" => {
                    state.decoration_manager =
                        Some(registry.bind::<ZxdgDecorationManagerV1, _, _>(name, 1, qh, ()));
                }
                _ => {
                    println!("[unhandled] {}", &interface[..]);
                }
            }
        }
    }
}

// test drawing function
pub fn draw(tmp: &mut File, (buf_x, buf_y): (u32, u32)) {
    let mut buf = std::io::BufWriter::new(tmp);

    buf.flush().unwrap();
}
