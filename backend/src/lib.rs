use std::sync::Once;

use anyhow::Result;
use geo::{Haversine, Length, LineString};
use geojson::Feature;

use wasm_bindgen::prelude::*;

static START: Once = Once::new();

/// TODO describe
#[wasm_bindgen(js_name = compareLines)]
pub fn compare_lines(input1: JsValue, input2: JsValue) -> Result<String, JsValue> {
    // Panics shouldn't happen, but if they do, console.log them.
    console_error_panic_hook::set_once();
    START.call_once(|| {
        console_log::init_with_level(log::Level::Info).unwrap();
    });

    let f1: Feature = serde_wasm_bindgen::from_value(input1)?;
    let line1: LineString = f1.try_into().map_err(err_to_js)?;

    let f2: Feature = serde_wasm_bindgen::from_value(input2)?;
    let line2: LineString = f2.try_into().map_err(err_to_js)?;

    Ok(serde_json::json!({
        "length1": line1.length::<Haversine>(),
        "length2": line2.length::<Haversine>(),
    })
    .to_string())
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
