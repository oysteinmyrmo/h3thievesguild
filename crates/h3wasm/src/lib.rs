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
//! The data set is a plain object. Its TypeScript types are generated from
//! [`h3dataset::Dataset`] and the error types into the `.d.ts` file wasm-bindgen writes.
//! Missing values are `null`, so each `values` row works directly as ECharts series data.

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
