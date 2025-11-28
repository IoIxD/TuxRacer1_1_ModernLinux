#[cfg(feature = "drm")]
use std::{
    ffi::{CStr, c_char, c_void},
    time::SystemTime,
};

use crate::{
    backend::wayland::WaylandWindow,
    egl::{EGL, EGLSurface, NativeDisplayType},
    type_defs::{self, SDL_VideoInfo},
};

#[cfg(feature = "drm")]
mod drm;
#[cfg(feature = "drm")]
use {crate::backend::drm::DRMWindow, std::env::VarError};

mod wayland;

const VIDEO_INFO: SDL_VideoInfo = SDL_VideoInfo {
    _bitfield_align_1: [],
    _bitfield_1: unsafe { std::mem::transmute(255_u8) },
    blit_fill: 255,
    video_mem: 512000,
    vfmt: unsafe { std::mem::transmute(0) },
};

pub trait Window {
    fn init(&mut self, _flags: u32) -> i32;
    fn quit(&mut self);

    fn get_error(&mut self) -> *const u8;
    fn get_key_state(&mut self, numkeys: *mut i32) -> *mut u8;
    fn get_mouse_state(&mut self, x: *mut i32, y: *mut i32) -> u8;

    fn egl(&self) -> &EGL;
    fn wait_for_egl(&mut self);
    fn egl_display(&self) -> NativeDisplayType;
    fn egl_surface(&self) -> EGLSurface;

    fn gl_get_attribute(&mut self, attr: type_defs::SDL_GLattr, value: *mut i32) -> i32 {
        // unsafe { *value = self.gl_attrs[attr as usize] };
        return 0;
    }
    fn gl_get_proc_address(&mut self, proc: *const c_char) -> *mut c_void {
        self.wait_for_egl();
        println!(
            "getting {}",
            unsafe { CStr::from_ptr(proc) }.to_string_lossy()
        );
        unsafe {
            self.egl()
                .get_proc_address(proc)
                .expect("eglGetProcAddress missing")
                .expect("eglGetProcAddress missing") as *mut c_void
        }
    }
    fn gl_set_attribute(&mut self, attr: type_defs::SDL_GLattr, value: i32) -> i32 {
        // self.gl_attrs[attr as usize] = value;
        0
    }

    fn gl_swap_buffers(&mut self) {
        self.gl_swap_buffers_impl();
    }

    fn gl_swap_buffers_impl(&self) {
        let egl = self.egl();

        unsafe {
            egl.panic_on_error(
                "Error swapping buffers",
                egl.swap_buffers(self.egl_display(), self.egl_surface())
                    .unwrap(),
            );
        }
    }

    fn poll_event(&mut self, event: *mut type_defs::SDL_Event) -> i32;
    fn set_video_mode(
        &mut self,
        width: i32,
        height: i32,
        bpp: i32,
        flags: u32,
    ) -> *mut type_defs::SDL_Surface;
    fn show_cursor(&mut self, toggle: i32) -> i32;
    fn warp_mouse(&mut self, x: u16, y: u16);
    fn wm_set_caption(&mut self, title: &str, icon: &str);

    fn delay(&mut self, ms: u32) {
        let time = SystemTime::now();
        while time.elapsed().unwrap().as_millis() <= ms as u128 {
            // self.event_loop();
        }
    }

    fn enable_key_repeat(&mut self, delay: i32, interval: i32) -> i32 {
        return 0;
    }

    fn get_video_info(&mut self) -> *mut type_defs::SDL_VideoInfo {
        // the game never actually "mutates" VideoInfo so to speak so this is fine.
        #[allow(const_item_mutation)]
        return &mut VIDEO_INFO;
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
        return 0;
    }
}

pub fn choose_window() -> Box<dyn Window> {
    match std::env::var("XDG_SESSION_TYPE") {
        Ok(xdg_session_type) => match xdg_session_type.as_str() {
            "x11" => {
                panic!("no x11 support yet.")
            }
            "wayland" => {
                return Box::new(WaylandWindow::new());
            }

            #[cfg(feature = "drm")]
            "tty" => match DRMWindow::new() {
                Ok(a) => Box::new(a),
                Err(err) => {
                    panic!(
                        "XDG_SESSION_TYPE not set (implying you aren't using a supported window manager).\nIf you meant to use DRM, there was an error getting a device: {}\n",
                        err
                    )
                }
            },
            _ => {
                panic!("Unknown session type {}", xdg_session_type);
            }
        },

        #[cfg(feature = "drm")]
        Err(VarError::NotPresent) => match DRMWindow::new() {
            Ok(a) => Box::new(a),
            Err(err) => {
                panic!(
                    "XDG_SESSION_TYPE not set (implying you aren't using a supported window manager).\nIf you meant to use DRM, there was an error getting a device: {}\n",
                    err
                )
            }
        },
        Err(err) => panic!("{:?}", err),
    }
}
