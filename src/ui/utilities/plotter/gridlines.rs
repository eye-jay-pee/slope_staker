use eframe::egui::{
    pos2, vec2, Color32, Response, Sense, Stroke, Ui, Vec2, Widget,
};
pub struct GridLines {
    area: Vec2,
    min: Vec2,
    max: Vec2,
    freq: Vec2,
    fore: Color32,
    back: Color32,
}
impl GridLines {
    pub fn area(mut self, new: Vec2) -> Self {
        self.area = new;
        self
    }
    pub fn range(mut self, min: Vec2, max: Vec2) -> Self {
        self.min = min;
        self.max = max;
        self
    }
    pub fn freq(mut self, new: Vec2) -> Self {
        self.freq = new;
        self
    }
    pub fn fore(mut self, new: Color32) -> Self {
        self.fore = new;
        self
    }
    pub fn back(mut self, new: Color32) -> Self {
        self.back = new;
        self
    }
}
impl Default for GridLines {
    fn default() -> Self {
        Self {
            area: vec2(100.0, 100.0),
            min: Vec2::ZERO,
            max: vec2(10.0, 10.0),
            freq: vec2(1.0, 1.0),
            back: Color32::BLACK,
            fore: Color32::WHITE,
        }
    }
}
impl Widget for GridLines {
    fn ui(self, ui: &mut Ui) -> Response {
        let (response, painter) =
            ui.allocate_painter(self.area, Sense::hover());
        let rect = response.rect;

        // Fill background
        painter.rect_filled(rect, 0.0, self.back);

        let stroke = Stroke::new(1.0, self.fore);
        if self.freq.x != 0.0 {
            let mut x = rect.left();
            while x <= rect.left() {
                let pts = [pos2(x, rect.top()), pos2(x, rect.bottom())];
                painter.line_segment(pts, stroke);
                x += self.freq.x;
            }
        }

        response
    }
}
