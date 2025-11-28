use nix::sys::termios::{FlushArg, Termios, cfmakeraw, tcflush, tcgetattr, tcsetattr};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::stdin;
use std::os::raw::c_void;
use std::os::unix::fs::OpenOptionsExt;
use std::ptr::{null, null_mut};
use std::sync::Arc;

use drm::Device;
use drm::control::connector::State;
use drm::control::{Device as ControlDevice, PageFlipFlags, PageFlipTarget};
use gbm::{
    AsRaw as GbmAsRaw, BufferObject, BufferObjectFlags, BufferObjectHandle, Device as GBMDevice,
    Surface,
};
use input::event::KeyboardEvent;
use input::ffi::{libinput_event_keyboard_get_key, libinput_event_keyboard_get_key_state};
use input::{AsRaw, Libinput, LibinputInterface};
use libc::{O_RDONLY, O_RDWR, O_WRONLY};
use xkbcommon_rs::xkb_context::ContextFlags;
use xkbcommon_rs::xkb_keymap::CompileFlags;
use xkbcommon_rs::{Context, Keymap};

use crate::backend::Window;
use crate::egl::{EGL, EGLDisplay, EGLSurface, EGLWindowType};
use crate::type_defs::{
    SDL_EventType, SDL_Rect, SDL_Surface, SDL_keysym, SDLKey, SDLKey_SDLK_LAST,
};
use crate::xcb::sdl_key_from_keysym;

#[derive(Debug)]
struct Card(std::fs::File);

impl std::os::unix::io::AsFd for Card {
    fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl Device for Card {}
impl ControlDevice for Card {}

impl Card {
    pub fn open(path: &str) -> std::io::Result<Self> {
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        options.write(true);
        Ok(Card(options.open(path)?))
    }

    pub fn open_first() -> Self {
        let mut i = 0;
        while i < 255 {
            match Self::open(format!("/dev/dri/card{}", i).as_str()) {
                Ok(a) => return a,
                Err(err) => {
                    println!("Cannot open /dev/dri/card{}: {}", i, err)
                }
            }
            i += 1;
        }
        panic!("no card avaliable")
    }
}

struct InputInterface;

impl LibinputInterface for InputInterface {
    fn open_restricted(
        &mut self,
        path: &std::path::Path,
        flags: i32,
    ) -> Result<std::os::unix::prelude::OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }

    fn close_restricted(&mut self, fd: std::os::unix::prelude::OwnedFd) {
        let _ = File::from(fd);
    }
}

pub struct DRMWindow {
    card: Arc<Card>,
    fake_surface: SDL_Surface,
    keys: [SDLKey; SDLKey_SDLK_LAST as usize],
    egl: EGL,
    surface: EGLSurface,
    gbm_device: GBMDevice<Arc<Card>>,
    gbm_surface: Surface<Card>,
    display: EGLDisplay,

    connector: drm::control::connector::Info,
    crtc: drm::control::crtc::Handle,

    // prev_fence: Option<EGLFence>,
    // crtc_properties: HashMap<String, property::Info>,
    // plane: drm::control::plane::Handle,
    // plane_properties: HashMap<String, property::Info>,
    input: Libinput,
    xkb_keymap: Keymap,
    xkb_state: xkbcommon_rs::State,
    pending_keys: Vec<(u32, SDL_keysym)>,

    framebuffers: HashMap<u64, drm::control::framebuffer::Handle>,

    termios: Termios,
}

impl DRMWindow {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let card = Arc::new(Card::open_first());

        let handles = card.resource_handles().unwrap();

        // Get the first connector that's connected
        let connector = handles
            .connectors()
            .iter()
            .map(|f| card.get_connector(*f, true))
            .filter(|f| f.is_ok())
            .map(|f| f.unwrap())
            .filter(|f| f.state() == State::Connected)
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .clone();

        let crtc = *handles.crtcs.first().unwrap();

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

        let mode = connector.modes().first().unwrap();

        let plane = **card
            .clone()
            .plane_handles()
            .unwrap()
            .iter()
            .filter(|plane_handle| {
                let plane = card.get_plane(**plane_handle).unwrap();

                let compatible_crtcs = handles.filter_crtcs(plane.possible_crtcs());

                let properties = card.get_properties(**plane_handle).unwrap();
                for (&id, &value) in properties.iter() {
                    let Ok(info) = card.get_property(id) else {
                        continue;
                    };

                    if info
                        .name()
                        .to_str()
                        .map(|name| name == "type")
                        .unwrap_or(false)
                    {
                        return true;
                    }
                }
                false
            })
            .collect::<Vec<_>>()
            .first()
            .unwrap();

        let width = mode.size().0 as u32;
        let height = mode.size().1 as u32;

        let gbm_device = GBMDevice::new(card.clone()).unwrap();

        let gbm_surface = gbm_device
            .create_surface(
                width,
                height,
                gbm::Format::Xrgb8888,
                BufferObjectFlags::SCANOUT | BufferObjectFlags::RENDERING,
            )
            .unwrap();

        pub const EGL_PLATFORM_GBM_KHR: u32 = 0x31D7;
        let (egl, surface, display) = unsafe {
            EGL::setup(
                EGL_PLATFORM_GBM_KHR,
                gbm_device.as_raw() as *mut c_void,
                EGLWindowType::Pointer(gbm_surface.as_raw() as *mut c_void),
            )
        };

        let mut input = Libinput::new_with_udev(InputInterface);
        input.udev_assign_seat("seat0").unwrap();

        let xkb_keymap = Keymap::new_from_names(
            Context::new(ContextFlags::NO_FLAGS).unwrap(),
            None,
            CompileFlags::NO_FLAGS,
        )
        .unwrap();
        let xkb_state = xkbcommon_rs::State::new(xkb_keymap.clone());

        let crtc_properties = card
            .get_properties(crtc)?
            .as_hashmap(&*card.clone())
            .unwrap();

        let plane_properties = card
            .get_properties(plane)?
            .as_hashmap(&*card.clone())
            .unwrap();

        Ok(Self {
            card: card.clone(),
            fake_surface,
            keys: [0; SDLKey_SDLK_LAST as usize],
            egl,
            surface,
            display,
            gbm_device,
            gbm_surface,
            connector,
            crtc,
            framebuffers: HashMap::new(),
            input,
            xkb_keymap,
            xkb_state,
            pending_keys: vec![],
            termios: tcgetattr(stdin()).unwrap(),
            // crtc_properties,
            // plane,
            // plane_properties,
        })
    }

    fn handle_libinput(&mut self) {
        self.input.dispatch().unwrap();
        for ev in &mut self.input {
            match ev {
                input::Event::Keyboard(keyboard_event) => match keyboard_event {
                    KeyboardEvent::Key(keyboard_key_event) => {
                        // yeah for some reason it just doesn't bind these functions? idk
                        let key = unsafe {
                            libinput_event_keyboard_get_key(keyboard_key_event.as_raw() as *mut _)
                        } + 8;
                        let state = unsafe {
                            libinput_event_keyboard_get_key_state(
                                keyboard_key_event.as_raw() as *mut _
                            )
                        };

                        let layout = self.xkb_state.key_get_layout(key).unwrap();
                        let level = self.xkb_keymap.num_levels_for_key(key, layout) - 1;
                        let syms_out = self
                            .xkb_keymap
                            .key_get_syms_by_level(key, layout, level)
                            .unwrap();

                        for sym in syms_out {
                            let key = sdl_key_from_keysym(sym);
                            // println!("{}", key);
                            self.pending_keys.push((
                                state,
                                SDL_keysym {
                                    scancode: 25,
                                    sym: key,
                                    mod_: 0,
                                    unicode: 0,
                                },
                            ));
                            self.keys[key as usize] = state;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

impl Window for DRMWindow {
    fn init(&mut self, _flags: u32) -> i32 {
        let mut t = self.termios.clone();
        cfmakeraw(&mut t);
        tcsetattr(stdin(), nix::sys::termios::SetArg::TCSANOW, &t).unwrap();
        return 0;
    }

    fn quit(&mut self) {
        let t = self.termios.clone();
        tcflush(stdin(), FlushArg::TCIOFLUSH).unwrap();
        tcsetattr(stdin(), nix::sys::termios::SetArg::TCSANOW, &t).unwrap();
    }

    fn get_error(&mut self) -> *const u8 {
        null()
    }

    fn get_key_state(&mut self, numkeys: *mut i32) -> *mut u8 {
        if numkeys != null_mut() {
            unsafe {
                *numkeys = SDLKey_SDLK_LAST as i32;
            }
        }
        return self.keys.as_mut_ptr() as *mut u8;
    }

    fn get_mouse_state(&mut self, x: *mut i32, y: *mut i32) -> u8 {
        // todo!()
        unsafe {
            *x = 0;
            *y = 0;
        };
        return 0;
    }

    fn egl(&self) -> &EGL {
        &self.egl
    }

    fn wait_for_egl(&mut self) {
        // todo?
    }

    fn egl_display(&self) -> crate::egl::NativeDisplayType {
        self.display
    }

    fn egl_surface(&self) -> crate::egl::EGLSurface {
        self.surface
    }

    fn poll_event(&mut self, event: *mut crate::type_defs::SDL_Event) -> i32 {
        self.handle_libinput();
        unsafe {
            if self.pending_keys.len() >= 1 {
                if let Some(ev) = self.pending_keys.pop() {
                    if ev.0 == 0 {
                        (*event).key.type_ = SDL_EventType::SDL_KEYUP as u8;
                    } else {
                        (*event).key.type_ = SDL_EventType::SDL_KEYDOWN as u8;
                    }
                    (*event).key.which = 0;
                    (*event).key.state = ev.0 as u8;
                    (*event).key.keysym = ev.1;
                }
                return 1;
            }
        }
        return 0;
    }

    fn set_video_mode(
        &mut self,
        width: i32,
        height: i32,
        bpp: i32,
        flags: u32,
    ) -> *mut crate::type_defs::SDL_Surface {
        return &mut self.fake_surface;
    }

    fn show_cursor(&mut self, toggle: i32) -> i32 {
        return 0;
    }

    fn warp_mouse(&mut self, x: u16, y: u16) {
        // todo!()
    }

    fn wm_set_caption(&mut self, title: &str, icon: &str) {
        // todo!()
    }

    fn gl_swap_buffers(&mut self) {
        self.gbm_device
            .wait_vblank(
                drm::VblankWaitTarget::Relative(1),
                drm::VblankWaitFlags::empty(),
                0,
                0,
            )
            .unwrap();

        // let mut atomic_req = AtomicModeReq::new();
        // let flags = AtomicCommitFlags::PAGE_FLIP_EVENT | AtomicCommitFlags::ALLOW_MODESET;

        let egl = &self.egl();
        self.gl_swap_buffers_impl();
        // let fence = unsafe { EGLFence::new(self) };
        //
        let mode = self.connector.modes().first().unwrap();

        unsafe {
            let front_buffer = self.gbm_surface.lock_front_buffer().unwrap();
            let bpp = front_buffer.bpp();
            let handle = front_buffer.handle();

            // if let Some(fence) = &self.prev_fence {
            //     fence.destroy(self);
            //     self.prev_fence = None;
            // }

            if !self.framebuffers.contains_key(&handle.u64_) {
                self.framebuffers.insert(
                    handle.u64_,
                    self.card.add_framebuffer(&front_buffer, 24, 32).unwrap(),
                );
            }

            self.card
                .set_crtc(
                    self.crtc,
                    self.framebuffers.get(&handle.u64_).copied(),
                    (0, 0),
                    &[self.connector.handle()],
                    Some(*mode),
                )
                .unwrap();

            // if let Some(fence_prop) = self.crtc_properties.get("IN_FENCE_FD") {
            //     atomic_req.add_property(
            //         self.crtc,
            //         fence_prop.handle(),
            //         Value::SignedRange(fence.fd() as i64),
            //     );
            // } else if let Some(fence_prop) = self.plane_properties.get("IN_FENCE_FD") {
            //     atomic_req.add_property(
            //         self.plane,
            //         fence_prop.handle(),
            //         property::Value::SignedRange(fence.fd() as i64),
            //     );
            // }
            // self.prev_fence = Some(fence);
        }

        // self.card.atomic_commit(flags, atomic_req).unwrap();
    }
}

// struct EGLFence {
//     sync: EGLSyncKHR,
//     fd: i32,
// }

// impl EGLFence {
//     pub unsafe fn new(win: &DRMWindow) -> Self {
//         let egl = win.egl();
//         const EGL_SYNC_FENCE_KHR: u32 = 0x3144;
//         let disp = egl.get_current_display().unwrap();

//         let sync = egl
//             .create_sync_khr(disp, EGL_SYNC_FENCE_KHR, null())
//             .unwrap();
//         assert!(!sync.is_null());

//         let fd = egl.dup_native_fence(disp, sync).unwrap();

//         Self { sync, fd }
//     }
//     pub unsafe fn destroy(&self, win: &DRMWindow) {
//         win.egl()
//             .destroy_sync_khr(win.egl().get_current_display().unwrap(), self.sync)
//             .unwrap();
//     }

//     pub fn sync(&self) -> EGLSyncKHR {
//         self.sync
//     }

//     pub fn fd(&self) -> i32 {
//         self.fd
//     }
// }
