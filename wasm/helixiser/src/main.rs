use helixiser::diffraction_analytic::{ diff_analytic };
use helixiser::utilities::{adjust_contrast, array_to_luminance, luminance_to_rgba};
use helixiser::fft_2D::{ FFT_2D, pad_image };
use helixiser::diffraction_simulator::{projection_from_helix_family};
use helixiser::helix::{ Helix, Handedness };

use open::that;
use helixiser::export::{ helixiser_web_link };

use rustfft::num_complex::{Complex64};
use rustfft::num_traits::Zero;

use num:: { traits::Pow };

extern crate image;

use std::time::{Duration, Instant};
use image::GenericImageView;

use ndarray::{Array, Array2};


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

    let dna_helices: Vec<Helix> = vec![strand_1, strand_2];

    let now = Instant::now();  // start a timer

    // lets get an image
    let mut complex_image = diff_analytic(dna_helices, 5, 1, 0.01, 512);
    complex_image.rescale(511.);

    println!("It took {} ms to produce analytic diffraction pattern.", now.elapsed().as_millis());

    let image_rgba = luminance_to_rgba(complex_image.intensities().into_raw_vec());


    // lets make it a bit more contrast-y
    let im_arr = adjust_contrast(image_rgba, 0., 0., 100.);

    // convert to uint8
    let u_im_arr: Vec<u8> = im_arr.iter().map(|val| *val as u8).collect();

    // use the [image] crate to save array as an image to the project directory
    image::save_buffer("B-DNA_via_Rust.png", &u_im_arr, 512, 512, image::ColorType::Rgba8).unwrap();

    complex_image.save_image("B-DNA_via_Rust_direct.png")
    //
    // ///////////////////////////////////////// FFT
    // println!("> FFT testing ...");
    // let img_obj = image::open("../img/testing/fft_test_diffraction.png").unwrap().grayscale();
    // let img_int: Vec<u8> = img_obj.to_bytes();
    // let img_float: Vec<Complex64> = img_int.iter().map(|pix| Complex64::new(*pix as f64, 0f64)).collect();
    //
    // let img: Array2<Complex64> = Array::from_shape_vec((512, 512), img_float).unwrap().into();
    //
    // // Compute FFT, take natural log of value and scale so it uses the dynamic range of u8
    // let logged_fft: Vec<f64> = FFT_2D(img).iter().map(|val| val.norm().ln() * 20f64).collect();
    //
    // // convert to u8 and save as png
    // let out: Vec<u8> = logged_fft.iter().map(|val| *val as u8).collect();
    // image::save_buffer("fft_test_diffraction_out.png", &out, 512, 512, image::ColorType::L8).unwrap();
    // println!("[ Finished saving FFT ]");
    //
    // ///////////////////////////////////////// PADDING TESTING
    // println!("> FFT testing - part 2 ...");
    // let img_obj_2 = image::open("../img/testing/MT_nonsquare.png").unwrap().grayscale();  //MT_nonsquare.png
    // let img_int_2: Vec<u8> = img_obj_2.to_bytes();
    // let img_float_2: Vec<f64> = img_int_2.iter().map(|pix| *pix as f64).collect();
    // let mean: f64 = &img_float_2.iter().sum::<f64>() / (img_float_2.len() as f64);
    // println!("Mean is {}", mean);
    // let img_2: Array2<f64> = Array::from_shape_vec((418, 172), img_float_2).unwrap().into(); //172, 418
    // let padded = pad_image(img_2, mean);  // mean pad
    // println!("Padded image dimensions {} and {}", padded.nrows(), padded.ncols());
    // let out_2: Vec<u8> = padded.iter().map(|val| val.clone() as u8).collect();
    // image::save_buffer("MT_nonsquare_out.png", &out_2, padded.nrows() as u32, padded.ncols() as u32, image::ColorType::L8).unwrap();
    // //also save FFT
    // let FFT_2: Vec<f64> = FFT_2D(padded.mapv(|val| Complex64::new(val as f64, 0f64)))
    //     .iter().map(|val| val.norm().ln() * 20f64).collect();
    // let fft_out_2: Vec<u8> = FFT_2.iter().map(|val| *val as u8).collect();
    // image::save_buffer("MT_nonsquare_fft_out.png", &fft_out_2, 512, 512, image::ColorType::L8).unwrap();
    //
    // ///////////////////////////////////////// SIMULATION TESTING
    // let helix1 = Helix {
    //     radius: 12.5,
    //     rise: 0.946,
    //     frequency: 13.,
    //     unit_size: 3.,
    //     offset: 0.,
    //     rotation: 0.,
    //     handedness: Handedness::Right,
    // };
    //
    // let helix2 = Helix {
    //     offset: 4.1,
    //     ..helix1  // copy remaining fields over from helix1
    // };
    //
    // let helix3 = Helix {
    //     offset: 8.3,
    //     ..helix1  // copy remaining fields over from helix1
    // };
    //
    // let MT_13PF: Vec<Helix> = vec![helix1, helix2, helix3];
    //
    // let sim_projection: Array2<f64> = projection_from_helix_family(MT_13PF, 0.3, 10., 0.3);
    // let sim_out: Vec<u8> = sim_projection.iter().map(|val| *val as u8).collect();
    // image::save_buffer("simulation_out.png", &sim_out, sim_projection.ncols() as u32,
    //                    sim_projection.nrows() as u32, image::ColorType::L8).unwrap();


}