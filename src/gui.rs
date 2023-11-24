use eframe::egui::{self, CollapsingHeader};
mod style;
use crate::person::*;

use self::style::set_style;

#[derive(Default)]
pub struct MyEguiApp {}

impl MyEguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        set_style(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {

                let person = build_person("Паладин".to_string(),
                build_bio("Jane Doe".to_string(), "Человек".to_string(), 24),
                build_stats(8, 16));

                let ork = build_person("Варвар".to_string(),
                build_bio("Брут".to_string(), "Орк".to_string(), 52),
                build_stats(12, 24));

                CollapsingHeader::new("Список персонажей")
                .default_open(false)
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(person.show_stats());
                        ui.label(ork.show_stats());
                    });
                });
            });
        });
    }
}
