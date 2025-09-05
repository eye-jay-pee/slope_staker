use crate::engine::{
    breakpoint::{kind::BreakPointKind, slope::Slope, BreakPoint},
    elevation::Elevation,
    station::Station,
};
use eframe::egui;

#[derive(Default)]
pub struct SlopeStakerApp {
    _file: Option<std::path::PathBuf>,

    sta: Station,
    elev: Elevation,
    bpk: BreakPointKind,
    slope: Slope,
    bp: BreakPoint,
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
        use crate::engine::breakpoint::kind::ui::BreakPointKindSelector;
        use crate::engine::breakpoint::slope::ui::SlopeEditor;
        use crate::engine::breakpoint::ui::BreakPointEditor;
        use crate::engine::elevation::ui::ElevationEditor;
        use crate::engine::station::ui::StationEditor;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(StationEditor::new(&mut self.sta));
            ui.add(ElevationEditor::new(&mut self.elev));
            ui.add(BreakPointKindSelector::new(&mut self.bpk));
            ui.add(SlopeEditor::new(&mut self.slope));
            ui.add(BreakPointEditor::new(&mut self.bp))
        });
    }
}
