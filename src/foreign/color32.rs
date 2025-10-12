use super::DisplayExt;
use eframe::egui::{
    color_picker::{color_edit_button_srgba, Alpha},
    Color32, Response, Ui, Widget,
};

#[allow(dead_code)]
pub struct Color32Editor<'a>(&'a mut Color32);
impl<'a> Widget for Color32Editor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        color_edit_button_srgba(ui, self.0, Alpha::Opaque)
    }
}

use std::fmt::{Formatter, Result};
impl DisplayExt for Color32 {
    fn fmt_ext(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_hex())
    }
}
