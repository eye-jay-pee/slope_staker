use crate::foreign::PainterExt as _;
use eframe::egui::{
    Color32, Painter, Rect, Response, Sense, Stroke, Ui, Vec2, Widget,
};

pub struct Plot {
    desired_size: Vec2, // layout size on screen
    range_min: Vec2,
    range_max: Vec2,

    background: Color32,
    grid_freq: Vec2,
    grid_stroke: Stroke,

    prescale_points: Vec<Vec2>,
    point_stroke: Stroke,
    _line_stroke: Stroke,
}
#[allow(dead_code)]
impl Plot {
    pub fn add_point(&mut self, prescaled: Vec2) {
        self.prescale_points.push(prescaled);
    }
    pub fn background(mut self, bg: Color32) -> Self {
        self.background = bg;
        self
    }
    pub fn scope(mut self, min: Vec2, max: Vec2) -> Self {
        self.range_min = min;
        self.range_max = max;
        self
    }
    pub fn grid(mut self, freq: Vec2, color: Color32, width: f32) -> Self {
        self.grid_freq = freq;
        self.grid_stroke.color = color;
        self.grid_stroke.width = width;
        self
    }
    pub fn points_stroke(mut self, color: Color32, width: f32) -> Self {
        self.point_stroke = Stroke::new(width, color);
        self
    }
}
impl Default for Plot {
    fn default() -> Self {
        Self {
            desired_size: Vec2::new(400.0, 400.0),
            range_min: Vec2::new(-200.0, -200.0),
            range_max: Vec2::new(200.0, 200.0),

            background: Color32::BLACK,
            grid_freq: Vec2::new(10.0, 10.0),
            grid_stroke: Stroke::new(1.0 / 16.0, Color32::WHITE),

            prescale_points: Vec::new(),
            point_stroke: Stroke::new(2.0, Color32::GREEN),
            _line_stroke: Stroke::new(1.0, Color32::GREEN),
        }
    }
}
impl Widget for Plot {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let (response, rect, painter) = self.allocate(ui);

        painter.rect_filled(rect, 0.0, self.background);
        painter.grid_lines(rect, self.grid_stroke, self.grid_freq);

        painter.line(
            self.prescale_points
                .into_iter()
                .map(|v: Vec2| rect.center() + v)
                .collect(),
            self._line_stroke,
        );

        response
    }
}
impl Plot {
    fn allocate(&mut self, ui: &mut Ui) -> (Response, Rect, Painter) {
        let (response, painter) =
            ui.allocate_painter(self.desired_size, Sense::empty());
        (response.clone(), response.rect, painter)
    }
}
