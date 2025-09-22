use std::fmt;

pub trait DisplayExt {
    fn fmt_ext(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    fn to_display_string(&self) -> String
    where
        Self: Sized,
    {
        format!("{}", FmtHelper(self))
    }
}

struct FmtHelper<'a>(&'a dyn DisplayExt);

impl<'a> fmt::Display for FmtHelper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt_ext(f)
    }
}
