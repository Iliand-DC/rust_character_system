use eframe::egui::{self};
mod style;
mod ui_controler;
use self::{style::set_style, ui_controler::show_characters_list};

#[derive(Default)]
pub struct MyEguiApp {
    show_all_characters: bool,
    get_access_to_character: bool,
}

impl MyEguiApp {
    pub fn new(&mut self, _cc: &eframe::CreationContext<'_>) -> Self {
        self.show_all_characters = false;
        self.get_access_to_character = false;
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        set_style(ctx);

        egui::CentralPanel::default().show(&ctx, |ui| {
            if self.show_all_characters == true {
                ui.collapsing("Показать всех", |ui| {
                    show_characters_list(ui);
                });
            }
            // Сделать несколько страниц для показа, в зависимости от нажатой кнопки
        });
    }
}
