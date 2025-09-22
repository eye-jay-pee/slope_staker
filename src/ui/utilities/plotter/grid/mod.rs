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
    fn vert_line(&self, x: f32) -> bool {
        if x >= self.rect.left() && x <= self.rect.right() {
            let top = pos2(x, self.rect.top());
            let btm = pos2(x, self.rect.bottom());
            let pts = [top, btm];
            self.painter.line_segment(pts, self.state.stroke);
            true
        } else {
            false
        }
    }
    fn vert_lines(&self) {
        if self.state.freq.x > 0.0 {
            let mut x = 0.0;
            let mut in_bounds_left = true;
            let mut in_bounds_right = true;
            loop {
                in_bounds_left =
                    in_bounds_left && self.vert_line(self.origin.x - x);
                in_bounds_right =
                    in_bounds_right && self.vert_line(self.origin.x + x);

                if in_bounds_left || in_bounds_right {
                    x += self.state.freq.x;
                } else {
                    break;
                }
            }
        }
    }
    fn hori_line(&self, y: f32) -> bool {
        if y >= self.rect.top() && y <= self.rect.bottom() {
            let left = pos2(self.rect.left(), y);
            let right = pos2(self.rect.right(), y);
            let pts = [left, right];
            self.painter.line_segment(pts, self.state.stroke);
            true
        } else {
            false
        }
    }
    fn hori_lines(&self) {
        if self.state.freq.y > 0.0 {
            let mut y = 0.0;
            let mut in_bounds_top = true;
            let mut in_bounds_bottom = true;
            loop {
                in_bounds_bottom =
                    in_bounds_bottom && self.hori_line(self.origin.y - y);
                in_bounds_top =
                    in_bounds_top && self.hori_line(self.origin.y + y);

                if in_bounds_top || in_bounds_bottom {
                    y += self.state.freq.y;
                } else {
                    break;
                }
            }
        }
    }
    fn cache_state(&self) {}
}
impl<'a> GridLines<'a> {
    pub fn draw(self) {
        self.hori_lines();
        self.vert_lines();
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
