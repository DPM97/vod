#![cfg(target_os = "macos")]
#![allow(improper_ctypes)]

use objc_foundation::INSData;

mod app;
pub use app::UI;

mod sys {
    use objc_foundation::NSData;
    #[link(name = "cam")]
    extern "C" {
        pub fn start_capture_loop();
        pub fn get_last_capture() -> *const NSData;
    }
}

pub fn start_capture() {
    unsafe { sys::start_capture_loop() }
}

pub fn get_last_capture() -> Option<Vec<u8>> {
    match unsafe { sys::get_last_capture().as_ref() } {
        Some(data) => Some(Vec::from(data.bytes())),
        None => None,
    }
}
