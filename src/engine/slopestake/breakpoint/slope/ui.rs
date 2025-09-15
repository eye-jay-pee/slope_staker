use super::*;
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
impl std::fmt::Display for Slope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(s) => write!(f, "{:1.02}:1", s),
            None => write!(f, "____:1"),
        }
    }
}
impl eframe::egui::emath::Numeric for Slope {
    const INTEGRAL: bool = false;
    const MIN: Self = Slope(Some(0.0)); // a vetical wall
    const MAX: Self = Slope(Some(f64::INFINITY)); // level ground

    fn to_f64(self) -> f64 {
        // an unset slope is not the same as a slope of 0 (which would be
        // a wall, but dragvalue can only handle numeric values (not NaN),
        // so just give it 0.  This will not be used for anything except a
        // reference point if user initally sets the value by dragging
        // rather than typing in the dragvalue.
        self.0.unwrap_or(0.0)
    }
    fn from_f64(v: f64) -> Self {
        Self::from(v)
    }
}
