#![windows_subsystem = "windows"]

mod gui;
mod mouse_handler;

extern crate native_windows_gui as nwg;

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let _app = gui::App::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
