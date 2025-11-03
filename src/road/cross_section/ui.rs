use super::CrossSection;
use eframe::egui::{Response, Ui, Widget};

pub struct CrossSectionEditor<'a>(&'a mut CrossSection);

impl<'a> CrossSectionEditor<'a> {
    pub fn new(cs: &'a mut CrossSection) -> Self {
        Self(cs)
    }
}

impl<'a> Widget for CrossSectionEditor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label("todo: slooepstakeeditor")
    }
}
