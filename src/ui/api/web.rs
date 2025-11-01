use eframe::wasm_bindgen::{JsCast as _, JsValue};
use super::SlopeStakerApp;

impl SlopeStakerApp {
    fn get_inner_box(&self) -> Box<dyn eframe::App> {
        Box::new(SlopeStakerApp::default())
    }
    pub fn launch_web(self) -> Result<(), JsValue>{
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();
        let web_options = eframe::WebOptions::default();



        let creator: eframe::AppCreator = Box::new(move |_cc| { 
            Ok(self.get_inner_box())
        });

        let canvas = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document")
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        wasm_bindgen_futures::spawn_local(async {
            let _ = eframe::WebRunner::new()
                .start(
                    canvas,
                    web_options,
                    creator,
                )
                .await;
        });
        Ok(())
    }
}
