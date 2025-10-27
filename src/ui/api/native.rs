use super::SlopeStakerApp;
use eframe::Result;

impl SlopeStakerApp {
    pub fn launch_native(&self) -> Result {
        use eframe::AppCreator;

        let title = "slope-staker";
        let options = eframe::NativeOptions::default();
        let creator: AppCreator =
            Box::new(|_cc| Ok(Box::new(SlopeStakerApp::default())));

        eframe::run_native(title, options, creator)
    }
}
