use super::SlopeStake;
use crate::foreign::PainterExt;
use crate::slopestake::CanPaintBreakPoint as _;
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

        let stroke = Stroke::new(
            2.0,
            if self.0.is_valid() {
                Color32::WHITE
            } else {
                Color32::RED
            },
        );
        let gridlines_stroke = Stroke::new(0.4, Color32::WHITE);
        let gridlines_freq = Vec2::new(10.0, 10.0);

        painter.rect_filled(response.rect, 0.0, Color32::BLACK);
        painter.grid_lines(response.rect, gridlines_stroke, gridlines_freq);

        for window in self.0.pts.windows(2) {
            let (cur, next) = (&window[0], &window[1]);
            let offset_to_next = f32::from(next.offset) - f32::from(cur.offset);
            painter.break_point(&cur, response.rect, stroke, offset_to_next);
        }
        if let Some(last) = self.0.pts.last() {
            painter.break_point(last, response.rect, stroke, 0.0);
        }

        response
    }
}
