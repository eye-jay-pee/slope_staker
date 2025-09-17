pub mod breakpoint;
pub use breakpoint::{BreakPoint, BreakPointEditor};
pub mod station;
#[allow(unused_imports)]
pub use station::{Station, StationEditor};

pub mod ui;
pub use ui::SlopeStakeEditor;

#[derive(Debug, Clone, PartialEq)]
pub struct SlopeStake {
    _station: Station,
    points: Vec<BreakPoint>,
}

impl SlopeStake {
    pub fn remove_point_at(&mut self, index: usize) {
        self.points.remove(index);
    }
    pub fn add_point_at(&mut self, point: BreakPoint, index: usize) {
        if (1..self.points.len()).contains(&index) {
            self.points.insert(index, point);
        } else {
            println!("invalid insertion index: {index}");
        }
    }
}

impl Default for SlopeStake {
    fn default() -> Self {
        let mut new_one = Self {
            points: Vec::new(),
            _station: Station::default(),
        };
        new_one.points.push(BreakPoint::crown());
        new_one.points.push(BreakPoint::limit());

        new_one
    }
}
