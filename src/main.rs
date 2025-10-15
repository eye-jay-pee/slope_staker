mod foreign;
mod slopestake;

mod ui;
use ui::SlopeStakerApp;

#[cfg(target_arch = "wasm32")]
use eframe::m_bindgen::JsValue;
use eframe::Result;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result {
    SlopeStakerApp::new().launch_native()
}

#[cfg(target_arch = "wasm32")]
fn main() -> Result<(), JsValue> {
    SlopeStakerApp::new().launch_web()
}
