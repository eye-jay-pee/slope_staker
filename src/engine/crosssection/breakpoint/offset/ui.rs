use super::*;
use eframe::egui::{Response, Ui, Widget};

pub struct OffsetEditor<'a>(&'a mut Offset);
impl<'a> OffsetEditor<'a> {
    pub fn new(os: &'a mut Offset) -> Self {
        Self(os)
    }
}
impl<'a> Widget for OffsetEditor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        use eframe::egui::DragValue;

        ui.horizontal(|ui| {
            ui.label("O/S:");
            ui.add(
                DragValue::new(self.0)
                    .custom_formatter(|val, _| Offset::from(val).to_string()),
            )
        })
        .response
    }
}
impl std::fmt::Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.read() {
            Some(os) => write!(f, "{:1.02}'", os,),
            None => write!(f, "____'"),
        }
    }
}
impl eframe::egui::emath::Numeric for Offset {
    const INTEGRAL: bool = false;
    const MIN: Self = constants::ONE_MILE_BACKWARDS;
    const MAX: Self = constants::ONE_MILE;

    fn to_f64(self) -> f64 {
        if let Some(os) = self.0 {
            if !os.is_nan() {
                return os;
            }
        }
        0.0
    }
    fn from_f64(v: f64) -> Self {
        Offset::from(v)
    }
}
