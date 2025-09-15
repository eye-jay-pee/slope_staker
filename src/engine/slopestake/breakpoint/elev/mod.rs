pub mod constants;
pub mod ui;
pub use ui::ElevationEditor;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Elevation(Option<f64>);

impl Elevation {
    pub fn read(&self) -> Option<f64> {
        self.0
    }
    pub fn write(&mut self, val: Option<f64>) {
        self.0 = val
    }
}
impl From<f64> for Elevation {
    fn from(v: f64) -> Self {
        if v.is_nan() {
            Self(None)
        } else {
            Self(Some(v))
        }
    }
}
impl From<Elevation> for f64 {
    fn from(os: Elevation) -> f64 {
        os.0.unwrap_or(f64::NAN)
    }
}
