use super::{CrossSectionEditor, Road, Station};
use eframe::egui::{Response, Ui, Widget};

pub struct RoadEditor<'a> {
    road: &'a mut Road,
    station: Station,
}

impl<'a> RoadEditor<'a> {
    pub fn new(road: &'a mut Road) -> Self {
        Self {
            road: road,
            station: Station::from(0.0),
        }
    }
}
impl<'a> Widget for RoadEditor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        if let Some(cross_section) = self.road.0.get_mut(&self.station) {
            ui.add(CrossSectionEditor::new(cross_section));
        }
        ui.label("bad station")
    }
}

//        ui.vertical(|ui| {
//            ui.group(|ui| {
//                ui.add(SlopeStakeViewer::new(&self.data.0.left));
//            });
//            ui.group(|ui| {
//                ui.add(SlopeStakeEditor::new(&mut self.data.0.left));
//            });
//        })
