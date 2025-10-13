pub mod utilities;

use crate::slopestake::{SlopeStake, SlopeStakeEditor, SlopeStakeViewer};
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
    #[cfg(target_arch = "wasm32")]
        pub fn launch_web(self) -> Self{
            
        use eframe::wasm_bindgen::JsCast as _;

        // Redirect `log` message to `console.log` and friends:
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();

        let web_options = eframe::WebOptions::default();

        wasm_bindgen_futures::spawn_local(async {
            let document = web_sys::window()
                .expect("No window")
                .document()
                .expect("No document");

            let canvas = document
                .get_element_by_id("the_canvas_id")
                .expect("Failed to find the_canvas_id")
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .expect("the_canvas_id was not a HtmlCanvasElement");

            
            let _start_result = eframe::WebRunner::new()
                .start(
                    canvas,
                    web_options,
                    Box::new(|_cc| Ok(Box::new(SlopeStakerApp::default()))),
                )
                .await;

        });
        self
    }
#[cfg(not(target_arch = "wasm32"))]
    pub fn launch_native(&self) -> eframe::Result {
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
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.add(SlopeStakeEditor::new(&mut self.ss));
                });
                ui.group(|ui| {
                    ui.add(SlopeStakeViewer::new(&self.ss));
                });
            });
        });
    }
}
