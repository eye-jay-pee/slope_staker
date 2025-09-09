#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Elevation(Option<f64>);

impl Elevation {
    pub fn read(&self) -> Option<f64> {
        self.0
    }
    pub fn write(&mut self, val: Option<f64>) {
        self.0 = val
    }
}
impl From<f64> for Elevation {
    fn from(v: f64) -> Self {
        if v.is_nan() {
            Self(None)
        } else {
            Self(Some(v))
        }
    }
}
impl From<Elevation> for f64 {
    fn from(os: Elevation) -> f64 {
        os.0.unwrap_or(f64::NAN)
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
                ui.label("Elev");
                ui.add(DragValue::new(self.0).custom_formatter(|val, _| {
                    Elevation::from(val).to_string()
                }));
            })
            .response
        }
    }
    impl std::fmt::Display for Elevation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.0 {
                Some(e) => write!(f, "{:1.02}'", e),
                None => write!(f, "____'"),
            }
        }
    }
    impl eframe::egui::emath::Numeric for Elevation {
        const INTEGRAL: bool = false;
        const MIN: Self = constants::MEAN_SEA_LEVEL;
        const MAX: Self = constants::MT_EVEREST;

        fn to_f64(self) -> f64 {
            if let Some(v) = self.0 {
                if !v.is_nan() {
                    return v;
                }
            }
            constants::DEFAULT_ELEV.0.unwrap()
        }

        fn from_f64(v: f64) -> Self {
            Elevation::from(v)
        }
    }
}
pub mod constants {
    use super::*;
    pub const MEAN_SEA_LEVEL: Elevation = Elevation(Some(0.0));
    pub const MT_EVEREST: Elevation = Elevation(Some(29032.0));
    pub const LINDSBORG_KS: Elevation = Elevation(Some(1306.0));
    pub const DEFAULT_ELEV: Elevation = LINDSBORG_KS;
}
