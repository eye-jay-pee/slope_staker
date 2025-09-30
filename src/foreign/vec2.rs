use super::DisplayExt;
use eframe::egui::{Response, Slider, Ui, Vec2, Widget};

pub struct Vec2Editor<'a>(&'a mut Vec2);
impl<'a> Vec2Editor<'a> {
    pub fn new(data: &'a mut Vec2) -> Self {
        Self(data)
    }
}
impl<'a> Widget for Vec2Editor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let x_res =
            ui.add(Slider::new(&mut self.0.x, -100.0..=100.0).text("x"));
        let y_res =
            ui.add(Slider::new(&mut self.0.y, -100.0..=100.0).text("y"));
        x_res | y_res
    }
}

use std::fmt::{Formatter, Result};
impl DisplayExt for Vec2 {
    fn fmt_ext(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
