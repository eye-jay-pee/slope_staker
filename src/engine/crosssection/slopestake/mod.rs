//#[derive(Debug, Clone, PartialEq)]
//pub struct SlopeStake {
//    //pts: Vec<BreakPoint>,
//    sta: Station,
//}
//
//impl SlopeStake {
//    //     pub fn add_at(&mut self, index: usize) {
//    //         let len = self.pts.len();
//    //         if len > 0 && len > index {
//    //             self.pts.insert(index, BreakPoint::default());
//    //         }
//    //     }
//    //     pub fn rm_at(&mut self, index: usize) {
//    //         if index > 0 && index < self.pts.len() {
//    //             self.pts.remove(index);
//    //         }
//    //     }
//    //     pub fn _get_pts(&self) -> Option<Vec<(f32, f32)>> {
//    //         //let pts: Vec<(f32, f32)> = Vec::new();
//    //
//    //         //let elev = self.pts[0].elev;
//    //
//    //         //for bp in self.pts {}
//    //
//    //}
//}
//impl Default for SlopeStake {
//    fn default() -> Self {
//        Self {
//            pts: vec![BreakPoint::default()],
//            sta: Station::default(),
//        }
//    }
//}
//
//impl std::fmt::Display for SlopeStake {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        for (i, p) in self.pts.iter().enumerate() {
//            writeln!(f, "{}: {}", i, p)?;
//        }
//        Ok(())
//    }
//}
//
//pub mod ui;
