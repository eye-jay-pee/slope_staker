use eframe::egui::{Color32, Pos2, Response, Shape, Stroke, Ui, Vec2, Widget};

pub struct AutoPlot {
    pts: Vec<Vec2>,
    area: Vec2,
}
impl AutoPlot {
    pub fn new(pts: Vec<Vec2>, area: Vec2) -> Self {
        Self { pts, area }
    }
}

impl Widget for AutoPlot {
    fn ui(self, ui: &mut Ui) -> Response {
        let (response, painter) =
            ui.allocate_painter(self.area, eframe::egui::Sense::hover());
        let rect = response.rect;

        // Fill background
        painter.rect_filled(rect, 0.0, Color32::DARK_GRAY);

        if self.pts.is_empty() {
            return response;
        }

        // Find bounding box of points
        let (min, max) = self.pts.iter().fold(
            (self.pts[0], self.pts[0]),
            |(min, max), v| {
                (
                    Vec2::new(min.x.min(v.x), min.y.min(v.y)),
                    Vec2::new(max.x.max(v.x), max.y.max(v.y)),
                )
            },
        );

        let size = max - min;
        let scale_x = rect.width() / size.x.max(1e-6);
        let scale_y = rect.height() / size.y.max(1e-6);
        let scale = scale_x.min(scale_y); // uniform scaling

        // map Vec2 -> Pos2 in screen coords
        let to_screen = |pt: Vec2| -> Pos2 {
            Pos2::new(
                rect.left() + (pt.x - min.x) * scale,
                rect.bottom() - (pt.y - min.y) * scale,
            )
        };

        let mut screen_pts: Vec<Pos2> =
            self.pts.into_iter().map(to_screen).collect();

        // scatter dots (optional)
        for &p in &screen_pts {
            painter.circle_filled(p, 2.0, Color32::WHITE);
        }

        // connecting polyline
        painter.add(Shape::line(
            screen_pts.drain(..).collect(),
            Stroke::new(1.5, Color32::LIGHT_GREEN),
        ));

        response
    }
}
