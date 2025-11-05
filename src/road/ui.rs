use super::{CrossSectionEditor, Road, Station, StationEditor};
use eframe::egui::{Response, Ui, Widget};

pub struct RoadEditor<'a> {
    road: &'a mut Road,
    selected_station: Station,
}

impl<'a> RoadEditor<'a> {
    pub fn new(road: &'a mut Road) -> Self {
        Self {
            road: road,
            selected_station: Station::from(0.0),
        }
    }
    fn _insert_new(&mut self) {
        self.road.insert_new(self.selected_station);
    }
    fn _next_existing(&mut self) -> Option<Station> {
        self.road
            ._specified_stations()
            .iter()
            .find(|&&v| v > self.selected_station)
            .copied()
    }
    fn _prev_existing(&mut self) -> Option<Station> {
        self.road
            ._specified_stations()
            .iter()
            .find(|&&v| v < self.selected_station)
            .copied()
    }
}
impl<'a> Widget for RoadEditor<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        if self
            .road
            .cross_sections
            .get(&self.selected_station)
            .is_none()
        {
            self.road.insert_new(self.selected_station)
        }
        ui.vertical(|ui| {
            ui.add(StationEditor::new(&mut self.selected_station));
            ui.add(CrossSectionEditor::new(
                self.road
                    .cross_sections
                    .get_mut(&self.selected_station)
                    .unwrap(),
            ));
        })
        .response
    }
}
