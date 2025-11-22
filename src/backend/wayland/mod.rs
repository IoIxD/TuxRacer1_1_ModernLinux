#![allow(unused_variables)]
use std::{
    ffi::{c_char, c_void},
    io::ErrorKind,
    process::exit,
    ptr::{null, null_mut},
    time::SystemTime,
};

mod keyboard;
mod pointer;
mod seat;
mod xdg;

use wayland_client::{
    Connection, EventQueue, Proxy, QueueHandle,
    backend::WaylandError,
    delegate_noop,
    protocol::{
        wl_buffer::WlBuffer, wl_compositor::WlCompositor, wl_display::WlDisplay,
        wl_pointer::WlPointer, wl_region::WlRegion, wl_shm::WlShm, wl_shm_pool::WlShmPool,
        wl_surface::WlSurface,
    },
};
use wayland_client::{
    Dispatch,
    protocol::{
        wl_compositor,
        wl_registry::{self},
        wl_seat,
    },
};
use wayland_egl::WlEglSurface;
use wayland_protocols::xdg::shell::client::xdg_wm_base;
use wayland_sys::{
    client::{wayland_client_handle, wl_display},
    ffi_dispatch,
};

use crate::{
    backend::Window,
    egl::{EGL, EGL_TRUE, EGLBoolean, EGLDisplay, EGLSurface},
    type_defs::{
        self, SDL_EventType, SDL_Rect, SDL_Surface, SDL_VideoInfo, SDL_keysym, SDLKey,
        SDLKey_SDLK_0, SDLKey_SDLK_LAST, SDLKey_SDLK_UP, SDLMod_KMOD_NONE,
    },
};
use wayland_protocols::{
    wp::pointer_warp::v1::client::wp_pointer_warp_v1::WpPointerWarpV1,
    xdg::{
        decoration::zv1::client::{
            zxdg_decoration_manager_v1::ZxdgDecorationManagerV1,
            zxdg_toplevel_decoration_v1::{Mode, ZxdgToplevelDecorationV1},
        },
        shell::client::{
            xdg_surface::XdgSurface, xdg_toplevel::XdgToplevel, xdg_wm_base::XdgWmBase,
        },
    },
};

#[derive(Default)]
pub struct WaylandState {
    compositor: Option<WlCompositor>,
    compositor_surface: Option<WlSurface>,
    wm_base: Option<XdgWmBase>,
    xdg_surface: Option<XdgSurface>,
    xdg_top_level: Option<XdgToplevel>,
    native_surface: EGLSurface,
    egl_surface: Option<WlEglSurface>,
    egl: Option<EGL>,
    display: EGLDisplay,
    configured: bool,
    native_display: Option<WlDisplay>,
    running: bool,
    quit_attempts: u8,
    pointer: Option<WlPointer>,
    pointer_serial: u32,

    pointer_events: Vec<wayland_client::protocol::wl_pointer::Event>,
    last_pointer_x: f64,
    last_pointer_y: f64,

    keys: Vec<SDLKey>,

    // below protocols are staging/unstable and thus shouldn't have getters that assume they're there.
    decoration_manager: Option<ZxdgDecorationManagerV1>,
    toplevel_decoration: Option<ZxdgToplevelDecorationV1>,
    pointer_warp: Option<WpPointerWarpV1>,
}

impl WaylandState {
    pub fn wait_for_egl(&self) {
        while !self.configured {}
    }
}

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
                "wp_pointer_warp_v1" => {
                    state.pointer_warp =
                        Some(registry.bind::<WpPointerWarpV1, _, _>(name, 1, qh, ()))
                }
                _ => {
                    println!("[unhandled] {}", &interface[..]);
                }
            }
        }
    }
}

pub struct WaylandWindow {
    state: WaylandState,
    event_queue: EventQueue<WaylandState>,
    video_info: SDL_VideoInfo,
    fake_surface: SDL_Surface,
}

impl WaylandWindow {
    pub fn new() -> Self {
        let conn = Connection::connect_to_env().unwrap();

        let mut event_queue = conn.new_event_queue();
        let qhandle = event_queue.handle();

        let display = conn.display();
        display.get_registry(&qhandle, ());

        let fake_surface = SDL_Surface {
            flags: 0,
            format: null_mut(),
            w: 640,
            h: 480,
            pitch: 0,
            pixels: null_mut(),
            clip_rect: SDL_Rect {
                x: 0,
                y: 0,
                w: 640,
                h: 480,
            },
            refcount: 0,
        };

        let mut state = WaylandState {
            configured: false,
            native_display: Some(display),
            running: true,
            ..Default::default()
        };
        state.keys.resize(SDLKey_SDLK_LAST as usize, 0);
        event_queue.roundtrip(&mut state).unwrap();

        let video_info = SDL_VideoInfo {
            _bitfield_align_1: [],
            _bitfield_1: unsafe { std::mem::transmute(255_u8) },
            blit_fill: 0,
            video_mem: 0,
            vfmt: unsafe { std::mem::transmute(0) },
        };

        Self {
            state,
            event_queue,
            video_info,
            fake_surface,
        }
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

        // If you're able to do this in the safe API then I'm simply not finding out where.
        if let Some(display) = self.state.native_display.as_ref() {
            unsafe {
                ffi_dispatch!(
                    wayland_client_handle(),
                    wl_display_dispatch_pending,
                    display.id().as_ptr() as *mut wl_display
                );
            }
        }

        if let Some(guard) = self.event_queue.prepare_read() {
            let read = guard.read().unwrap();
            if read <= 0 {
                return;
            }
            print!("read {} events\t\t\t\t\n", read);
        }

        self.event_queue.dispatch_pending(&mut self.state).unwrap();
    }

    /*fn sanity_test(&mut self) {
        loop {
            self.event_loop();

            unsafe {
                gl::ClearColor(1.0, 0.0, 0.0, 1.0);
                gl::Clear(COLOR_BUFFER_BIT);
                gl::Flush();

                let egl = self.state.egl();

                self.state.panic_on_error(
                    "Error swapping buffers",
                    egl.swap_buffers(self.state.display, self.state.native_surface)
                        .unwrap(),
                );
            };
        }
    }*/
}

impl WaylandState {
    #[allow(dead_code)]
    pub fn wm_base(&self) -> &XdgWmBase {
        self.wm_base.as_ref().unwrap()
    }
    pub fn compositor(&self) -> &WlCompositor {
        self.compositor.as_ref().unwrap()
    }
    pub fn compositor_surface(&self) -> &WlSurface {
        self.compositor_surface.as_ref().unwrap()
    }
    pub fn egl(&self) -> &EGL {
        self.egl.as_ref().unwrap()
    }
    pub fn egl_surface(&self) -> &WlEglSurface {
        self.egl_surface.as_ref().unwrap()
    }
    pub fn native_display(&self) -> &WlDisplay {
        self.native_display.as_ref().unwrap()
    }
    pub fn xdg_top_level(&self) -> &XdgToplevel {
        self.xdg_top_level.as_ref().unwrap()
    }
    fn init_xdg_surface(&mut self, qh: &QueueHandle<WaylandState>) {
        let wm_base = self.wm_base.as_ref().unwrap();
        let compositor_surface = self.compositor_surface.as_ref().unwrap();

        let xdg_surface = wm_base.get_xdg_surface(compositor_surface, qh, ());
        let toplevel = xdg_surface.get_toplevel(qh, ());

        if let Some(decoration_manager) = self.decoration_manager.as_mut() {
            let toplevel_decoration = decoration_manager.get_toplevel_decoration(&toplevel, qh, ());

            toplevel_decoration.set_mode(Mode::ServerSide);

            self.toplevel_decoration = Some(toplevel_decoration);
        }

        compositor_surface.commit();

        self.xdg_surface = Some(xdg_surface);
        self.xdg_top_level = Some(toplevel);
    }

    pub unsafe fn panic_on_error(&self, reason: &str, err: EGLBoolean) {
        if err != EGL_TRUE {
            if let Some(egl) = self.egl.as_ref() {
                panic!(
                    "{}: {} ({:X})",
                    reason,
                    egl.get_error_str().unwrap(),
                    egl.get_error().unwrap()
                );
            }
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
    fn quit(&mut self) {
        self.state.running = false;
    }

    fn delay(&mut self, ms: u32) {
        let time = SystemTime::now();
        while time.elapsed().unwrap().as_millis() <= ms as u128 {
            self.event_loop();
        }
    }
    fn enable_key_repeat(&mut self, delay: i32, interval: i32) -> i32 {
        println!("enable_key_repeat");
        return 0;
    }
    fn get_error(&mut self) -> *const u8 {
        println!("get_error");
        return null();
        // return Box::leak(Box::new(CString::new("")));
    }
    fn get_key_state(&mut self, numkeys: *mut i32) -> *mut u8 {
        println!("get_key_state");
        if numkeys != null_mut() {
            unsafe {
                *numkeys = 0;
            }
        }
        return self.state.keys.as_mut_ptr() as *mut u8;
    }
    fn get_mod_state(&mut self) -> type_defs::SDLMod {
        println!("get_mod_state");
        return SDLMod_KMOD_NONE;
    }
    fn get_mouse_state(&mut self, x: *mut i32, y: *mut i32) -> u8 {
        unsafe {
            *x = self.state.last_pointer_x as i32;
            *y = self.state.last_pointer_y as i32;
        }
        return 0;
    }
    fn get_video_info(&mut self) -> *mut type_defs::SDL_VideoInfo {
        return &mut self.video_info;
    }
    fn gl_get_attribute(&mut self, attr: type_defs::SDL_GLattr, value: *mut i32) -> i32 {
        self.state.wait_for_egl();
        // self.sanity_test();

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
        self.state.wait_for_egl();
        unsafe {
            match self
                .state
                .egl()
                .get_proc_address(proc)
                .expect("eglGetProcAddress missing")
            {
                Some(a) => a as *mut c_void,
                None => null_mut(),
            }
        }
    }
    fn gl_set_attribute(&mut self, attr: type_defs::SDL_GLattr, value: i32) -> i32 {
        // todo?
        0
    }
    fn gl_swap_buffers(&mut self) {
        let egl = self.state.egl();
        unsafe {
            self.state.panic_on_error(
                "Error swapping buffers",
                egl.swap_buffers(self.state.display, self.state.native_surface)
                    .unwrap(),
            );
        }
    }

    fn joystick_event_state(&mut self, state: i32) -> i32 {
        return 0;
        // unimplemented!("joystick_event_state");
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
        println!("num_joysticks");
        return 0;
    }
    fn poll_event(&mut self, event: *mut type_defs::SDL_Event) -> i32 {
        self.event_loop();
        unsafe {
            // pointer events
            if self.state.pointer_events.len() >= 1 {
                use wayland_client::protocol::wl_pointer::Event;
                if let Some(ev) = self.state.pointer_events.pop() {
                    match ev {
                        Event::Motion {
                            time,
                            surface_x,
                            surface_y,
                        } => {
                            (*event).motion.type_ = SDL_EventType::SDL_MOUSEMOTION as u8;
                            (*event).motion.state = 0;
                            (*event).motion.x = surface_x as u16;
                            (*event).motion.y = surface_y as u16;
                            (*event).motion.xrel = (surface_x - self.state.last_pointer_x) as i16;
                            (*event).motion.yrel = (surface_y - self.state.last_pointer_y) as i16;
                            self.state.last_pointer_x = surface_x;
                            self.state.last_pointer_y = surface_y;
                        }
                        Event::Button {
                            serial,
                            time,
                            button,
                            state,
                        } => {
                            if let wayland_client::WEnum::Value(a) = state {
                                match a {
                                    wayland_client::protocol::wl_pointer::ButtonState::Released => {
                                        (*event).button.type_ =
                                            SDL_EventType::SDL_MOUSEBUTTONUP as u8;
                                        (*event).button.state = 0;
                                    }
                                    wayland_client::protocol::wl_pointer::ButtonState::Pressed => {
                                        (*event).button.type_ =
                                            SDL_EventType::SDL_MOUSEBUTTONDOWN as u8;
                                        (*event).button.state = 1;
                                    }
                                    _ => {}
                                }
                            }
                            (*event).button.button = (button - 271) as u8;
                            (*event).button.x = self.state.last_pointer_x as u16;
                            (*event).button.y = self.state.last_pointer_y as u16;
                        }
                        _ => {}
                    }
                }
                return 1;
            }

            // quit state
            if !self.state.running {
                self.state.quit_attempts += 1;
                (*event).quit.type_ = SDL_EventType::SDL_QUIT as u8;
                if self.state.quit_attempts >= 10 {
                    println!(
                        "WARNING: had to exit manually because the application didn't respond to SDL_QUIT."
                    );
                    exit(0);
                }
                return 1;
            }
        }
        return 0;
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
        self.state.wait_for_egl();

        self.state.egl_surface().resize(width, height, 0, 0);

        self.fake_surface.w = width;
        self.fake_surface.h = height;
        self.fake_surface.clip_rect.w = height;
        self.fake_surface.clip_rect.h = height;

        return &mut self.fake_surface;
    }
    fn show_cursor(&mut self, toggle: i32) -> i32 {
        println!("show_cursor");
        return 0;
    }
    fn warp_mouse(&mut self, x: u16, y: u16) {
        println!("warp mouse: {} {}", x, y);
        if let Some(pointer) = &self.state.pointer {
            if let Some(pointer_warp) = &self.state.pointer_warp {
                let surface = self.state.compositor_surface();
                pointer_warp.warp_pointer(
                    surface,
                    &pointer,
                    x as f64,
                    y as f64,
                    self.state.pointer_serial,
                );
            }
        }
    }
    fn wm_set_caption(&mut self, title: &str, icon: &str) {
        self.state.xdg_top_level().set_title(title.into());
    }
}
delegate_noop!(WaylandState: ignore WlCompositor);
delegate_noop!(WaylandState: ignore WlShm);
delegate_noop!(WaylandState: ignore WlRegion);
delegate_noop!(WaylandState: ignore WlBuffer);
delegate_noop!(WaylandState: ignore WlShmPool);
delegate_noop!(WaylandState: ignore WlSurface);
delegate_noop!(WaylandState: ignore ZxdgDecorationManagerV1);
delegate_noop!(WaylandState: ignore ZxdgToplevelDecorationV1);
delegate_noop!(WaylandState: ignore WpPointerWarpV1);
