use super::SlopeStake;
use crate::ui::utilities::Plot;
use eframe::egui::{Response, Ui, Vec2, Widget};

pub struct SlopeStakeViewer<'a>(&'a SlopeStake);
impl<'a> SlopeStakeViewer<'a> {
    pub fn new(ss: &'a SlopeStake) -> Self {
        Self(ss)
    }
}

impl<'a> Widget for SlopeStakeViewer<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let _points: Vec<Vec2> =
            self.0.pts.iter().map(|pt| Vec2::from(*pt)).collect();

        ui.add(Plot::default())
    }
}
