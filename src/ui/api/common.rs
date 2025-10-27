use crate::road::{
    slopestake::{SlopeStake, SlopeStakeEditor, SlopeStakeViewer},
    Road,
};
use eframe::{
    egui::{CentralPanel, Context},
    App, Frame,
};

#[derive(Default)]
pub struct SlopeStakerApp {
    ss: SlopeStake,
    _road: Road,
}

impl SlopeStakerApp {
    pub fn new() -> Self {
        SlopeStakerApp::default()
    }
}

impl App for SlopeStakerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.add(SlopeStakeViewer::new(&self.ss));
                });
                ui.group(|ui| {
                    ui.add(SlopeStakeEditor::new(&mut self.ss));
                });
            });
        });
    }
}
