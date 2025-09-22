use super::SlopeStake;
use crate::ui::utilities::AutoPlot;
use eframe::egui::{vec2, Response, Ui, Vec2, Widget};

pub struct SlopeStakeViewer<'a> {
    data: &'a SlopeStake,
    desired_size: Vec2,
}
impl<'a> SlopeStakeViewer<'a> {
    pub fn new(ss: &'a SlopeStake) -> Self {
        Self {
            data: ss,
            desired_size: vec2(410.0, 340.0),
        }
    }
}

impl<'a> Widget for SlopeStakeViewer<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let points: Vec<Vec2> =
            self.data.pts.iter().map(|pt| Vec2::from(*pt)).collect();

        ui.add(AutoPlot::new(points, self.desired_size))
    }
}
