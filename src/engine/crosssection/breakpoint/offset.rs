#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Offset(f64);
impl std::fmt::Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:1.02}'", self.0,)
    }
}
impl eframe::egui::emath::Numeric for Offset {
    const INTEGRAL: bool = false;
    const MIN: Self = Self(0.0);
    const MAX: Self = Self(f64::MAX);

    fn to_f64(self) -> f64 {
        self.0
    }
    fn from_f64(v: f64) -> Self {
        Self(v)
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
                    DragValue::new(self.0)
                        .custom_formatter(|val, _| Offset(val).to_string()),
                );
            })
            .response
        }
    }
}
