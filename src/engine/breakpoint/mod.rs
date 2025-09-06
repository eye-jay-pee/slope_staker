pub mod kind;
pub mod offset;
pub mod slope;

use super::elevation::Elevation;
use offset::Offset;
use slope::Slope;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BreakPoint {
    kind: kind::BreakPointKind,
    _cut_or_fill_from_slope_stake: Option<f32>,

    slope: Slope,
    elev: Elevation,
    offset: Offset,
}
impl BreakPoint {}

impl std::fmt::Display for BreakPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} @ {} L of CL @ {}. Then {} slope until...",
            self.kind, self.offset, self.elev, self.slope
        )
    }
}

pub mod ui {
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
            use super::super::elevation::ui::ElevationEditor;
            use super::kind::ui::BreakPointKindSelector;
            use super::offset::ui::OffsetEditor;
            use super::slope::ui::SlopeEditor;

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add(BreakPointKindSelector::new(&mut self.0.kind));
                    ui.add(SlopeEditor::new(&mut self.0.slope));
                });
                ui.horizontal(|ui| {
                    ui.add(OffsetEditor::new(&mut self.0.offset));
                    ui.add(ElevationEditor::new(&mut self.0.elev));
                });
            })
            .response
        }
    }
}
