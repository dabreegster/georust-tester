use std::sync::Once;

use anyhow::Result;

use wasm_bindgen::prelude::*;

static START: Once = Once::new();

/// TODO describe
#[wasm_bindgen(js_name = exampleCall)]
pub fn example_call(input: String) -> Result<String, JsValue> {
    // Panics shouldn't happen, but if they do, console.log them.
    console_error_panic_hook::set_once();
    START.call_once(|| {
        console_log::init_with_level(log::Level::Info).unwrap();
    });

    Ok("TODO".to_string())
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
