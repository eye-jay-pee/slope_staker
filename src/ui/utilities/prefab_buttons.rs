use eframe::egui::{Button, Color32, Response, RichText, Ui, Widget};

#[derive(Debug)]
pub enum MiniButton {
    New,
    Kill,
    Incr,
    Decr,
    Prev,
    Next,
}
impl Widget for MiniButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let text_color = Color32::LIGHT_GRAY;
        let (color, rune) = match self {
            Self::New => (Color32::DARK_GREEN, "+"),
            Self::Kill => (Color32::DARK_RED, "x"),
            Self::Incr => (Color32::GREEN, "++"),
            Self::Decr => (Color32::RED, "--"),
            Self::Prev => (Color32::DARK_BLUE, "<"),
            Self::Next => (Color32::DARK_BLUE, ">"),
        };
        ui.add(Button::new(RichText::new(rune).color(text_color)).fill(color))
    }
}
