use super::GridLinesState;
use eframe::egui::{Response, Ui, Widget};

#[derive(Debug)]
pub struct GridLinesStateEditor<'a>(pub &'a mut GridLinesState);

impl<'a> Widget for GridLinesStateEditor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label(self.0.to_string())
    }
}
