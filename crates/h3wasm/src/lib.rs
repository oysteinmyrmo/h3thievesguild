//! WebAssembly bindings: build a plotting data set from save files in the browser.
//!
//! ```js
//! import init, { DatasetBuilder } from "./h3wasm.js";
//!
//! await init();
//! const builder = new DatasetBuilder();
//! for (const file of input.files) {
//!     builder.add(file.name, new Uint8Array(await file.arrayBuffer()));
//! }
//! try {
//!     const dataset = builder.finish(); // A `Dataset`; see the TypeScript types.
//!     if (dataset.missingDaysWarning && !confirm(dataset.missingDaysWarning)) {
//!         // Days are missing and the user chose not to go on.
//!     }
//! } catch (e) {
//!     // A `DatasetError`: e.message for the user, e.kind and e.details for the code.
//! }
//! ```
//!
//! The data set is a plain object, described by the TypeScript types in this crate and by
//! [`h3dataset::Dataset`]. Missing values are `null`, so each `values` row works directly as
//! ECharts series data.

// Save files come from users: the bindings must not panic on any input.
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::unreachable,
        clippy::todo,
        clippy::unimplemented,
        clippy::arithmetic_side_effects
    )
)]

use h3dataset::{Builder, DatasetError};
use serde::Serialize;
use wasm_bindgen::prelude::*;

/// TypeScript types of the data set and errors, added to the generated `.d.ts` file.
#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT: &str = include_str!("types.d.ts");

/// Serialize as plain objects with `null` for missing values, as ECharts expects.
fn to_js<T: Serialize + ?Sized>(value: &T) -> Result<JsValue, JsValue> {
    let serializer = serde_wasm_bindgen::Serializer::new()
        .serialize_maps_as_objects(true)
        .serialize_missing_as_null(true);
    value.serialize(&serializer).map_err(JsValue::from)
}

/// A JavaScript `Error` named `DatasetError`, with the `kind` and `details` of the error.
fn to_js_error(error: &DatasetError) -> JsValue {
    let js_error = js_sys::Error::new(&error.to_string());
    js_error.set_name("DatasetError");
    if let Ok(fields) = to_js(error) {
        for key in ["kind", "details"] {
            let key = JsValue::from_str(key);
            if let Ok(value) = js_sys::Reflect::get(&fields, &key) {
                if !value.is_undefined() {
                    let _ = js_sys::Reflect::set(&js_error, &key, &value);
                }
            }
        }
    }
    js_error.into()
}

/// Collects save files and builds a data set from them.
#[wasm_bindgen]
#[derive(Debug, Default)]
pub struct DatasetBuilder {
    inner: Builder,
}

#[wasm_bindgen]
impl DatasetBuilder {
    #[wasm_bindgen(constructor)]
    #[must_use]
    pub fn new() -> DatasetBuilder {
        DatasetBuilder::default()
    }

    /// Add a save file by name and contents. Files that cannot be read are reported, all
    /// together, by `finish`.
    pub fn add(&mut self, name: String, contents: &[u8]) {
        self.inner.add_file(name, contents);
    }

    /// Number of files added.
    #[wasm_bindgen(getter)]
    #[must_use]
    pub fn length(&self) -> usize {
        self.inner.len()
    }

    /// Build the data set. The builder cannot be used afterwards.
    ///
    /// # Errors
    ///
    /// Throws a `DatasetError` if the files are not the saves of one game with one save per
    /// day. Its message names every file involved.
    #[wasm_bindgen(unchecked_return_type = "Dataset")]
    pub fn finish(self) -> Result<JsValue, JsValue> {
        let dataset = self.inner.finish().map_err(|e| to_js_error(&e))?;
        to_js(&dataset)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::Path;

    use serde_json::Value;

    /// Every key of every object in `value`, except the keys of `values` maps, which are
    /// statistic IDs rather than fields.
    fn keys(value: &Value, out: &mut BTreeSet<String>) {
        match value {
            Value::Object(map) => {
                for (key, v) in map {
                    out.insert(key.clone());
                    if key != "values" {
                        keys(v, out);
                    }
                }
            }
            Value::Array(items) => items.iter().for_each(|v| keys(v, out)),
            _ => {}
        }
    }

    const TYPESCRIPT: &str = include_str!("types.d.ts");

    #[test]
    fn the_typescript_types_name_every_field_of_the_dataset_and_errors() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../test_data/resources");
        let files: Vec<(String, Vec<u8>)> = ["[hotseat] 111.GM8", "[hotseat] 113.GM5"]
            .iter()
            .map(|n| ((*n).to_string(), std::fs::read(dir.join(n)).unwrap()))
            .collect();
        let dataset = h3dataset::build(files.iter().map(|(n, b)| (n.as_str(), b.as_slice()))).unwrap();
        let mut fields = BTreeSet::new();
        keys(&serde_json::to_value(&dataset).unwrap(), &mut fields);
        let errors = [
            h3dataset::DatasetError::Unreadable(vec![h3dataset::UnreadableFile {
                file: "f".into(),
                reason: "r".into(),
            }]),
            h3dataset::DatasetError::MixedGames(vec![h3dataset::GameFiles {
                map_name: "m".into(),
                map_size: "S".into(),
                underground: false,
                files: vec![],
            }]),
            h3dataset::DatasetError::DuplicateDays(vec![h3dataset::DayFiles {
                date: "d".into(),
                files: vec![],
            }]),
            h3dataset::DatasetError::TooManyDays {
                days: 1,
                first: h3dataset::DayFiles {
                    date: "d".into(),
                    files: vec![],
                },
                last: h3dataset::DayFiles {
                    date: "d".into(),
                    files: vec![],
                },
            },
        ];
        for error in &errors {
            keys(&serde_json::to_value(error).unwrap(), &mut fields);
        }
        for field in &fields {
            let named = TYPESCRIPT.contains(&format!("{field}:")) || TYPESCRIPT.contains(&format!("{field}?:"));
            assert!(named, "TypeScript types lack `{field}`");
        }
        for kind in ["noSaves", "unreadable", "mixedGames", "duplicateDays", "tooManyDays"] {
            assert!(
                TYPESCRIPT.contains(&format!("\"{kind}\"")),
                "TypeScript types lack kind {kind}"
            );
        }
    }
}
