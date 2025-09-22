use crate::utilities::DisplayExt;
use eframe::egui::{Stroke, Vec2};

#[derive(Debug, Default, Clone)]
pub struct GridLinesState {
    pub stroke: Stroke,
    pub freq: Vec2,
}
impl std::fmt::Display for GridLinesState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "gridlines: {}", self.stroke.to_display_string())?;
        writeln!(f, "vertical: {}", self.freq.x)?;
        writeln!(f, "horiziontal: {}", self.freq.y)
    }
}
pub mod ui {
    use super::GridLinesState;
    use eframe::egui::{Response, Ui, Widget};

    #[derive(Debug)]
    pub struct GridLinesStateEditor<'a>(pub &'a mut GridLinesState);

    impl<'a> Widget for GridLinesStateEditor<'a> {
        fn ui(self, ui: &mut Ui) -> Response {
            ui.label(self.0.to_string())
        }
    }
}
