#![allow(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::ffi::{CStr, c_char, c_int, c_void};

use crate::{sigsegv_handler, type_defs, window};

unsafe extern "C" {
    fn Mix_AllocateChannels(numchans: c_int) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_Delay(ms: u32) {
    window().lock().delay(ms);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_EnableKeyRepeat(delay: c_int, interval: c_int) -> c_int {
    window().lock().enable_key_repeat(delay, interval)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetError() -> *const u8 {
    window().lock().get_error()
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetKeyState(numkeys: *mut c_int) -> *mut u8 {
    window().lock().get_key_state(numkeys)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetModState() -> type_defs::SDLMod {
    window().lock().get_mod_state()
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetMouseState(x: *mut c_int, y: *mut c_int) -> u8 {
    window().lock().get_mouse_state(x, y)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GetVideoInfo() -> *mut type_defs::SDL_VideoInfo {
    window().lock().get_video_info()
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GL_GetAttribute(
    attr: type_defs::SDL_GLattr,
    value: *mut c_int,
) -> c_int {
    window().lock().gl_get_attribute(attr, value)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GL_GetProcAddress(proc_: *const c_char) -> *mut c_void {
    window().lock().gl_get_proc_address(proc_)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GL_SetAttribute(attr: type_defs::SDL_GLattr, value: c_int) -> c_int {
    window().lock().gl_set_attribute(attr, value)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_GL_SwapBuffers() {
    window().lock().gl_swap_buffers()
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_Init(flags: u32) -> c_int {
    // take this opprutunity to override the sigsegv handler
    unsafe {
        let f = sigsegv_handler as *const fn(libc::c_int);
        libc::signal(libc::SIGSEGV, f as libc::size_t);
    }

    // Also mix channel allocation
    Mix_AllocateChannels(6969);

    window().lock().init(flags)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_JoystickEventState(state: c_int) -> c_int {
    window().lock().joystick_event_state(state)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_JoystickGetAxis(
    joystick: *mut type_defs::SDL_Joystick,
    axis: c_int,
) -> i16 {
    window().lock().joystick_get_axis(joystick, axis)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_JoystickGetButton(
    joystick: *mut type_defs::SDL_Joystick,
    button: c_int,
) -> u8 {
    window().lock().joystick_get_button(joystick, button)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_JoystickName(index: c_int) -> *const c_char {
    window().lock().joystick_name(index)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_JoystickNumAxes(joystick: *mut type_defs::SDL_Joystick) -> c_int {
    window().lock().joystick_num_axes(joystick)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_JoystickNumButtons(joystick: *mut type_defs::SDL_Joystick) -> c_int {
    window().lock().joystick_num_buttons(joystick)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_JoystickOpen(index: c_int) -> *mut type_defs::SDL_Joystick {
    window().lock().joystick_open(index)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_NumJoysticks() -> c_int {
    window().lock().num_joysticks()
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_PollEvent(event: *mut type_defs::SDL_Event) -> c_int {
    window().lock().poll_event(event)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_Quit() {
    window().lock().quit()
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_SetVideoMode(
    width: c_int,
    height: c_int,
    bpp: c_int,
    flags: u32,
) -> *mut type_defs::SDL_Surface {
    window().lock().set_video_mode(width, height, bpp, flags)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_ShowCursor(toggle: c_int) -> c_int {
    window().lock().show_cursor(toggle)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_WarpMouse(x: u16, y: u16) {
    window().lock().warp_mouse(x, y)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SDL_WM_SetCaption(title: *const c_char, icon: *const c_char) {
    window().lock().wm_set_caption(
        &CStr::from_ptr(title).to_string_lossy(),
        &CStr::from_ptr(icon).to_string_lossy(),
    );
}
