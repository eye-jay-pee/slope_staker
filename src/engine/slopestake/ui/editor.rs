use super::{BreakPoint, BreakPointEditor, SlopeStake};
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
        let mut remove_me = None;
        let mut insert_at = None;

        let n = self.0.pts.len();
        let resp = ui
            .vertical(|ui| {
                for i in 0..n {
                    ui.horizontal(|ui| {
                        if ui
                            .add_enabled((1..n - 1).contains(&i), XButton())
                            .clicked()
                        {
                            remove_me = Some(i);
                        }
                        ui.add(BreakPointEditor::new(&mut self.0.pts[i]));
                        if ui
                            .add_enabled((0..n - 1).contains(&i), PlusButton())
                            .clicked()
                        {
                            insert_at = Some(i + 1);
                        }
                    });
                }
            })
            .response;

        if let Some(i) = remove_me {
            self.0.remove_point_at(i);
        }
        if let Some(i) = insert_at {
            self.0.add_point_at(BreakPoint::default(), i);
        }

        resp
    }
}
