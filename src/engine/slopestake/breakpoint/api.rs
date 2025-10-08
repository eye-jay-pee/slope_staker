use super::{BreakPointKind, Elevation, Offset, Slope};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BreakPoint {
    pub kind: BreakPointKind,
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
