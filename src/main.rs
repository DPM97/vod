use std::thread;

use screenshot::{start_capture, UI};

fn main() {
    thread::spawn(start_capture);
    start_gui();
}

fn start_gui() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("vod", native_options, Box::new(|cc| Box::new(UI::new(cc))));
}
