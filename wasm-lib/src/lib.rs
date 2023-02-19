use wasm_bindgen::prelude::*;

use gloo_utils::format::JsValueSerdeExt;
use serde::{Serialize, Deserialize};


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[derive(Serialize, Deserialize, Debug)]
struct Dictionary {
    id: usize,
    x: f64,
    y: f64,
}

#[wasm_bindgen]
pub fn process_dictionaries(input: JsValue) -> JsValue {
    console_log!("pre vec");
    let bebra: Vec<Dictionary> = match input.into_serde() {
        Ok(omg) => omg,
        Err(err) => { console_log!("Can not parse input: {}", err); Vec::new() }
    };
    console_log!("post vec");

    let mut result = Dictionary { id: 0, x: 0., y: 0. };
    for mut d in bebra.into_iter() {
        console_log!("iterate!!");
        d.x += d.y;
        d.y = 1.;
        result = d;
    }

    JsValue::from_serde(&result).unwrap()
}

// #[wasm_bindgen]
// pub fn build_fractal(dots: JsValue) -> JsValue {
//     process_dictionaries(dots)
// }
