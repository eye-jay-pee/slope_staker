mod engine;
mod foreign;
mod ui;

fn main() -> eframe::Result {
    ui::SlopeStakerApp::new().launch()
}
