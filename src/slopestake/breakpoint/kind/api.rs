//use super::BreakPoint;
use strum_macros::EnumIter;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum BreakPointKind {
    Crown,
    Ditch,
    #[default]
    Unspecified,
    Berm,
    Limit,
    Step,
    DoesNotDaylight,
    Shoulder,
}
mod display {
    use super::BreakPointKind;
    use std::fmt::{Display, Formatter, Result};
    impl Display for BreakPointKind {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_str(match self {
                BreakPointKind::Unspecified => "breakpoint",
                BreakPointKind::Ditch => "ditch",
                BreakPointKind::Berm => "berm",
                BreakPointKind::DoesNotDaylight => "does not daylight",
                BreakPointKind::Limit => "limit",
                BreakPointKind::Shoulder => "shoulder",
                BreakPointKind::Crown => "crown",
                BreakPointKind::Step => "step",
            })
        }
    }
}
