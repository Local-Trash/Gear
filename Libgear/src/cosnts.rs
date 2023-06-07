#![allow(non_upper_case_globals)]
use std::ffi::*;
use super::glfwGetProcAddress;

pub const GLFWRelease: i32 = 0i32;
pub const GLFWPress: i32 = 1i32;
pub const GLFWRepeat: i32 = 2i32;

pub const GLFWKeyEscape: i32 = 256;

pub const True: i32 = 0;
pub const False: i32 = 1;

pub const glColorBufferBit: u32 = 16384;

pub const GLViewport: *const c_void = loadfn("glViewport");

const fn loadfn(procname: &str) -> *const c_void {
    unsafe { glfwGetProcAddress(procname.as_ptr() as *const i8) }
}