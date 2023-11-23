mod gui;
use crate::gui::MyEguiApp;
mod person;
use crate::person::*;

#[warn(unused_must_use)]
fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("Character System App", 
    native_options, 
    Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}