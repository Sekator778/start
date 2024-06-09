#![windows_subsystem = "windows"]

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwg::NativeUi;

#[derive(Default, nwd::NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (300, 115), position: (300, 300), title: "Hello World")]
    #[nwg_events( OnWindowClose: [BasicApp::exit] )]
    window: nwg::Window,

    #[nwg_control(text: "Hello, World!", size: (280, 50), position: (10, 10))]
    label: nwg::Label,

    #[nwg_control(text: "Закрити", size: (280, 40), position: (10, 60))]
    #[nwg_events( OnButtonClick: [BasicApp::exit] )]
    close_button: nwg::Button,
}

impl BasicApp {
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
