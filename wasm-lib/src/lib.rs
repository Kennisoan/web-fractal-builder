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

    let (_, xf, yf) = init_dots[0             ].into(); // first point coords
    let (_, xl, yl) = init_dots[dots_count - 1].into(); // last point coords

    let (x, y) = (xl - xf, yl - yf); // vector from first point to last

    let mut vec_length = vec![(x.powi(2) + y.powi(2)).sqrt()];
    let mut vec_current_length = vec![1.];

    for i in 1..dots_count {
        let (_, xn, yn) = init_dots[i].into();

        vec_length.push((xn.powi(2) + yn.powi(2)).sqrt());
        vec_current_length.push(vec_length[i] / vec_length[0]);

        cosinus.push((xn * x + yn * y)/(vec_length[i] * vec_length[0]));
        sinuses.push((1. - cosinus[i].powi(2)).sqrt());
    }

    let frac = build_fractal(
        dots_count,
        (0., 0.), // first dot coords
        (1000., 0.), // last dot coords
        &sinuses,
        &cosinus,
        &vec_current_length,
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
    dots_count: usize,
    vecn: (f64, f64),
    vece: (f64, f64),
    sinuses: &Vec<f64>,
    cosinus: &Vec<f64>,
    vec_current_length: &Vec<f64>,
    iteration: usize,
    max_iters: usize
) -> Vec<(f64, f64)> {
    let mut fractal: Vec<(f64, f64)> = vec![];

    let mut coords = vec![vecn];

    let (x, y) = (vece.0 - vecn.0, vece.1 - vecn.1);

    for i in 1..dots_count - 1 {
        // calculate current vector (from vecn)
        let xc = (x * cosinus[i] + y * sinuses[i]) * vec_current_length[i];
        let yc = (-x * sinuses[i] + y * cosinus[i]) * vec_current_length[i];

        coords.push((vecn.0 + xc, vecn.1 + yc));
    }
    coords.push(vece);

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
                vec_current_length,
                iteration + 1,
                max_iters
            ));
        }
    }

    fractal
}