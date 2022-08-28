extern crate cc;

fn main() {
  cc::Build::new()
    .file("objc/cam.m")
    .flag("-fmodules")
    .flag("-Wno-deprecated-declarations")
    // `cc` doesn't try to pick up on this automatically, but `clang` needs it to
    // generate a "correct" Objective-C symbol table which better matches XCode.
    // See https://github.com/h4llow3En/mac-notification-sys/issues/45.
    .compile("cam");
}
