use super::{CrossSectionEditor, Road, Station, StationEditor};
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
    fn ui(mut self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.add(StationEditor::new(&mut self.station));

            match &mut self.road.cross_sections.get_mut(&self.station) {
                Some(cs) => ui.add(CrossSectionEditor::new(cs)),
                None => {
                    ui.horizontal(|ui| {
                        ui.label("no cross section at this station");
                        let resp = ui.button("add one?");
                        if resp.clicked() {
                            println!("a cross sectino needs to be added");
                        }
                        resp
                    })
                    .response
                }
            }
        })
        .response
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
