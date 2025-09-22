use crate::utilities::DisplayExt;
use eframe::egui::{
    color_picker::{color_edit_button_srgba, Alpha},
    Color32, Response, Ui, Widget,
};
use std::fmt::Formatter;

pub struct Color32Editor<'a>(&'a mut Color32);
impl<'a> Widget for Color32Editor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        color_edit_button_srgba(ui, self.0, Alpha::Opaque)
    }
}

impl DisplayExt for Color32 {
    fn fmt_ext(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = match *self {
            Color32::TRANSPARENT => "transparent".to_owned(),
            Color32::BLACK => "black".to_owned(),
            Color32::DARK_GRAY => "dark gray".to_owned(),
            Color32::GRAY => "gray".to_owned(),
            Color32::LIGHT_GRAY => "light gray".to_owned(),
            Color32::WHITE => "white".to_owned(),
            Color32::BROWN => "brown".to_owned(),
            Color32::DARK_RED => "dark red".to_owned(),
            Color32::RED => "red".to_owned(),
            Color32::LIGHT_RED => "light red".to_owned(),
            Color32::CYAN => "cyan".to_owned(),
            Color32::MAGENTA => "magenta".to_owned(),
            Color32::YELLOW => "yellow".to_owned(),
            Color32::ORANGE => "orange".to_owned(),
            Color32::LIGHT_YELLOW => "light yellow".to_owned(),
            Color32::KHAKI => "khaki".to_owned(),
            Color32::DARK_GREEN => "dark green".to_owned(),
            Color32::GREEN => "green".to_owned(),
            Color32::LIGHT_GREEN => "light green".to_owned(),
            Color32::DARK_BLUE => "dark blue".to_owned(),
            Color32::BLUE => "blue".to_owned(),
            Color32::LIGHT_BLUE => "light blue".to_owned(),
            Color32::PURPLE => "purple".to_owned(),
            Color32::GOLD => "gold".to_owned(),
            Color32::DEBUG_COLOR => "debug color".to_owned(),
            _ => self.to_hex(),
        };
        write!(f, "{}", text)
    }
}
