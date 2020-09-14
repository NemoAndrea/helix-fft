# Helixiser 

This directory contains the Rust code for helixiser that is compiled to WebAssembly. Various data types (arrays, number) can be passed between WebAssembly and Javascript, and hence we can use WebAssembly for the computationally intensive parts of `helixiser`. 

Rust to WebAssembly is handled by the `wasm-bindgen` crate.

## Files

`display.rs` functions that relate to the modification of an existing image's appearance (e.g. contrast)

`wasm_functions.rs` functions that manage the wasm-bindgen  bindings between javascript and webassembly

`bessel_utils.rs` functions to compute bessel function of the first kind

`analytic_diffraction.rs` functions and structs to produce an analytic diffraction pattern of helical structures.

`fft_2D` 2D FFT functions, with padding utilities. Check out [details of the FFT implementation in docs](../doc/fft.md)

--

`main.rs` contains some tests involving IO that are used to test standalone functionality of the Rust library. The goal is maintain basic functionality of helixiser in pure Rust (without WebAssembly or the vue frontend).