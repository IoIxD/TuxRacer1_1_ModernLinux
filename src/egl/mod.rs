#![allow(dead_code)]
#![allow(unsafe_op_in_unsafe_fn)]

// Custom EGL bindings. Unlike other crate(s).unwrap(), this one not only supports extensions
// but aims to be as straightforward as possible.

mod ffi;
pub use ffi::*;

use std::{error::Error, ffi::c_void, fmt::Display, os::raw::c_char};

use libloading::os::unix::{Library, Symbol};

use crate::egl::ffi::PFNEGLGETPROCADDRESSPROC;

#[derive(Debug)]
pub enum EGLError {
    NotFound,
}

impl Display for EGLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("EGLError::NotFound")
    }
}

impl Error for EGLError {}

pub struct EGL {
    lib: Library,
    proc_address: Symbol<PFNEGLGETPROCADDRESSPROC>,
    sym_table: EGLSymbolTable,
}

unsafe fn load_library(path: &str) -> Option<Library> {
    // On Linux, load library with `RTLD_NOW | RTLD_NODELETE` to fix a SIGSEGV
    // See https://github.com/timothee-haudebourg/khronos-egl/issues/14 for more details.
    Library::open(Some(path), 0x2 | 0x1000).ok()
}

macro_rules! gen_func {
    ($name:ident, () -> $ret:ty$(,)?) => {
        pub unsafe fn $name(&self) -> Result<$ret, EGLError> {
            match self.sym_table.$name {
                Some(a) => Ok(a()),
                None => Err(EGLError::NotFound)
            }
        }
    };
    ($name:ident, ($($arg_name:ident : $arg_type:ty),+$(,)?) -> $ret:ty$(,)?) => {
        pub unsafe fn $name(&self, $($arg_name: $arg_type),*) -> Result<$ret, EGLError> {
            match self.sym_table.$name {
                Some(a) => Ok(a($($arg_name),*)),
                None => Err(EGLError::NotFound)
            }
        }
    };
}

impl EGL {
    pub unsafe fn new() -> Self {
        let lib = load_library("libEGL.so.1")
            .or(load_library("libEGL.so"))
            .expect("EGL not found");

        let proc_address = lib
            .get::<PFNEGLGETPROCADDRESSPROC>(b"eglGetProcAddress")
            .unwrap();

        let sym_table = EGLSymbolTable::new(&lib);

        Self {
            lib,
            proc_address,
            sym_table,
        }
    }
    gen_func!(choose_config, (
            dpy: EGLDisplay,
            attrib_list: *const EGLint,
            configs: *mut EGLConfig,
            config_size: EGLint,
            num_config: *mut EGLint
        ) -> EGLBoolean
    );

    gen_func!(copy_buffers, (
            dpy: EGLDisplay,
            surface: EGLSurface,
            target: EGLNativePixmapType) -> EGLBoolean,
    );

    gen_func!(create_context, (
            dpy: EGLDisplay,
            config: EGLConfig,
            share_context: EGLContext,
            attrib_list: *const EGLint,
        ) -> EGLContext,
    );

    gen_func!(create_pbuffer_surface, (
            dpy: EGLDisplay,
            config: EGLConfig,
            attrib_list: *const EGLint,
        ) -> EGLSurface,
    );

    gen_func!(create_pixmap_surface, (
            dpy: EGLDisplay,
            config: EGLConfig,
            pixmap: EGLNativePixmapType,
            attrib_list: *const EGLint,
        ) -> EGLSurface,
    );

    gen_func!(create_window_surface, (
            dpy: EGLDisplay,
            config: EGLConfig,
            win: EGLNativeWindowType,
            attrib_list: *const EGLint,
        ) -> EGLSurface,
    );

    gen_func!(destroy_context, (
            dpy: EGLDisplay,
            config: EGLConfig,
        ) -> EGLBoolean,
    );

    gen_func!(get_configs, (
            dpy: EGLDisplay,
            configs: *mut EGLConfig,
            config_size: EGLint,
            num_config: *mut EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(get_current_display, () -> EGLDisplay,
    );

    gen_func!(initialize, (dpy: EGLDisplay, major: *mut EGLint, minor: *mut EGLint) -> EGLBoolean,
    );

    gen_func!(make_current, (
            dpy: EGLDisplay,
            draw: EGLSurface,
            read: EGLSurface,
            ctx: EGLContext,
        ) -> EGLBoolean,
    );

    gen_func!(query_context, (
            dpy: EGLDisplay,
            ctx: EGLContext,
            attribute: EGLint,
            value: *mut EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(query_string, (dpy: EGLDisplay, name: EGLint) -> *const ::std::os::raw::c_char,
    );

    gen_func!(query_surface, (
            dpy: EGLDisplay,
            surface: EGLSurface,
            attribute: EGLint,
            value: *mut EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(swap_buffers, (dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean,
    );

    gen_func!(release_tex_image, (dpy: EGLDisplay, surface: EGLSurface, buffer: EGLint) -> EGLBoolean,
    );

    gen_func!(surface_attrib, (
            dpy: EGLDisplay,
            surface: EGLSurface,
            attribute: EGLint,
            value: EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(swap_interval, (
            dpy: EGLDisplay,
            buftype: i32,
        ) -> u32,
    );

    gen_func!(release_thread, () -> u32);

    gen_func!(destroy_sync, (dpy: EGLDisplay, sync: EGLSync) -> u32);

    gen_func!(get_sync_attrib, (
            dpy: EGLDisplay,
            sync: EGLSync,
            attribute: EGLint,
            value: *mut EGLAttrib,
        ) -> EGLBoolean,
    );

    gen_func!(create_image, (
            dpy: EGLDisplay,
            ctx: EGLContext,
            target: EGLenum,
            buffer: EGLClientBuffer,
            attrib_list: *const EGLAttrib,
        ) -> EGLImage,
    );

    gen_func!(destroy_image, (
            platform: *mut c_void,
            native_display: *mut c_void,
        ) -> u32,
    );

    gen_func!(create_platform_window_surface, (
            dpy: EGLDisplay,
            config: EGLConfig,
            native_window: *mut c_void,
            attrib_list: *const EGLAttrib,
        ) -> EGLSurface,
    );

    gen_func!(create_platform_pixmap_surface, (
            dpy: EGLDisplay,
            config: EGLConfig,
            native_pixmap: *mut c_void,
            attrib_list: *const EGLAttrib,
        ) -> EGLSurface,
    );

    gen_func!(wait_sync, (dpy: EGLDisplay, sync: EGLSync, flags: EGLint) -> EGLBoolean,
    );

    gen_func!(debug_message_control_khr, (callback: EGLDEBUGPROCKHR, attrib_list: *const EGLAttrib) -> EGLint,
    );

    gen_func!(query_debug_khr, (attribute: EGLint, value: *mut EGLAttrib) -> EGLBoolean,
    );

    gen_func!(label_object_khr, (
            display: EGLDisplay,
            object_type: EGLenum,
            object: EGLObjectKHR,
            label: EGLLabelKHR,
        ) -> EGLint,
    );

    gen_func!(query_display_attrib_khr, (dpy: EGLDisplay, name: EGLint, value: *mut EGLAttrib) -> EGLBoolean,
    );

    gen_func!(create_sync_khr, (dpy: EGLDisplay, type_: EGLenum, attrib_list: *const EGLint) -> EGLSyncKHR,
    );

    gen_func!(destroy_sync_khr, (
            dpy: EGLDisplay,
        sync: EGLSyncKHR,
        ) -> u32,
    );

    gen_func!(get_sync_attrib_khr, (
            dpy: EGLDisplay,
            sync: EGLSyncKHR,
            attribute: EGLint,
            value: *mut EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(create_image_khr, (
            dpy: EGLDisplay,
            ctx: EGLContext,
            target: EGLenum,
            buffer: EGLClientBuffer,
            attrib_list: *const EGLint,
        ) -> EGLImageKHR,
    );

    gen_func!(destroy_image_khr, (
            dpy: EGLDisplay,
            surface: EGLSurface,
        ) -> EGLBoolean,
    );

    gen_func!(unlock_surface_khr, (
            dpy: EGLDisplay,
            surface: EGLSurface,
        ) -> EGLBoolean,
    );

    gen_func!(set_damageregionkhr, (
            dpy: EGLDisplay,
            surface: EGLSurface,
            rects: *mut EGLint,
            n_rects: EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(signal_sync_khr, (dpy: EGLDisplay, sync: EGLSyncKHR, mode: EGLenum) -> EGLBoolean,
    );

    gen_func!(create_stream_khr, (dpy: EGLDisplay, attrib_list: *const EGLint) -> EGLStreamKHR,
    );

    gen_func!(destroy_stream_khr, (dpy: EGLDisplay, stream: EGLStreamKHR) -> EGLBoolean,
    );

    gen_func!(stream_attrib_khr, (
            dpy: EGLDisplay,
            stream: EGLStreamKHR,
            attribute: EGLenum,
            value: EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(query_stream_khr, (
            dpy: EGLDisplay,
            stream: EGLStreamKHR,
            attribute: EGLenum,
            value: *mut EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(create_stream_attrib_khr, (dpy: EGLDisplay, attrib_list: *const EGLAttrib) -> EGLStreamKHR,
    );

    gen_func!(set_stream_attrib_khr, (
            dpy: EGLDisplay,
            stream: EGLStreamKHR,
            attribute: EGLenum,
            value: EGLAttrib,
        ) -> EGLBoolean,
    );

    gen_func!(query_stream_attrib_khr, (
            dpy: EGLDisplay,
            stream: EGLStreamKHR,
            attribute: EGLenum,
            value: *mut EGLAttrib,
        ) -> EGLBoolean,
    );

    gen_func!(stream_consumer_accquire_attrib_khr, (
            dpy: EGLDisplay,
            stream: EGLStreamKHR,
            attrib_list: *const EGLAttrib,
        ) -> EGLBoolean,
    );

    gen_func!(stream_consumer_release_attrib_khr, (
            dpy: EGLDisplay,
            stream: EGLStreamKHR,
            attrib_list: *const EGLAttrib,
        ) -> EGLBoolean,
    );

    gen_func!(stream_consumer_gltexture_external_khr, (dpy: EGLDisplay, stream: EGLStreamKHR) -> EGLBoolean,
    );

    gen_func!(stream_consumer_accquire_khr, (dpy: EGLDisplay, stream: EGLStreamKHR) -> EGLBoolean,
    );

    gen_func!(stream_consumer_release_khr, (dpy: EGLDisplay, stream: EGLStreamKHR) -> EGLBoolean,
    );

    gen_func!(get_stream_file_descriptor_khr, (dpy: EGLDisplay, stream: EGLStreamKHR) -> EGLNativeFileDescriptorKHR,
    );

    gen_func!(create_stream_file_descriptor_khr, (
            dpy: EGLDisplay,
            file_descriptor: EGLNativeFileDescriptorKHR,
        ) -> EGLStreamKHR,
    );

    gen_func!(query_stream_time_khr, (
            dpy: EGLDisplay,
            stream: EGLStreamKHR,
            attribute: EGLenum,
            value: *mut EGLTimeKHR,
        ) -> EGLBoolean,
    );

    gen_func!(create_stream_producer_surface_khr, (
            dpy: EGLDisplay,
            config: EGLConfig,
            stream: EGLStreamKHR,
            attrib_list: *const EGLint,
        ) -> EGLSurface,
    );

    gen_func!(swap_buffers_with_damage_khr, (
            dpy: EGLDisplay,
            surface: EGLSurface,
            rects: *const EGLint,
            n_rects: EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(wait_sync_khr, (dpy: EGLDisplay, sync: EGLSyncKHR, flags: EGLint) -> EGLint,
    );

    gen_func!(client_signal_sync_ext, (
            dpy: EGLDisplay,
            sync: EGLSync,
            attrib_list: *const EGLAttrib,
        ) -> EGLBoolean,
    );

    gen_func!(compositor_set_context_list_ext, (external_ref_ids: *const EGLint, num_entries: EGLint) -> EGLBoolean,
    );

    gen_func!(compositor_set_context_attributes_ext, (
            external_ref_id: EGLint,
            context_attributes: *const EGLint,
            num_entries: EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(compositor_set_window_list_ext, (
            external_ref_id: EGLint,
            external_win_ids: *const EGLint,
            num_entries: EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(compositor_set_window_attributes_ext, (
            external_win_id: EGLint,
            window_attributes: *const EGLint,
            num_entries: EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(compositor_bind_tex_window_ext, (external_win_id: EGLint) -> EGLBoolean,
    );

    gen_func!(compositor_swap_policy_ext, (external_win_id: EGLint, policy: EGLint) -> EGLBoolean,
    );

    gen_func!(query_device_attrib_ext, (
            device: EGLDeviceEXT,
            attribute: EGLint,
            value: *mut EGLAttrib,
        ) -> EGLBoolean,
    );

    gen_func!(query_device_string_ext, (device: EGLDeviceEXT, name: EGLint) -> *const ::std::os::raw::c_char,
    );

    gen_func!(query_devices_ext, (
            max_devices: EGLint,
            devices: *mut EGLDeviceEXT,
            num_devices: *mut EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(query_display_attrib_ext, (dpy: EGLDisplay, attribute: EGLint, value: *mut EGLAttrib) -> EGLBoolean,
    );

    gen_func!(query_device_binary_ext, (
            device: EGLDeviceEXT,
            name: EGLint,
            max_size: EGLint,
            value: *mut c_void,
            size: *mut EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(destroy_display_ext, (
            dpy: EGLDisplay,
        ) -> EGLBoolean,
    );

    gen_func!(query_dmabufmodifiers_ext, (
            dpy: EGLDisplay,
            format: EGLint,
            max_modifiers: EGLint,
            modifiers: *mut EGLuint64KHR,
            external_only: *mut EGLBoolean,
            num_modifiers: *mut EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(get_output_layers_ext, (
            dpy: EGLDisplay,
            attrib_list: *const EGLAttrib,
            layers: *mut EGLOutputLayerEXT,
            max_layers: EGLint,
            num_layers: *mut EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(get_output_ports_ext, (
            dpy: EGLDisplay,
            attrib_list: *const EGLAttrib,
            ports: *mut EGLOutputPortEXT,
            max_ports: EGLint,
            num_ports: *mut EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(output_layer_attrib_ext, (
            dpy: EGLDisplay,
            layer: EGLOutputLayerEXT,
            attribute: EGLint,
            value: EGLAttrib,
        ) -> EGLBoolean,
    );

    gen_func!(query_output_layer_attrib_ext, (
            dpy: EGLDisplay,
            layer: EGLOutputLayerEXT,
            attribute: EGLint,
            value: *mut EGLAttrib,
        ) -> EGLBoolean,
    );

    gen_func!(query_output_layer_string_ext, (
            dpy: EGLDisplay,
            layer: EGLOutputLayerEXT,
            name: EGLint,
        ) -> *const ::std::os::raw::c_char,
    );

    gen_func!(output_port_attrib_ext, (
            dpy: EGLDisplay,
            port: EGLOutputPortEXT,
            attribute: EGLint,
            value: EGLAttrib,
        ) -> EGLBoolean,
    );

    gen_func!(query_output_port_attrib_ext, (
            dpy: EGLDisplay,
            port: EGLOutputPortEXT,
            attribute: EGLint,
            value: *mut EGLAttrib,
        ) -> EGLBoolean,
    );

    gen_func!(get_proc_address, (name: *const c_char) -> __eglMustCastToProperFunctionPointerType);

    gen_func!(query_output_port_string_ext, (
            dpy: EGLDisplay,
            port: EGLOutputPortEXT,
            name: EGLint,
        ) -> *const ::std::os::raw::c_char,
    );
    gen_func!(get_platform_display, (
            platform: EGLenum,
            native_display: *mut c_void,
            attrib_list: *const isize,
        ) -> EGLDisplay,
    );

    gen_func!(get_platform_display_ext, (
            platform: EGLenum,
            native_display: *mut c_void,
            attrib_list: *const EGLint,
        ) -> EGLDisplay,
    );

    gen_func!(create_platform_window_surface_ext, (
            dpy: EGLDisplay,
            config: EGLConfig,
            native_window: *mut c_void,
            attrib_list: *const EGLint,
        ) -> EGLSurface,
    );

    gen_func!(create_platform_pixmap_surface_ext, (
            dpy: EGLDisplay,
            config: EGLConfig,
            native_pixmap: *mut c_void,
            attrib_list: *const EGLint,
        ) -> EGLSurface,
    );

    gen_func!(stream_consumer_output_ext, (
            dpy: EGLDisplay,
            stream: EGLStreamKHR,
            layer: EGLOutputLayerEXT,
        ) -> EGLBoolean,
    );

    gen_func!(query_supported_compression_rates_ext, (
            dpy: EGLDisplay,
            config: EGLConfig,
            attrib_list: *const EGLAttrib,
            rates: *mut EGLint,
            rate_size: EGLint,
            num_rates: *mut EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(swap_buffers_with_damage_ext, (
            dpy: EGLDisplay,
            surface: EGLSurface,
            rects: *const EGLint,
            n_rects: EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(unsignal_sync_ext, (
            dpy: EGLDisplay,
            sync: EGLSync,
            attrib_list: *const EGLAttrib,
        ) -> EGLBoolean,
    );

    gen_func!(create_pixmap_surface_hi, (
            dpy: EGLDisplay,
            config: EGLConfig,
            pixmap: *mut EGLClientPixmapHI,
        ) -> EGLSurface,
    );

    /*  gen_func!(create_drm_image_mesa, (dpy: EGLDisplay, attrib_list: *const EGLint) -> EGLImageKHR,
    );

    gen_func!(export_drm_image_mesa, (
            dpy: EGLDisplay,
            image: EGLImageKHR,
            name: *mut EGLint,
            handle: *mut EGLint,
            stride: *mut EGLint,
        ) -> EGLBoolean,
    );

    gen_func!(export_dmabuf_image_query_mesa, (
            dpy: EGLDisplay,
            image: EGLImageKHR,
            fourcc: *mut ::std::os::raw::c_int,
            num_planes: *mut ::std::os::raw::c_int,
            modifiers: *mut EGLuint64KHR,
        ) -> EGLBoolean,
    );

    gen_func!(export_dmabuf_image_mesa, (
            dpy: EGLDisplay,
            image: EGLImageKHR,
            fds: *mut ::std::os::raw::c_int,
            strides: *mut EGLint,
            offsets: *mut EGLint,
        ) -> EGLBoolean,
    );*/

    gen_func!(get_display_driver_config, (
            dpy: EGLDisplay,
        ) -> *mut i8,
    );

    /*gen_func!(swap_buffers_region2nok, (
            dpy: EGLDisplay,
            surface: EGLSurface,
            numRects: EGLint,
            rects: *const EGLint,
        ) -> EGLBoolean,
    );*/

    gen_func!(unbind_wayland_display_wl, (dpy: EGLDisplay, display: *mut wl_display) -> EGLBoolean,
    );

    gen_func!(query_wayland_buffer_wl, (
            dpy: EGLDisplay,
            buffer: *mut wl_resource,
            attribute: EGLint,
            value: *mut EGLint,
        ) -> EGLBoolean,
    );

    /*gen_func!(create_wayland_buffer_from_image_wl, (dpy: EGLDisplay, image: EGLImageKHR) -> *mut wl_buffer,
    );*/
}

struct EGLSymbolTable {
    choose_config: crate::egl::ffi::PFNEGLCHOOSECONFIGPROC,
    copy_buffers: crate::egl::ffi::PFNEGLCOPYBUFFERSPROC,
    create_context: crate::egl::ffi::PFNEGLCREATECONTEXTPROC,
    create_pbuffer_surface: crate::egl::ffi::PFNEGLCREATEPBUFFERSURFACEPROC,
    create_pixmap_surface: crate::egl::ffi::PFNEGLCREATEPIXMAPSURFACEPROC,
    create_window_surface: crate::egl::ffi::PFNEGLCREATEWINDOWSURFACEPROC,
    destroy_context: crate::egl::ffi::PFNEGLDESTROYCONTEXTPROC,
    destroy_surface: crate::egl::ffi::PFNEGLDESTROYSURFACEPROC,
    get_config_attrib: crate::egl::ffi::PFNEGLGETCONFIGATTRIBPROC,
    get_configs: crate::egl::ffi::PFNEGLGETCONFIGSPROC,
    get_current_display: crate::egl::ffi::PFNEGLGETCURRENTDISPLAYPROC,
    get_current_surface: crate::egl::ffi::PFNEGLGETCURRENTSURFACEPROC,
    get_display: crate::egl::ffi::PFNEGLGETDISPLAYPROC,
    get_error: crate::egl::ffi::PFNEGLGETERRORPROC,
    get_proc_address: crate::egl::ffi::PFNEGLGETPROCADDRESSPROC,
    initialize: crate::egl::ffi::PFNEGLINITIALIZEPROC,
    make_current: crate::egl::ffi::PFNEGLMAKECURRENTPROC,
    query_context: crate::egl::ffi::PFNEGLQUERYCONTEXTPROC,
    query_string: crate::egl::ffi::PFNEGLQUERYSTRINGPROC,
    query_surface: crate::egl::ffi::PFNEGLQUERYSURFACEPROC,
    swap_buffers: crate::egl::ffi::PFNEGLSWAPBUFFERSPROC,
    terminate: crate::egl::ffi::PFNEGLTERMINATEPROC,
    wait_gl: crate::egl::ffi::PFNEGLWAITGLPROC,
    wait_native: crate::egl::ffi::PFNEGLWAITNATIVEPROC,
    bind_tex_image: crate::egl::ffi::PFNEGLBINDTEXIMAGEPROC,
    release_tex_image: crate::egl::ffi::PFNEGLRELEASETEXIMAGEPROC,
    surface_attrib: crate::egl::ffi::PFNEGLSURFACEATTRIBPROC,
    swap_interval: crate::egl::ffi::PFNEGLSWAPINTERVALPROC,
    bind_api: crate::egl::ffi::PFNEGLBINDAPIPROC,
    query_api: crate::egl::ffi::PFNEGLQUERYAPIPROC,
    create_pbuffer_from_client_buffer: crate::egl::ffi::PFNEGLCREATEPBUFFERFROMCLIENTBUFFERPROC,
    release_thread: crate::egl::ffi::PFNEGLRELEASETHREADPROC,
    wait_client: crate::egl::ffi::PFNEGLWAITCLIENTPROC,
    get_current_context: crate::egl::ffi::PFNEGLGETCURRENTCONTEXTPROC,
    create_sync: crate::egl::ffi::PFNEGLCREATESYNCPROC,
    destroy_sync: crate::egl::ffi::PFNEGLDESTROYSYNCPROC,
    client_wait_sync: crate::egl::ffi::PFNEGLCLIENTWAITSYNCPROC,
    get_sync_attrib: crate::egl::ffi::PFNEGLGETSYNCATTRIBPROC,
    create_image: crate::egl::ffi::PFNEGLCREATEIMAGEPROC,
    destroy_image: crate::egl::ffi::PFNEGLDESTROYIMAGEPROC,
    get_platform_display: crate::egl::ffi::PFNEGLGETPLATFORMDISPLAYPROC,
    create_platform_window_surface: crate::egl::ffi::PFNEGLCREATEPLATFORMWINDOWSURFACEPROC,
    create_platform_pixmap_surface: crate::egl::ffi::PFNEGLCREATEPLATFORMPIXMAPSURFACEPROC,
    wait_sync: crate::egl::ffi::PFNEGLWAITSYNCPROC,
    debug_message_control_khr: crate::egl::ffi::PFNEGLDEBUGMESSAGECONTROLKHRPROC,
    query_debug_khr: crate::egl::ffi::PFNEGLQUERYDEBUGKHRPROC,
    label_object_khr: crate::egl::ffi::PFNEGLLABELOBJECTKHRPROC,
    query_display_attrib_khr: crate::egl::ffi::PFNEGLQUERYDISPLAYATTRIBKHRPROC,
    create_sync_khr: crate::egl::ffi::PFNEGLCREATESYNCKHRPROC,
    destroy_sync_khr: crate::egl::ffi::PFNEGLDESTROYSYNCKHRPROC,
    client_wait_sync_khr: crate::egl::ffi::PFNEGLCLIENTWAITSYNCKHRPROC,
    get_sync_attrib_khr: crate::egl::ffi::PFNEGLGETSYNCATTRIBKHRPROC,
    create_image_khr: crate::egl::ffi::PFNEGLCREATEIMAGEKHRPROC,
    destroy_image_khr: crate::egl::ffi::PFNEGLDESTROYIMAGEKHRPROC,
    lock_surface_khr: crate::egl::ffi::PFNEGLLOCKSURFACEKHRPROC,
    unlock_surface_khr: crate::egl::ffi::PFNEGLUNLOCKSURFACEKHRPROC,
    set_damageregionkhr: crate::egl::ffi::PFNEGLSETDAMAGEREGIONKHRPROC,
    signal_sync_khr: crate::egl::ffi::PFNEGLSIGNALSYNCKHRPROC,
    create_stream_khr: crate::egl::ffi::PFNEGLCREATESTREAMKHRPROC,
    destroy_stream_khr: crate::egl::ffi::PFNEGLDESTROYSTREAMKHRPROC,
    stream_attrib_khr: crate::egl::ffi::PFNEGLSTREAMATTRIBKHRPROC,
    query_stream_khr: crate::egl::ffi::PFNEGLQUERYSTREAMKHRPROC,
    create_stream_attrib_khr: crate::egl::ffi::PFNEGLCREATESTREAMATTRIBKHRPROC,
    set_stream_attrib_khr: crate::egl::ffi::PFNEGLSETSTREAMATTRIBKHRPROC,
    query_stream_attrib_khr: crate::egl::ffi::PFNEGLQUERYSTREAMATTRIBKHRPROC,
    stream_consumer_accquire_attrib_khr: crate::egl::ffi::PFNEGLSTREAMCONSUMERACQUIREATTRIBKHRPROC,
    stream_consumer_release_attrib_khr: crate::egl::ffi::PFNEGLSTREAMCONSUMERRELEASEATTRIBKHRPROC,
    stream_consumer_gltexture_external_khr:
        crate::egl::ffi::PFNEGLSTREAMCONSUMERGLTEXTUREEXTERNALKHRPROC,
    stream_consumer_accquire_khr: crate::egl::ffi::PFNEGLSTREAMCONSUMERACQUIREKHRPROC,
    stream_consumer_release_khr: crate::egl::ffi::PFNEGLSTREAMCONSUMERRELEASEKHRPROC,
    get_stream_file_descriptor_khr: crate::egl::ffi::PFNEGLGETSTREAMFILEDESCRIPTORKHRPROC,
    create_stream_file_descriptor_khr: crate::egl::ffi::PFNEGLCREATESTREAMFROMFILEDESCRIPTORKHRPROC,
    query_stream_time_khr: crate::egl::ffi::PFNEGLQUERYSTREAMTIMEKHRPROC,
    create_stream_producer_surface_khr: crate::egl::ffi::PFNEGLCREATESTREAMPRODUCERSURFACEKHRPROC,
    swap_buffers_with_damage_khr: crate::egl::ffi::PFNEGLSWAPBUFFERSWITHDAMAGEKHRPROC,
    wait_sync_khr: crate::egl::ffi::PFNEGLWAITSYNCKHRPROC,
    client_signal_sync_ext: crate::egl::ffi::PFNEGLCLIENTSIGNALSYNCEXTPROC,
    compositor_set_context_list_ext: crate::egl::ffi::PFNEGLCOMPOSITORSETCONTEXTLISTEXTPROC,
    compositor_set_context_attributes_ext:
        crate::egl::ffi::PFNEGLCOMPOSITORSETCONTEXTATTRIBUTESEXTPROC,
    compositor_set_window_list_ext: crate::egl::ffi::PFNEGLCOMPOSITORSETWINDOWLISTEXTPROC,
    compositor_set_window_attributes_ext:
        crate::egl::ffi::PFNEGLCOMPOSITORSETWINDOWATTRIBUTESEXTPROC,
    compositor_bind_tex_window_ext: crate::egl::ffi::PFNEGLCOMPOSITORBINDTEXWINDOWEXTPROC,
    compositor_set_size_ext: crate::egl::ffi::PFNEGLCOMPOSITORSETSIZEEXTPROC,
    compositor_swap_policy_ext: crate::egl::ffi::PFNEGLCOMPOSITORSWAPPOLICYEXTPROC,
    query_device_attrib_ext: crate::egl::ffi::PFNEGLQUERYDEVICEATTRIBEXTPROC,
    query_device_string_ext: crate::egl::ffi::PFNEGLQUERYDEVICESTRINGEXTPROC,
    query_devices_ext: crate::egl::ffi::PFNEGLQUERYDEVICESEXTPROC,
    query_display_attrib_ext: crate::egl::ffi::PFNEGLQUERYDISPLAYATTRIBEXTPROC,
    query_device_binary_ext: crate::egl::ffi::PFNEGLQUERYDEVICEBINARYEXTPROC,
    destroy_display_ext: crate::egl::ffi::PFNEGLDESTROYDISPLAYEXTPROC,
    query_dmabufformats_ext: crate::egl::ffi::PFNEGLQUERYDMABUFFORMATSEXTPROC,
    query_dmabufmodifiers_ext: crate::egl::ffi::PFNEGLQUERYDMABUFMODIFIERSEXTPROC,
    get_output_layers_ext: crate::egl::ffi::PFNEGLGETOUTPUTLAYERSEXTPROC,
    get_output_ports_ext: crate::egl::ffi::PFNEGLGETOUTPUTPORTSEXTPROC,
    output_layer_attrib_ext: crate::egl::ffi::PFNEGLOUTPUTLAYERATTRIBEXTPROC,
    query_output_layer_attrib_ext: crate::egl::ffi::PFNEGLQUERYOUTPUTLAYERATTRIBEXTPROC,
    query_output_layer_string_ext: crate::egl::ffi::PFNEGLQUERYOUTPUTLAYERSTRINGEXTPROC,
    output_port_attrib_ext: crate::egl::ffi::PFNEGLOUTPUTPORTATTRIBEXTPROC,
    query_output_port_attrib_ext: crate::egl::ffi::PFNEGLQUERYOUTPUTPORTATTRIBEXTPROC,
    query_output_port_string_ext: crate::egl::ffi::PFNEGLQUERYOUTPUTPORTSTRINGEXTPROC,
    get_platform_display_ext: crate::egl::ffi::PFNEGLGETPLATFORMDISPLAYEXTPROC,
    create_platform_window_surface_ext: crate::egl::ffi::PFNEGLCREATEPLATFORMWINDOWSURFACEEXTPROC,
    create_platform_pixmap_surface_ext: crate::egl::ffi::PFNEGLCREATEPLATFORMPIXMAPSURFACEEXTPROC,
    stream_consumer_output_ext: crate::egl::ffi::PFNEGLSTREAMCONSUMEROUTPUTEXTPROC,
    query_supported_compression_rates_ext:
        crate::egl::ffi::PFNEGLQUERYSUPPORTEDCOMPRESSIONRATESEXTPROC,
    swap_buffers_with_damage_ext: crate::egl::ffi::PFNEGLSWAPBUFFERSWITHDAMAGEEXTPROC,
    unsignal_sync_ext: crate::egl::ffi::PFNEGLUNSIGNALSYNCEXTPROC,
    create_pixmap_surface_hi: crate::egl::ffi::PFNEGLCREATEPIXMAPSURFACEHIPROC,
    create_drmimage_mesa: crate::egl::ffi::PFNEGLCREATEDRMIMAGEMESAPROC,
    export_drmimage_mesa: crate::egl::ffi::PFNEGLEXPORTDRMIMAGEMESAPROC,
    export_dmabufimage_query_mesa: crate::egl::ffi::PFNEGLEXPORTDMABUFIMAGEQUERYMESAPROC,
    export_dmabufimage_mesa: crate::egl::ffi::PFNEGLEXPORTDMABUFIMAGEMESAPROC,
    get_display_driver_config: crate::egl::ffi::PFNEGLGETDISPLAYDRIVERCONFIGPROC,
    get_display_driver_name: crate::egl::ffi::PFNEGLGETDISPLAYDRIVERNAMEPROC,
    swap_buffers_region_nok: crate::egl::ffi::PFNEGLSWAPBUFFERSREGIONNOKPROC,
    swap_buffers_region2_nok: crate::egl::ffi::PFNEGLSWAPBUFFERSREGION2NOKPROC,
    bind_wayland_display_wl: crate::egl::ffi::PFNEGLBINDWAYLANDDISPLAYWLPROC,
    unbind_wayland_display_wl: crate::egl::ffi::PFNEGLUNBINDWAYLANDDISPLAYWLPROC,
    query_wayland_buffer_wl: crate::egl::ffi::PFNEGLQUERYWAYLANDBUFFERWLPROC,
}

macro_rules! func_load {
    ($get_proc_address:ident, $name:literal) => {{
        // println!("EGL {:?}", $name);
        // let time = SystemTime::now();
        // while time.elapsed().unwrap().as_secs() <= 1 {}
        std::mem::transmute(($get_proc_address)($name.as_ptr()))
    }};
}

unsafe extern "C" {
    fn eglGetProcAddress(
        procname: *const ::std::os::raw::c_char,
    ) -> __eglMustCastToProperFunctionPointerType;
}

impl EGLSymbolTable {
    pub unsafe fn new(lib: &Library) -> Self {
        let get_proc_address = (*lib
            .get::<PFNEGLGETPROCADDRESSPROC>(b"eglGetProcAddress")
            .unwrap())
        .unwrap();

        Self {
            choose_config: func_load!(get_proc_address, c"eglChooseConfig"),
            copy_buffers: func_load!(get_proc_address, c"eglCopyBuffers"),
            create_context: func_load!(get_proc_address, c"eglCreateContext"),
            create_pbuffer_surface: func_load!(get_proc_address, c"eglCreatePbufferSurface"),
            create_pixmap_surface: func_load!(get_proc_address, c"eglCreatePixmapSurface"),
            create_window_surface: func_load!(get_proc_address, c"eglCreateWindowSurface"),
            destroy_context: func_load!(get_proc_address, c"eglDestroyContext"),
            destroy_surface: func_load!(get_proc_address, c"eglDestroySurface"),
            get_config_attrib: func_load!(get_proc_address, c"eglGetConfigAttric"),
            get_configs: func_load!(get_proc_address, c"eglGetConfigs"),
            get_current_display: func_load!(get_proc_address, c"eglGetCurrentDisplay"),
            get_current_surface: func_load!(get_proc_address, c"eglGetCurrentSurface"),
            get_display: func_load!(get_proc_address, c"eglGetDisplay"),
            get_error: func_load!(get_proc_address, c"eglGetError"),
            get_proc_address: func_load!(get_proc_address, c"eglGetProcAddress"),
            initialize: func_load!(get_proc_address, c"eglInitialize"),
            make_current: func_load!(get_proc_address, c"eglMakeCurrent"),
            query_context: func_load!(get_proc_address, c"eglQueryContext"),
            query_string: func_load!(get_proc_address, c"eglQueryString"),
            query_surface: func_load!(get_proc_address, c"eglQuerySurface"),
            swap_buffers: func_load!(get_proc_address, c"eglSwapBuffers"),
            terminate: func_load!(get_proc_address, c"eglTerminate"),
            wait_gl: func_load!(get_proc_address, c"eglWaitGL"),
            wait_native: func_load!(get_proc_address, c"eglWaitNative"),
            bind_tex_image: func_load!(get_proc_address, c"eglBindTexImage"),
            release_tex_image: func_load!(get_proc_address, c"eglReleaseTexImage"),
            surface_attrib: func_load!(get_proc_address, c"eglSurfaceAttric"),
            swap_interval: func_load!(get_proc_address, c"eglSwapInterval"),
            bind_api: func_load!(get_proc_address, c"eglBindAPI"),
            query_api: func_load!(get_proc_address, c"eglQueryAPI"),
            create_pbuffer_from_client_buffer: func_load!(
                get_proc_address,
                c"eglCreatePbufferFromClientBuffer"
            ),
            release_thread: func_load!(get_proc_address, c"eglReleaseThread"),
            wait_client: func_load!(get_proc_address, c"eglWaitClient"),
            get_current_context: func_load!(get_proc_address, c"eglGetCurrentContext"),
            create_sync: func_load!(get_proc_address, c"eglCreateSync"),
            destroy_sync: func_load!(get_proc_address, c"eglDestroySync"),
            client_wait_sync: func_load!(get_proc_address, c"eglClientWaitSync"),
            get_sync_attrib: func_load!(get_proc_address, c"eglGetSyncAttric"),
            create_image: func_load!(get_proc_address, c"eglCreateImage"),
            destroy_image: func_load!(get_proc_address, c"eglDestroyImage"),
            get_platform_display: func_load!(get_proc_address, c"eglGetPlatformDisplay"),
            create_platform_window_surface: func_load!(
                get_proc_address,
                c"eglCreatePlatformWindowSurface"
            ),
            create_platform_pixmap_surface: func_load!(
                get_proc_address,
                c"eglCreatePlatformPixmapSurface"
            ),
            wait_sync: func_load!(get_proc_address, c"eglWaitSync"),
            debug_message_control_khr: func_load!(get_proc_address, c"eglDebugMessageControlKHR"),
            query_debug_khr: func_load!(get_proc_address, c"eglQueryDebugKHR"),
            label_object_khr: func_load!(get_proc_address, c"eglLabelObjectKHR"),
            query_display_attrib_khr: func_load!(get_proc_address, c"eglQueryDisplayAttribKHR"),
            create_sync_khr: func_load!(get_proc_address, c"eglCreateSyncKHR"),
            destroy_sync_khr: func_load!(get_proc_address, c"eglDestroySyncKHR"),
            client_wait_sync_khr: func_load!(get_proc_address, c"eglClientWaitSyncKHR"),
            get_sync_attrib_khr: func_load!(get_proc_address, c"eglGetSyncAttribKHR"),
            create_image_khr: func_load!(get_proc_address, c"eglCreateImageKHR"),
            destroy_image_khr: func_load!(get_proc_address, c"eglDestroyImageKHR"),
            lock_surface_khr: func_load!(get_proc_address, c"eglLockSurfaceKHR"),
            unlock_surface_khr: func_load!(get_proc_address, c"eglUnlockSurfaceKHR"),
            set_damageregionkhr: func_load!(get_proc_address, c"eglSetDAMAGEREGIONKHR"),
            signal_sync_khr: func_load!(get_proc_address, c"eglSignalSyncKHR"),
            create_stream_khr: func_load!(get_proc_address, c"eglCreateStreamKHR"),
            destroy_stream_khr: func_load!(get_proc_address, c"eglDestroyStreamKHR"),
            stream_attrib_khr: func_load!(get_proc_address, c"eglStreamAttribKHR"),
            query_stream_khr: func_load!(get_proc_address, c"eglQueryStreamKHR"),
            create_stream_attrib_khr: func_load!(get_proc_address, c"eglCreateStreamAttribKHR"),
            set_stream_attrib_khr: func_load!(get_proc_address, c"eglSetStreamAttribKHR"),
            query_stream_attrib_khr: func_load!(get_proc_address, c"eglQueryStreamAttribKHR"),
            stream_consumer_accquire_attrib_khr: func_load!(
                get_proc_address,
                c"eglStreamConsumerAccquireAttribKHR"
            ),
            stream_consumer_release_attrib_khr: func_load!(
                get_proc_address,
                c"eglStreamConsumerReleaseAttribKHR"
            ),
            stream_consumer_gltexture_external_khr: func_load!(
                get_proc_address,
                c"eglStreamConsumerGLTextureExternalKHR"
            ),
            stream_consumer_accquire_khr: func_load!(
                get_proc_address,
                c"eglStreamConsumerAccquireKHR"
            ),
            stream_consumer_release_khr: func_load!(
                get_proc_address,
                c"eglStreamConsumerReleaseKHR"
            ),
            get_stream_file_descriptor_khr: func_load!(
                get_proc_address,
                c"eglGetStreamFileDescriptorKHR"
            ),
            create_stream_file_descriptor_khr: func_load!(
                get_proc_address,
                c"eglCreateStreamFileDescriptorKHR"
            ),
            query_stream_time_khr: func_load!(get_proc_address, c"eglQueryStreamTimeKHR"),
            create_stream_producer_surface_khr: func_load!(
                get_proc_address,
                c"eglCreateStreamProducerSurfaceKHR"
            ),
            swap_buffers_with_damage_khr: func_load!(
                get_proc_address,
                c"eglSwapBuffersWithDamageKHR"
            ),
            wait_sync_khr: func_load!(get_proc_address, c"eglWaitSyncKHR"),
            client_signal_sync_ext: func_load!(get_proc_address, c"eglClientSignalSyncEXT"),
            compositor_set_context_list_ext: func_load!(
                get_proc_address,
                c"eglCompositorSetContextListEXT"
            ),
            compositor_set_context_attributes_ext: func_load!(
                get_proc_address,
                c"eglCompositorSetContextAttributesEXT"
            ),
            compositor_set_window_list_ext: func_load!(
                get_proc_address,
                c"eglCompositorSetWindowListEXT"
            ),
            compositor_set_window_attributes_ext: func_load!(
                get_proc_address,
                c"eglCompositorSetWindowAttributesEXT"
            ),
            compositor_bind_tex_window_ext: func_load!(
                get_proc_address,
                c"eglCompositorBindTexWindowEXT"
            ),
            compositor_set_size_ext: func_load!(get_proc_address, c"eglCompositorSetSizeEXT"),
            compositor_swap_policy_ext: func_load!(get_proc_address, c"eglCompositorSwapPolicyEXT"),
            query_device_attrib_ext: func_load!(get_proc_address, c"eglQueryDeviceAttribEXT"),
            query_device_string_ext: func_load!(get_proc_address, c"eglQueryDeviceStringEXT"),
            query_devices_ext: func_load!(get_proc_address, c"eglQueryDevicesEXT"),
            query_display_attrib_ext: func_load!(get_proc_address, c"eglQueryDisplayAttribEXT"),
            query_device_binary_ext: func_load!(get_proc_address, c"eglQueryDeviceBinaryEXT"),
            destroy_display_ext: func_load!(get_proc_address, c"eglDestroyDisplayEXT"),
            query_dmabufformats_ext: func_load!(get_proc_address, c"eglQueryDMABUFFormatsEXT"),
            query_dmabufmodifiers_ext: func_load!(get_proc_address, c"eglQueryDMABUFModifiersEXT"),
            get_output_layers_ext: func_load!(get_proc_address, c"eglGetOutputLayersEXT"),
            get_output_ports_ext: func_load!(get_proc_address, c"eglGetOutputPortsEXT"),
            output_layer_attrib_ext: func_load!(get_proc_address, c"eglOutputLayerAttribEXT"),
            query_output_layer_attrib_ext: func_load!(
                get_proc_address,
                c"eglQueryOutputLayerAttribEXT"
            ),
            query_output_layer_string_ext: func_load!(
                get_proc_address,
                c"eglQueryOutputLayerStringEXT"
            ),
            output_port_attrib_ext: func_load!(get_proc_address, c"eglOutputPortAttribEXT"),
            query_output_port_attrib_ext: func_load!(
                get_proc_address,
                c"eglQueryOutputPortAttribEXT"
            ),
            query_output_port_string_ext: func_load!(
                get_proc_address,
                c"eglQueryOutputPortStringEXT"
            ),
            get_platform_display_ext: func_load!(get_proc_address, c"eglGetPlatformDisplayEXT"),
            create_platform_window_surface_ext: func_load!(
                get_proc_address,
                c"eglCreatePlatformWindowSurfaceEXT"
            ),
            create_platform_pixmap_surface_ext: func_load!(
                get_proc_address,
                c"eglCreatePlatformPixmapSurfaceEXT"
            ),
            stream_consumer_output_ext: func_load!(get_proc_address, c"eglStreamConsumerOutputEXT"),
            query_supported_compression_rates_ext: func_load!(
                get_proc_address,
                c"eglQuerySupportedCompressionRatesEXT"
            ),
            swap_buffers_with_damage_ext: func_load!(
                get_proc_address,
                c"eglSwapBuffersWithDamageEXT"
            ),
            unsignal_sync_ext: func_load!(get_proc_address, c"eglUnsignalSyncEXT"),
            create_pixmap_surface_hi: func_load!(get_proc_address, c"eglCreatePixmapSurfaceHI"),
            create_drmimage_mesa: func_load!(get_proc_address, c"eglCreateDRMImageMESA"),
            export_drmimage_mesa: func_load!(get_proc_address, c"eglExportDRMImageMESA"),
            export_dmabufimage_query_mesa: func_load!(
                get_proc_address,
                c"eglExportDMABUFImageQueryMESA"
            ),
            export_dmabufimage_mesa: func_load!(get_proc_address, c"eglExportDMABUFImageMESA"),
            get_display_driver_config: func_load!(get_proc_address, c"eglGetDisplayDriverConfig"),
            get_display_driver_name: func_load!(get_proc_address, c"eglGetDisplayDriverName"),
            swap_buffers_region_nok: func_load!(get_proc_address, c"eglSwapBuffersRegionNOK"),
            swap_buffers_region2_nok: func_load!(get_proc_address, c"eglSwapBuffersRegion2NOK"),
            bind_wayland_display_wl: func_load!(get_proc_address, c"eglBindWaylandDisplayWL"),
            unbind_wayland_display_wl: func_load!(get_proc_address, c"eglUnbindWaylandDisplayWL"),
            query_wayland_buffer_wl: func_load!(get_proc_address, c"eglQueryWaylandBufferWL"),
        }
    }

    fn destroy_surface(&self) -> &crate::egl::ffi::PFNEGLDESTROYSURFACEPROC {
        &self.destroy_surface
    }
}
