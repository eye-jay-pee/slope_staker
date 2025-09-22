use eframe::egui::{pos2, Painter, Rect, Vec2};

pub mod state;
pub use state::GridLinesState;

pub struct GridLines<'a> {
    painter: &'a Painter,
    rect: Rect,
    origin: Vec2,
    state: &'a mut GridLinesState,
}
impl<'a> GridLines<'a> {
    fn vert_line(&self, x: f32) {
        let top = pos2(x, self.rect.top());
        let btm = pos2(x, self.rect.bottom());
        let pts = [top, btm];
        self.painter.line_segment(pts, self.state.stroke);
    }
    fn hori_line(&self, y: f32) {
        let left = pos2(self.rect.left(), y);
        let right = pos2(self.rect.right(), y);
        let pts = [left, right];
        self.painter.line_segment(pts, self.state.stroke);
    }
}
impl<'a> GridLines<'a> {
    pub fn draw(self) {
        let mut i = Vec2::new(0.0, 0.0);
        if self.state.freq.x > 0.0 {
            loop {
                let left = self.origin.x - i.x;
                let right = self.origin.x + i.x;
                let in_bounds_left = left >= self.rect.left();
                let in_bounds_right = right <= self.rect.right();
                if in_bounds_left || in_bounds_right {
                    if in_bounds_left {
                        self.vert_line(left);
                    }
                    if in_bounds_right {
                        self.vert_line(right);
                    }
                    i.x += self.state.freq.x;
                } else {
                    break;
                }
            }
        }
        if self.state.freq.y > 0.0 {
            loop {
                let top = self.origin.y - i.y;
                let bottom = self.origin.y + i.y;
                let in_bounds_top = top >= self.rect.top();
                let in_bounds_bottom = bottom <= self.rect.bottom();
                if in_bounds_bottom || in_bounds_top {
                    if in_bounds_top {
                        self.hori_line(top);
                    }
                    if in_bounds_bottom {
                        self.hori_line(bottom);
                    }
                    i.y += self.state.freq.y;
                } else {
                    break;
                }
            }
        }
    }
}
impl<'a> GridLines<'a> {
    pub fn new(
        painter: &'a Painter,
        rect: Rect,
        state: &'a mut GridLinesState,
    ) -> Self {
        Self {
            painter: painter,
            rect: rect,
            origin: Vec2::new(rect.center().x, rect.center().y),
            state: state,
        }
    }
    pub fn _origin(mut self, origin: Vec2) -> Self {
        self.origin = origin;
        self
    }
}
