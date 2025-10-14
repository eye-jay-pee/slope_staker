mod foreign;
mod slopestake;
mod ui;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    ui::SlopeStakerApp::new().launch_native()
}

#[cfg(target_arch = "wasm32")]
fn main() -> Result<(), eframe::wasm_bindgen::JsValue> {
    ui::SlopeStakerApp::new().launch_web()
}
