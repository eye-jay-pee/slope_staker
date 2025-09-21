use eframe::egui::{pos2, Painter, Rect, Stroke, Vec2};

pub struct GridLines<'a> {
    painter: &'a Painter,
    stroke: Stroke,
    rect: Rect,
    freq: Vec2,
}
impl<'a> GridLines<'a> {
    fn vert_line(&self, x: f32) {
        let top = pos2(x, self.rect.top());
        let btm = pos2(x, self.rect.bottom());
        let pts = [top, btm];
        self.painter.line_segment(pts, self.stroke);
    }
    fn hori_line(&self, y: f32) {
        let left = pos2(self.rect.left(), y);
        let right = pos2(self.rect.right(), y);
        let pts = [left, right];
        self.painter.line_segment(pts, self.stroke);
    }
}
impl<'a> GridLines<'a> {
    pub fn draw(self) {
        let mut i = Vec2::new(0.0, 0.0);
        if self.freq.x > 0.0 {
            loop {
                let in_bounds_left = -i.x >= self.rect.left();
                let in_bounds_right = i.x <= self.rect.right();
                if in_bounds_left || in_bounds_right {
                    if in_bounds_left {
                        self.vert_line(-i.x);
                    }
                    if in_bounds_right {
                        self.vert_line(i.x);
                    }
                    i.x += self.freq.x;
                } else {
                    break;
                }
            }
        }
        if self.freq.y > 0.0 {
            loop {
                let in_bounds_top = -i.y >= self.rect.top();
                let in_bounds_bottom = i.y <= self.rect.bottom();
                if in_bounds_bottom || in_bounds_top {
                    if in_bounds_top {
                        self.hori_line(-i.y);
                    }
                    if in_bounds_bottom {
                        self.hori_line(i.y);
                    }
                    i.y += self.freq.y;
                } else {
                    break;
                }
            }
        }
    }
}
impl<'a> GridLines<'a> {
    pub fn new(painter: &'a Painter, rect: Rect) -> Self {
        Self {
            painter: painter,
            stroke: Stroke::default(),
            rect: rect,
            freq: Vec2::new(1.0, 1.0),
        }
    }
    pub fn stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = stroke;
        self
    }
    pub fn freq(mut self, freq: Vec2) -> Self {
        self.freq = freq;
        self
    }
}
