use crate::utilities::DisplayExt;
use eframe::egui::{Response, Stroke, Ui, Widget};

pub struct StrokeEditor<'a>(&'a mut Stroke);
impl<'a> Widget for StrokeEditor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label("im out here stroking my shit ")
    }
}

impl DisplayExt for Stroke {
    fn fmt_ext(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "width:{} color:{}",
            self.width,
            self.color.to_display_string()
        )
    }
}
