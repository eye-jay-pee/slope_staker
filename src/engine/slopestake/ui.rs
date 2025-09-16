use super::*;
use crate::ui::utilities::{PlusButton, XButton};
use eframe::egui::{Response, Ui, Widget};

pub struct SlopeStakeEditor<'a>(&'a mut SlopeStake);

impl<'a> SlopeStakeEditor<'a> {
    pub fn new(ss: &'a mut SlopeStake) -> Self {
        Self(ss)
    }
}

impl<'a> Widget for SlopeStakeEditor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let n = self.0.points.len();
        ui.vertical(|ui| {
            for i in 0..n {
                ui.horizontal(|ui| {
                    if ui.add_enabled(i != 0, XButton::default()).clicked() {
                        println!("remove {}", i);
                    }
                    ui.add(BreakPointEditor::new(&mut self.0.points[i]));
                    if ui
                        .add_enabled(i != n - 1, PlusButton::default())
                        .clicked()
                    {
                        println!("add after {}", i);
                    }
                });
            }
        })
        .response
    }
}
