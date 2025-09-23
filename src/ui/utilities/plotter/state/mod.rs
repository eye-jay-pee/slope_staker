pub mod ui;
//pub use ui::PlotterStateEditor;

use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Default, Clone)]
pub struct PlotterState {
    lines: Option<GridLinesState>,
    _user_control: bool,
}
impl PlotterState {
    pub fn _toggle_user_control(&mut self) {
        self._user_control = !self._user_control;
    }
}
impl Display for PlotterState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Some(gls) = self.lines {
            writeln!(f, "plot with lines: {}", gls)
        } else {
            writeln!(f, "plot without lines")
        }
    }
}
