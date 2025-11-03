use crate::road::{
    cross_section::slopestake::{
        SlopeStake, SlopeStakeEditor, SlopeStakeViewer,
    },
    Road, RoadEditor,
};

use eframe::{
    egui::{CentralPanel, Context},
    App, Frame,
};
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct SlopeStakerApp {
    ss: SlopeStake,
    road: Road,
}

impl SlopeStakerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        #[cfg(feature = "persistence")]
        if let Some(storage) = cc.storage {
            if let Some(app) =
                eframe::get_value::<Self>(storage, eframe::APP_KEY)
            {
                return app;
            }
        }
        SlopeStakerApp::default()
    }
}

impl App for SlopeStakerApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add(RoadEditor::new(&mut self.road));

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
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
