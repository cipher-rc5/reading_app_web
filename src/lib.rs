//! Browser-first reading application compiled to WebAssembly.
//!
//! The crate exposes a single wasm entrypoint, [`start`], which boots the egui
//! app on a target canvas element.

#[cfg(target_arch = "wasm32")]
mod database;
#[cfg(target_arch = "wasm32")]
mod ui;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
/// Starts the web app and mounts it to the canvas with the provided DOM id.
pub fn start(canvas_id: &str) -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    tracing::info!("Starting Reading App Web");

    let web_options = eframe::WebOptions::default();
    let canvas_id = canvas_id.to_string();

    wasm_bindgen_futures::spawn_local(async move {
        use ui::ReadingApp;
        use wasm_bindgen::JsCast;

        let canvas = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id(&canvas_id))
            .and_then(|e| e.dyn_into::<web_sys::HtmlCanvasElement>().ok())
            .expect("Failed to get canvas element");

        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(ReadingApp::new(cc)))),
            )
            .await
            .expect("Failed to start eframe");
    });

    Ok(())
}
