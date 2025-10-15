use super::{constants, Elevation};
use eframe::egui::{DragValue, Response, Ui, Widget};

pub struct ElevationEditor<'a>(&'a mut Elevation);
impl<'a> ElevationEditor<'a> {
    pub fn new(feet: &'a mut Elevation) -> Self {
        Self(feet)
    }
}
impl<'a> Widget for ElevationEditor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            ui.label("Elev:");
            //ui.add(
            ui.add_sized(
                [60.0, ui.spacing().interact_size.y],
                DragValue::new(self.0).custom_formatter(|val, _| {
                    Elevation::from(val).to_string()
                }),
            )
        })
        .response
    }
}
impl eframe::egui::emath::Numeric for Elevation {
    const INTEGRAL: bool = false;
    const MIN: Self = constants::MEAN_SEA_LEVEL;
    const MAX: Self = constants::MT_EVEREST;

    fn to_f64(self) -> f64 {
        f64::from(self)
    }

    fn from_f64(v: f64) -> Self {
        Self::from(v)
    }
}
