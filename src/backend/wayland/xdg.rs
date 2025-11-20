use std::{ffi::CString, os::raw::c_void, ptr::null_mut};

use wayland_client::{Dispatch, Proxy};
use wayland_egl::WlEglSurface;
use wayland_protocols::xdg::shell::client::{
    xdg_surface::XdgSurface, xdg_toplevel::XdgToplevel, xdg_wm_base::XdgWmBase,
};

use crate::{
    backend::wayland::WaylandState,
    egl::{EGL, EGL_BLUE_SIZE, EGL_GREEN_SIZE, EGL_NONE, EGL_RED_SIZE},
};
use wayland_protocols::xdg::shell::client::xdg_surface::Event;

impl Dispatch<XdgSurface, ()> for WaylandState {
    fn event(
        state: &mut Self,
        proxy: &XdgSurface,
        event: <XdgSurface as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        match event {
            Event::Configure { serial } => unsafe {
                let base_surface = state.base_surface.as_ref().unwrap();

                let native_display = state.native_display.as_ref().unwrap();

                let egl = EGL::new();
                println!("{}", native_display.id().interface().name);

                pub const EGL_PLATFORM_WAYLAND_KHR: u32 = 0x31D8;
                let display = egl
                    .get_platform_display(
                        EGL_PLATFORM_WAYLAND_KHR,
                        native_display.id().as_ptr() as *mut c_void,
                        null_mut(),
                    )
                    .or(egl.get_platform_display_ext(
                        EGL_PLATFORM_WAYLAND_KHR,
                        native_display.id().as_ptr() as *mut c_void,
                        null_mut(),
                    ))
                    .unwrap();
                println!("got display");

                egl.initialize(display, null_mut(), null_mut()).unwrap();

                // egl.bind_api(EGL_OPENGL_API);
                gl::load_with(|name| {
                    let cstr = CString::new(name).unwrap();
                    match egl.get_proc_address(cstr.as_ptr()).unwrap() {
                        Some(a) => a as *const c_void,
                        None => null_mut(),
                    }
                });

                let attributes = [
                    EGL_RED_SIZE,
                    8,
                    EGL_GREEN_SIZE,
                    8,
                    EGL_BLUE_SIZE,
                    8,
                    EGL_NONE,
                ];

                /*let config = egl
                    .choose_config(display, &attributes)
                    .unwrap()
                    .expect("unable to find an appropriate ELG configuration");

                let context_attributes = [
                    CONTEXT_MAJOR_VERSION,
                    4,
                    CONTEXT_MINOR_VERSION,
                    0,
                    CONTEXT_OPENGL_PROFILE_MASK,
                    CONTEXT_OPENGL_CORE_PROFILE_BIT,
                    NONE,
                ];

                egl.create_context(display, config, None, &context_attributes)
                    .unwrap();*/
                let egl_surface = WlEglSurface::new(base_surface.id(), 640, 480).unwrap();
                state.egl_surface = Some(egl_surface);
                state.egl = Some(egl);
                state.display = Some(display);

                state.configured = true;
            },
            _ => todo!(),
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
    }
}
