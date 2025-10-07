/// traits defined elsewhere are implemented on Breakpoint here
use super::BreakPoint;
use eframe::egui::Vec2;
impl From<BreakPoint> for Vec2 {
    fn from(bp: BreakPoint) -> Self {
        Vec2::new(f32::from(bp.offset), -f32::from(bp.elev))
    }
}
impl From<&BreakPoint> for Vec2 {
    fn from(bp: &BreakPoint) -> Self {
        Vec2::new(f32::from(bp.offset), -f32::from(bp.elev))
    }
}
