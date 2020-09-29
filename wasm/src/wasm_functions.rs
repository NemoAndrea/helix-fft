use wasm_bindgen::prelude::*;
use crate::analytic_diffraction::*;
use crate::display::{ adjust_contrast, arr_to_rgba };
use crate::fft_2D::{ FFT_2D, pad_image };
use crate::bessel_utils::bessel_first_max;

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

// unfortunately I do not know how to return multiple values atm through WASM, so we make a messy array
#[wasm_bindgen]  // calculate FFT of image (return the norm of complex-valued fourier transform) and dimensions
pub fn wasm_FFT(image: Vec<f64>, width: u32, height: u32) -> Vec<f64> {
    //alert(&format!("input image is {} pixels and {} x {}", image.len(), width, height) );

    // we take the red channel and map it into vector (we assume all channels rgb are equal)
    let data_mean: f64 = &image.iter().step_by(4).sum::<f64>() / (image.len()as f64 / 4f64 );  // mean pixel value
    //alert(&format!("Mean pixel value is {}", data_mean) );
    let data: Vec<num_complex::Complex64> = image.iter().step_by(4).map(
        |val| num_complex::Complex64::new(*val, 0f64)).collect();


    // make vector into 2D array and pad the resulting array with the mean pixel value.
    let padded_image: Array2<num_complex::Complex64> =
        pad_image(Array::from_shape_vec((height as usize,width as usize),data)
                      .unwrap().into(),num_complex::Complex64::new(data_mean, 0f64) );

    let out_width = padded_image.ncols().clone() as f64;
    let out_height = padded_image.nrows().clone() as f64;
    //alert(&format!("output is {} x {}", out_width, out_height) );

    // Take FFT and take the log of the norm for display purposes
    let mut out_arr: Vec<f64> = arr_to_rgba( FFT_2D(padded_image).iter().map(
        |val| val.norm().ln()*20f64).collect() );  // value of 20 is a bit ad-hoc currently
    //alert(&out_arr.len().to_string());

    // we add the dimensions of the image to the end of the vector as we cannot have multiple return vals
    out_arr.push(out_width);
    out_arr.push(out_height);

    return out_arr
}

#[wasm_bindgen]  // get x value of the first maximum of Jn(x) for given n.
pub fn wasm_bessel_first_max(n: u32) -> f64 {
    bessel_first_max(n)
}