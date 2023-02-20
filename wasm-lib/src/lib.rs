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

    let (x, y) = (xl - xf, yl - yf);

    let mut vec_length = vec![(x.powi(2) + y.powi(2)).sqrt()];
    let mut vec_current_length = vec![1.];

    for vec in &init_dots {
        let (_, xn, yn) = (*vec).into();

        vec_length.push(xn.powi(2) + yn.powi(2));
        vec_current_length.push(vec_length.last().unwrap()/vec_length[0]);
        
        cosinus.push((xn * x + yn * y)/(vec_length.last().unwrap() * vec_length[0]));
        sinuses.push((1. - cosinus.last().unwrap().powi(2)).sqrt());
    }

    console_log!("Hello?");
    let frac = build_fractal(
        dots_count,
        (0., 0.), // first dot coords
        (1000., 1000.), // last dot coords
        &sinuses,
        &cosinus,
        &vec_current_length,
        0,
        iterations
    );
    let mut frac_dict = vec![];
    console_log!("No fractals?");

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

    for i in 1..dots_count - 1 {
        let (x, y) = (vece.0 - vecn.0, vece.1 - vecn.1);

        let xr = (x * cosinus[i] + y * sinuses[i]) * vec_current_length[i];
        let yr = (-x * sinuses[i] + y * cosinus[i]) * vec_current_length[i];

        coords.push((vecn.0 + xr, vecn.1 + yr));
    }
    coords.push(vece);

    if iteration == max_iters {
        fractal.extend(&coords);
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
