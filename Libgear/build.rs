use std::{fs::File, path::Path};

use gl_generator::*;

#[allow(non_snake_case)]
fn main() {
    let outDir = std::env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&outDir).join("bindings.rs")).unwrap();

    println!("cargo:rustc-link-search=native={}", outDir);
    println!("cargo:rustc-link-lib=dylib=glfw3");

    Registry::new(Api::Gl, (4, 6), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();
}