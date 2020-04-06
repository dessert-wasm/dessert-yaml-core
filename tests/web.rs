//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use dessert_yaml_core::*;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate serde_json;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn basic_parse_error() {
    assert!(load("**thisisnotvalid", &JsValue::UNDEFINED).is_err());
}

#[wasm_bindgen_test]
fn basic_parse_success() {
    assert!(load("---\na: b", &JsValue::UNDEFINED).is_ok());
}

#[wasm_bindgen_test]
fn basic_dump() {
    let obj = JsValue::from_serde(&json!({"a": "b"})).unwrap();
    let res = dump(&obj, &JsValue::UNDEFINED).expect("Coundl't parse object as yaml");
    assert_eq!(res, "---\na: b");
}
