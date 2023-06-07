#[allow(non_snake_case)]
fn main() {
    let outDir = std::env::var("OUT_DIR").unwrap();

    println!("cargo:rustc-link-search=native={}", outDir);
    println!("cargo:rustc-link-lib=dylib=glfw3");
}