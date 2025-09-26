use super::GridLinesStyle;
use eframe::egui::{Painter, Rect};
pub trait PainterCanDrawGridLines {
    fn grid_lines(&self, rect: Rect, style: GridLinesStyle);
}

impl PainterCanDrawGridLines for Painter {
    fn grid_lines(&self, rect: Rect, style: GridLinesStyle) {
        style.hori_lines(self, rect);
        style.vert_lines(self, rect);
    }
}
