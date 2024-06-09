#![windows_subsystem = "windows"]

extern crate winapi;

use std::thread;
use std::time::Duration;
use winapi::shared::windef::POINT;
use winapi::um::winuser::{GetCursorPos, SetCursorPos};

fn main() {
    let mut prev_pos = POINT { x: 0, y: 0 };

    unsafe {
        // Отримання початкової позиції курсора
        GetCursorPos(&mut prev_pos);

        loop {
            let mut current_pos = POINT { x: 0, y: 0 };
            // Отримання поточної позиції курсора
            GetCursorPos(&mut current_pos);

            // Обчислення зміни позиції
            let dx = current_pos.x - prev_pos.x;
            let dy = current_pos.y - prev_pos.y;

            if dx != 0 || dy != 0 {
                // Встановлення нової позиції курсора з протилежними змінами
                SetCursorPos(prev_pos.x - dx, prev_pos.y - dy);
                // Оновлення попередньої позиції
                prev_pos = current_pos;
            }

            // Затримка для зменшення навантаження на CPU
            thread::sleep(Duration::from_millis(10));
        }
    }
}
