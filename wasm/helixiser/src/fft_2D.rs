
use std::cmp;
use num::traits::Pow;

use rustfft::algorithm::Radix4;  // power of 2-sized vector FFT algorithm
use rustfft::FFT;
use rustfft::num_complex::{Complex, Complex64};
use rustfft::num_traits::Zero;

use ndarray::{Array2, ArrayViewMut1, ArrayView1, ArrayView, Array, ViewRepr};
// 2D fourier transform of images using the  rustfft library

// Perform a forward FFT of size 1234

fn FFT_1D(mut slice: ArrayViewMut1<Complex64>, length: usize){
    let mut input: Vec<Complex64> = slice.iter().map(|val| val.clone()).collect();
    let mut output: Vec<Complex<f64>> = vec![Complex::zero(); length];

    let fft = Radix4::new(length, false);

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


// Pad an image with fill value to multiple of 2 for radix4 algorithm. Image will be square.
pub fn pad_image<T: Copy> (image: Array2<T>, fill: T) ->  Array2<T> {
    let nrows = image.nrows() as usize;  // first dimension
    let ncols = image.ncols() as usize;
    println!("Originals image dimensions {} and {}", nrows, ncols );

    // round up to nearest power of 2
    let new_width  = 2f64.pow( (nrows as f64).log2().ceil() ) as usize;
    let new_height = 2f64.pow( (ncols as f64).log2().ceil() ) as usize;
    let dim = cmp::max(new_height, new_width);  // ensure the image is square

    let mut padded_image: Array2<T> = Array::from_elem((dim, dim), fill).into();

    //copy the elements of image over to the padded image
    let down: usize  = ( ( dim - nrows ) / 2 ) as usize;
    let up: usize    = &down + nrows;
    let left: usize  = ( ( dim - ncols ) / 2 ) as usize;
    let right: usize = &left + ncols;
    println!("down {}, up {}, left {}, right {}", down, up, left, right );
    padded_image.slice_mut( s!( down..up, left..right ) ).assign(&image);

    return padded_image
}


// Pad an image with repetition to multiple of 2 for radix4 algorithm. Image will be square.
// fill value will be used, but not be present in final output
pub fn repetition_pad_image<T: Copy> (image: Array2<T>, fill: T) ->  Array2<T> {
    let nrows = image.nrows() as usize;  // first dimension
    let ncols = image.ncols() as usize;
    println!("Originals image dimensions {} and {}", nrows, ncols );

    // round up to nearest power of 2
    let new_width  = 2f64.pow( (nrows as f64).log2().ceil() ) as usize;
    let new_height = 2f64.pow( (ncols as f64).log2().ceil() ) as usize;
    let dim = cmp::max(new_height, new_width);  // ensure the image is square

    let mut padded_image: Array2<T> = Array::from_elem((dim, dim), image[[nrows-1,ncols-1]].clone()).into();

    //copy the elements of image over to the padded image
    let down: usize  = ( ( dim - nrows ) / 2 ) as usize;
    let up: usize    = &down + nrows;
    let left: usize  = ( ( dim - ncols ) / 2 ) as usize;
    let right: usize = &left + ncols;
    println!("down {}, up {}, left {}, right {}", down, up, left, right );
    padded_image.slice_mut( s!( down..up, left..right ) ).assign(&image);

    //add repetitions
    //sides
    padded_image.slice_mut( s!( up.., left..right ) ).assign(&image.slice(s![..down, ..]));
    padded_image.slice_mut( s!( ..down, left..right ) ).assign(&image.slice(s![nrows-down.., ..]));
    padded_image.slice_mut( s!( down..up, right.. ) ).assign(&image.slice(s![.., ncols-left..]));
    padded_image.slice_mut( s!( down..up, ..left ) ).assign(&image.slice(s![.., ..left]));
    // corners
    padded_image.slice_mut( s!( up.., right.. ) ).assign(&image.slice(s![..down, ncols-left..]));
    padded_image.slice_mut( s!( up.., ..left ) ).assign(&image.slice(s![..down, ..left]));
    padded_image.slice_mut( s!( ..down, right.. ) ).assign(&image.slice(s![nrows-down.., ncols-left..]));
    padded_image.slice_mut( s!( ..down, ..left ) ).assign(&image.slice(s![nrows-down.., ..left]));


    return padded_image
}