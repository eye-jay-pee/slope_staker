use eframe::egui::{self, Button, Color32};
use eframe::egui::{Response, Ui, Widget};

#[derive(Debug, Default)]
pub struct PlusButton();
impl Widget for PlusButton {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add(
            Button::new(egui::RichText::new("+").color(Color32::BLACK))
                .fill(Color32::GREEN),
        )
    }
}
#[derive(Debug, Default)]
pub struct XButton();
impl Widget for XButton {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add(
            Button::new(egui::RichText::new("X").color(Color32::WHITE))
                .fill(Color32::RED),
        )
    }
}
