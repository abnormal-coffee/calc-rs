#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe;

mod app;
use app::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "rcalc",
        native_options,
        Box::new(|cc| Box::new(app::CalcState::new(cc))),
    );
}

// fix later
#[cfg(target_arch = "wasm32")]
fn main() {
    
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "rcalc",
            web_options,
            Box::new(|cc| Box::new(app::CalcState::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}