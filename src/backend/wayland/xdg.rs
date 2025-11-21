use std::{ffi::CString, os::raw::c_void, ptr::null_mut};

use gl::COLOR_BUFFER_BIT;
use wayland_client::{Dispatch, Proxy};
use wayland_egl::WlEglSurface;
use wayland_protocols::xdg::shell::client::{
    xdg_surface::XdgSurface, xdg_toplevel::XdgToplevel, xdg_wm_base::XdgWmBase,
};

use crate::{
    backend::wayland::{WaylandState, surface},
    egl::{
        EGL, EGL_BLUE_SIZE, EGL_CONTEXT_MAJOR_VERSION, EGL_CONTEXT_MINOR_VERSION,
        EGL_CONTEXT_OPENGL_CORE_PROFILE_BIT, EGL_CONTEXT_OPENGL_PROFILE_MASK, EGL_GREEN_SIZE,
        EGL_NONE, EGL_RED_SIZE, EGLConfig, EGLDisplay, EGLSurface, EGLint,
    },
};
use std::mem::MaybeUninit;
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
                    .get_platform_display_ext(
                        EGL_PLATFORM_WAYLAND_KHR,
                        native_display.id().as_ptr() as *mut c_void,
                        null_mut(),
                    )
                    .or(egl.get_platform_display(
                        EGL_PLATFORM_WAYLAND_KHR,
                        native_display.id().as_ptr() as *mut c_void,
                        null_mut(),
                    ))
                    .unwrap() as EGLDisplay;

                assert!(!display.is_null());
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

                let attributes: [i32; _] = [
                    EGL_RED_SIZE as i32,
                    8,
                    EGL_GREEN_SIZE as i32,
                    8,
                    EGL_BLUE_SIZE as i32,
                    8,
                    EGL_NONE as i32,
                ];

                let mut configs: MaybeUninit<EGLConfig> = MaybeUninit::uninit();
                let mut config_num: EGLint = 0;

                let config = egl
                    .choose_config(
                        display,
                        attributes.as_ptr(),
                        configs.as_mut_ptr(),
                        attributes.len() as i32,
                        &mut config_num,
                    )
                    .unwrap();

                let configs =
                    std::slice::from_raw_parts_mut(configs.as_mut_ptr(), config_num as usize);

                let context_attributes = [
                    EGL_CONTEXT_MAJOR_VERSION as i32,
                    4,
                    EGL_CONTEXT_MINOR_VERSION as i32,
                    0,
                    EGL_CONTEXT_OPENGL_PROFILE_MASK as i32,
                    EGL_CONTEXT_OPENGL_CORE_PROFILE_BIT as i32,
                    EGL_NONE as i32,
                ];

                let ctx = egl
                    .create_context(display, configs[0], null_mut(), context_attributes.as_ptr())
                    .unwrap();
                let egl_surface = WlEglSurface::new(base_surface.id(), 640, 480).unwrap();
                let surface = egl_surface.ptr() as EGLSurface;
                egl.make_current(display, surface, surface, ctx).unwrap();

                state.egl_surface = Some(egl_surface);
                state.egl = Some(egl);
                state.display = display;

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
