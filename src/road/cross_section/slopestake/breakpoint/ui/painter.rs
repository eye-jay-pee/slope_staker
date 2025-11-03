use super::BreakPoint;
use eframe::egui::{Painter, Rect, Stroke, Vec2};

pub trait CanPaintBreakPoint {
    fn break_point(
        &self,
        breakpoint: &BreakPoint,
        rect: Rect,
        stroke: Stroke,
        run: f32,
    );
}

impl CanPaintBreakPoint for Painter {
    fn break_point(
        &self,
        breakpoint: &BreakPoint,
        rect: Rect,
        stroke: Stroke,
        run: f32,
    ) {
        let slope_vec = Vec2::from(breakpoint.slope);

        let a = rect.center() + Vec2::from(breakpoint);
        let b = a + slope_vec * run;

        self.line_segment([a, b], stroke);
        self.circle_filled(a, stroke.width, stroke.color);
    }
}
