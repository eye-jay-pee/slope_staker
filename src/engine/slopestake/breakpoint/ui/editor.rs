pub use super::{BreakPoint, BreakPointKind};
pub use super::{
    BreakPointKindSelector, ElevationEditor, OffsetEditor, SlopeEditor,
};
use eframe::egui::{Response, Ui, Widget};

pub struct BreakPointEditor<'a>(&'a mut BreakPoint);
impl<'a> BreakPointEditor<'a> {
    pub fn new(bp: &'a mut BreakPoint) -> Self {
        Self(bp)
    }
}
impl<'a> Widget for BreakPointEditor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let is_crown = self.0.kind == BreakPointKind::Crown;
        let is_limit = self.0.kind == BreakPointKind::Limit;

        ui.horizontal_wrapped(|ui| {
            ui.add_enabled(
                !is_crown && !is_limit,
                BreakPointKindSelector::new(&mut self.0.kind),
            );
            ui.add(ElevationEditor::new(&mut self.0.elev));
            ui.add(OffsetEditor::new(&mut self.0.offset));
            ui.add(SlopeEditor::new(&mut self.0.slope));
        })
        .response
    }
}
