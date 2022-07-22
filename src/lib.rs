use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn tada_alert() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    window.alert_with_message("hey yo")?;

    Ok(())
}
