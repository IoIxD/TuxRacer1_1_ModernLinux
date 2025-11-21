// Dude I spent a day and a half trying to debug this and I found out that the Mesa libraries were tranparently failing to load because of "missing zlib functions" for a version of it from 2005. I am going to cry.

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

// pub type off_t = ::std::os::raw::c_long;
// pub const ZLIB_VERSION: &[u8; 8] = b"1.2.2.3\0";
// pub const ZLIB_VERNUM: u32 = 4643;
// pub const Z_NO_FLUSH: u32 = 0;
// pub const Z_PARTIAL_FLUSH: u32 = 1;
// pub const Z_SYNC_FLUSH: u32 = 2;
// pub const Z_FULL_FLUSH: u32 = 3;
// pub const Z_FINISH: u32 = 4;
// pub const Z_BLOCK: u32 = 5;
// pub const Z_OK: u32 = 0;
// pub const Z_STREAM_END: u32 = 1;
// pub const Z_NEED_DICT: u32 = 2;
// pub const Z_ERRNO: i32 = -1;
// pub const Z_STREAM_ERROR: i32 = -2;
// pub const Z_DATA_ERROR: i32 = -3;
// pub const Z_MEM_ERROR: i32 = -4;
// pub const Z_BUF_ERROR: i32 = -5;
// pub const Z_VERSION_ERROR: i32 = -6;
// pub const Z_NO_COMPRESSION: u32 = 0;
// pub const Z_BEST_SPEED: u32 = 1;
// pub const Z_BEST_COMPRESSION: u32 = 9;
// pub const Z_DEFAULT_COMPRESSION: i32 = -1;
// pub const Z_FILTERED: u32 = 1;
// pub const Z_HUFFMAN_ONLY: u32 = 2;
// pub const Z_RLE: u32 = 3;
// pub const Z_FIXED: u32 = 4;
// pub const Z_DEFAULT_STRATEGY: u32 = 0;
// pub const Z_BINARY: u32 = 0;
// pub const Z_TEXT: u32 = 1;
// pub const Z_ASCII: u32 = 1;
// pub const Z_UNKNOWN: u32 = 2;
// pub const Z_DEFLATED: u32 = 8;
// pub const Z_NULL: u32 = 0;
// pub type Byte = ::std::os::raw::c_uchar;
// pub type Bytef = Byte;
// pub type uInt = ::std::os::raw::c_uint;
// pub type uLong = ::std::os::raw::c_ulong;
// pub type charf = ::std::os::raw::c_char;
// pub type intf = ::std::os::raw::c_int;
// pub type uIntf = uInt;
// pub type uLongf = uLong;
// pub type voidpc = *const ::std::os::raw::c_void;
// pub type voidpf = *mut ::std::os::raw::c_void;
// pub type voidp = *mut ::std::os::raw::c_void;
// pub type z_crc_t = ::std::os::raw::c_uint;
// pub type z_size_t = usize;
// pub type alloc_func =
//     ::std::option::Option<unsafe extern "C" fn(opaque: voidpf, items: uInt, size: uInt) -> voidpf>;
// pub type free_func = ::std::option::Option<unsafe extern "C" fn(opaque: voidpf, address: voidpf)>;
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct z_stream_s {
//     pub next_in: *mut Bytef,
//     pub avail_in: uInt,
//     pub total_in: uLong,
//     pub next_out: *mut Bytef,
//     pub avail_out: uInt,
//     pub total_out: uLong,
//     pub msg: *mut ::std::os::raw::c_char,
//     pub state: *mut internal_state,
//     pub zalloc: alloc_func,
//     pub zfree: free_func,
//     pub opaque: voidpf,
//     pub data_type: ::std::os::raw::c_int,
//     pub adler: uLong,
//     pub reserved: uLong,
// }

// pub type z_stream = z_stream_s;
// pub type z_streamp = *mut z_stream;
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct gz_header_s {
//     pub text: ::std::os::raw::c_int,
//     pub time: uLong,
//     pub xflags: ::std::os::raw::c_int,
//     pub os: ::std::os::raw::c_int,
//     pub extra: *mut Bytef,
//     pub extra_len: uInt,
//     pub extra_max: uInt,
//     pub name: *mut Bytef,
//     pub name_max: uInt,
//     pub comment: *mut Bytef,
//     pub comm_max: uInt,
//     pub hcrc: ::std::os::raw::c_int,
//     pub done: ::std::os::raw::c_int,
// }

// pub type gz_header = gz_header_s;
// pub type gz_headerp = *mut gz_header;
// pub type gzFile = voidp;

// pub type in_func = ::std::option::Option<
//     unsafe extern "C" fn(
//         arg1: *mut ::std::os::raw::c_void,
//         arg2: *mut *mut ::std::os::raw::c_uchar,
//     ) -> ::std::os::raw::c_uint,
// >;
// pub type out_func = ::std::option::Option<
//     unsafe extern "C" fn(
//         arg1: *mut ::std::os::raw::c_void,
//         arg2: *mut ::std::os::raw::c_uchar,
//         arg3: ::std::os::raw::c_uint,
//     ) -> ::std::os::raw::c_int,
// >;

// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct internal_state {
//     pub dummy: ::std::os::raw::c_int,
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn zlibVersion() -> *const ::std::os::raw::c_char {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn deflate(
//     strm: z_streamp,
//     flush: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn deflateEnd(strm: z_streamp) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn inflate(
//     strm: z_streamp,
//     flush: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn inflateEnd(strm: z_streamp) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn deflateSetDictionary(
//     strm: z_streamp,
//     dictionary: *const Bytef,
//     dictLength: uInt,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn deflateCopy(dest: z_streamp, source: z_streamp) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn deflateReset(strm: z_streamp) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn deflateParams(
//     strm: z_streamp,
//     level: ::std::os::raw::c_int,
//     strategy: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn deflateTune(
//     strm: z_streamp,
//     good_length: ::std::os::raw::c_int,
//     max_lazy: ::std::os::raw::c_int,
//     nice_length: ::std::os::raw::c_int,
//     max_chain: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn deflateBound(strm: z_streamp, sourceLen: uLong) -> uLong {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn deflatePrime(
//     strm: z_streamp,
//     bits: ::std::os::raw::c_int,
//     value: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn deflateSetHeader(
//     strm: z_streamp,
//     head: gz_headerp,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn inflateSetDictionary(
//     strm: z_streamp,
//     dictionary: *const Bytef,
//     dictLength: uInt,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn inflateSync(strm: z_streamp) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn inflateCopy(dest: z_streamp, source: z_streamp) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn inflateReset(strm: z_streamp) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn inflateGetHeader(
//     strm: z_streamp,
//     head: gz_headerp,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn inflateBack(
//     strm: z_streamp,
//     in_: in_func,
//     in_desc: *mut ::std::os::raw::c_void,
//     out: out_func,
//     out_desc: *mut ::std::os::raw::c_void,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn inflateBackEnd(strm: z_streamp) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn zlibCompileFlags() -> uLong {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn compress(
//     dest: *mut Bytef,
//     destLen: *mut uLongf,
//     source: *const Bytef,
//     sourceLen: uLong,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn compress2(
//     dest: *mut Bytef,
//     destLen: *mut uLongf,
//     source: *const Bytef,
//     sourceLen: uLong,
//     level: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn compressBound(sourceLen: c_ulong) -> c_ulong {
    panic!();
}

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn uncompress(
//     dest: *mut Bytef,
//     destLen: *mut uLongf,
//     source: *const Bytef,
//     sourceLen: uLong,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

use std::os::raw::{c_ulong, c_void};

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn gzopen(
//     path: *const ::std::os::raw::c_char,
//     mode: *const ::std::os::raw::c_char,
// ) -> gzFile {
//     panic!();
// }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gzopen64(
    path: *const ::std::os::raw::c_char,
    mode: *const ::std::os::raw::c_char,
) -> *mut c_void {
    panic!();
}
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn gzdopen(
//     fd: ::std::os::raw::c_int,
//     mode: *const ::std::os::raw::c_char,
// ) -> gzFile {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn gzsetparams(
//     file: gzFile,
//     level: ::std::os::raw::c_int,
//     strategy: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn gzread(
//     file: gzFile,
//     buf: voidp,
//     len: ::std::os::raw::c_uint,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn gzwrite(
//     file: gzFile,
//     buf: voidpc,
//     len: ::std::os::raw::c_uint,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

// // #[unsafe(no_mangle)]
// // pub unsafe extern "C" fn gzprintf(
// //     file: gzFile,
// //     format: *const ::std::os::raw::c_char,
// //     ...
// // ) -> ::std::os::raw::c_int {
// //     panic!();
// // }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn gzputs(
//     file: gzFile,
//     s: *const ::std::os::raw::c_char,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn gzgets(
//     file: gzFile,
//     buf: *mut ::std::os::raw::c_char,
//     len: ::std::os::raw::c_int,
// ) -> *mut ::std::os::raw::c_char {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn gzputc(file: gzFile, c: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn gzgetc(file: gzFile) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn gzungetc(c: ::std::os::raw::c_int, file: gzFile) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn gzflush(
//     file: gzFile,
//     flush: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn gzseek(
//     file: gzFile,
//     offset: off_t,
//     whence: ::std::os::raw::c_int,
// ) -> off_t {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn gzrewind(file: gzFile) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn gztell(file: gzFile) -> off_t {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn gzeof(file: gzFile) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn gzdirect(file: gzFile) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn gzclose(file: gzFile) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn gzerror(
//     file: gzFile,
//     errnum: *mut ::std::os::raw::c_int,
// ) -> *const ::std::os::raw::c_char {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn gzclearerr(file: gzFile) {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn adler32(adler: uLong, buf: *const Bytef, len: uInt) -> uLong {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn adler32_combine(adler1: uLong, adler2: uLong, len2: off_t) -> uLong {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn crc32_combine(crc1: uLong, crc2: uLong, len2: off_t) -> uLong {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn deflateInit_(
//     strm: z_streamp,
//     level: ::std::os::raw::c_int,
//     version: *const ::std::os::raw::c_char,
//     stream_size: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn inflateInit_(
//     strm: z_streamp,
//     version: *const ::std::os::raw::c_char,
//     stream_size: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn deflateInit2_(
//     strm: z_streamp,
//     level: ::std::os::raw::c_int,
//     method: ::std::os::raw::c_int,
//     windowBits: ::std::os::raw::c_int,
//     memLevel: ::std::os::raw::c_int,
//     strategy: ::std::os::raw::c_int,
//     version: *const ::std::os::raw::c_char,
//     stream_size: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn inflateInit2_(
//     strm: z_streamp,
//     windowBits: ::std::os::raw::c_int,
//     version: *const ::std::os::raw::c_char,
//     stream_size: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn inflateBackInit_(
//     strm: z_streamp,
//     windowBits: ::std::os::raw::c_int,
//     window: *mut ::std::os::raw::c_uchar,
//     version: *const ::std::os::raw::c_char,
//     stream_size: ::std::os::raw::c_int,
// ) -> ::std::os::raw::c_int {
//     panic!();
// }
// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn zError(arg1: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn inflateSyncPoint(z: z_streamp) -> ::std::os::raw::c_int {
//     panic!();
// }

// #[unsafe(no_mangle)]
// pub unsafe extern "C" fn get_crc_table() -> *const uLongf {
//     panic!();
// }
