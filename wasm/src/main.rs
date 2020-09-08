use helixiser::analytic_diffraction::{ Helix, Handedness, diff_analytic };
use helixiser::display::adjust_contrast;
use helixiser::fft_2D::{ test, FFT_2D };

use rustfft::num_complex::{Complex, Complex64, Complex32};
use rustfft::num_traits::Zero;

extern crate image;

use std::time::{Duration, Instant};
use image::GenericImageView;

use ndarray::{Array, Array1, Array2, arr2};


fn main () {
    println!("> Example of Helical diffraction, using B-DNA as a model");

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

    let dna_helices:Vec<Helix> = vec![strand_1, strand_2];

    let now = Instant::now();  // start a timer

    // lets get an image (as a 1D array, with values in order (R,G,B,A) and then next pixel etc.
    let mut im_arr:Vec<f64> = diff_analytic(dna_helices, 5, 1, 0.01, 512 );

    println!("It took {} ms to produce analytic diffraction pattern.", now.elapsed().as_millis());

    // lets make it a bit more contrast-y
    im_arr = adjust_contrast(im_arr, 0., 0., 100.);

    // convert to uint8
    let u_im_arr:Vec<u8> = im_arr.iter().map(|val| *val as u8).collect();

    // use the [image] crate to save array as an image to the project directory
    image::save_buffer("B-DNA_via_Rust.png", &u_im_arr, 512, 512, image::ColorType::Rgba8).unwrap();

    ///////////////////////////////////////// FFT
    println!("> FFT testing ...");
    let img_obj = image::open("../img/testing/fft_test_diffraction.png").unwrap().grayscale();
    let img_int:Vec<u8> = img_obj.to_bytes();
    let img_float:  Vec<Complex64> = img_int.iter().map(|pix| Complex64::new(*pix as f64, 0f64) ).collect();

    let img: Array2<Complex64> = Array::from_shape_vec((512,512),(img_float)).unwrap().into();

    // Compute FFT, take natural log of value and scale so it uses the dynamic range of u8
    let logged_fft: Vec<f64>= FFT_2D(img).iter().map(|val| val.norm().ln()*20f64).collect();

    // convert to u8 and save as png
    let out: Vec<u8> = logged_fft.iter().map(|val| *val as u8).collect();
    image::save_buffer("fft_test_diffraction_out.png", &out, 512, 512, image::ColorType::L8).unwrap();
    println!("[ Finished saving FFT ]");
}