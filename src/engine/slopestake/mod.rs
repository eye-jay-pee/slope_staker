pub mod breakpoint;
pub use breakpoint::{BreakPoint, BreakPointEditor, BreakPointKind};
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
    fn _is_valid(&self) -> bool {
        let len = self.points.len();
        if len >= 2 {
            if self.points[0]._get_kind() == BreakPointKind::Crown {
                if self.points[len - 1]._get_kind() == BreakPointKind::Limit {
                    return true;
                }
            }
        }
        false
    }
}

impl Default for SlopeStake {
    fn default() -> Self {
        Self {
            points: Vec::from([BreakPoint::crown(), BreakPoint::limit()]),
            _station: Station::default(),
        }
    }
}
