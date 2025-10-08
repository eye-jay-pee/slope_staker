use super::Slope;
use eframe::egui::emath::Numeric;

use eframe::egui::Vec2;

impl Numeric for Slope {
    const INTEGRAL: bool = false;
    const MIN: Self = Self::VERTICAL_WALL;
    const MAX: Self = Self::LEVEL_GROUND;

    fn to_f64(self) -> f64 {
        // an unset slope is not the same as a slope of 0 (which would be
        // a wall, but dragvalue can only handle numeric values (not NaN),
        // so just give it 0.  This will not be used for anything except a
        // reference point if user initally sets the value by dragging
        // rather than typing in the dragvalue.
        let val = f64::from(self);
        if val.is_nan() {
            0.0
        } else {
            val
        }
    }
    fn from_f64(v: f64) -> Self {
        Self::from(v)
    }
}

impl From<Slope> for Vec2 {
    fn from(slp: Slope) -> Vec2 {
        Vec2::new(f32::from(slp), 1.0)
    }
}
impl From<Vec2> for Slope {
    fn from(v: Vec2) -> Self {
        Self::from(v.x / -v.y)
    }
}
