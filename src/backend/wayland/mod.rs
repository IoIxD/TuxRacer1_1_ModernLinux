#![allow(unused_variables)]
use std::{
    ffi::{c_char, c_void},
    io::ErrorKind,
    ptr::null_mut,
};

mod registry;
mod seat;
mod xdg;

use gl::COLOR_BUFFER_BIT;
use wayland_client::{
    Connection, EventQueue, QueueHandle,
    backend::WaylandError,
    delegate_noop,
    protocol::{
        wl_buffer::WlBuffer, wl_compositor::WlCompositor, wl_display::WlDisplay, wl_shm::WlShm,
        wl_shm_pool::WlShmPool, wl_surface::WlSurface,
    },
};
use wayland_egl::WlEglSurface;

use crate::{
    backend::Window,
    egl::{EGL, EGL_SUCCESS, EGL_TRUE, EGLBoolean, EGLDisplay, EGLSurface, wl_buffer},
    type_defs::{self, SDL_Rect, SDL_Surface},
};
use wayland_protocols::xdg::shell::client::{
    xdg_surface::XdgSurface, xdg_toplevel::XdgToplevel, xdg_wm_base::XdgWmBase,
};

#[derive(Default)]
pub struct WaylandState {
    base_surface: Option<WlSurface>,
    wm_base: Option<XdgWmBase>,
    xdg_surface: Option<XdgSurface>,
    xdg_top_level: Option<XdgToplevel>,
    egl_surface: Option<WlEglSurface>,
    egl: Option<EGL>,
    display: EGLDisplay,
    configured: bool,
    native_display: Option<WlDisplay>,
    buffer: Option<WlBuffer>,
}

impl WaylandState {
    pub fn wait_for_egl(&mut self) -> (&mut WlEglSurface, &mut EGL, &mut EGLDisplay) {
        while !self.configured {}

        (
            unsafe { self.egl_surface.as_mut().unwrap_unchecked() },
            unsafe { self.egl.as_mut().unwrap_unchecked() },
            &mut self.display,
        )
    }
}

pub struct WaylandWindow {
    state: WaylandState,
    event_queue: EventQueue<WaylandState>,
}

impl WaylandWindow {
    pub fn new() -> Self {
        // upper_sanity_test();
        let conn = Connection::connect_to_env().unwrap();

        let mut event_queue = conn.new_event_queue();
        let qhandle = event_queue.handle();

        let display = conn.display();
        display.get_registry(&qhandle, ());

        let mut state = WaylandState {
            configured: false,
            native_display: Some(display),
            ..Default::default()
        };
        event_queue.roundtrip(&mut state).unwrap();

        Self { state, event_queue }
    }

    pub fn event_loop(&mut self) {
        let dispatched = self.event_queue.dispatch_pending(&mut self.state).unwrap();
        if dispatched > 0 {
            return;
        }

        while let Err(err) = self.event_queue.flush() {
            if let WaylandError::Io(err) = err {
                if err.kind() == ErrorKind::WouldBlock {}
            } else {
                println!("{}", err);
            }
        }

        if let Some(guard) = self.event_queue.prepare_read() {
            let read = guard.read().unwrap();
            if read <= 0 {
                return;
            }
            println!("{}", read);
        }

        self.event_queue.dispatch_pending(&mut self.state).unwrap();
    }

    fn sanity_test(&mut self) {
        loop {
            unsafe {
                gl::ClearColor(1.0, 0.0, 0.0, 1.0);
                gl::Clear(COLOR_BUFFER_BIT);
                gl::Flush();

                self.state
                    .egl
                    .as_mut()
                    .unwrap()
                    .swap_buffers(
                        self.state.display,
                        self.state.egl_surface.as_mut().unwrap().ptr() as EGLSurface,
                    )
                    .unwrap();
                self.state.base_surface.as_mut().unwrap().commit();
            };
            self.event_loop();
        }
    }
}

impl WaylandState {
    fn init_xdg_surface(&mut self, qh: &QueueHandle<WaylandState>) {
        let wm_base = self.wm_base.as_ref().unwrap();
        let base_surface = self.base_surface.as_ref().unwrap();

        let xdg_surface = wm_base.get_xdg_surface(base_surface, qh, ());
        let toplevel = xdg_surface.get_toplevel(qh, ());
        toplevel.set_title("A fantastic window!".into());

        base_surface.commit();

        self.xdg_surface = Some(xdg_surface);
        self.xdg_top_level = Some(toplevel);
    }

    pub unsafe fn panic_on_error(&self, reason: &str, err: EGLBoolean) {
        if err != EGL_TRUE {
            panic!(
                "{}: {}",
                reason,
                self.egl.as_ref().unwrap().get_error_str().unwrap()
            );
        }
    }
}

impl Window for WaylandWindow {
    fn init(&mut self, _flags: u32) -> i32 {
        while !self.state.configured {
            self.event_loop();
        }
        println!("init done");

        0
    }
    fn quit(&mut self) {}

    fn delay(&mut self, ms: u32) {
        unimplemented!("delay");
    }
    fn enable_key_repeat(&mut self, delay: i32, interval: i32) -> i32 {
        unimplemented!("enable_key_repeat");
    }
    fn get_error(&mut self) -> &'static str {
        unimplemented!("get_error");
    }
    fn get_key_state(&mut self, numkeys: *mut i32) -> *mut u8 {
        unimplemented!("get_key_state");
    }
    fn get_mod_state(&mut self) -> type_defs::SDLMod {
        unimplemented!("get_mod_state");
    }
    fn get_mouse_state(&mut self, x: *mut i32, y: *mut i32) -> u8 {
        unimplemented!("get_mouse_state");
    }
    fn get_video_info(&mut self) -> *mut type_defs::SDL_VideoInfo {
        unimplemented!("get_video_info");
    }
    fn gl_get_attribute(&mut self, attr: type_defs::SDL_GLattr, value: *mut i32) -> i32 {
        let (surface, egl, display) = self.state.wait_for_egl();
        self.sanity_test();

        let attr = match attr {
            type_defs::SDL_GLattr::RED_SIZE => gl::RENDERBUFFER_RED_SIZE,
            type_defs::SDL_GLattr::GREEN_SIZE => gl::RENDERBUFFER_GREEN_SIZE,
            type_defs::SDL_GLattr::BLUE_SIZE => gl::RENDERBUFFER_BLUE_SIZE,
            type_defs::SDL_GLattr::ALPHA_SIZE => gl::RENDERBUFFER_ALPHA_SIZE,
            type_defs::SDL_GLattr::DOUBLEBUFFER => gl::DOUBLEBUFFER,
            type_defs::SDL_GLattr::BUFFER_SIZE => gl::BUFFER_SIZE,
            type_defs::SDL_GLattr::DEPTH_SIZE => gl::RENDERBUFFER_DEPTH_SIZE,
            type_defs::SDL_GLattr::STENCIL_SIZE => gl::RENDERBUFFER_STENCIL_SIZE,
            _ => todo!(),
        };
        let mut ret: i32 = 0;

        unsafe { gl::GetIntegerv(attr, &mut ret) };
        ret
    }
    fn gl_get_proc_address(&mut self, proc: *const c_char) -> *mut c_void {
        let (_, egl, display) = self.state.wait_for_egl();
        unsafe {
            egl.get_proc_address(proc)
                .expect("eglGetProcAddress missing")
                .expect("eglGetProcAddress missing") as *mut c_void
        }
    }
    fn gl_set_attribute(&mut self, attr: type_defs::SDL_GLattr, value: i32) -> i32 {
        // todo?
        0
    }
    fn gl_swap_buffers(&mut self) {
        unimplemented!("gl_swap_buffers");
    }

    fn joystick_event_state(&mut self, state: i32) -> i32 {
        unimplemented!("joystick_event_state");
    }
    fn joystick_get_axis(&mut self, joystick: *mut type_defs::SDL_Joystick, axis: i32) -> i16 {
        unimplemented!("joystick_get_axis");
    }
    fn joystick_get_button(&mut self, joystick: *mut type_defs::SDL_Joystick, button: i32) -> u8 {
        unimplemented!("joystick_get_button");
    }
    fn joystick_name(&mut self, index: i32) -> *const c_char {
        unimplemented!("joystick_name");
    }
    fn joystick_num_axes(&mut self, joystick: *mut type_defs::SDL_Joystick) -> i32 {
        unimplemented!("joystick_num_axes");
    }
    fn joystick_num_buttons(&mut self, joystick: *mut type_defs::SDL_Joystick) -> i32 {
        unimplemented!("joystick_num_buttons");
    }
    fn joystick_open(&mut self, index: i32) -> *mut type_defs::SDL_Joystick {
        unimplemented!("joystick_open");
    }
    fn num_joysticks(&mut self) -> i32 {
        unimplemented!("num_joysticks");
    }
    fn poll_event(&mut self, event: *mut type_defs::SDL_Event) -> i32 {
        unimplemented!("poll_event");
    }

    fn rwfrom_file(&mut self, file: &str, mode: &str) -> *mut type_defs::SDL_RWops {
        unimplemented!("rwfrom_file");
    }
    fn set_video_mode(
        &mut self,
        width: i32,
        height: i32,
        bpp: i32,
        flags: u32,
    ) -> *mut type_defs::SDL_Surface {
        // let (surface, egl, display) = self.state.wait_for_egl();

        // just resize for now
        // surface.resize(width, height, 0, 0);

        Box::leak(Box::new(SDL_Surface {
            flags,
            format: null_mut(), // todo
            w: width,
            h: height,
            pitch: 0, // ?
            pixels: null_mut(),
            clip_rect: SDL_Rect {
                x: 0,
                y: 0,
                w: width,
                h: height,
            },
            refcount: 0,
        }))
    }
    fn show_cursor(&mut self, toggle: i32) -> i32 {
        unimplemented!("show_cursor");
    }
    fn warp_mouse(&mut self, x: u16, y: u16) {
        unimplemented!("warp_mouse");
    }
    fn wm_set_caption(&mut self, title: &str, icon: &str) {
        unimplemented!("wm_set_caption");
    }
}
delegate_noop!(WaylandState: ignore WlCompositor);
delegate_noop!(WaylandState: ignore WlShm);
delegate_noop!(WaylandState: ignore WlBuffer);
delegate_noop!(WaylandState: ignore WlShmPool);

// for now
delegate_noop!(WaylandState: ignore WlSurface);
