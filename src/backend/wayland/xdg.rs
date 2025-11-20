use std::{os::raw::c_void, ptr::null_mut};

use khronos_egl::{Display, EGL1_5};
use wayland_client::{Dispatch, Proxy};
use wayland_egl::WlEglSurface;
use wayland_protocols::xdg::shell::client::{
    xdg_surface::XdgSurface, xdg_toplevel::XdgToplevel, xdg_wm_base::XdgWmBase,
};

use crate::backend::wayland::WaylandState;
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
            Event::Configure { serial } => {
                let base_surface = state.base_surface.as_ref().unwrap();

                let native_display = state.native_display.as_ref().unwrap();

                let egl =
                    unsafe { khronos_egl::DynamicInstance::<EGL1_5>::load_required() }.unwrap();

                println!("{}", native_display.id().interface().name);

                pub const EGL_PLATFORM_WAYLAND_KHR: u32 = 0x31D8;
                let display = unsafe {
                    let eglGetPlatformDisplay: fn(
                        khronos_egl::Enum,
                        *mut c_void,
                        *const khronos_egl::Attrib,
                    ) -> khronos_egl::EGLDisplay =
                        std::mem::transmute(egl.get_proc_address("eglGetPlatformDisplay").unwrap());

                    let disp = eglGetPlatformDisplay(
                        EGL_PLATFORM_WAYLAND_KHR,
                        native_display.id().as_ptr() as *mut c_void,
                        null_mut(),
                    );

                    if disp.is_null() {
                        panic!("eglGetPlatformDisplay returns null");
                    }

                    Display::from_ptr(disp)
                };
                println!("got display");

                egl.initialize(display).unwrap();

                egl.bind_api(khronos_egl::OPENGL_API)
                    .expect("unable to select OpenGL API");
                gl::load_with(|name| {
                    egl.get_proc_address(name).unwrap() as *const std::ffi::c_void
                });

                let attributes = [
                    khronos_egl::RED_SIZE,
                    8,
                    khronos_egl::GREEN_SIZE,
                    8,
                    khronos_egl::BLUE_SIZE,
                    8,
                    khronos_egl::NONE,
                ];

                let config = egl
                    .choose_first_config(display, &attributes)
                    .unwrap()
                    .expect("unable to find an appropriate ELG configuration");

                let context_attributes = [
                    khronos_egl::CONTEXT_MAJOR_VERSION,
                    4,
                    khronos_egl::CONTEXT_MINOR_VERSION,
                    0,
                    khronos_egl::CONTEXT_OPENGL_PROFILE_MASK,
                    khronos_egl::CONTEXT_OPENGL_CORE_PROFILE_BIT,
                    khronos_egl::NONE,
                ];

                egl.create_context(display, config, None, &context_attributes)
                    .unwrap();
                let egl_surface = WlEglSurface::new(base_surface.id(), 640, 480).unwrap();
                state.egl_surface = Some(egl_surface);
                state.egl = Some(egl);
                state.display = Some(display);

                state.configured = true;
            }
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
