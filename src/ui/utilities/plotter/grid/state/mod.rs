pub mod ui;
//pub use ui::GridLinesStateEditor;

use crate::utilities::DisplayExt;
use eframe::egui::{Stroke, Vec2};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Default, Copy, Clone)]
pub struct GridLinesState {
    pub stroke: Stroke,
    pub freq: Vec2,
}
impl Display for GridLinesState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "gridlines: {}", self.stroke.to_display_string())?;
        writeln!(f, "vertical: {}", self.freq.x)?;
        writeln!(f, "horiziontal: {}", self.freq.y)
    }
}
