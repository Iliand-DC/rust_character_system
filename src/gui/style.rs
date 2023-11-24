use eframe::egui;
use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;
use egui::Context;

pub fn set_style(ctx: &Context) {
    // Get current context style
    let mut style = (*ctx.style()).clone();

    // Redefine text_styles
    style.text_styles = [
    (Heading, FontId::new(25.0, Proportional)),
    (Body, FontId::new(20.0, Proportional)),
    (Monospace, FontId::new(18.0, Proportional)),
    (Button, FontId::new(18.0, Proportional)),
    (Small, FontId::new(14.0, Proportional)),
    ].into();

    // Mutate global style with above changes
    ctx.set_style(style);
}