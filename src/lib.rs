mod meat;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    #[cfg(debug_assertions)] {
        console_log::init_with_level(log::Level::Debug).unwrap();
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }
    
    meat::run().await;
    Ok(())
}
