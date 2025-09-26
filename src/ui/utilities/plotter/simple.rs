use super::{GridLinesStyle, GridLinesStyleEditor, PainterCanDrawGridLines};
use eframe::egui::{Color32, Response, Sense, Ui, Vec2, Widget};

#[derive(Debug, Default, Clone)]
pub struct SimplePlot {
    _pts: Vec<Vec2>,
    area: Vec2,
    _trim_min: Vec2,
    _trim_max: Vec2,
    _scale_factor: Vec2,
}
impl SimplePlot {
    pub fn area(mut self, new: Vec2) -> Self {
        self.area = new;
        self
    }
    pub fn _trim_min(mut self, new: Vec2) -> Self {
        self._trim_min = new;
        self
    }
    pub fn _trim_max(mut self, new: Vec2) -> Self {
        self._trim_max = new;
        self
    }
    pub fn _scale_factor(mut self, new: Vec2) -> Self {
        self._scale_factor = new;
        self
    }
    pub fn _add_pt(mut self, new: Vec2) -> Self {
        self._pts.push(new);
        self
    }
}
impl Widget for SimplePlot {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            let (response, painter) =
                ui.allocate_painter(self.area, Sense::empty());
            let rect = response.rect;

            painter.rect_filled(rect, 0.0, Color32::BLACK);

            let mut gls = GridLinesStyle::default().set_freq(25.0, 25.0);
            ui.add(GridLinesStyleEditor::new(&mut gls));
            painter.grid_lines(rect, gls);
        })
        .response
    }
}
