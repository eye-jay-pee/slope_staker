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

    point_stroke: Stroke,
}
#[allow(dead_code)]
impl Plot {
    fn add_pt(self, painter: Painter, rect: Rect, prescaled: Vec2) -> Self {
        let range_size = self.range_max - self.range_min;
        let normalized = (prescaled - self.range_min) / range_size;
        let absolute = rect.min + normalized * range_size;

        painter.circle_filled(
            absolute,
            self.point_stroke.width / 2.0,
            self.point_stroke.color,
        );
        self
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

            background: Color32::BLACK,
            grid_freq: Vec2::new(10.0, 10.0),
            grid_stroke: Stroke::new(1.0 / 16.0, Color32::WHITE),

            point_stroke: Stroke::new(2.0, Color32::GREEN),
            range_min: Vec2::new(-200.0, -200.0),
            range_max: Vec2::new(200.0, 200.0),
        }
    }
}
impl Widget for Plot {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let (response, rect, painter) = self.allocate(ui);

        painter.rect_filled(rect, 0.0, self.background);
        painter.grid_lines(rect, self.grid_stroke, self.grid_freq);

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
