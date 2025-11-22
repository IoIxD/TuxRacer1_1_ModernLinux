use std::{
    ffi::CString,
    os::raw::c_void,
    ptr::{null, null_mut},
};

use wayland_client::{
    Dispatch, Proxy,
    backend::ObjectId,
    protocol::{wl_region::WlRegion, wl_surface::WlSurface},
};
use wayland_egl::WlEglSurface;
use wayland_protocols::xdg::shell::client::{
    xdg_surface::{self, XdgSurface},
    xdg_toplevel::XdgToplevel,
    xdg_wm_base::{self, XdgWmBase},
};

use crate::{
    backend::wayland::WaylandState,
    egl::{
        EGL, EGL_BLUE_SIZE, EGL_CONTEXT_MAJOR_VERSION, EGL_CONTEXT_MINOR_VERSION,
        EGL_CONTEXT_OPENGL_CORE_PROFILE_BIT, EGL_CONTEXT_OPENGL_PROFILE_MASK, EGL_FALSE,
        EGL_GREEN_SIZE, EGL_NATIVE_VISUAL_ID, EGL_NONE, EGL_OPENGL_API, EGL_OPENGL_BIT,
        EGL_RED_SIZE, EGL_RENDERABLE_TYPE, EGL_SURFACE_TYPE, EGL_WINDOW_BIT, EGLConfig, EGLDisplay,
        EGLNativeWindowType, EGLSurface, EGLint,
    },
};
use std::mem::MaybeUninit;

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

                // !!! We have to get the EGL in this weird way
                // because the rest of the functions assume a globally
                // stored EGL (state.panic_on_error). This is weird,
                // but do not change it.
                state.egl = Some(EGL::new());
                // SAFETY: if the above line didn't set it then we have bigger problems.
                let egl = state.egl.as_ref().unwrap_unchecked();

                let compositor = state.compositor();
                let compositor_surface = state.compositor_surface();
                let native_display = state.native_display();
                let egl_surface = state.egl_surface();

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
                println!("got display ({:?})", display);

                let mut major = 0;
                let mut minor = 0;
                state.panic_on_error(
                    "Error on initialization",
                    egl.initialize(display, &mut major, &mut minor).unwrap(),
                );
                println!("Initialized EGL version {}.{}", major, minor);

                state.panic_on_error(
                    "Error on binding API",
                    egl.bind_api(EGL_OPENGL_API).unwrap(),
                );

                let attributes: [i32; _] = [
                    EGL_SURFACE_TYPE as i32,
                    EGL_WINDOW_BIT as i32,
                    //
                    EGL_RENDERABLE_TYPE as i32,
                    EGL_OPENGL_BIT as i32,
                    //
                    EGL_RED_SIZE as i32,
                    8,
                    EGL_GREEN_SIZE as i32,
                    8,
                    EGL_BLUE_SIZE as i32,
                    8,
                    EGL_NONE as i32,
                ];

                let mut config_num: EGLint = 0;
                let mut matched_config_num: EGLint = 0;

                state.panic_on_error(
                    "Error getting configs",
                    egl.get_configs(display, null_mut(), 0, &mut config_num)
                        .unwrap(),
                );

                let mut configs: Vec<EGLConfig> = Vec::new();
                configs.resize(config_num as usize, null_mut());

                state.panic_on_error(
                    "Error choosing config",
                    egl.choose_config(
                        display,
                        attributes.as_ptr(),
                        configs.as_mut_ptr(),
                        config_num,
                        &mut matched_config_num,
                    )
                    .unwrap(),
                );

                // get a surface using the first config that yields a valid result
                let mut n = 0;
                let mut surface: MaybeUninit<EGLSurface> = MaybeUninit::uninit();
                for i in 0..matched_config_num {
                    let mut vis_id = 0;
                    if egl
                        .get_config_attrib(
                            display,
                            configs[i as usize],
                            EGL_NATIVE_VISUAL_ID as i32,
                            &mut vis_id,
                        )
                        .unwrap()
                        == EGL_FALSE
                    {
                        continue;
                    };
                    surface.write(
                        egl.create_window_surface(
                            display,
                            configs[0],
                            egl_surface.ptr() as EGLNativeWindowType,
                            null_mut(),
                        )
                        .unwrap(),
                    );
                    if !surface.assume_init().is_null() {
                        n = i;
                        break;
                    }
                    println!("config: {:?}", vis_id);
                }
                let surface = surface.assume_init();
                if surface.is_null() {
                    panic!("null egl surface!");
                }
                println!("got surface ({:?})", surface);

                let context_attributes = [
                    EGL_CONTEXT_MAJOR_VERSION as i32,
                    1,
                    EGL_CONTEXT_MINOR_VERSION as i32,
                    0,
                    EGL_CONTEXT_OPENGL_PROFILE_MASK as i32,
                    EGL_CONTEXT_OPENGL_CORE_PROFILE_BIT as i32,
                    EGL_NONE as i32,
                ];

                let window = MaybeUninit::<EGLNativeWindowType>::uninit();

                let ctx = egl
                    .create_context(
                        display,
                        configs[n as usize],
                        null_mut(),
                        context_attributes.as_ptr(),
                    )
                    .unwrap();

                state.panic_on_error(
                    "Error making context current",
                    egl.make_current(display, surface, surface, ctx).unwrap(),
                );

                let new_ctx = egl.get_current_context().unwrap();
                assert!(ctx == new_ctx);

                gl::load_with(|name| {
                    let cstr = CString::new(name).unwrap();
                    match egl.get_proc_address(cstr.as_ptr()).unwrap() {
                        Some(a) => a as *const c_void,
                        None => null_mut(),
                    }
                });

                // compositor_surface.attach(state.buffer.as_ref(), 0, 0);
                // compositor_surface.commit();

                state.display = display;
                state.native_surface = surface;

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
        if let xdg_wm_base::Event::Ping { serial } = event {
            proxy.pong(serial);
        }
    }
}

// impl Dispatch<XdgSurface, ()> for WaylandState {
//     fn event(
//         state: &mut Self,
//         xdg_surface: &XdgSurface,
//         event: Event,
//         _: &(),
//         _: &Connection,
//         _: &QueueHandle<Self>,
//     ) {
//         if let xdg_surface::Event::Configure { serial, .. } = event {
//             state.configured = true;
//             let surface = state.base_surface.as_ref().unwrap();
//             if let Some(ref buffer) = state.buffer {
//                 surface.commit();
//             }
//         }
//     }
// }
