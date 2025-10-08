mod foreign;
mod slopestake;
mod ui;

fn main() -> eframe::Result {
    ui::SlopeStakerApp::new().launch()
}
