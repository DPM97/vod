use image::ImageFormat;
use std::{
    thread,
    time::{self},
};

use screenshot::{get_last_capture, start_capture};

fn main() {
    thread::spawn(move || loop {
        let cap = get_last_capture();
        println!(
            "{}",
            if cap.is_none() {
                "no frame captured"
            } else {
                "frame captured"
            }
        );
        match cap {
            Some(x) => {
                // println!("{:?}", x);
                // println!("{}", str::from_utf8(&x).unwrap());

                match image::load_from_memory_with_format(&x, ImageFormat::Jpeg) {
                    Ok(_img) => {
                        println!("input in jpg");
                        std::fs::write("output.jpeg", x).unwrap();
                    }
                    Err(_) => {
                        println!("input is not jgp");
                    }
                }
            }
            None => {}
        }
        thread::sleep(time::Duration::from_secs(1));
    });
    start_capture();
}
