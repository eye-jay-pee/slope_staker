use super::*;
use eframe::egui::{Response, Ui, Widget};

pub struct CrossSectionEditor<'a>(&'a mut SlopeStake);
impl<'a> CrossSectionEditor<'a> {
    pub fn new(bp: &'a mut SlopeStake) -> Self {
        Self(bp)
    }
}
impl<'a> Widget for CrossSectionEditor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        use super::super::breakpoint::ui::BreakPointEditor;
        use crate::ui::utilities::{PlusButton, XButton};

        let mut for_slaughter: Option<usize> = None;
        let mut new_life: Option<usize> = None;
        let resp = ui
            .horizontal(|ui| {
                ui.vertical(|ui| {
                    for (i, mut bp) in self.0.pts.iter_mut().enumerate() {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.add_enabled_ui(i != 0, |ui| {
                                        if ui.add(XButton()).clicked() {
                                            for_slaughter = Some(i);
                                        }
                                    });
                                    if ui.add(PlusButton()).clicked() {
                                        new_life = Some(i);
                                    }
                                });
                                ui.add_space(4.0);
                                ui.add(BreakPointEditor::new(&mut bp));
                            });
                        });
                    }
                });
                ui.with_layout(
                    eframe::egui::Layout::left_to_right(
                        eframe::egui::Align::Min,
                    ),
                    |ui| {
                        ui.group(|ui| {
                            ui.add(CrossSectionVisualizer::new(self.0));
                        });
                    },
                );
            })
            .response;
        if let Some(i) = for_slaughter {
            self.0.rm_at(i);
        }
        if let Some(i) = new_life {
            self.0.add_at(i);
        }
        resp
    }
}

struct CrossSectionVisualizer<'a>(&'a SlopeStake);
impl<'a> CrossSectionVisualizer<'a> {
    pub fn new(bp: &'a SlopeStake) -> Self {
        Self(bp)
    }
}
impl<'a> Widget for CrossSectionVisualizer<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        use eframe::egui::{self, Color32, Pos2, Stroke, Vec2};

        // Reserve a 255x255 region
        let (rect, response) = ui
            .allocate_exact_size(Vec2::new(300.0, 300.0), egui::Sense::hover());

        let painter = ui.painter_at(rect);

        // White background
        painter.rect_filled(rect, 0.0, Color32::WHITE);

        // Hardcoded 5 vertices inside this rect
        let points = [
            Pos2::new(rect.left() + 20.0, rect.bottom() - 20.0),
            Pos2::new(rect.left() + 60.0, rect.top() + 80.0),
            Pos2::new(rect.left() + 120.0, rect.bottom() - 100.0),
            Pos2::new(rect.left() + 180.0, rect.top() + 40.0),
            Pos2::new(rect.right() - 20.0, rect.bottom() - 60.0),
        ];

        // Draw polyline
        painter.add(egui::Shape::line(
            points.to_vec(),
            Stroke::new(2.0, Color32::BLACK),
        ));

        response
    }
}
