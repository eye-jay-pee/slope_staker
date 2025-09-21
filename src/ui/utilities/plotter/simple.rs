use super::GridLines;
use eframe::egui::{vec2, Color32, Response, Sense, Stroke, Ui, Vec2, Widget};

#[derive(Debug, Default, Clone)]
pub struct SimplePlot {
    _pts: Vec<Vec2>,
    area: Vec2,
    _trim_min: Vec2,
    _trim_max: Vec2,
    _scale_factor: Vec2,
}
impl SimplePlot {
    pub fn area(mut self, new: Vec2) -> Self {
        self.area = new;
        self
    }
    pub fn _trim_min(mut self, new: Vec2) -> Self {
        self._trim_min = new;
        self
    }
    pub fn _trim_max(mut self, new: Vec2) -> Self {
        self._trim_max = new;
        self
    }
    pub fn _scale_factor(mut self, new: Vec2) -> Self {
        self._scale_factor = new;
        self
    }
    pub fn _add_pt(mut self, new: Vec2) -> Self {
        self._pts.push(new);
        self
    }
}

impl Widget for SimplePlot {
    fn ui(self, ui: &mut Ui) -> Response {
        let (response, painter) =
            ui.allocate_painter(self.area, Sense::hover());
        let rect = response.rect;

        // Fill background
        painter.rect_filled(rect, 0.0, Color32::DARK_GRAY);

        GridLines::new(&painter, rect)
            .stroke(Stroke::new(0.5, Color32::WHITE))
            .freq(vec2(5.0, 5.0))
            .draw();
        response

        // if !self.pts.is_empty() {
        //     // Find bounding box of points
        //     let (min, max) = self.pts.iter().fold(
        //         (self.pts[0], self.pts[0]),
        //         |(min, max), v| {
        //             (
        //                 Vec2::new(min.x.min(v.x), min.y.min(v.y)),
        //                 Vec2::new(max.x.max(v.x), max.y.max(v.y)),
        //             )
        //         },
        //     );

        //     let size = max - min;
        //     let scale_x = rect.width() / size.x.max(1e-6);
        //     let scale_y = rect.height() / size.y.max(1e-6);
        //     let scale = scale_x.min(scale_y); // uniform scaling

        //     // map Vec2 -> Pos2 in screen coords
        //     let to_screen = |pt: Vec2| -> Pos2 {
        //         Pos2::new(
        //             rect.left() + (pt.x - min.x) * scale,
        //             rect.bottom() - (pt.y - min.y) * scale,
        //         )
        //     };

        //     let mut screen_pts: Vec<Pos2> =
        //         self.pts.into_iter().map(to_screen).collect();

        //     // scatter dots (optional)
        //     for &p in &screen_pts {
        //         painter.circle_filled(p, 2.0, Color32::WHITE);
        //     }

        //     // connecting polyline
        //     painter.add(Shape::line(
        //         screen_pts.drain(..).collect(),
        //         Stroke::new(1.5, Color32::LIGHT_GREEN),
        //     ));
        // }

        // response
    }
}
