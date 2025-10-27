use super::SlopeStake;
use crate::foreign::PainterExt;
use crate::road::slopestake::CanPaintBreakPoint as _;
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
            ui.allocate_painter(Vec2::new(455.0, 455.0), Sense::empty());

        let mut stroke = Stroke::new(3.0, Color32::WHITE);

        let gridlines_stroke = Stroke::new(0.4, Color32::WHITE);
        let gridlines_freq = Vec2::new(20.0, 20.0);

        painter.rect_filled(response.rect, 0.0, Color32::BLACK);
        painter.grid_lines(response.rect, gridlines_stroke, gridlines_freq);

        for window in self.0.pts.borrow().windows(2) {
            let (cur, next) = (&window[0], &window[1]);
            if !SlopeStake::intercepts(cur, next) {
                stroke.color = Color32::RED;
            }
            let offset_to_next = f32::from(next.offset) - f32::from(cur.offset);
            painter.break_point(&cur, response.rect, stroke, offset_to_next);
        }
        if let Some(last) = self.0.pts.borrow().last() {
            painter.break_point(last, response.rect, stroke, 0.0);
        }

        response
    }
}
