use std::fmt::{Display, Formatter, Result};

/// allows for implementing display on forign structs
pub trait DisplayExt {
    fn fmt_ext(&self, f: &mut Formatter<'_>) -> Result;

    fn to_display_string(&self) -> String
    where
        Self: Sized,
    {
        format!("{}", FmtHelper(self))
    }
}

struct FmtHelper<'a>(&'a dyn DisplayExt);
impl<'a> Display for FmtHelper<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.0.fmt_ext(f)
    }
}
