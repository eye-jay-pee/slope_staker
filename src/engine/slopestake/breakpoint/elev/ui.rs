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
            ui.label("Elev:");
            ui.add(
                DragValue::new(self.0).custom_formatter(|val, _| {
                    Elevation::from(val).to_string()
                }),
            )
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

#[cfg(test)]
mod tests {
    use super::{Elevation, ElevationEditor};
    use eframe::egui::{CentralPanel, Context};

    /// Verify the widget can be constructed and rendered without an error
    #[test]
    fn smoke_test() {
        let ctx = Context::default();

        let _ = ctx.run(Default::default(), |ctx| {
            CentralPanel::default().show(ctx, |ui| {
                let mut elev = Elevation::default();
                let editor = ElevationEditor::new(&mut elev);

                let _resp = ui.add(editor);
            });
        });
    }
}
