pub mod kind;
pub mod slope;
#[derive(Debug, Default, Clone, Copy, PartialEq)]

pub struct BreakPoint {
    kind: kind::BreakPointKind,
    offset: Option<f64>,
    _slope: Option<f32>,
}
impl std::fmt::Display for BreakPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}' O/S from \u{2104}", self.kind, self.offset)
    }
}

pub mod ui {
    use super::*;
    use eframe::egui::{Response, Ui, Widget};

    pub struct BreakPointEditor<'a>(&'a mut BreakPoint);
    impl<'a> BreakPointEditor<'a> {
        pub fn new(bp: &'a mut BreakPoint) -> Self {
            Self(bp)
        }
    }
    impl<'a> Widget for BreakPointEditor<'a> {
        fn ui(self, ui: &mut Ui) -> Response {
            use super::kind::ui::BreakPointKindSelector;

            use eframe::egui::DragValue;

            ui.horizontal(|ui| {
                ui.add(BreakPointKindSelector::new(&mut self.0.kind));
                ui.label("@");
                ui.add(DragValue::new(&mut self.0.offset))
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
