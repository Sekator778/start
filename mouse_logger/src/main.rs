extern crate winapi;

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use winapi::shared::windef::POINT;
use winapi::um::winuser::GetCursorPos;

fn log_cursor_position() {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("mouse_log.txt")
        .unwrap();

    let mut pos = POINT { x: 0, y: 0 };

    unsafe {
        GetCursorPos(&mut pos);
    }

    let log_entry = format!("Cursor position: ({}, {})\n", pos.x, pos.y);
    file.write_all(log_entry.as_bytes()).unwrap();
}

fn main() {
    loop {
        log_cursor_position();
        thread::sleep(Duration::from_secs(1));
    }
}
