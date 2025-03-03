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
        // TODO: canvas (why isn't it documented ;_; like I know it's a #[cfg] but just mention it manually?)
        ..Default::default()
    }).await;
    Ok(())
}
}

#[cfg(not(target_family = "wasm"))]
mod inner {}

pub use inner::*;
