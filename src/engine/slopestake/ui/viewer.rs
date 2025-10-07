use super::SlopeStake;
use crate::engine::slopestake::CanPaintBreakPoint as _;
use crate::foreign::PainterExt;
use eframe::egui::{Color32, Response, Sense, Stroke, Ui, Vec2, Widget};

pub struct SlopeStakeViewer<'a>(&'a SlopeStake);
impl<'a> SlopeStakeViewer<'a> {
    pub fn new(ss: &'a SlopeStake) -> Self {
        Self(ss)
    }
}

impl<'a> Widget for SlopeStakeViewer<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(300.0, 300.0), Sense::empty());
        let rect = response.rect;

        let stroke = Stroke::new(2.0, Color32::WHITE);
        let gridlines_stroke = Stroke::new(0.4, Color32::WHITE);
        let gridlines_freq = Vec2::new(10.0, 10.0);

        painter.rect_filled(rect, 0.0, Color32::BLACK);
        painter.grid_lines(rect, gridlines_stroke, gridlines_freq);

        for pt in &self.0.pts {
            painter.break_point(&pt, rect, stroke, 30.3);
        }

        response
    }
}
