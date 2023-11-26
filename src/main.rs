mod person;
mod gui;
use crate::person::*;
use crate::gui::MyEguiApp;

#[warn(unused_must_use)]
fn main() {
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
        .with_drag_and_drop(true),
        centered: true,
        ..Default::default()
    };
    
    let _ = eframe::run_native("Character System App", 
    native_options, 
    Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}