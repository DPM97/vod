extern crate cc;

fn main() {
    cc::Build::new()
        .file("objc/cam.m")
        .flag("-fmodules")
        .flag("-Wno-deprecated-declarations")
        .compile("cam");
}
