mod utils;
use wasm_bindgen::prelude::*;

extern crate serde_json;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn load(yml_str: &str) -> JsValue {
    let yml: serde_json::Value = match serde_yaml::from_str(&yml_str) {
        Ok(res) => res,
        Err(_) => { return JsValue::from_serde("x : 1").unwrap(); }
    };
    JsValue::from_serde(&yml).unwrap()
}
