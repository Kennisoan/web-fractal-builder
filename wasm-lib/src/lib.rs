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


fn calc_cos(v1: (f64, f64), v2: (f64, f64), l1: f64, l2: f64) -> f64 {
    (v1.0 * v2.0 + v1.1 * v2.1)/(l1 * l2) // (v1, v2) = x1x2 + y1y2 = |v1||v2|cos(phi)
}

fn calc_sin(y: f64, cos: f64) -> f64 {
    let sin = (1. - cos.powi(2)).sqrt(); //       { sqrt(1 - cos^2(x)), y >= 0
    if y < 0. { return -sin; }                // sin = {
    sin                                       //       { -sqrt(1 - cos^2(x)), y < 0
}

fn calc_vector_length(vec: (f64, f64)) -> f64 {
    (vec.0.powi(2) + vec.1.powi(2)).sqrt()
}


// input - js array of js dicts: [{"id": 0, "x": 1., "y": 4.}, ...]
// iterations - depth of generated fractal
#[wasm_bindgen]
pub fn fractal(input: JsValue, iterations: usize) -> JsValue {
    if iterations == 1 {
        return input;
    }

    let init_dots: Vec<Dictionary> = match input.into_serde() {
        Ok(value) => value,
        Err(err) => {
            console_log!("Can not parse input: {}", err);
            return JsValue::NULL;
        }
    };

    let dots_count = init_dots.len();

    if dots_count < 3 {
        return JsValue::NULL;
    }

    let (_, xf, yf) = init_dots[0             ].into();
    let (_, xl, yl) = init_dots[dots_count - 1].into();

    let (x, y) = (xl - xf, yl - yf); // vector from first point -> last
    
    let mut vec_length = vec![calc_vector_length((x, y))];
    let mut length_ratio_to_first = vec![1.];

    let norm_main_vec_y = y/vec_length[0];
    
    let mut cosinus = vec![1.];
    let mut sinuses = vec![0.];

    for i in 1..dots_count - 1 {
        console_log!("iteration {i}:");
        let (_, mut xn, mut yn) = init_dots[i].into();
        (xn, yn) = (xn - xf, yn - yf);

        vec_length.push(calc_vector_length((xn, yn)));
        length_ratio_to_first.push(vec_length[i] / vec_length[0]);

        let norm_i_vec_y = yn / vec_length[i];

        cosinus.push(calc_cos((xn, yn), (x, y), vec_length[i], vec_length[0]));
        sinuses.push(calc_sin(norm_i_vec_y - norm_main_vec_y, cosinus[i]));
    }

    let fractal = build_fractal(
        dots_count,
        (0., 0.), // first dot coords
        (x, y), // last dot coords
        &sinuses,
        &cosinus,
        &length_ratio_to_first,
        0,
        iterations - 1
    );
    let mut frac_dict = vec![];

    for i in 0..fractal.len() {
        frac_dict.push(Dictionary::new(i, fractal[i].0, fractal[i].1))
    }

    JsValue::from_serde(&frac_dict).unwrap()
}

pub fn build_fractal(
    dots_count: usize,
    vecf: (f64, f64),
    vecl: (f64, f64),
    sinuses: &Vec<f64>,
    cosinus: &Vec<f64>,
    length_ratio_to_first: &Vec<f64>,
    iteration: usize,
    max_iters: usize
) -> Vec<(f64, f64)> {
    let mut fractal: Vec<(f64, f64)> = vec![];

    let (x, y) = (vecl.0 - vecf.0, vecl.1 - vecf.1);

    let mut coords: Vec<(f64, f64)> = vec![vecf];
    for i in 1..dots_count - 1 {
        // calculate current vector (relative to vecf)
        let xc = (x * cosinus[i] - y * sinuses[i]) * length_ratio_to_first[i];
        let yc = (x * sinuses[i] + y * cosinus[i]) * length_ratio_to_first[i];

        coords.push((vecf.0 + xc, vecf.1 + yc));
    }
    coords.push(vecl);

    if iteration == max_iters {
        fractal.append(&mut coords);
    } else {
        for i in 1..dots_count {
            fractal.append(&mut build_fractal(
                dots_count,
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
