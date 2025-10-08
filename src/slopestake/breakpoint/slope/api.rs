#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Slope(f64);

impl Slope {
    pub const VERTICAL_WALL: Slope = Slope(0.0);
    pub const LEVEL_GROUND: Slope = Slope(f64::INFINITY);
}

impl Default for Slope {
    fn default() -> Self {
        Self(f64::INFINITY)
    }
}
impl From<f64> for Slope {
    fn from(v: f64) -> Self {
        // direction of slope is dependent on point type, not sign of slope
        Self(v.abs())
    }
}
impl From<Slope> for f64 {
    fn from(slp: Slope) -> f64 {
        slp.0
    }
}
impl From<f32> for Slope {
    fn from(v: f32) -> Self {
        Self::from(v as f64)
    }
}
impl From<Slope> for f32 {
    fn from(slp: Slope) -> f32 {
        f64::from(slp) as f32
    }
}
impl std::fmt::Display for Slope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_nan() {
            write!(f, "{:1.02}:1", self.0)
        } else {
            write!(f, "____:1")
        }
    }
}
