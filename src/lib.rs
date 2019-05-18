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

#[wasm_bindgen(js_name = safeLoad)]
pub fn safe_load(yml_str: &str) -> Result<JsValue, JsValue> {
    let yml: Value = match serde_yaml::from_str(&yml_str) {
        Ok(res) => res,
        Err(e) => { 
            let location = e.location();
            return Err(
                JsValue::from_serde(&json!({
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
