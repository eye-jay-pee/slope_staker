use super::{constants, Offset};
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
            ui.add_sized(
                [60.0, ui.spacing().interact_size.y],
                DragValue::new(self.0)
                    .custom_formatter(|val, _| Offset::from(val).to_string()),
            )
        })
        .response
    }
}
impl eframe::egui::emath::Numeric for Offset {
    const INTEGRAL: bool = false;
    const MIN: Self = constants::ONE_MILE_BACKWARDS;
    const MAX: Self = constants::ONE_MILE;

    fn to_f64(self) -> f64 {
        f64::from(self)
    }
    fn from_f64(v: f64) -> Self {
        Offset::from(v)
    }
}
