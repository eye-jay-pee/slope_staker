use super::*;
use crate::ui::utilities::{PlusButton, XButton};
use eframe::egui::{Response, Ui, Widget};

pub struct SlopeStakeEditor<'a>(&'a mut SlopeStake);

impl<'a> SlopeStakeEditor<'a> {
    pub fn new(ss: &'a mut SlopeStake) -> Self {
        Self(ss)
    }
    fn find_removeable(&self) -> Vec<bool> {
        Vec::from([false, true])
    }
    fn find_insertable(&self) -> Vec<bool> {
        Vec::from([true, false])
    }
}

impl<'a> Widget for SlopeStakeEditor<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let removable = self.find_removeable();
        let insertable = self.find_insertable();

        ui.vertical(|ui| {
            for (i, mut bp) in self.0.points.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    if ui
                        .add_enabled(removable[i], XButton::default())
                        .clicked()
                    {
                        println!("remove {}", i);
                    }
                    ui.add(BreakPointEditor::new(&mut bp));
                    if ui
                        .add_enabled(insertable[i], PlusButton::default())
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
