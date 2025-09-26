use super::GridLinesStyle;
use eframe::egui::{Response, Slider, Ui, Widget};

pub struct GridLinesStyleEditor<'a>(&'a mut GridLinesStyle);
impl<'a> GridLinesStyleEditor<'a> {
    pub fn new(gls: &'a mut GridLinesStyle) -> Self {
        Self(gls)
    }
}
impl<'a> Widget for GridLinesStyleEditor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (x, y, enabled) = match self.0.get_origin() {
            Some(v) => (v.x, v.y, true),
            None => (0.0, 0.0, false),
        };

        ui.horizontal(|ui| {
            let mut x_val = x;
            let mut y_val = y;

            let r1 = ui.add_enabled(
                enabled,
                Slider::new(&mut x_val, -100.0..=100.0).text("Origin X"),
            );
            let r2 = ui.add_enabled(
                enabled,
                Slider::new(&mut y_val, -100.0..=100.0).text("Origin Y"),
            );

            // If changed and origin is Some, update underlying data
            if enabled && (r1.changed() || r2.changed()) {
                if let Some(ref mut origin) = self.0.get_origin() {
                    origin.x = x_val;
                    origin.y = y_val;
                }
            }
        })
        .response
    }
}
//impl<'a> GridLinesStyleEditor
