use std::{mem::transmute, ffi::{CString, c_void}};

use crate::types::*;
use super::glfwGetProcAddress;

pub struct Viewport(extern "system" fn(GLint, GLint, GLsizei, GLsizei));

impl Viewport {
    pub fn new() -> Viewport {
        let procname = CString::new("glViewport").unwrap();
        let ptr = unsafe { 
            transmute::<
                *const c_void, 
                extern "system" fn(
                    GLint, 
                    GLint, 
                    GLsizei, 
                    GLsizei
                ) -> ()>(
                    glfwGetProcAddress(procname.as_ptr())
                )
        };

        Viewport(ptr)
    }

    pub fn run(&self, x: GLint, y: GLint, width: GLsizei, Height: GLsizei) {
        self.0(x, y, width, Height);
    }
}

pub struct ClearColor(extern "system" fn(GLcampf, GLcampf, GLcampf, GLcampf));

impl ClearColor {
    pub fn new() -> ClearColor {
        let procname = CString::new("glClearColor").unwrap();

        let ptr = unsafe { 
            transmute::<
                *const c_void, 
                extern "system" fn(
                    GLcampf, 
                    GLcampf, 
                    GLcampf, 
                    GLcampf
                ) -> ()
            >
            (
                glfwGetProcAddress(procname.as_ptr())
            )
        };

        ClearColor(ptr)
    }

    pub fn run(&self, red: GLcampf, green: GLcampf, blue: GLcampf, alpha: GLcampf) {
        if red > 1. || green > 1. || blue > 1. || alpha > 1. {
            println!("Clear Color parameters were not under 1")
        }
        self.0(red, green, blue, alpha);
    }
}

pub struct Clear(extern "system" fn(GLbitfield));

impl Clear {
    pub fn new() -> Self {
        let procname = CString::new("glClear").unwrap();

        let ptr = unsafe { 
            transmute::<
                *const c_void, 
                extern "system" fn(
                    GLbitfield
                ) -> ()
            >
            (
                glfwGetProcAddress(procname.as_ptr())
            )
        };

        Self(ptr)
    }

    pub fn run(&self, mask: GLbitfield) {
        self.0(mask);
    }
}