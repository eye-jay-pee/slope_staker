pub mod common;
pub use common::SlopeStakerApp;

#[cfg(target_arch = "wasm32")]
pub mod web;
