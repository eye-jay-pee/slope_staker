pub mod simple;
pub use simple::SimplePlot;

pub mod auto;
pub use auto::AutoPlot;

pub mod state;
//pub use state::PlotterState;

mod grid;
use grid::{GridLines, GridLinesState};
