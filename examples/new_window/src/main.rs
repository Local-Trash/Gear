use std::ffi::*;

extern {
    fn run(
        name: *const c_char,
        size: *const [c_int;2],
        upFunc: extern fn()
    );

    fn draw();
}

fn main() {
    unsafe {
        draw();
        let title = CString::new("Game").unwrap();
        run(
            title.as_ptr() as *const c_char,
            &[5i32, 5i32] as *const [c_int; 2],
            update as extern fn()
        );
    }
}

extern fn update() {
    unsafe {
        draw();
    }
}