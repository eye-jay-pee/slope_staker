use eframe::egui::{Button, Color32, Response, RichText, Ui, Widget};

#[derive(Debug, Default)]
pub struct PlusButton();
impl Widget for PlusButton {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add(
            Button::new(RichText::new("+").color(Color32::LIGHT_GRAY))
                .fill(Color32::DARK_GREEN),
        )
    }
}
#[derive(Debug, Default)]
pub struct XButton();
impl Widget for XButton {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add(
            Button::new(RichText::new("x").color(Color32::LIGHT_GRAY))
                .fill(Color32::DARK_RED),
        )
    }
}
