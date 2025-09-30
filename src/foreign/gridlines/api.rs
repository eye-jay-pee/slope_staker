use super::GridLinesStyle;
use eframe::egui::{Painter, Rect, Stroke, Vec2};

pub trait PainterExt {
    fn grid_lines(&self, rect: Rect, stroke: Stroke, freq: Vec2);
}
impl PainterExt for Painter {
    fn grid_lines(&self, rect: Rect, stroke: Stroke, freq: Vec2) {
        GridLinesStyle::default()
            .stroke(stroke)
            .freq(freq)
            .draw(self, rect);
    }
}
