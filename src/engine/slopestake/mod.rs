pub mod breakpoint;
pub use breakpoint::{BreakPoint, BreakPointEditor, BreakPointKind};
pub mod station;
pub use station::{Station, StationEditor};

pub mod ui;
pub use ui::SlopeStakeEditor;

#[derive(Debug, Clone, PartialEq)]
pub struct SlopeStake {
    points: Vec<BreakPoint>,
    station: Station,
}
impl Default for SlopeStake {
    fn default() -> Self {
        let mut new_ss = Self {
            points: Vec::new(),
            station: Station::default(),
        };
        new_ss
            .points
            .push(BreakPoint::new(BreakPointKind::Centerline));
        new_ss.points.push(BreakPoint::new(BreakPointKind::Limit));
        new_ss
    }
}
