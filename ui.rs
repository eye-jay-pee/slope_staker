use super::*;
use eframe::egui::{Response, Ui, Widget};

pub struct StationEditor<'a>(&'a mut Station);
impl<'a> Widget for StationEditor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        use eframe::egui::DragValue;

        let dv = DragValue::from_get_set(|v| self.0.get_set_raw_sta(v))
            .custom_formatter(|val, _| Station::from_f64(val).to_string());

        ui.horizontal(|ui| {
            ui.label("Station:");
            ui.add(dv)
        })
        .response
    }
}
