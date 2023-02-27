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
    let sin = (1. - cos.powi(2)).sqrt();
    if y < 0. { return -sin; }
    sin
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
    let vecm = (x, y); // main vector

    let mut vec_length = vec![calc_vector_length(vecm)];
    let mut length_ratio_to_main = vec![1.];

    let norm_main_vec_y = y/vec_length[0];

    let mut coses = vec![1.];
    let mut sines = vec![0.];

    for i in 1..dots_count - 1 {
        let (_, xn, yn) = init_dots[i].into();
        let vecn = (xn - xf, yn - yf);

        vec_length.push(calc_vector_length(vecn));
        length_ratio_to_main.push(vec_length[i] / vec_length[0]);

        let norm_i_vec_y = vecn.1 / vec_length[i];

        coses.push(calc_cos(vecn, vecm, vec_length[i], vec_length[0]));
        sines.push(calc_sin(norm_i_vec_y - norm_main_vec_y, coses[i]));
    }

    let fractal = build_fractal(
        dots_count,
        (0., 0.), // first dot coords
        vecm, // last dot coords
        &sines,
        &coses,
        &length_ratio_to_main,
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
    sines: &Vec<f64>,
    coses: &Vec<f64>,
    length_ratio_to_main: &Vec<f64>,
    iteration: usize
) -> Vec<(f64, f64)> {
    let mut fractal: Vec<(f64, f64)> = vec![];

    let (x, y) = (vecl.0 - vecf.0, vecl.1 - vecf.1);

    let mut coords: Vec<(f64, f64)> = vec![vecf];
    for i in 1..dots_count - 1 {
        // calculate current vector (relative to vecf)
        let xc = (x * coses[i] - y * sines[i]) * length_ratio_to_main[i];
        let yc = (x * sines[i] + y * coses[i]) * length_ratio_to_main[i];

        coords.push((vecf.0 + xc, vecf.1 + yc));
    }
    coords.push(vecl);

    if iteration == 0 {
        fractal.append(&mut coords);
    } else {
        for i in 1..dots_count {
            fractal.append(&mut build_fractal(
                dots_count,
                coords[i - 1],
                coords[i],
                sines,
                coses,
                length_ratio_to_main,
                iteration - 1,
            ));
        }
    }

    fractal
}
