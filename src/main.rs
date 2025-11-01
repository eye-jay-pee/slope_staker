mod foreign;
mod road;

mod ui;
use ui::SlopeStakerApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    eframe::run_native(
        "slope staker",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(SlopeStakerApp::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() -> Result<(), eframe::wasm_bindgen::JsValue> {
    SlopeStakerApp::new().launch_web()
}
