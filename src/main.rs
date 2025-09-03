fn main() -> eframe::Result {
    ui::SlopeStakerApp::new().launch()
}

mod engine {
    pub mod station {
        #[derive(Default)]
        pub struct Station {
            value: f32, // TODO implement as fixed-point value
        }
        impl std::fmt::Display for Station {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}+{:05.2}",
                    self.whole_station(),
                    self.station_plus()
                )
            }
        }
        impl Station {
            pub fn whole_station(&self) -> i32 {
                (self.value / 100.0).floor() as i32
            }
            pub fn station_plus(&self) -> f32 {
                self.value % 100.0
            }
        }
    }
}

mod ui {
    use crate::engine::station::Station;
    use eframe::egui;

    #[derive(Default)]
    pub struct SlopeStakerApp {
        _file: Option<std::path::PathBuf>,

        _sta: Station,
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
                ui.heading(format!("station: {}", self._sta));
            });
        }
    }
}
