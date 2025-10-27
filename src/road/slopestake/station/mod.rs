pub mod ui;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
pub use ui::StationEditor;

#[derive(
    Copy, Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize, Default,
)]
pub struct Station {
    value: f32,
}
impl std::fmt::Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}+{:05.2}",
            self.get_station_num(),
            self.get_station_plus().abs()
        )
    }
}
impl Station {
    pub fn _new(station_number: i32, station_plus: f32) -> Self {
        Station {
            value: (station_number * 100) as f32 + station_plus,
        }
    }
    pub fn get_station_num(&self) -> i32 {
        (self.value / 100.0).floor() as i32
    }
    pub fn get_station_plus(&self) -> f32 {
        self.value % 100.0
    }
    pub fn _set_station_num(&mut self, new_val: i32) -> () {
        self.value = self.get_station_plus() + 100.0 * new_val as f32;
    }
    pub fn _set_station_plus(&mut self, new_val: f32) -> () {
        self.value = self.get_station_num() as f32 + (new_val % 100.0)
    }
    #[allow(dead_code)]
    fn get_set_raw_sta(&mut self, new_val: Option<f64>) -> f64 {
        match new_val {
            Some(v) => self.value = v as f32,
            None => {}
        };
        self.value as f64
    }
}
impl From<f64> for Station {
    fn from(val: f64) -> Self {
        Self { value: val as f32 }
    }
}
impl From<f32> for Station {
    fn from(val: f32) -> Self {
        Self { value: val }
    }
}
impl Eq for Station {}

impl Hash for Station {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.value.is_nan() {
            panic!("NaN in Station::hash");
        }
        // hash the bit pattern to distinguish -0.0 and +0.0
        self.value.to_bits().hash(state);
    }
}
