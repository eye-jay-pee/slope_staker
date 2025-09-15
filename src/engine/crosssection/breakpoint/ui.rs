use super::*;
use eframe::egui::{Response, Ui, Widget};

pub struct BreakPointEditor<'a>(&'a mut BreakPoint);
impl<'a> BreakPointEditor<'a> {
    pub fn new(bp: &'a mut BreakPoint) -> Self {
        Self(bp)
    }
}
impl<'a> Widget for BreakPointEditor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal_wrapped(|ui| {
            ui.add(BreakPointKindSelector::new(&mut self.0.kind));
            ui.add(ElevationEditor::new(&mut self.0.elev));
            ui.add(OffsetEditor::new(&mut self.0.offset));
            ui.add(SlopeEditor::new(&mut self.0.slope));
        })
        .response
    }
}
