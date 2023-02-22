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


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct Dictionary {
    id: usize,
    x: f64,
    y: f64,
}

impl Dictionary {
    fn new(id: usize, x: f64, y: f64) -> Self{
        Dictionary { id, x, y }
    }
}

impl From<Dictionary> for (usize, f64, f64) {
    fn from(d: Dictionary) -> Self {
        (d.id, d.x, d.y)
    }
}

// input - js array of js dicts: [{"id": 0, "x": 1., "y": 4.}, ...]
// iterations - depth of generated fractal
#[wasm_bindgen]
pub fn fractal(input: JsValue, iterations: usize) -> JsValue {
    let init_dots: Vec<Dictionary> = match input.into_serde() {
        Ok(omg) => omg,
        Err(err) => {
            console_log!("Can not parse input: {}", err);
            return JsValue::NULL;
        }
    };

    let dots_count = init_dots.len();

    if dots_count < 3 {
        return JsValue::NULL;
    }

    let mut sinuses = vec![0.];
    let mut cosinus = vec![1.];

    let (_, xf, yf) = init_dots[0             ].into();
    let (_, xl, yl) = init_dots[dots_count - 1].into();

    let (x, y) = (xl - xf, yl - yf); // vector from first point -> last

    let mut vec_length = vec![(x.powi(2) + y.powi(2)).sqrt()];
    let mut length_ratio_to_first = vec![1.];

    for i in 1..dots_count {
        let (_, mut xn, mut yn) = init_dots[i].into();
        (xn, yn) = (xn - xf, yn - yf);

        vec_length.push((xn.powi(2) + yn.powi(2)).sqrt());
        length_ratio_to_first.push(vec_length[i] / vec_length[0]);

        cosinus.push((xn * x + yn * y)/(vec_length[i] * vec_length[0])); // (a, b) = x1x2 + y1y2 = |a||b|cos(phi)
        sinuses.push((1. - cosinus[i].powi(2)).sqrt());
        console_log!("sin: {}, cos: {}", sinuses.last().unwrap(), cosinus.last().unwrap());
    }

    let frac = build_fractal(
        (xf, yf), // first dot coords
        (xl, yl), // last dot coords
        &sinuses,
        &cosinus,
        &length_ratio_to_first,
        0,
        iterations - 1
    );
    let mut frac_dict = vec![];

    for i in 0..frac.len() {
        frac_dict.push(Dictionary::new(i, frac[i].0, frac[i].1))
    }

    JsValue::from_serde(&frac_dict).unwrap()
}

pub fn build_fractal(
    vecf: (f64, f64),
    vecl: (f64, f64),
    sinuses: &Vec<f64>,
    cosinus: &Vec<f64>,
    length_ratio_to_first: &Vec<f64>,
    iteration: usize,
    max_iters: usize
) -> Vec<(f64, f64)> {
    let dots_count = length_ratio_to_first.len();

    let mut fractal: Vec<(f64, f64)> = vec![];
    let mut coords = vec![vecf];

    let (x, y) = (vecl.0 - vecf.0, vecl.1 - vecf.1);

    for i in 1..dots_count - 1 {
        // calculate current vector (from vecf)
        let xc = (x * cosinus[i] + y * sinuses[i]) * length_ratio_to_first[i];
        let yc = (-x * sinuses[i] + y * cosinus[i]) * length_ratio_to_first[i];
        console_log!("xc: {}, yc: {}, cos: {}, sin: {}", xc, yc, cosinus[i], sinuses[i]);

        coords.push((vecf.0 + xc, vecf.1 + yc));
    }
    coords.push(vecl);

    if iteration == max_iters {
        fractal.append(&mut coords);
    } else {
        for i in 1..dots_count {
            fractal.append(&mut build_fractal(
                coords[i - 1],
                coords[i],
                sinuses,
                cosinus,
                length_ratio_to_first,
                iteration + 1,
                max_iters
            ));
        }
    }

    fractal
}