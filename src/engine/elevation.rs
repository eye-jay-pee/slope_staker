#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Elevation(f64);

impl std::fmt::Display for Elevation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:1.01}", self.0)
    }
}
impl eframe::egui::emath::Numeric for Elevation {
    const INTEGRAL: bool = false;
    const MIN: Self = Elevation(f64::MIN);
    const MAX: Self = Elevation(f64::MAX);

    fn to_f64(self) -> f64 {
        self.0
    }
    fn from_f64(v: f64) -> Self {
        Elevation(v)
    }
}

pub mod ui {
    use super::*;
    use eframe::egui::{Response, Ui, Widget};

    pub struct ElevationEditor<'a>(&'a mut Elevation);
    impl<'a> ElevationEditor<'a> {
        pub fn new(feet: &'a mut Elevation) -> Self {
            Self(feet)
        }
    }
    impl<'a> Widget for ElevationEditor<'a> {
        fn ui(self, ui: &mut Ui) -> Response {
            use eframe::egui::DragValue;

            ui.horizontal(|ui| {
                ui.label("Elevation:");
                ui.add(
                    DragValue::new(self.0)
                        .custom_formatter(|val, _| Elevation(val).to_string()),
                );
            })
            .response
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn sample_test() {}
}
