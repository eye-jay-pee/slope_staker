use super::_Road;
use eframe::egui::{Response, Ui, Widget};

pub struct _RoadEditor<'a>(&'a mut _Road);
impl<'a> _RoadEditor<'a> {
    pub fn _new(road: &'a mut _Road) -> Self {
        Self(road)
    }
}
impl<'a> Widget for _RoadEditor<'a> {
    fn ui(self, _ui: &mut Ui) -> Response {
        todo!()
    }
}
