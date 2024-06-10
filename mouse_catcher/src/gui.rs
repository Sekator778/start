extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use std::sync::{Arc, Mutex};
use std::thread;
use nwg::NativeUi;
use crate::mouse_handler::MouseHandler;

#[derive(Default, nwd::NwgUi)]
pub struct App {
    #[nwg_control(size: (300, 150), position: (300, 300), title: "Mouse Catcher")]
    #[nwg_events( OnWindowClose: [App::exit] )]
    window: nwg::Window,

    #[nwg_control(text: "Catch", size: (280, 50), position: (10, 10))]
    #[nwg_events( OnButtonClick: [App::catch_mouse] )]
    catch_button: nwg::Button,

    #[nwg_control(text: "Exit", size: (280, 50), position: (10, 70))]
    #[nwg_events( OnButtonClick: [App::exit] )]
    exit_button: nwg::Button,

    #[nwg_control(focus: true)]
    handler: Arc<Mutex<MouseHandler>>,
}

impl App {
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn catch_mouse(&self) {
        let exit_button = self.exit_button.handle.hwnd().unwrap();
        let handler = Arc::clone(&self.handler);
        thread::spawn(move || {
            handler.lock().unwrap().start(exit_button);
        });
    }
}
