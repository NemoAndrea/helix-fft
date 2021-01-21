A library for the calculation of diffraction patterns of helical objects

## Web application

The `helixiser` rust crate serves as the backend for the **Helixiser** web application when compiled to WebAssembly.

You can try the web application at [https://nemoandrea.github.io/helixiser/](https://nemoandrea.github.io/helixiser/)

## Getting started

Helixiser revolves around `helix` objects. 

1. Let's create a family of helices

```rust
let strand_1 = Helix {
    radius: 1.,
    rise: 0.34,
    frequency: 10.,
    unit_size: 0.18,
    offset: 0.,
    rotation: 0.,
    handedness: Handedness::Right,
};

let strand_2 = Helix {
    rotation: 143.,
    ..strand_1  // copy remaining fields over from strand 1
};

let dna_helices: Vec<Helix> = vec![strand_1, strand_2];
```

2. Compute the "analytic diffraction pattern" for this family of helices and save as image

```rust
extern crate image;

// lets get an image (as a 1D array, with values in order (R,G,B,A) and then next pixel etc.
let mut im_arr: Vec<f64> = diff_analytic(dna_helices, 5, 1, 0.01, 512);

// lets make it a bit more contrast-y
im_arr = adjust_contrast(im_arr, 0., 0., 100.);

// convert to uint8
let u_im_arr: Vec<u8> = im_arr.iter().map(|val| *val as u8).collect();

// use the [image] crate to save array as an image to the project directory
image::save_buffer("B-DNA_via_Rust.png", &u_im_arr, 512, 512, image::ColorType::Rgba8).unwrap();
```

