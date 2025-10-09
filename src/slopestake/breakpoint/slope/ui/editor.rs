use super::Slope;
use eframe::egui::{Response, Ui, Widget};
use regex::Regex;

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

        let dv = DragValue::new(self.data)
            .update_while_editing(true)
            .custom_parser(|s| Self::parse(s))
            .custom_formatter(|val, _rng| Slope::from(val).to_string());

        ui.horizontal(|ui| {
            ui.label("Slope:");
            ui.add(dv);
        })
        .response
    }
}

impl<'a> SlopeEditor<'a> {
    fn parse(input: &str) -> Option<f64> {
        let caps =
            Regex::new(r"^\D*?(-?\d+(?:\.\d+)?)(?::\D*?(-?\d+(?:\.\d+)?))?")
                .unwrap()
                .captures(input)?;

        let run: f64 = caps.get(1)?.as_str().parse().ok()?;
        let rise = if let Some(val) = caps.get(2) {
            val.as_str().parse().ok()?
        } else {
            1.0
        };

        Some(run / rise)
    }
}
