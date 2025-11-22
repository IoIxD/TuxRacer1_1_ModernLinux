use std::ffi::{c_char, c_void};

use crate::type_defs;

pub mod wayland;

pub trait Window {
    fn init(&mut self, _flags: u32) -> i32;
    fn quit(&mut self);

    fn delay(&mut self, ms: u32);
    fn enable_key_repeat(&mut self, delay: i32, interval: i32) -> i32;
    fn get_error(&mut self) -> *const u8;
    fn get_key_state(&mut self, numkeys: *mut i32) -> *mut u8;
    fn get_mod_state(&mut self) -> type_defs::SDLMod;
    fn get_mouse_state(&mut self, x: *mut i32, y: *mut i32) -> u8;
    fn get_video_info(&mut self) -> *mut type_defs::SDL_VideoInfo;
    fn gl_get_attribute(&mut self, attr: type_defs::SDL_GLattr, value: *mut i32) -> i32;
    fn gl_get_proc_address(&mut self, proc_: *const c_char) -> *mut c_void;
    fn gl_set_attribute(&mut self, attr: type_defs::SDL_GLattr, value: i32) -> i32;
    fn gl_swap_buffers(&mut self);

    fn joystick_event_state(&mut self, state: i32) -> i32;
    fn joystick_get_axis(&mut self, joystick: *mut type_defs::SDL_Joystick, axis: i32) -> i16;
    fn joystick_get_button(&mut self, joystick: *mut type_defs::SDL_Joystick, button: i32) -> u8;
    fn joystick_name(&mut self, index: i32) -> *const c_char;
    fn joystick_num_axes(&mut self, joystick: *mut type_defs::SDL_Joystick) -> i32;
    fn joystick_num_buttons(&mut self, joystick: *mut type_defs::SDL_Joystick) -> i32;
    fn joystick_open(&mut self, index: i32) -> *mut type_defs::SDL_Joystick;
    fn num_joysticks(&mut self) -> i32;
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
}
