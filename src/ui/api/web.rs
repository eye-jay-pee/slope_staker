use eframe::wasm_bindgen::{JsCast as _, JsValue};

impl SlopeStakerApp {
    pub fn launch_web(self) -> Result<(), JsValue>{
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();
        let web_options = eframe::WebOptions::default();

        wasm_bindgen_futures::spawn_local(async {
            let document = web_sys::window()
                .expect("No window")
                .document()
                .expect("No document");

            let canvas = document
                .get_element_by_id("the_canvas_id")
                .expect("Failed to find the_canvas_id")
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .expect("the_canvas_id was not a HtmlCanvasElement");

            let creator: eframe::AppCreator = Box::new(move |_cc| {
                Ok(Box::new(SlopeStakerApp::default()) as Box<dyn eframe::App>)
            });
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
