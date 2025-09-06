use super::breakpoint::BreakPoint;
use super::station::Station;

#[derive(Debug, Clone, PartialEq)]
pub struct CrossSection {
    pts: Vec<BreakPoint>,
    sta: Station,
}

impl CrossSection {
    pub fn add_at(&mut self, index: usize) {
        let len = self.pts.len();
        if len > 0 && len > index {
            self.pts.insert(index, BreakPoint::default());
        }
    }
    pub fn rm_at(&mut self, index: usize) {
        if index > 0 && index < self.pts.len() {
            self.pts.remove(index);
        }
    }
}
impl Default for CrossSection {
    fn default() -> Self {
        Self {
            pts: vec![BreakPoint::default()],
            sta: Station::default(),
        }
    }
}

impl std::fmt::Display for CrossSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, p) in self.pts.iter().enumerate() {
            writeln!(f, "{}: {}", i, p)?;
        }
        Ok(())
    }
}

pub mod ui {
    use super::*;
    use eframe::egui::{Response, Ui, Widget};

    pub struct CrossSectionEditor<'a>(&'a mut CrossSection);
    impl<'a> CrossSectionEditor<'a> {
        pub fn new(bp: &'a mut CrossSection) -> Self {
            Self(bp)
        }
    }
    impl<'a> Widget for CrossSectionEditor<'a> {
        fn ui(self, ui: &mut Ui) -> Response {
            use super::super::breakpoint::ui::BreakPointEditor;
            use super::super::station::ui::StationEditor;
            use crate::ui::utilities::{AddButton, CloseButton};

            ui.vertical(|ui| {
                ui.add(StationEditor::new(&mut self.0.sta));

                let mut for_slaughter: Option<usize> = None;
                let mut new_life: Option<usize> = None;
                for (i, mut bp) in self.0.pts.iter_mut().enumerate() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.add_enabled_ui(i != 0, |ui| {
                                    if ui.add(CloseButton()).clicked() {
                                        for_slaughter = Some(i);
                                    }
                                });
                                if ui.add(AddButton()).clicked() {
                                    new_life = Some(i);
                                }
                            });
                            ui.add_space(4.0);
                            ui.add(BreakPointEditor::new(&mut bp));
                        });
                    });
                }
                if let Some(i) = for_slaughter {
                    self.0.rm_at(i);
                }
                if let Some(i) = new_life {
                    self.0.add_at(i);
                }
            })
            .response
        }
    }
}
