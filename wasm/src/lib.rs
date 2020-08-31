extern crate serde_json;
extern crate wasm_bindgen;
extern crate web_sys;  // console messages, should probably be removed
extern crate num;  // complex numbers
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate ndarray;

// different components below

pub mod bessel_utils;
pub mod display;
pub mod analytic_diffraction;
pub mod wasm_functions;
