use crate::person::*;
use eframe::egui;

#[derive(Default)]
pub struct MyEguiApp {}

impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                let person_bio: Bio = build_bio("Jane Doe".to_string(), "Human".to_string(), 25);

                let person_stats: Stats = build_stats(8, 16);

                let person: Person = build_person("Paladin".to_string(), person_bio, person_stats);

                ui.heading(person.show_stats());
            });
        });
    }
}
