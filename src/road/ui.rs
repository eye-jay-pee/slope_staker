use super::Road;
use eframe::egui::{Response, Ui, Widget};

pub struct _RoadEditor<'a>(&'a mut Road);
impl<'a> _RoadEditor<'a> {
    pub fn _new(road: &'a mut Road) -> Self {
        Self(road)
    }
}
impl<'a> Widget for _RoadEditor<'a> {
    fn ui(self, _ui: &mut Ui) -> Response {
        todo!()
    }
}
