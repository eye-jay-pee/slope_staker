pub mod ui;
pub use ui::BreakPointKindSelector;

use strum::IntoEnumIterator;
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
impl std::fmt::Display for BreakPointKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BreakPointKind::*;
        f.write_str(match self {
            Unspecified => "breakpoint",
            Ditch => "ditch",
            Berm => "berm",
            DoesNotDaylight => "does not daylight",
            Limit => "limit",
            Shoulder => "shoulder",
            Crown => "crown",
            Step => "step",
        })
    }
}
