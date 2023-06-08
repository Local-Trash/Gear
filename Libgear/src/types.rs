#![allow(non_upper_case_globals)]
use std::ffi::*;

pub const GLFWRelease: i32 = 0i32;
pub const GLFWPress: i32 = 1i32;
pub const GLFWRepeat: i32 = 2i32;

pub const GLFWKeyEscape: i32 = 256;

pub const True: i32 = 0;
pub const False: i32 = 1;

pub const glColorBufferBit: u32 = 16384;

pub type GLint = c_int;
pub type GLsizei = c_int;
pub type GLcampf = f32;
pub type GLbitfield = c_uint;
pub type GLFWframebuffersizefun = extern "C" fn(*mut GLFWwindow, GLint, GLint);
pub type GLFWglproc = *const c_void;

#[allow(missing_copy_implementations)]
#[derive(Debug)]
pub enum GLFWwindow {}

#[allow(missing_copy_implementations)]
#[derive(Debug)]
pub enum GLFWmonitor {}