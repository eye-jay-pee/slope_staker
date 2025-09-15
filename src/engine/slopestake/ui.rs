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
        ui.vertical(|ui| {
            ui.add(StationEditor::new(&mut self.0.station));
            ui.group(|ui| {
                ui.vertical(|ui| {
                    for mut bp in self.0.points.iter_mut() {
                        ui.horizontal(|ui| {
                            ui.add(XButton::default());
                            ui.add(BreakPointEditor::new(&mut bp));
                            ui.add(PlusButton::default());
                        });
                    }
                });
            });
        })
        .response
    }
}
