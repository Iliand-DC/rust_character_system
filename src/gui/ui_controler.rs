use eframe::egui::Label;
pub use crate::person::*;

pub fn show_characters_list(ui: &mut eframe::egui::Ui) {
    let person = build_person("Паладин".to_string(),
    build_bio("Jane Doe".to_string(), "Человек".to_string(), 24),
    build_stats(8, 16));

    let ork = build_person("Варвар".to_string(),
    build_bio("Брут".to_string(), "Орк".to_string(), 52),
    build_stats(12, 24));

    ui.add(Label::new(person.show_stats()));
    ui.add(Label::new(ork.show_stats()));
}