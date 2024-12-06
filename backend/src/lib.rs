#[allow(unused)]
mod mercator;

use std::sync::Once;

use anyhow::Result;
use geo::{
    Distance, Euclidean, FrechetDistance, GeometryCollection, HausdorffDistance,
    LineInterpolatePoint, LineString,
};
use geojson::{Feature, Geometry, Value};

use wasm_bindgen::prelude::*;

static START: Once = Once::new();

/// Takes two GeoJSON `Feature<LineString>`s and returns a JSON object with different
/// comparisons
#[wasm_bindgen(js_name = compareLines)]
pub fn compare_lines(
    input1: JsValue,
    input2: JsValue,
    use_mercator: bool,
) -> Result<String, JsValue> {
    // Panics shouldn't happen, but if they do, console.log them.
    console_error_panic_hook::set_once();
    START.call_once(|| {
        console_log::init_with_level(log::Level::Info).unwrap();
    });

    let f1: Feature = serde_wasm_bindgen::from_value(input1)?;
    let mut line1: LineString = f1.try_into().map_err(err_to_js)?;

    let f2: Feature = serde_wasm_bindgen::from_value(input2)?;
    let mut line2: LineString = f2.try_into().map_err(err_to_js)?;

    if use_mercator {
        let mercator =
            mercator::Mercator::from(GeometryCollection::from(vec![line1.clone(), line2.clone()]))
                .unwrap();
        mercator.to_mercator_in_place(&mut line1);
        mercator.to_mercator_in_place(&mut line2);
    }

    Ok(serde_json::json!({
        "euclidean": Euclidean::distance(&line1, &line2),
        "frechet": line1.frechet_distance(&line2),
        "hausdorff": line1.hausdorff_distance(&line2),
    })
    .to_string())
}

/// Takes a GeoJSON `Feature<LineString>` and fraction and returns a `Feature<Point>`
#[wasm_bindgen(js_name = lineInterpolatePoint)]
pub fn line_interpolate_point(
    input: JsValue,
    fraction: f64,
    use_mercator: bool,
) -> Result<String, JsValue> {
    // Panics shouldn't happen, but if they do, console.log them.
    console_error_panic_hook::set_once();
    START.call_once(|| {
        console_log::init_with_level(log::Level::Info).unwrap();
    });

    let f: Feature = serde_wasm_bindgen::from_value(input)?;
    let mut line: LineString = f.try_into().map_err(err_to_js)?;

    let mercator = mercator::Mercator::from(line.clone()).unwrap();
    if use_mercator {
        mercator.to_mercator_in_place(&mut line);
    }

    let Some(pt) = line.line_interpolate_point(fraction) else {
        return Err(err_to_js("no output"));
    };
    let f = if use_mercator {
        mercator.to_wgs84_gj(&pt)
    } else {
        Feature::from(Geometry::from(Value::from(&pt)))
    };
    serde_json::to_string(&f).map_err(err_to_js)
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
