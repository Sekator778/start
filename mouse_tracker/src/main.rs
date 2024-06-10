#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;
extern crate winapi;

use nwg::NativeUi;
use std::time::Duration;
use winapi::shared::windef::POINT;
use winapi::um::winuser::GetCursorPos;

#[derive(Default, nwd::NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (300, 150), position: (300, 300), title: "Mouse Position")]
    #[nwg_events( OnWindowClose: [BasicApp::exit], OnInit: [BasicApp::setup_timer] )]
    window: nwg::Window,

    #[nwg_control(text: "Cursor position: (0, 0)", size: (280, 100), position: (10, 10))]
    position_label: nwg::Label,

    #[nwg_control(interval: Duration::from_secs(1))]
    #[nwg_events( OnTimerTick: [BasicApp::update_position_label] )]
    timer: nwg::AnimationTimer,
}

impl BasicApp {
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn setup_timer(&self) {
        self.timer.start();
    }

    fn update_position_label(&self) {
        let mut pos = POINT { x: 0, y: 0 };

        unsafe {
            GetCursorPos(&mut pos);
        }

        let position_text = format!("Cursor position: ({}, {})", pos.x, pos.y);
        self.position_label.set_text(&position_text);
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
