use dioxus_web::launch::launch;

mod app;
mod components;
mod pages;
mod router;
mod config;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn main() {
    // Set up panic hook for WASM
    std::panic::set_hook(Box::new(|panic_info| {
        web_sys::console::error_1(&format!("Panic: {:?}", panic_info).into());
    }));
    
    wasm_logger::init(wasm_logger::Config::default());
    // In WASM, always use default config since file system is not available
    launch(app::app, Vec::new(), Vec::new());
}