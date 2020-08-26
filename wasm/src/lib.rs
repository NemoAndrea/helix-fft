extern crate serde_json;
extern crate wasm_bindgen;
extern crate web_sys;  // console messages, should probably be removed
extern crate num;  // complex numbers

use wasm_bindgen::prelude::*;
use ndarray::{Array2, Array1, Array, ArrayView};
use std::f64::consts::PI;
use num::Complex;
use num::complex::Complex64;
use num::traits::{ Pow };

mod bessel_utils; // load HELIX-FFT's bessel library
use bessel_utils::{ nextBessel, Jn };


#[macro_use]
extern crate serde_derive;


#[macro_use]
extern crate ndarray;


// demo code

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet_num(number: u8) {
    alert(&format!("Hello, {}!", number));
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}


// FFT-helix functions

#[wasm_bindgen]  // slightly faster than the new one, but messes up alpha
pub fn set_contrast_old(image_data: Vec<f32>, offset: f32, min: f32, max: f32) -> Vec<f32> {
    let val_range: f32 = 255 as f32 / ( max - min );

    // set the new contrast
    let new_image_data = image_data.iter().map(
        |rgbaval|  ( rgbaval - min ) * val_range + offset
    ).collect();

    // return new contrast
    new_image_data
}


#[wasm_bindgen]
pub fn set_contrast(image_data: Vec<f32>, offset: f32, min: f32, max: f32) -> Vec<f32> {
    let val_range: f32 = 255 as f32 / (max - min);

    let mut new_image_data: Vec<f32> = vec![0 as f32; image_data.len()];
    for i in (0..image_data.len()).step_by(4) {
        let val: f32 = (image_data[i] - min) * val_range + offset;
        new_image_data[i] = val as f32;  // set Red
        new_image_data[i + 1] = val as f32;  // set Green
        new_image_data[i + 2] = val as f32;  // set Blue
        new_image_data[i + 3] = 255 as f32;  // set Alpha
    }

    // return new contrast
    new_image_data
}

#[derive(Serialize, Deserialize)]  // I havent figured out how to do this directly with numeric values
pub struct Element {
    radius: String,
    rise: String,
    frequency: String,
    unit_size: String,
    offset: String,
    rotation: String,
    handedness: String
}

#[wasm_bindgen]  //
pub fn FFT_analytic( helix_family: &JsValue, n_range: u8, m_range: u8, scale: f64, raster_size: u32) -> Vec<f64> {
    let num_pix: usize = (raster_size * raster_size) as usize;

    // helix_family is an array of dicts in JS.
    let helices: Vec<Element> = helix_family.into_serde().unwrap();

    let mut m_vals: Vec<f64> = vec![];
    for i in 0..m_range {
        if i == 0{ m_vals.push(i as f64) }
        else {
            // add +m and -m
            m_vals.push( (i as f64));
            m_vals.push(-(i as f64));
        }
    };

    // intialise the 2D array that represents our image.
    let mut image:Array2<Complex64> = Array::zeros((raster_size as usize, raster_size as usize));

    // get a set of coordinates [-rastersize/2 to rastersize/2]
    let coords:Array1<f64> = Array::range(0.,raster_size as f64, 1 as f64) - (raster_size / 2) as f64;

    let mut z_line:f64;  // the layerline coordinate
    let mut z_draw_coord:f64;  // rounded value of z_line (for drawing in image)
    let mut bessel:Array1<f64> = Array::zeros(raster_size as usize);
    let mut Ufac:Array1<Complex64> = Array::zeros(raster_size as usize);

    let mut phi_j:f64;  // phase change due to angular offset helix
    let mut phi_z:f64;  // phase change due to offset along helix axis (z)

    //loop over different helices
    for helix in &helices {
        // first get helix info into a datatype we can handle
        let radius:f64 = helix.radius.parse::<f64>().unwrap_or(0 as f64);
        let rise:f64 = helix.rise.parse::<f64>().unwrap_or(0 as f64);
        let frequency:f64 = helix.frequency.parse::<f64>().unwrap_or(0 as f64);
        //let unit_size:f64 = helix.unit_size.parse::<f64>().unwrap_or(0 as f64);  // not used currently
        let offset:f64 = helix.offset.parse::<f64>().unwrap_or(0 as f64);
        let rotation:f64 = helix.rotation.parse::<f64>().unwrap_or(0 as f64);
        //let handedness:String = helix.handedness.parse::<String>().unwrap_or("right".clone().parse().unwrap());  // not used currently

        // compute some useful quantities
        let pitch:f64 = rise * frequency;
        phi_j = rotation * PI/180.; // rotation due to offset AND the rotation
        let midpoint:f64 = raster_size as f64 / 2.;

        // compute analytic FFT solution and modify <image>
        for n in 0..n_range {
            // we compute the bessel function (depends only on n, not m)
            for index in 0..raster_size as usize {
                bessel[index] = Jn(n.into(), 2.*PI*coords[index].abs()*radius*scale);
            }

            for m in &m_vals {
                // we determine the layerline position
                z_line = (n as f64) / ( pitch * scale ) + m / ( rise * scale);
                z_draw_coord = z_line.round();  // get the rounded value, for plotting

                phi_z =  2. * PI * z_line * offset * scale;
                for index in 0..raster_size as usize {
                    Ufac[index] =  bessel[index] * (Complex::new(0.0, (n as f64) * (PI / 2. - phi_j) + phi_z) ).exp();
                }

                //web_sys::console::log_1(&format!("N is {n}, M is {m}, and z_line is {z}", n=n, m=m, z=z_line).into());

                // check if we are still within the limits of the rasterised image
                // if so, add the layerline amplitudes (Ufac)
                if z_draw_coord > -(raster_size as f64)/2. && z_draw_coord < (raster_size as f64)/2. {
                    let mut line =  image.slice_mut(s![(midpoint + z_draw_coord) as usize, ..]);
                    line += &ArrayView::from(&Ufac);
                    if n != 0{
                        let mut line =  image.slice_mut(s![(midpoint - z_draw_coord) as usize, ..]);
                        line += &ArrayView::from(&Ufac);
                    }
                }
            }
        }
    }

    // flatten the array
    let image_flat = image.into_shape((num_pix, 1)).unwrap();
    let normalisation = helices.len() as f64;  // ensure that more helices does not mean higher intensities

    let intensity = image_flat.mapv(
        //  we compute the intensity, which is |amplitude|^2.
        //  We clip some values (>256), but it's probably best for now with limited dynamic range
        |amplitude| (amplitude/normalisation).norm().pow(2) * 511.
    );

    let mut out:Vec<f64> = vec![0 as f64; 4 * num_pix];
    for i in 0..num_pix {
        out[i*4] =     intensity[[i, 0]]; //image_flat[[i, 0]]; // set Red
        out[i*4 + 1] = intensity[[i, 0]]; // set Green
        out[i*4 + 2] = intensity[[i, 0]]; // set Blue
        out[i*4 + 3] = 255 as f64;  // set Alpha
    }
    out
}