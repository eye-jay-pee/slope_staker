#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Slope(f64);

impl std::fmt::Display for Slope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:1.02}:1", self.0)
    }
}
impl eframe::egui::emath::Numeric for Slope {
    const INTEGRAL: bool = false;
    const MIN: Self = Slope(0.0); // a vetical wall
    const MAX: Self = Slope(f64::MAX); // almost level ground

    fn to_f64(self) -> f64 {
        self.0
    }
    fn from_f64(v: f64) -> Self {
        Slope(v)
    }
}
pub mod ui {
    use super::*;
    use eframe::egui::{Response, Ui, Widget};

    pub struct SlopeEditor<'a>(&'a mut Slope);
    impl<'a> SlopeEditor<'a> {
        pub fn new(slope: &'a mut Slope) -> Self {
            Self(slope)
        }
    }
    impl<'a> Widget for SlopeEditor<'a> {
        fn ui(self, ui: &mut Ui) -> Response {
            use eframe::egui::DragValue;
            ui.horizontal(|ui| {
                ui.label("Slope");
                ui.add(
                    DragValue::new(self.0)
                        .custom_formatter(|val, _| Slope(val).to_string())
                        .custom_parser(|s| {
                            let cleaned = s.trim().trim_end_matches(":1");
                            cleaned.parse::<f64>().ok()
                        }),
                );
            })
            .response
        }
    }
}
