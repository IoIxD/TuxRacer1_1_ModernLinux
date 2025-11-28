use std::{
    fs::File,
    io::Read,
    mem::ManuallyDrop,
    os::fd::{AsRawFd, FromRawFd},
};

use crate::{type_defs::SDL_keysym, xcb::sdl_key_from_keysym};
use wayland_client::{Dispatch, protocol::wl_keyboard::WlKeyboard};
use xkbcommon_rs::{
    Context, Keymap, KeymapFormat, State, xkb_context::ContextFlags, xkb_keymap::CompileFlags,
};

use crate::backend::wayland::WaylandState;

impl Dispatch<WlKeyboard, ()> for WaylandState {
    fn event(
        state: &mut Self,
        proxy: &WlKeyboard,
        event: <WlKeyboard as wayland_client::Proxy>::Event,
        data: &(),
        conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        match event {
            wayland_client::protocol::wl_keyboard::Event::Keymap { format, fd, size } => {
                let mut f = ManuallyDrop::new(unsafe { File::from_raw_fd(fd.as_raw_fd()) });
                let mut input = String::new();
                f.read_to_string(&mut input).unwrap();

                let keymap = Keymap::new_from_string(
                    Context::new(ContextFlags::NO_FLAGS).unwrap(),
                    &input,
                    KeymapFormat::TextV1,
                    CompileFlags::NO_FLAGS,
                )
                .unwrap();

                state.xkb_state = Some(State::new(keymap.clone()));
                state.xkb_keymap = Some(keymap);
            }
            wayland_client::protocol::wl_keyboard::Event::Enter {
                serial,
                surface,
                keys,
            } => {
                // for key in keys {
                //     if let Some(key_state) = state.xkb_state.as_ref() {
                //         let sym = key_state.key_get_one_sym(key);
                //         key_state.key_get_utf8(kc)
                //     }
                // }
            }
            wayland_client::protocol::wl_keyboard::Event::Key {
                serial,
                time,
                key: key_,
                state: keystate,
            } => {
                let keycode = key_ + 8;
                if let Some(keymap) = state.xkb_keymap.as_ref() {
                    if let Some(key_state) = state.xkb_state.as_ref() {
                        if let wayland_client::WEnum::Value(keystate) = keystate {
                            let layout = key_state.key_get_layout(keycode).unwrap();
                            let level = keymap.num_levels_for_key(keycode, layout) - 1;
                            let syms_out = keymap
                                .key_get_syms_by_level(keycode, layout, level)
                                .unwrap();

                            for sym in syms_out {
                                let key = sdl_key_from_keysym(sym);
                                match keystate {
                                    wayland_client::protocol::wl_keyboard::KeyState::Released => {
                                        state.active_keysyms.push((
                                            0,
                                            SDL_keysym {
                                                scancode: 25,
                                                sym: key,
                                                mod_: 0,
                                                unicode: 0,
                                            },
                                        ));
                                        state.keys[key as usize] = 0
                                    }
                                    wayland_client::protocol::wl_keyboard::KeyState::Pressed
                                    | wayland_client::protocol::wl_keyboard::KeyState::Repeated => {
                                        state.active_keysyms.push((
                                            1,
                                            SDL_keysym {
                                                scancode: 25,
                                                sym: key,
                                                mod_: 0,
                                                unicode: 0,
                                            },
                                        ));

                                        state.keys[key as usize] = 1 as u32;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        // match event {}
    }
}
