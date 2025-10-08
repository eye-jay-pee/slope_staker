use super::Slope;
use eframe::egui::{Response, Ui, Widget};

pub struct SlopeEditor<'a> {
    data: &'a mut Slope,
}

impl<'a> SlopeEditor<'a> {
    pub fn new(slope: &'a mut Slope) -> Self {
        Self { data: slope }
    }
}
impl<'a> Widget for SlopeEditor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        use eframe::egui::DragValue;

        ui.horizontal(|ui| {
            ui.label("Slope:");
            ui.add(
                DragValue::new(self.data)
                    .custom_formatter(|val, _| Slope::from(val).to_string())
                    .custom_parser(|s| {
                        let cleaned = s.trim().trim_end_matches(":1");
                        cleaned.parse::<f64>().ok()
                    }),
            )
        })
        .response
    }
}
