use std::sync::Arc;
use rustfft::FFTplanner;
use rustfft::FFT;
use rustfft::num_complex::{Complex, Complex64};
use rustfft::num_traits::Zero;

use ndarray::{Array2, ArrayViewMut1, ArrayView1, ArrayView, Array, ViewRepr};
use wasm_bindgen::__rt::core::sync::atomic::Ordering::AcqRel;

// 2D fourier transform of images using the  rustfft library

// Perform a forward FFT of size 1234

pub fn test (mut input: Vec<Complex<f32>>) -> Vec<Complex<f32>> {
    let mut output: Vec<Complex<f32>> = vec![Complex::zero(); 65536];

    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(65536);
    fft.process(&mut input, &mut output);

    // The fft instance returned by the planner is stored behind an `Arc`, so it's cheap to clone
    output
}

fn FFT_1D(mut slice: ArrayViewMut1<Complex64>, length: usize){
    let mut input: Vec<Complex64> = slice.iter().map(|val| val.clone()).collect();
    let mut output: Vec<Complex<f64>> = vec![Complex::zero(); length];

    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(length);

    fft.process( &mut input, &mut output);

    slice.assign(&Array::from(output))
}

pub fn FFT_2D(mut image: Array2<Complex64>) -> Array2<Complex64>{
    let ncols = image.ncols();
    let nrows = image.nrows();

    // we first take 1D fft of each row
    for row_num in 0..nrows {
        let mut row = image.row_mut(row_num);
        FFT_1D(row, ncols);
    }

    // then we need to take the 1D fft of each column
    for col_num in 0..ncols {
        let mut column = image.column_mut(col_num);
        FFT_1D(column, nrows);
    }

    FFTshift(&mut image);

    return image
}

// swap quadrant 1->3 and 2->4
fn FFTshift(image: &mut Array2<Complex64>){
    let half_row: usize = image.nrows() / 2;
    let half_col: usize = image.ncols() / 2;
    let copy = image.clone();  // bit of a waste of time and memory
    image.slice_mut(s!(..half_row, ..half_col)).assign(&copy.slice(s!(half_row.., half_col..)));
    image.slice_mut(s!(..half_row, half_col..)).assign(&copy.slice(s!(half_row.., ..half_col)));
    image.slice_mut(s!(half_row.., ..half_col)).assign(&copy.slice(s!(..half_row, half_col..)));
    image.slice_mut(s!(half_row.., half_col..)).assign(&copy.slice(s!(..half_row, ..half_col)));
}