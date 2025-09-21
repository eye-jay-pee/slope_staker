pub mod simple;
pub use simple::SimplePlot;

pub mod auto;
pub use auto::AutoPlot;

mod gridlines;
use gridlines::GridLines;
