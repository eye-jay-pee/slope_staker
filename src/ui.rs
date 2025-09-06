use crate::engine::crosssection::slopestake::SlopeStake;
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
        use crate::engine::crosssection::slopestake::ui::CrossSectionEditor;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.group(|ui| {
                ui.add(CrossSectionEditor::new(&mut self.ss));
            });

            ui.group(|ui| {
                let text = format!("{}", self.ss);
                ui.add(
                    eframe::egui::TextEdit::multiline(
                        &mut text.to_string().as_str(),
                    )
                    .interactive(false) // disables editing
                    .desired_rows(3) // controls visible height
                    .desired_width(400.0),
                );
            });
        });
    }
}

pub mod utilities {
    use eframe::egui::{self, Button, Color32};
    use eframe::egui::{Response, Ui, Widget};

    #[derive(Debug, Default)]
    pub struct PlusButton();
    impl Widget for PlusButton {
        fn ui(self, ui: &mut Ui) -> Response {
            ui.add(
                Button::new(egui::RichText::new("+").color(Color32::BLACK))
                    .fill(Color32::GREEN),
            )
        }
    }
    #[derive(Debug, Default)]
    pub struct XButton();
    impl Widget for XButton {
        fn ui(self, ui: &mut Ui) -> Response {
            ui.add(
                Button::new(egui::RichText::new("X").color(Color32::WHITE))
                    .fill(Color32::RED),
            )
        }
    }
}
