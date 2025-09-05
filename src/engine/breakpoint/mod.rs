pub mod kind;
//impl Vertex {
//    pub fn new(x: f32, y: f32) -> Self {
//        Elevation()
//    }
//}
//
//pub mod ui {
//    use super::*;
//    use eframe::egui::{Response, Ui, Widget};
//
//    pub struct ElevationEditor<'a>(&'a mut Elevation);
//    impl<'a> ElevationEditor<'a> {
//        pub fn new(feet: &'a mut Elevation) -> Self {
//            Self(feet)
//        }
//    }
//    impl<'a> Widget for ElevationEditor<'a> {
//        fn ui(self, ui: &mut Ui) -> Response {
//            use eframe::egui::DragValue;
//
//            ui.horizontal(|ui| {
//                ui.label("Elevation:");
//                ui.add(
//                    DragValue::new(self.0).custom_formatter(|val, _| {
//                        Elevation::new(val).to_string()
//                    }),
//                )
//            })
//            .response
//        }
//    }
//}
//
//#[cfg(test)]
//pub mod test {
//    use super::*;
//
//    #[test]
//    fn sample_test() {}
//}
