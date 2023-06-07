#![allow(non_snake_case)]
use core::panic;
use std::{ffi::*, ptr::null_mut};

mod cosnts;
use cosnts::*;

extern "C" {
    fn glfwInit() -> c_int;

    pub fn glfwCreateWindow(
        width: c_int,
        height: c_int,
        title: *const c_char,
        monitor: *mut GLFWmonitor,
        share: *mut GLFWwindow,
    ) -> *mut GLFWwindow;

    fn glfwWindowShouldClose(window: *mut GLFWwindow) -> c_int;

    fn glfwMakeContextCurrent(window: *mut GLFWwindow);
    
    fn glfwSwapInterval(interval: c_int);

    fn glfwTerminate();

    fn glfwSetFramebufferSizeCallback(
        window: *mut GLFWwindow,
        cbfun: GLFWframebuffersizefun,
    );

    fn glfwSwapBuffers(window: *mut GLFWwindow);

    fn glfwPollEvents();

    fn glfwGetKey(window: *mut GLFWwindow, key: c_int) -> c_int;

    fn glfwSetWindowShouldClose(window: *mut GLFWwindow, value: c_int);

    fn glfwGetProcAddress(procname: *const c_char) -> GLFWglproc;
}

#[no_mangle]
extern "C" fn run(
    title: *const c_char
) {
    unsafe {
        if glfwInit() == 0 {
            panic!("Fail to initialize the library. Exit Code: 0");
        };

        let window = glfwCreateWindow(500, 500, title, null_mut(), null_mut());
        gl::load_with(|s| window.get_proc_address(s) as *const _);

        if window.is_null() {
            println!("Failed to create window. Error Code: 2");
            glfwTerminate();
            return;
        }

        //GLViewport(100, 100, 500, 500);

        glfwSetFramebufferSizeCallback(window, frameBufferSizeCallBack);

        while glfwWindowShouldClose(window) == 0 {

            if glfwGetKey(window, GLFWKeyEscape) == GLFWPress {
                glfwSetWindowShouldClose(window, True);
            }

            //glClearColor(0.2, 0.3, 0.3, 1.0);

            //glClear(glColorBufferBit);

            glfwSwapBuffers(window);
            glfwPollEvents(); 
        }

        println!("Library closed successfully. Exit Code: 1");
        glfwTerminate();
    }
}

extern "C" fn frameBufferSizeCallBack(window: *mut GLFWwindow, width: GLint, height: GLint) {
    unsafe { //glViewport(0, 0, width, height) };
}

#[allow(missing_copy_implementations)]
#[derive(Debug)]
pub enum GLFWwindow {}

#[allow(missing_copy_implementations)]
#[derive(Debug)]
pub enum GLFWmonitor {}

type GLint = c_int;
type GLcampf = f32;
type GLbitfield = c_uint;
type GLFWframebuffersizefun = extern "C" fn(*mut GLFWwindow, GLint, GLint);
//type GLFWglproc = *const c_void;

mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}