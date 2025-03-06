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
    }).await.map_err(|error| {   
        let msg = format!("Failed to initialize three-d/winit window with error:\n\t{error}");
        show_error_in_HTML(&msg).map_or_else(
            |jserror| jserror,
            |()| JsValue::from_str(&msg),
        )
    })
}

fn show_error_in_HTML(msg: &str) -> Result<(), JsValue> {
    let doc = web_sys::window().and_then(|w| w.document()).ok_or_else(
        || JsValue::from_str("Failed to get document while displaying error!")
    )?;
    let body = doc.body().ok_or_else(
        || JsValue::from_str("Failed to get document body while displaying error!")
    )?;

    let elem = doc.create_element("h3")?;
    elem.set_text_content(Some(msg));
    
    if let Some(before) = body.first_child() {
        body.replace_child(&elem, &before)?;
    } else {
        body.append_child(&elem)?;
    }

    Ok(())
}

}

#[cfg(target_family = "wasm")]
pub use inner::*;
