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

        ui.horizontal(|ui| {
            ui.label("Slope:");
            ui.add_sized(
                [60.0, ui.spacing().interact_size.y],
                DragValue::new(self.data)
                    .update_while_editing(true)
                    .custom_parser(|s| Self::parse(s))
                    .custom_formatter(|val, _rng| Slope::from(val).to_string()),
            )
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

        if let Ok(lt) = caps.get(1)?.as_str().parse::<f64>() {
            if let Ok(rt) = caps.get(2)?.as_str().parse::<f64>() {
                Some(rt / lt)
            } else if lt.abs() > 1.0 {
                Some(1.0 / lt)
            } else {
                Some(lt)
            }
        } else {
            None
        }
    }
}
