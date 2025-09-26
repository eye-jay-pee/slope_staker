use crate::foreign::DisplayExt;
use eframe::egui::{pos2, Painter, Rect};
use eframe::egui::{Color32, Stroke, Vec2};
use std::fmt::{Display, Formatter, Result};

pub struct GridLinesStyle {
    origin: Option<Vec2>, // defaults to rect.center() if none given
    freq: Vec2,
    stroke: Stroke,
}
impl GridLinesStyle {
    pub fn set_freq(mut self, x: f32, y: f32) -> Self {
        self.freq = Vec2::new(x, y);
        self
    }
    #[allow(dead_code)]
    pub fn get_freq(&self) -> Vec2 {
        self.freq
    }
    #[allow(dead_code)]
    pub fn set_origin(mut self, origin: Vec2) -> Self {
        self.origin = Some(origin);
        self
    }
    #[allow(dead_code)]
    pub fn get_origin(&self) -> Option<Vec2> {
        self.origin
    }
    #[allow(dead_code)]
    pub fn set_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = stroke; // oh yeah? what was his name?
        self
    }
    #[allow(dead_code)]
    pub fn get_stroke(&self) -> Stroke {
        self.stroke
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
impl Display for GridLinesStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let origin_txt = match self.origin {
            Some(v2) => String::from(format!("({},{})", v2.x, v2.y)),
            None => String::from("unset"),
        };
        write!(
            f,
            "stroke:{} freq:{} origin offset:{}",
            self.stroke.to_display_string(),
            self.freq,
            origin_txt
        )
    }
}
impl GridLinesStyle {
    pub fn origin_x(&self, rect: Rect) -> f32 {
        match self.origin {
            Some(v) => rect.left() + v.x,
            None => rect.center().x,
        }
    }
    pub fn origin_y(&self, rect: Rect) -> f32 {
        match self.origin {
            Some(v) => rect.bottom() - v.y,
            None => rect.center().y,
        }
    }
    pub fn vert_line(&self, painter: &Painter, rect: Rect, x: f32) -> bool {
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
    pub fn vert_lines(&self, painter: &Painter, rect: Rect) {
        if self.freq.x > 0.0 {
            let origin_x = self.origin_x(rect);
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
    pub fn hori_line(&self, painter: &Painter, rect: Rect, y: f32) -> bool {
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
    pub fn hori_lines(&self, painter: &Painter, rect: Rect) {
        if self.freq.y > 0.0 {
            let origin_y = self.origin_y(rect);
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
