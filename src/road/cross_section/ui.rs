use super::{CrossSection, SlopeStake, SlopeStakeEditor};
use eframe::egui::{Response, Ui, Widget};

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct CrossSectionEditor<'a> {
    cross_section: &'a mut CrossSection,
    side: Side,
}

impl<'a> CrossSectionEditor<'a> {
    pub fn new(cs: &'a mut CrossSection) -> Self {
        Self {
            cross_section: cs,
            side: Side::Left,
        }
    }
}

impl<'a> Widget for CrossSectionEditor<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.side, Side::Left, "Left");
                ui.radio_value(&mut self.side, Side::Right, "Right");
            });
            let side = match self.side {
                Side::Left => &mut self.cross_section.left.clone(),
                Side::Right => &mut self.cross_section.right.clone(),
            };
            if side.is_none() {
                *side = Some(SlopeStake::default());
            }
            ui.add(SlopeStakeEditor::new(&mut side.as_mut().unwrap()))
        })
        .response
    }
}
