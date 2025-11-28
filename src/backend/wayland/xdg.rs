use std::{env::current_dir, io::Write, os::fd::AsFd};

use image::ImageReader;
use wayland_client::{Dispatch, Proxy, delegate_noop, protocol::wl_shm};
use wayland_protocols::xdg::{
    shell::client::{
        xdg_surface::{self, XdgSurface},
        xdg_toplevel::XdgToplevel,
        xdg_wm_base::{self, XdgWmBase},
    },
    toplevel_icon::v1::client::{
        xdg_toplevel_icon_manager_v1::XdgToplevelIconManagerV1,
        xdg_toplevel_icon_v1::XdgToplevelIconV1,
    },
};

use crate::{
    backend::wayland::WaylandState,
    egl::{EGL, EGLWindowType, NativeDisplayType, NativeWindowType},
};

impl Dispatch<XdgSurface, ()> for WaylandState {
    fn event(
        state: &mut Self,
        xdg_surface: &XdgSurface,
        event: <XdgSurface as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        match event {
            xdg_surface::Event::Configure { serial } => unsafe {
                xdg_surface.ack_configure(serial);

                if state.configured {
                    return;
                }
                let native_display = state.native_display();
                let egl_surface = state.egl_surface();
                pub const EGL_PLATFORM_WAYLAND_KHR: u32 = 0x31D8;

                let (egl, surface, display) = EGL::setup(
                    EGL_PLATFORM_WAYLAND_KHR,
                    native_display.id().as_ptr() as NativeDisplayType,
                    EGLWindowType::Window(egl_surface.ptr() as NativeWindowType),
                );

                state.egl = Some(egl);
                state.display = display;
                state.native_surface = surface;

                if let Some(manager) = state.fifo_manager.as_ref() {
                    let surface = state.compositor_surface();
                    let fifo = manager.get_fifo(surface, &qhandle, ());

                    fifo.set_barrier();

                    state.fifo = Some(fifo);
                }

                state.configured = true;
            },
            _ => {}
        }
    }
}

impl Dispatch<XdgToplevel, ()> for WaylandState {
    fn event(
        state: &mut Self,
        proxy: &XdgToplevel,
        event: <XdgToplevel as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        match event {
            wayland_protocols::xdg::shell::client::xdg_toplevel::Event::Close => {
                state.running = false;
            }
            wayland_protocols::xdg::shell::client::xdg_toplevel::Event::Configure {
                width,
                height,
                states,
            } => {
                // if !state.resize_cycle {
                //     println!("resized to {} {}", width, height);
                //     state.egl_surface().resize(width, height, 0, 0);

                //     state.resize_happened = true;
                //     state.resized_x = width;
                //     state.resized_y = height;
                // }
            }
            // wayland_protocols::xdg::shell::client::xdg_toplevel::Event::WmCapabilities {
            //     capabilities,
            // } => todo!(),
            // _ => todo!(),
            _ => {}
        }
    }
}
impl Dispatch<XdgWmBase, ()> for WaylandState {
    fn event(
        state: &mut Self,
        proxy: &XdgWmBase,
        event: <XdgWmBase as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        if let xdg_wm_base::Event::Ping { serial } = event {
            proxy.pong(serial);
        }
    }
}

impl wayland_client::Dispatch<XdgToplevelIconManagerV1, ()> for WaylandState {
    fn event(
        state: &mut Self,
        manager: &XdgToplevelIconManagerV1,
        event: <XdgToplevelIconManagerV1 as Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        if let Some(xdg_top_level) = state.xdg_top_level.as_mut() {
            if let Some(wl_shm) = state.wl_shm.as_mut() {
                if let None = state.toplevel_icon {
                    // The original Linux version of the game didn't actually set the icon it seems. So
                    // we fudge something by loading the lives image and cropping it, since that's roughly what it is
                    // on Windows.
                    //
                    // TODO: Really? It didn't? I'm not finding anything in the install directory and
                    // when the game sets the 'icon path' it uses the same string as the title (down to the same pointer).
                    // Maybe get an old Linux VM set up one day to confirm this.

                    match ImageReader::open(
                        current_dir().unwrap().join("textures").join("tuxlife.png"),
                    ) {
                        Ok(img) => {
                            let icon = manager.create_icon(&qhandle, ());

                            let mut img = img.decode().unwrap();
                            let (init_w, init_h) = (32, 32);
                            img = img.crop(0, 0, init_w, init_h);
                            // The image's hue is wrong when we end up writing it to the wl_shm.
                            // TODO: Why
                            img = img.huerotate(140);

                            let mut file = tempfile::tempfile().unwrap();
                            file.write(img.as_bytes()).unwrap();
                            let pool = wl_shm.create_pool(
                                file.as_fd(),
                                (init_w * init_h * 4) as i32,
                                qhandle,
                                (),
                            );
                            let buffer = pool.create_buffer(
                                0,
                                init_w as i32,
                                init_h as i32,
                                (init_w * 4) as i32,
                                wl_shm::Format::Argb8888,
                                qhandle,
                                (),
                            );

                            icon.add_buffer(&buffer, 1);

                            manager.set_icon(&xdg_top_level, Some(&icon));
                        }
                        Err(err) => {
                            println!("error setting icon: {}", err);
                        }
                    };
                }
            }
        }
    }
}
delegate_noop!(WaylandState: ignore XdgToplevelIconV1);
