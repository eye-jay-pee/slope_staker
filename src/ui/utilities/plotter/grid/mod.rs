use eframe::egui::{pos2, Color32, Painter, Pos2, Rect, Stroke, Vec2};

pub struct GridLinesStyle {
    origin: Option<Pos2>,
    freq: Vec2,
    stroke: Stroke,
}
pub trait PainterExt {
    fn grid_lines(&self, rect: Rect, style: GridLinesStyle);
}

impl PainterExt for Painter {
    fn grid_lines(&self, rect: Rect, style: GridLinesStyle) {
        style.hori_lines(self, rect);
        style.vert_lines(self, rect);
    }
}

/// public interface
impl GridLinesStyle {
    pub fn freq(mut self, freq: Vec2) -> Self {
        self.freq = freq;
        self
    }
    #[allow(dead_code)]
    /// if unset, rect.center() will be used
    pub fn origin(mut self, origin: Pos2) -> Self {
        self.origin = Some(origin);
        self
    }
    #[allow(dead_code)]
    pub fn stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = stroke;
        self
    }
}
impl Default for GridLinesStyle {
    fn default() -> Self {
        Self {
            origin: None,
            freq: Vec2::new(1.0, 1.0),
            stroke: Stroke::new(0.2, Color32::WHITE),
        }
    }
}

/// internal workings
impl GridLinesStyle {
    fn vert_line(&self, painter: &Painter, rect: Rect, x: f32) -> bool {
        if x >= rect.left() && x <= rect.right() {
            let top = pos2(x, rect.top());
            let btm = pos2(x, rect.bottom());
            let pts = [top, btm];
            painter.line_segment(pts, self.stroke);
            true
        } else {
            false
        }
    }
    fn vert_lines(&self, painter: &Painter, rect: Rect) {
        if self.freq.x > 0.0 {
            let origin_x = self.origin.unwrap_or(rect.center()).x;
            let mut x = 0.0;
            let mut in_bounds_left = true;
            let mut in_bounds_right = true;
            loop {
                in_bounds_left = in_bounds_left
                    && self.vert_line(painter, rect, origin_x - x);
                in_bounds_right = in_bounds_right
                    && self.vert_line(painter, rect, origin_x + x);

                if in_bounds_left || in_bounds_right {
                    x += self.freq.x;
                } else {
                    break;
                }
            }
        }
    }
    fn hori_line(&self, painter: &Painter, rect: Rect, y: f32) -> bool {
        if y >= rect.top() && y <= rect.bottom() {
            let left = pos2(rect.left(), y);
            let right = pos2(rect.right(), y);
            let pts = [left, right];
            painter.line_segment(pts, self.stroke);
            true
        } else {
            false
        }
    }
    fn hori_lines(&self, painter: &Painter, rect: Rect) {
        if self.freq.y > 0.0 {
            let origin_y = self.origin.unwrap_or(rect.center()).y;
            let mut y = 0.0;
            let mut in_bounds_top = true;
            let mut in_bounds_bottom = true;
            loop {
                in_bounds_bottom = in_bounds_bottom
                    && self.hori_line(painter, rect, origin_y - y);
                in_bounds_top = in_bounds_top
                    && self.hori_line(painter, rect, origin_y + y);

                if in_bounds_top || in_bounds_bottom {
                    y += self.freq.y;
                } else {
                    break;
                }
            }
        }
    }
}
