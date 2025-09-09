#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Offset(Option<f64>);

impl Offset {
    pub fn read(&self) -> Option<f64> {
        self.0
    }
    pub fn write(&mut self, val: Option<f64>) {
        self.0 = val
    }
}

impl From<f64> for Offset {
    fn from(v: f64) -> Self {
        if v.is_nan() {
            Self(None)
        } else {
            Self(Some(v))
        }
    }
}
impl From<Offset> for f64 {
    fn from(os: Offset) -> f64 {
        os.0.unwrap_or(f64::NAN)
    }
}
pub mod ui {
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
                ui.label("O/S to CL");
                ui.add(
                    DragValue::new(self.0).custom_formatter(|val, _| {
                        Offset::from(val).to_string()
                    }),
                );
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
}

pub mod constants {
    use super::*;
    pub const ONE_MILE: Offset = Offset(Some(5280.0));
    pub const ONE_MILE_BACKWARDS: Offset = Offset(Some(-5280.0));
}
