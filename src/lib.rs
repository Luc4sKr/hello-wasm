use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello_world() -> wasm_bindgen::JsValue {
    return wasm_bindgen::JsValue::from_str("Hello World!");
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}