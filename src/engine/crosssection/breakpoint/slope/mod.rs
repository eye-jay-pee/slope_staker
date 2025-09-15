pub mod ui;
pub use ui::SlopeEditor;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Slope(Option<f64>);

impl Slope {
    pub fn read(&self) -> Option<f64> {
        self.0
    }
    pub fn write(&mut self, val: Option<f64>) {
        self.0 = val
    }
}
impl Default for Slope {
    fn default() -> Self {
        Self(None)
    }
}

impl From<f64> for Slope {
    fn from(v: f64) -> Self {
        if v.is_nan() {
            Self(None)
        } else {
            Self(Some(v))
        }
    }
}
impl From<Slope> for f64 {
    fn from(slp: Slope) -> f64 {
        slp.0.unwrap_or(f64::NAN)
    }
}
