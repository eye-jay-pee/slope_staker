fn main() -> eframe::Result {
    ui::SlopeStakerApp::new().launch()
}
mod engine;
mod ui;
mod utilities;
