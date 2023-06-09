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

    pub fn run(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        if width < 0 || height < 0 {
            println!("Viewport height can't be less than 0. Exit Code: 3");
            panic!();
        }

        self.0(x, y, width, height);
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
            println!("Clear Color parameters were not under 1: Exit Code 4");
            panic!()
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

pub struct GenBuffers(extern "system" fn(GLsizei, *mut GLuint));

impl GenBuffers {
    pub fn new() -> Self {
        let procname = CString::new("glGenBuffers").unwrap();

        let ptr = unsafe { 
            transmute::<
                *const c_void, 
                extern "system" fn(
                    GLsizei,
                    *mut GLuint
                ) -> ()
            >
            (
                glfwGetProcAddress(procname.as_ptr())
            )
        };

        GenBuffers(ptr)
    }

    pub fn run(&self, n: GLsizei, buffers: *mut GLuint) {
        if n < 0 {
            println!("GenBuffer can have a negative n: Exit Code: 3");
            panic!()
        }

        self.0(n, buffers);
    }
}

pub struct BindBuffer(extern "system" fn(GLenum, GLuint));

impl BindBuffer {
    pub fn new() -> Self {
        let procname = CString::new("glBindBuffer").unwrap();

        let ptr = unsafe { 
            transmute::<
                *const c_void, 
                extern "system" fn(
                    GLenum,
                    GLuint
                ) -> ()
            >
            (
                glfwGetProcAddress(procname.as_ptr())
            )
        };

        Self(ptr)
    }

    pub fn run(&self, target: GLenum, buffer: GLuint) {
        self.0(target, buffer);
    }
}

pub struct BufferData(extern "system" fn(GLenum, GLsizeiptr, *const c_void, GLbitfield));

impl BufferData {
    pub fn new() -> Self {
        let procname = CString::new("glBindBuffer").unwrap();

        let ptr = unsafe { 
            transmute::<
                *const c_void, 
                extern "system" fn(
                    GLenum,
                    GLsizeiptr,
                    *const c_void, 
                    GLbitfield
                ) -> ()
            >
            (
                glfwGetProcAddress(procname.as_ptr())
            )
        };

        BufferData(ptr)
    }

    pub fn run(&self, target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLbitfield) {
        self.0(target, size, data, usage);
    }
}

pub struct CreateShader(extern "system" fn(GLenum) -> GLuint);

impl CreateShader {
    pub fn new() -> Self {
        let procname = CString::new("glBindBuffer").unwrap();

        let ptr = unsafe { 
            transmute::<
                *const c_void, 
                extern "system" fn(
                    GLenum
                ) -> GLuint
            >
            (
                glfwGetProcAddress(procname.as_ptr())
            )
        };

        CreateShader(ptr)
    }

    pub fn run(&self, shadertype: GLenum) -> GLuint {
        self.0(shadertype)
    }
}

pub struct ShaderSource(extern "system" fn(GLuint, GLsizei, *mut GLchar, *mut GLint));

impl ShaderSource {
    pub fn new() -> Self {
        let procname = CString::new("glShaderSource").unwrap();

        let ptr = unsafe { 
            transmute::<
                *const c_void, 
                extern "system" fn(
                    GLuint,
                    GLsizei,
                    *mut GLchar,
                    *mut GLint
                ) -> ()
            >
            (
                glfwGetProcAddress(procname.as_ptr())
            )
        };

        Self(ptr)
    }

    pub fn run(&self, shader: GLuint, count: GLsizei, string: *mut GLchar, length: *mut GLint) {
        if count < 0 {
            println!("ShaderSource count can't be negative. Exit Code: 3");
            panic!()
        }

        self.0(shader, count, string, length);
    }
}

pub struct CompileShader(extern "system" fn(GLuint));

impl CompileShader {
    pub fn new() -> Self {
        let procname = CString::new("glCompileShader").unwrap();

        let ptr = unsafe { 
            transmute::<
                *const c_void, 
                extern "system" fn(
                    GLuint
                ) -> ()
            >
            (
                glfwGetProcAddress(procname.as_ptr())
            )
        };

        Self(ptr)
    }

    pub fn run(&self, shader: GLuint) {
        self.0(shader);
    }
}