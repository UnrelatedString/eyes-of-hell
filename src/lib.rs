mod meat;

#[cfg(target_family = "wasm")]
mod inner {

use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

use crate::meat::run;
use three_d::WindowSettings;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    #[cfg(feature = "web_debug")] {
        log_1(&JsValue::from_str("WASM loaded and running!"));
        console_log::init_with_level(log::Level::Debug).unwrap();
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }
    
    run(WindowSettings {
        initial_size: None,
        // Can put undocumented canvas setting here for fancier itch layout lol
        ..Default::default()
    }).await;
    Ok(())
}
}

#[cfg(not(target_family = "wasm"))]
mod inner {}

pub use inner::*;
