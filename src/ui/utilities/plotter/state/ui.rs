use super::PlotterState;
use eframe::egui::{Response, Ui, Widget};

#[derive(Debug)]
pub struct PlotterStateEditor<'a>(pub &'a mut PlotterState);

impl<'a> Widget for PlotterStateEditor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label(self.0.to_string())
    }
}
