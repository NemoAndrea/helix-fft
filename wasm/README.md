# Helixiser 

This directory contains the Rust code for helixiser that is compiled to WebAssembly

## Files

`display.rs` functions that relate to the modification of an existing image's appearance (e.g. contrast)

`wasm_functions.rs` functions that manage the wasm-bindgen  bindings between javascript and webassembly

`bessel_utils.rs` functions to compute bessel function of the first kind

`analytic_diffraction.rs` functions and structs to produce an analytic diffraction pattern of helical structures.