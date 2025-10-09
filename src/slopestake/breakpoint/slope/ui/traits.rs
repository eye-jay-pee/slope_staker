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
#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui::Vec2;

    #[test]
    fn slope_numeric_roundtrip() {
        let s = Slope::from(2.0f64);
        let f = s.to_f64();
        let s2 = Slope::from_f64(f);
        assert_eq!(s, s2);
    }

    #[test]
    fn slope_vec2_roundtrip() {
        let s = Slope::from(1.0f64);
        let v = Vec2::from(s);
        let s2 = Slope::from(v);
        // allow some tolerance if floating point
        assert!((f32::from(s2) - f32::from(s)).abs() < 1e-6);
    }

    #[test]
    fn numeric_constants() {
        assert!(f64::from(Slope::MIN) <= f64::from(Slope::MAX));
        assert!(!Slope::INTEGRAL);
    }

    #[test]
    fn nan_to_f64_returns_zero() {
        let nan_slope = Slope::from(f64::NAN);
        assert_eq!(nan_slope.to_f64(), 0.0);
    }
}
