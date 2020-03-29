#![feature(rustc_private)]

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[allow(non_camel_case_types)]
mod libc {
    pub type uintptr_t = usize;
    pub type intptr_t = isize;
    pub type c_ulonglong = u64;
    pub type c_ulong = u32;
    pub type c_longlong = i64;
    pub type c_long = i32;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub type c_void = std::ffi::c_void;
    pub type c_double = f64;
    pub type c_float = f32;
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
extern crate libc;

pub use libc::*;
