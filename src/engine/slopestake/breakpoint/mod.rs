pub mod elev;
pub use elev::{Elevation, ElevationEditor};
pub mod kind;
pub use kind::{BreakPointKind, BreakPointKindSelector};
pub mod offset;
pub use offset::{Offset, OffsetEditor};
pub mod slope;
pub use slope::{Slope, SlopeEditor};

pub mod ui;
pub use ui::BreakPointEditor;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BreakPoint {
    kind: BreakPointKind,
    pub slope: Slope,
    pub elev: Elevation,
    pub offset: Offset,
}

impl BreakPoint {
    pub fn new(kind: BreakPointKind) -> Self {
        Self {
            kind: kind,
            slope: Slope::default(),
            elev: Elevation::default(),
            offset: Offset::default(),
        }
    }

    pub fn _get_kind(&self) -> BreakPointKind {
        self.kind
    }
    pub fn crown() -> Self {
        Self::new(BreakPointKind::Crown)
    }
    pub fn limit() -> Self {
        Self::new(BreakPointKind::Limit)
    }
}

impl std::fmt::Display for BreakPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]@[{}]from CL & [{}], then [{}] slope",
            self.kind, self.offset, self.elev, self.slope
        )
    }
}
