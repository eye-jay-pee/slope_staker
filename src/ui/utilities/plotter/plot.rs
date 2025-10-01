use crate::foreign::PainterExt as _;
use eframe::egui::{Color32, Response, Sense, Stroke, Ui, Vec2, Widget};

pub struct Plot {
    background: Color32,
    grid_freq: Vec2,
    grid_stroke: Stroke,

    points: Vec<Vec2>,
    point_stroke: Stroke,
    min_val: Vec2,
    max_val: Vec2,
}
#[allow(dead_code)]
impl Plot {
    pub fn background(mut self, bg: Color32) -> Self {
        self.background = bg;
        self
    }
    pub fn scope(mut self, min: Vec2, max: Vec2) -> Self {
        self.min_val = min;
        self.max_val = max;
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
    pub fn points(mut self, points: Vec<Vec2>) -> Self {
        self.points = points;
        self
    }
}
impl Default for Plot {
    fn default() -> Self {
        Self {
            background: Color32::BLACK,
            grid_freq: Vec2::new(10.0, 10.0),
            grid_stroke: Stroke::new(1.0 / 16.0, Color32::WHITE),

            points: Vec::new(),
            point_stroke: Stroke::new(2.0, Color32::GREEN),
            min_val: Vec2::new(-200.0, -200.0),
            max_val: Vec2::new(200.0, 200.0),
        }
    }
}
impl Widget for Plot {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            let (response, painter) = ui
                .allocate_painter(self.max_val - self.min_val, Sense::empty());

            let rect = response.rect;
            painter.rect_filled(rect, 0.0, self.background);
            painter.grid_lines(rect, self.grid_stroke, self.grid_freq);

            for pt in self.points {
                painter.circle_filled(
                    rect.center() + pt,
                    self.point_stroke.width,
                    self.point_stroke.color,
                );
            }
        })
        .response
    }
}
