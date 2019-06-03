//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use dessert_yaml_core;
use wasm_bindgen::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn parse_error() {
    assert!(dessert_yaml_core::load("**blblbl", JsValue::UNDEFINED).is_err());
}

#[wasm_bindgen_test]
fn parse_success() {
    assert!(dessert_yaml_core::load("---\na: b", JsValue::UNDEFINED).is_ok());
}