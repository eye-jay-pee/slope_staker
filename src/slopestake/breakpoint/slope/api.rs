#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Slope(f64);

impl Slope {
    pub const VERTICAL_WALL: Slope = Slope(f64::INFINITY);
    pub const LEVEL_GROUND: Slope = Slope(0.0);
}

impl Default for Slope {
    fn default() -> Self {
        Self::LEVEL_GROUND
    }
}
impl From<f64> for Slope {
    fn from(v: f64) -> Self {
        Self(v)
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
mod display {
    use super::Slope;
    use std::fmt::{Display, Formatter, Result};
    impl Display for Slope {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            if self.0.abs() > 0.01 {
                write!(f, "{:1.01}:1", 1.0 / self.0)
            } else {
                write!(f, "{:1.03}", self.0)
            }
        }
    }
}
