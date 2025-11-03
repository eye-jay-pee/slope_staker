use super::Slope;
use eframe::egui::emath::Numeric;

use eframe::egui::Vec2;

impl Numeric for Slope {
    const INTEGRAL: bool = false;
    const MIN: Self = Self::VERTICAL_WALL;
    const MAX: Self = Self::LEVEL_GROUND;

    fn to_f64(self) -> f64 {
        f64::from(self)
    }
    fn from_f64(v: f64) -> Self {
        Self::from(v)
    }
}

impl From<Slope> for Vec2 {
    fn from(slp: Slope) -> Vec2 {
        Vec2::new(1.0, -f32::from(slp))
    }
}
impl From<Vec2> for Slope {
    fn from(v: Vec2) -> Self {
        Self::from(v.x / -v.y)
    }
}
