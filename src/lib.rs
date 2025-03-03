mod meat;
use meat::run;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    #[cfg(feature = "web_debug")] {
        console_log::init_with_level(log::Level::Debug).unwrap();
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }
    
    run(None).await;
    Ok(())
}
