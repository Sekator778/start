extern crate winapi;

use std::thread;
use std::time::Duration;
use winapi::um::winuser::{GetCursorPos, SetCursorPos, SendMessageW, BM_CLICK};
use winapi::shared::windef::POINT;
use winapi::shared::windef::HWND;

#[derive(Default)]
pub struct MouseHandler;

impl MouseHandler {
    pub fn start(&self, exit_button: HWND) {
        // Переміщення курсора до кнопки "Exit" через 5 секунд
        thread::sleep(Duration::from_secs(5));

        // Отримання позиції кнопки "Exit"
        let mut pos = POINT { x: 0, y: 0 };
        unsafe {
            // Переміщення курсора до кнопки "Exit"
            GetCursorPos(&mut pos);
            SetCursorPos(pos.x, pos.y);

            // Натискання кнопки "Exit"
            SendMessageW(exit_button, BM_CLICK, 0, 0);
        }
    }
}
