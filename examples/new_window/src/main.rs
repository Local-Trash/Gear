use std::ffi::*;

extern {
    fn run(title: *const c_char);
}

fn main() {
    unsafe {
        let title = CString::new("Window").unwrap();
        println!("{:#?} {:#?}", title, title.as_ptr());
        run(title.as_ptr());
    }
}