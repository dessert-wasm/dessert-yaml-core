mod utils;
use wasm_bindgen::prelude::*;

extern crate serde_json;
use serde_json::json;
use serde_json::Value;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn load(yml_str: &str, options: JsValue) -> Result<JsValue, JsValue> {
    // log(&format!("{:?}", options));
    let yml: serde_json::Value = match serde_yaml::from_str(&yml_str) {
        Ok(res) => res,
        Err(e) => {
            let location = e.location();
            return Err(JsValue::from_serde(&json!({
                "mark": {
                    "buffer": yml_str.to_string(),
                    "name": Value::Null,
                    "column": match &location {
                        Some(l) => Value::Number(l.column().into()),
                        None => Value::Null,
                    },
                    "line": match &location {
                        Some(l) => Value::Number(l.line().into()),
                        None => Value::Null,
                    },
                    "position": match &location {
                        Some(l) => Value::Number(l.index().into()),
                        None => Value::Null,
                    },
                },
                "message": e.to_string(),
                "name": "YAMLException"
            }))
            .unwrap());
        }
    };
    Ok(JsValue::from_serde(&yml).unwrap())
}

#[wasm_bindgen(js_name = safeLoad)]
pub fn safe_load(yml_str: &str, options: JsValue) -> Result<JsValue, JsValue> {
    load(yml_str, options)
}

#[wasm_bindgen(js_name = loadAll)]
pub fn load_all(yml_str: &str, options: JsValue) -> Result<JsValue, JsValue> {
    load(yml_str, options)
}

#[wasm_bindgen]
pub fn dump(object: JsValue, options: JsValue) -> Result<String, JsValue> {
    // log(&format!("{:?}", options));

    let value: serde_yaml::Value = match object.into_serde() {
        Ok(v) => v,
        Err(e) => {
            return Err(JsValue::from_str(&format!("{}", e)));
        }
    };
    Ok(serde_yaml::to_string(&value).unwrap())
}

#[wasm_bindgen(js_name = safeDump)]
pub fn safe_dump(object: JsValue, options: JsValue) -> Result<String, JsValue> {
    dump(object, options)
}
