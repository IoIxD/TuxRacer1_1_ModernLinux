#![allow(unused_variables)]
use std::{
    ffi::{c_char, c_void},
    ptr::null_mut,
};

mod compositor;
mod registry;
mod seat;
mod surface;
mod xdg;

use khronos_egl::{Display, Dynamic, EGL1_5, Instance, Surface};
use libloading::Library;
use wayland_client::{
    Connection, EventQueue, Proxy, QueueHandle,
    protocol::{wl_display::WlDisplay, wl_surface::WlSurface},
};
use wayland_egl::WlEglSurface;

use crate::{
    backend::Window,
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
    egl: Option<Instance<Dynamic<Library, EGL1_5>>>,
    display: Option<Display>,
    configured: bool,
    native_display: Option<WlDisplay>,
}

impl WaylandState {
    pub fn wait_for_egl(
        &mut self,
    ) -> (
        &mut WlEglSurface,
        &mut Instance<Dynamic<Library, EGL1_5>>,
        &mut Display,
    ) {
        while !self.configured {}

        (
            unsafe { self.egl_surface.as_mut().unwrap_unchecked() },
            unsafe { self.egl.as_mut().unwrap_unchecked() },
            unsafe { self.display.as_mut().unwrap_unchecked() },
        )
    }
}

pub struct WaylandWindow {
    state: WaylandState,
    event_queue: EventQueue<WaylandState>,
}

impl WaylandWindow {
    pub fn new() -> Self {
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

        while !state.configured {
            let dispatched = event_queue.dispatch_pending(&mut state).unwrap();
            if dispatched > 0 {
                break;
            }

            event_queue.flush().unwrap();

            if let Some(guard) = event_queue.prepare_read() {
                let read = guard.read().unwrap();
                if read <= 0 {
                    if state.configured {
                        break;
                    }
                }
            }

            event_queue.dispatch_pending(&mut state).unwrap();
        }

        Self {
            state: state,
            event_queue,
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
}

impl Window for WaylandWindow {
    fn init(&mut self, _flags: u32) -> i32 {
        println!("init done");
        return 0;
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
    fn gl_get_proc_address(&mut self, proc_: &str) -> *mut c_void {
        unimplemented!("gl_get_proc_address");
    }
    fn gl_set_attribute(&mut self, attr: type_defs::SDL_GLattr, value: i32) -> i32 {
        // todo?
        return 0;
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

        return Box::leak(Box::new(SDL_Surface {
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
        }));
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
