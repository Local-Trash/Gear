use std::ffi::*;

extern {
    fn run(title: *const c_char);
}

fn main() {
    unsafe {
        let title = CString::new("Window").unwrap().as_c_str().as_ptr();
        run(title);
    }
}