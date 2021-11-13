#[macro_use]
extern crate sval_generic_api;

use wasm_bindgen::prelude::*;

use sval_generic_api_js as js;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str, v: JsValue);
}

#[derive(Value)]
pub struct Data {
    id: i32,
    title: String,
    attributes: Vec<Attribute>,
}

#[derive(Value)]
pub struct Attribute {
    id: i32,
    value: String,
}

#[wasm_bindgen]
pub fn greet() {
    let data = Data {
        id: 1,
        title: "A title!".into(),
        attributes: vec![
            Attribute {
                id: 0,
                value: "An attribute!".into(),
            },
            Attribute {
                id: 1,
                value: "Another attribute!".into(),
            },
        ],
    };

    log("This is a message", js::value(&data).unwrap());
}
