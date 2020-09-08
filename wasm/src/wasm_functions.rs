use wasm_bindgen::prelude::*;
use crate::analytic_diffraction::*;
use crate::display::{ adjust_contrast, arr_to_rgba };
use crate::fft_2D::FFT_2D;

use rustfft::num_complex;

// Functions that use the WASM bindings to be called from javascript
// For some functions this will result in a simple wrapper, but it keeps things organised

use ndarray::{Array, Array2};

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]  // just needed to hold the JS values
pub struct Element {
    radius: String,
    rise: String,
    frequency: String,
    unit_size: String,
    offset: String,
    rotation: String,
    handedness: String
}

#[wasm_bindgen]  // function that converts JsValue to Helix Struct and then calls diff_analytic
pub fn wasm_diffraction_analytic(helix_family: &JsValue, n_range: u8, m_range: u8, scale: f64, raster_size: u32) -> Vec<f64> {
    // helix_family is an array of dicts in JS.
    let JS_helices: Vec<Element> = helix_family.into_serde().unwrap();
    let mut helices: Vec<Helix> = Vec::new();

    // We now convert our JS helices into our Helix struct
    for JS_helix in &JS_helices {
        // convert handedness string to enum 'Handedness'
        let mut hand: Handedness = Handedness::Right;
        let JS_string = JS_helix.handedness.parse::<String>().unwrap_or("right".clone().parse().unwrap());
        if let JS_string = String::from("left") {
            hand = Handedness::Left
        } else { hand = Handedness::Right };

        helices.push(Helix {
            radius: JS_helix.radius.parse::<f64>().unwrap_or(0 as f64),
            rise: JS_helix.rise.parse::<f64>().unwrap_or(0 as f64),
            frequency: JS_helix.frequency.parse::<f64>().unwrap_or(0 as f64),
            unit_size: JS_helix.unit_size.parse::<f64>().unwrap_or(0 as f64),
            offset: JS_helix.offset.parse::<f64>().unwrap_or(0 as f64),
            rotation: JS_helix.rotation.parse::<f64>().unwrap_or(0 as f64),
            handedness: hand,
        });
    }

    // now we call the function that actually draws the image and returns it
    diff_analytic(helices, n_range, m_range, scale, raster_size)
}


#[wasm_bindgen]  // just a wrapper for the main adjust_contrast() function
pub fn wasm_adjust_contrast(image_data: Vec<f64>, offset: f64, min: f64, max: f64) -> Vec<f64> {
    adjust_contrast(image_data, offset, min, max)
}

#[wasm_bindgen]  // calculate FFT of image (return the norm of complex-valued fourier transform)
pub fn wasm_FFT(image: Vec<f64>) -> Vec<f64> {
    let mut data: Vec<u8> = image.iter().step_by(4).map(|val| *val as u8).collect();
    let image_data_complex:  Vec<num_complex::Complex64> = data.iter().map(|val| num_complex::Complex64::new(*val as f64, 0f64) ).collect();
    let img: Array2<num_complex::Complex64> = Array::from_shape_vec((512,512),image_data_complex).unwrap().into();

    let mut out_arr: Vec<f64> = FFT_2D(img).iter().map(|val| val.norm().ln()*20f64).collect();

    return arr_to_rgba(out_arr)
}