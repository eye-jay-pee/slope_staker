mod foreign;
mod slopestake;

mod ui;
use ui::SlopeStakerApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    SlopeStakerApp::new().launch_native()
}

#[cfg(target_arch = "wasm32")]
fn main() -> Result<(), eframe::wasm_bindgen::JsValue> {
    SlopeStakerApp::new().launch_web()
}
