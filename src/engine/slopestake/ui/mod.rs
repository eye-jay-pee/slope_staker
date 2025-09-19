use super::{BreakPoint, BreakPointEditor, SlopeStake};

pub mod editor;
pub use editor::SlopeStakeEditor;

pub mod viewer;
pub use viewer::SlopeStakeViewer;

pub mod utilities {
    use super::BreakPoint;
    use eframe::egui::Vec2;

    impl From<BreakPoint> for Vec2 {
        fn from(bp: BreakPoint) -> Vec2 {
            Vec2::new(
                bp.offset.read().unwrap_or(0.0) as f32,
                bp.elev.read().unwrap_or(0.0) as f32,
            )
        }
    }
}
