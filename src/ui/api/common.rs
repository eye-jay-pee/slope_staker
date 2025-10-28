use crate::road::{
    slopestake::{SlopeStake, SlopeStakeEditor, SlopeStakeViewer},
    Road,
};
use eframe::{
    egui::{CentralPanel, Context},
    App, Frame,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
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
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if let Ok(json) = serde_json::to_string(self) {
            storage.set_string("my_app_state", json);
        }
    }
}
