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
        let mut plot = Plot::default();
        for pt in &self.0.pts {
            plot.add_point(Vec2::from(pt));
        }
        ui.add(plot)
    }
}
