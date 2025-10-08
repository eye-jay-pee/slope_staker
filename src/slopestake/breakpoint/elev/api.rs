#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Elevation(f64);

impl From<f64> for Elevation {
    fn from(v: f64) -> Self {
        Self(v)
    }
}
impl From<Elevation> for f64 {
    fn from(el: Elevation) -> f64 {
        el.0
    }
}
impl From<f32> for Elevation {
    fn from(v: f32) -> Self {
        Self::from(v as f64)
    }
}
impl From<Elevation> for f32 {
    fn from(el: Elevation) -> f32 {
        f64::from(el) as f32
    }
}
impl std::fmt::Display for Elevation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_nan() {
            write!(f, "UNSET")
        } else {
            write!(f, "{:1.02}'", self.0)
        }
    }
}

#[allow(unused_imports)]
pub use constants::*;
#[allow(dead_code)]
pub mod constants {
    use super::Elevation;
    pub const MEAN_SEA_LEVEL: Elevation = Elevation(0.0);
    pub const MT_EVEREST: Elevation = Elevation(29032.0);
    pub const LINDSBORG_KS: Elevation = Elevation(1306.0);
    pub const DEFAULT_ELEV: Elevation = LINDSBORG_KS;
}
