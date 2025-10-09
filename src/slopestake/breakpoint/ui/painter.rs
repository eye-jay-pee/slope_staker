use super::{BreakPoint, BreakPointKind};
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

        if breakpoint.kind != BreakPointKind::Limit {
            self.line_segment([a, b], stroke); //impl this as slope/ui/painter
        }
        self.circle_filled(a, stroke.width, stroke.color);
    }
}
