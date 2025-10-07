pub mod breakpoint;
pub use breakpoint::{BreakPoint, BreakPointEditor, CanPaintBreakPoint};
pub mod station;
#[allow(unused_imports)]
pub use station::{Station, StationEditor};

pub mod ui;
pub use ui::{SlopeStakeEditor, SlopeStakeViewer};

#[derive(Debug, Clone, PartialEq)]
pub struct SlopeStake {
    sta: Station,
    pts: Vec<BreakPoint>,
}

impl SlopeStake {
    pub fn remove_point_at(&mut self, index: usize) {
        self.pts.remove(index);
    }
    pub fn add_point_at(&mut self, point: BreakPoint, index: usize) {
        if (1..self.pts.len()).contains(&index) {
            self.pts.insert(index, point);
        } else {
            println!("invalid insertion index: {index}");
        }
    }
}

impl Default for SlopeStake {
    fn default() -> Self {
        let mut new_one = Self {
            pts: Vec::new(),
            sta: Station::default(),
        };
        new_one.pts.push(BreakPoint::crown());
        new_one.pts.push(BreakPoint::limit());

        new_one
    }
}

impl std::fmt::Display for SlopeStake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.sta)?;
        for pt in &self.pts {
            writeln!(f, "\t{}", pt)?;
        }
        Ok(())
    }
}
