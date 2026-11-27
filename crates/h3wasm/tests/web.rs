//! Tests of the JavaScript side of the bindings. They run in WebAssembly under Node.js:
//!
//! ```sh
//! CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
//!     cargo test -p h3wasm --target wasm32-unknown-unknown
//! ```

#![cfg(target_arch = "wasm32")]

use h3wasm::DatasetBuilder;
use js_sys::{Array, Error, Map, Object, Reflect};
use serde_json::Value;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_test::wasm_bindgen_test;

const DAY_1: &[u8] = include_bytes!("../../../test_data/resources/[hotseat] 111.GM8");
const DAY_2: &[u8] = include_bytes!("../../../test_data/resources/[hotseat] 112.GM8");
const DAY_3: &[u8] = include_bytes!("../../../test_data/resources/[hotseat] 113.GM5");
const OTHER_GAME: &[u8] = include_bytes!("../../../test_data/towns/[hotseat] 111.GM2");

fn get(value: &JsValue, key: &str) -> JsValue {
    Reflect::get(value, &JsValue::from_str(key)).unwrap()
}

fn builder(files: &[(&str, &[u8])]) -> DatasetBuilder {
    let mut builder = DatasetBuilder::new();
    for (name, contents) in files {
        builder.add((*name).to_string(), contents);
    }
    builder
}

/// JSON with every number as a float, so integers from JavaScript compare equal to Rust's.
fn normalize(value: Value) -> Value {
    match value {
        Value::Number(n) => Value::from(n.as_f64().unwrap()),
        Value::Array(items) => Value::Array(items.into_iter().map(normalize).collect()),
        Value::Object(map) => Value::Object(map.into_iter().map(|(k, v)| (k, normalize(v))).collect()),
        other => other,
    }
}

#[wasm_bindgen_test]
fn the_dataset_is_the_same_as_the_rust_one() {
    let files: [(&str, &[u8]); 3] = [("d3.GM5", DAY_3), ("d1.GM8", DAY_1), ("d2.GM8", DAY_2)];
    let js = builder(&files).finish().unwrap();
    let from_js: Value = serde_wasm_bindgen::from_value(js).unwrap();
    let native = h3dataset::build(files).unwrap();
    assert_eq!(normalize(from_js), normalize(serde_json::to_value(&native).unwrap()));
}

#[wasm_bindgen_test]
fn values_are_plain_objects_and_arrays_with_null_for_missing_days() {
    let js = builder(&[("d1", DAY_1), ("d2", DAY_2), ("d3", DAY_3)])
        .finish()
        .unwrap();
    let players: Array = get(&js, "players").dyn_into().unwrap();
    assert_eq!(players.length(), 8);
    let blue = players.iter().find(|p| get(p, "id") == "blue").unwrap();
    let values = get(&blue, "values");
    // A plain object, so `values[stat]` works, not a Map.
    assert!(values.is_instance_of::<Object>() && !values.is_instance_of::<Map>());
    let gold: Array = get(&values, "gold").dyn_into().unwrap();
    assert_eq!(gold.get(0).as_f64(), Some(3400.0));
    assert_eq!(gold.get(1).as_f64(), Some(6700.0));
    // Blue is out of the game on day 3, so every value is 0 that day. (Days without a save are
    // null, not undefined, which ECharts shows as a gap; see the missing days test.)
    assert_eq!(gold.get(2).as_f64(), Some(0.0));
    assert_eq!(get(&blue, "eliminatedDay").as_f64(), Some(3.0));
    // Labels are a plain object too, with text on days with a name and null otherwise.
    let labels = get(&blue, "labels");
    assert!(labels.is_instance_of::<Object>() && !labels.is_instance_of::<Map>());
    let best: Array = get(&labels, "bestCreature").dyn_into().unwrap();
    assert_eq!(best.get(0).as_string().as_deref(), Some("Air Elemental"));
    assert!(best.get(2).is_null());
    let icons: Array = get(&get(&blue, "icons"), "bestCreature").dyn_into().unwrap();
    assert_eq!(icons.get(0).as_string().as_deref(), Some("creatures/AirElemental"));
    let red = players.iter().find(|p| get(p, "id") == "red").unwrap();
    assert!(get(&red, "eliminatedDay").is_null());
    assert_eq!(get(&red, "color"), "#f80000");
    let days: Array = get(&js, "days").dyn_into().unwrap();
    assert_eq!(get(&days.get(2), "label"), "1-1-3");
}

#[wasm_bindgen_test]
fn length_counts_every_file() {
    let builder = builder(&[("d1", DAY_1), ("junk", b"junk")]);
    assert_eq!(builder.length(), 2);
}

/// The error a builder throws, checked to be a `DatasetError`.
fn error_of(builder: DatasetBuilder) -> Error {
    let error: Error = builder.finish().unwrap_err().dyn_into().expect("an Error");
    assert_eq!(error.name(), "DatasetError");
    error
}

#[wasm_bindgen_test]
fn duplicate_days_throw_an_error_naming_the_files() {
    let error = error_of(builder(&[("b.GM8", DAY_1), ("d2.GM8", DAY_2), ("a.GM8", DAY_1)]));
    assert_eq!(get(&error, "kind"), "duplicateDays");
    let message = String::from(error.message());
    assert!(
        message.contains("a.GM8") && message.contains("b.GM8") && !message.contains("d2.GM8"),
        "{message}"
    );
    let details: Value = serde_wasm_bindgen::from_value(get(&error, "details")).unwrap();
    assert_eq!(
        details,
        serde_json::json!([{"date": "Month 1, Week 1, Day 1", "files": ["a.GM8", "b.GM8"]}])
    );
}

#[wasm_bindgen_test]
fn mixed_games_throw_an_error_naming_the_files() {
    let error = error_of(builder(&[("resources.GM8", DAY_1), ("towns.GM2", OTHER_GAME)]));
    assert_eq!(get(&error, "kind"), "mixedGames");
    let message = String::from(error.message());
    assert!(
        message.contains("resources.GM8") && message.contains("towns.GM2"),
        "{message}"
    );
    let details: Array = get(&error, "details").dyn_into().unwrap();
    assert_eq!(details.length(), 2);
}

#[wasm_bindgen_test]
fn unreadable_files_throw_an_error_naming_them() {
    let error = error_of(builder(&[
        ("d1", DAY_1),
        ("notes.txt", b"hello"),
        ("cut.GM8", &DAY_2[..100]),
    ]));
    assert_eq!(get(&error, "kind"), "unreadable");
    let details: Value = serde_wasm_bindgen::from_value(get(&error, "details")).unwrap();
    let files: Vec<&str> = details
        .as_array()
        .unwrap()
        .iter()
        .map(|d| d["file"].as_str().unwrap())
        .collect();
    assert_eq!(files, ["cut.GM8", "notes.txt"]);
}

#[wasm_bindgen_test]
fn no_files_throw_an_error_without_details() {
    let error = error_of(DatasetBuilder::new());
    assert_eq!(get(&error, "kind"), "noSaves");
    assert!(get(&error, "details").is_undefined());
    assert_eq!(String::from(error.message()), "no save files were given");
}

#[wasm_bindgen_test]
fn missing_days_are_null_with_a_warning_to_show() {
    let js = builder(&[("d1", DAY_1), ("d3", DAY_3)]).finish().unwrap();
    let days: Array = get(&js, "days").dyn_into().unwrap();
    assert_eq!(days.length(), 3);
    assert_eq!(get(&days.get(1), "label"), "1-1-2");
    assert!(get(&days.get(1), "file").is_null());
    let warning = get(&js, "missingDaysWarning").as_string().unwrap();
    assert!(
        warning.contains("Month 1, Week 1, Day 2") && warning.ends_with("Continue?"),
        "{warning}"
    );
    let missing: Value = serde_wasm_bindgen::from_value(get(&js, "missingDays")).unwrap();
    assert_eq!(
        missing,
        serde_json::json!([{"first": 2, "last": 2, "label": "Month 1, Week 1, Day 2"}])
    );
    let players: Array = get(&js, "players").dyn_into().unwrap();
    let red = players.iter().find(|p| get(p, "id") == "red").unwrap();
    let gold: Array = get(&get(&red, "values"), "gold").dyn_into().unwrap();
    assert!(gold.get(1).is_null());
    // A complete series has no warning.
    let js = builder(&[("d1", DAY_1), ("d2", DAY_2)]).finish().unwrap();
    assert!(get(&js, "missingDaysWarning").is_null());
}
