#![allow(non_snake_case)]
use core::panic;
use std::{ffi::*, ptr::{null_mut, null}};

mod types;
mod functions;
mod shader;
use functions::*;
use types::*;

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

#[allow(non_upper_case_globals)]
static mut glfunctions: Option<GLFunctions> = None;

#[no_mangle]
extern "C" fn run(
    title: *const c_char
) {
    unsafe {
        if glfwInit() == 0 {
            panic!("Fail to initialize the library. Exit Code: 0");
        };

        let window = glfwCreateWindow(500, 500, title, null_mut(), null_mut());

        println!("{:?}", &CStr::from_ptr(title).to_str().unwrap());

        if window.is_null() {
            println!("Failed to create window. Error Code: 2");
            glfwTerminate();
            return;
        }

        glfwMakeContextCurrent(window);

        glfunctions = Some(GLFunctions::new());
        let vertexShadersource = CString::new(shader::vertex).unwrap();

        glfwSetFramebufferSizeCallback(window, frameBufferSizeCallBack);

        let verteices: [f32; 9] = [
            -0.5, -0.5, 0.,
            0.5, 0.5, 0.,
            0., 0.5, 0.
        ];

        while glfwWindowShouldClose(window) == 0 {

            if glfwGetKey(window, GLFWKeyEscape) == GLFWPress {
                glfwSetWindowShouldClose(window, True);
            }

            glfunctions.as_ref().unwrap().clearcolor.run(0.2, 0.3, 0.3, 1.0);

            let mut VBO: GLuint = 0;
            glfunctions.as_ref().unwrap().genbuffers.run(1, VBO as *mut GLuint);
            glfunctions.as_ref().unwrap().bindbuffers.run(glArrayBuffer, VBO);
            glfunctions.as_ref().unwrap().bufferdata.run(
                glArrayBuffer, 
                verteices.len() as GLsizeiptr, &verteices[0] as *const f32 as *const c_void, 
                glStaticDraw
            );
            let vertexShader = glfunctions.as_ref().unwrap().createShader.run(glVertexShader);
            glfunctions.as_ref().unwrap().shaderSource.run(vertexShader, 1, vertexShadersource.as_ptr() as *mut GLchar, null::<GLint>() as *mut _);

            glfunctions.as_ref().unwrap().clear.run(glColorBufferBit);

            glfwSwapBuffers(window);
            glfwPollEvents(); 
        }

        println!("Library closed successfully. Exit Code: 1");
        glfwTerminate();
    }
}

extern "C" fn frameBufferSizeCallBack(window: *mut GLFWwindow, width: GLint, height: GLint) {
    unsafe {
        glfunctions.as_ref().unwrap().viewport.run(0, 0, width, height);
    }
}

struct GLFunctions {
    viewport: Viewport,
    clearcolor: ClearColor,
    clear: Clear,
    genbuffers: GenBuffers,
    bindbuffers: BindBuffer,
    bufferdata: BufferData,
    createShader: CreateShader,
    shaderSource: ShaderSource,
}

impl GLFunctions {
    fn new() -> Self {
        GLFunctions { 
            viewport: Viewport::new(), 
            clearcolor: ClearColor::new(),
            clear: Clear::new(),
            genbuffers: GenBuffers::new(),
            bindbuffers: BindBuffer::new(),
            bufferdata: BufferData::new(),
            createShader: CreateShader::new(),
            shaderSource: ShaderSource::new()
        }
    }
}