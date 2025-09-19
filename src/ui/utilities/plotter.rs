//use eframe::egui::{vec2, Pos2, Response, Ui, Vec2, Widget};
use eframe::egui::{Response, Ui, Vec2, Widget};

pub struct Plot {
    pts: Vec<Vec2>,
    area: Vec2,
}
impl Plot {
    pub fn new(pts: Vec<Vec2>, area: Vec2) -> Self {
        Self { pts, area }
    }
}

impl Widget for Plot {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label("test")
    }
}

// pub struct Plot<T>
// where
//     I: IntoIterator<Item = (f32, f32)>,
// {
//     points: I,
//     size: Vec2,
// }
//
// impl<I> Viewer<I>
// where
//     I: IntoIterator<Item = (f32, f32)>,
// {
//     pub fn new(points: I, size: Vec2) -> Self {
//         Self { points, size }
//     }
// }
//
// impl<I> Widget for Viewer<I>
// where
//     I: IntoIterator<Item = (f32, f32)>,
// {
//     fn ui(self, ui: &mut Ui) -> Response {
//         let (response, painter) = ui
//             .allocate_painter(self.desired_size, eframe::egui::Sense::hover());
//         let rect = response.rect;
//
//         // fill background
//         painter.rect_filled(rect, 0.0, Color32::DARK_GRAY);
//
//         let center = rect.center();
//         painter.line_segment(
//             [pos2(rect.left(), center.y), pos2(rect.right(), center.y)],
//             Stroke::new(1.0, Color32::WHITE),
//         );
//         painter.line_segment(
//             [pos2(center.x, rect.top()), pos2(center.x, rect.bottom())],
//             Stroke::new(1.0, Color32::WHITE),
//         );
//
//
//         // Draw them
//         for p in points {
//             painter.circle_filled(p, 4.0, Color32::RED);
//         }
//
//         Connect them
//         for w in points.windows(2) {
//             painter.line_segment(
//                 [w[0], w[1]],
//                 Stroke::new(2.0, Color32::LIGHT_BLUE),
//             );
//         }
//
//         response
//     }
// }
