use std::ops::Sub;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Offset(f64);

impl std::fmt::Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_nan() {
            write!(f, "UNSET")
        } else {
            write!(f, "{:1.02}'", self.0)
        }
    }
}
impl From<f64> for Offset {
    fn from(v: f64) -> Self {
        Self(v)
    }
}
impl From<Offset> for f64 {
    fn from(os: Offset) -> f64 {
        os.0
    }
}
impl From<f32> for Offset {
    fn from(v: f32) -> Self {
        Self::from(v as f64)
    }
}
impl From<Offset> for f32 {
    fn from(os: Offset) -> f32 {
        f64::from(os) as f32
    }
}
impl Sub for Offset {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

#[allow(unused_imports)]
pub use constants::*;
pub mod constants {
    use super::Offset;
    pub const ONE_MILE: Offset = Offset(5280.0);
    pub const ONE_MILE_BACKWARDS: Offset = Offset(-5280.0);
}
