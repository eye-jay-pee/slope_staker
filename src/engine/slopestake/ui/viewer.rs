use super::SlopeStake;
use eframe::egui::{vec2, Response, Ui, Vec2, Widget};

pub struct SlopeStakeViewer<'a> {
    data: &'a SlopeStake,
    desired_size: Vec2,
}
impl<'a> SlopeStakeViewer<'a> {
    pub fn new(ss: &'a SlopeStake) -> Self {
        Self {
            data: ss,
            desired_size: vec2(300.0, 300.0),
        }
    }
}

impl<'a> Widget for SlopeStakeViewer<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        use crate::ui::utilities::Plot;
        use eframe::egui::Vec2;

        let points: Vec<Vec2> =
            self.data.pts.iter().map(|pt| Vec2::from(*pt)).collect();

        ui.add(Plot::new(points, self.desired_size))
    }
}
