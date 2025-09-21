pub mod utilities;
use crate::engine::slopestake::{
    SlopeStake, SlopeStakeEditor, SlopeStakeViewer,
};
use eframe::egui;

#[derive(Default)]
pub struct SlopeStakerApp {
    _file: Option<std::path::PathBuf>,
    ss: SlopeStake,
}

impl SlopeStakerApp {
    pub fn new() -> Self {
        SlopeStakerApp::default()
    }
    pub fn launch(&self) -> eframe::Result {
        use eframe::AppCreator;

        let title = "slope-staker";
        let options = eframe::NativeOptions::default();
        let creator: AppCreator =
            Box::new(|_cc| Ok(Box::new(SlopeStakerApp::default())));

        eframe::run_native(title, options, creator)
    }
}

impl eframe::App for SlopeStakerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.group(|ui| {
                ui.add(SlopeStakeViewer::new(&self.ss));
            });
            ui.group(|ui| {
                ui.add(SlopeStakeEditor::new(&mut self.ss));
            });
        });
    }
}
