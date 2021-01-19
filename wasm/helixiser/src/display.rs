// Image Adjustment functions


pub fn adjust_contrast(image_data: Vec<f64>, offset: f64, min: f64, max: f64) -> Vec<f64> {
    let val_range: f64 = 256f64 / (max - min);

    // compute the rescaled array
    let rescaled_data: Vec<f64> = image_data.iter().step_by(4)
        .map(|val| (val - min + offset) * val_range).collect();
    // make into appropriate format
    let test = arr_to_rgba(rescaled_data);

    return test
}

pub fn arr_to_rgba(luminance: Vec<f64>) -> Vec<f64>{
    // we need a vector that is 4 times the length of the luminance (red, green, blue, alpha)
    let mut rgba: Vec<f64> = vec![0f64; luminance.len()*4];
    for i in 0..luminance.len() {
        rgba[i*4    ] = luminance[i];  // set Red
        rgba[i*4 + 1] = luminance[i];  // set Green
        rgba[i*4 + 2] = luminance[i];  // set Blue
        rgba[i*4 + 3] = 255f64;        // set Alpha
    }
    return rgba
}



#[cfg(test)]
mod tests {
    use crate::display::adjust_contrast;

    #[test]
    fn max_100() {
        let data: Vec<f64> = vec![0., 0., 0., 10.,
                                  12., 12., 12., 10.,
                                  1.2, 1.2, 1.2, 10.];
        let adjusted_data = adjust_contrast(data, 0. ,0., 100.);

        // println!("{:?}", adjusted_data);

        // Find the largest non-NaN in vector, or NaN otherwise:
        let max =  adjusted_data.iter().cloned().fold(0./0., f64::max);

        let pixel_2 = adjusted_data[4];

        assert_eq!( pixel_2, 12. * 256. / (100. - 0.));
    }

    #[test]
    fn min_100() {
        let data: Vec<f64> = vec![0., 0., 0., 10.,
                                  12., 12., 12., 10.,
                                  1.2, 1.2, 1.2, 10.];
        let adjusted_data = adjust_contrast(data, 0. ,0., 100.);

        // Find the smallest non-NaN in vector, or NaN otherwise:
        let min =  adjusted_data.iter().cloned().fold(0./0., f64::min);

        let pixel_3 = adjusted_data[8];

        assert_eq!( pixel_3, 1.2 * 256. / (100. - 0.));
    }

    #[test]
    fn alpha() {
        let data: Vec<f64> = vec![0., 0., 0., 10.,
                                  12., 12., 12., 10.,
                                  -8.2, -8.2, -8.2, 10.];
        let adjusted_data = adjust_contrast(data, 0. ,-8.2, 12.);

        // get alpha value (just take the last element of vector)
        let alpha = adjusted_data[adjusted_data.len()-1];

        assert_eq!( alpha, 255.);
    }

    #[test]
    fn offset() {
        let val = 25.;
        let offset = 50.;

        let data: Vec<f64> = vec![val, val, val, 10.];
        let adjusted_data = adjust_contrast(data, offset ,0., 256.);

        // get alpha value (just take the last element of vector)
        let pixel = adjusted_data[0];

        assert_eq!( pixel, val + offset );
    }
}