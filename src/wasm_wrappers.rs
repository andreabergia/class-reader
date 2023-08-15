use wasm_bindgen::prelude::*;

use crate::read_buffer;

#[wasm_bindgen]
pub fn wasm_read_buffer(buffer: &[u8]) -> Result<JsValue, JsValue> {
    let class_file = read_buffer(buffer);
    match class_file {
        Ok(class_file) => Ok(serde_wasm_bindgen::to_value(&class_file)?),
        Err(err) => Err(serde_wasm_bindgen::to_value(&err)?),
    }
}
