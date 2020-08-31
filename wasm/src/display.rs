// Image Adjustment functions


pub fn adjust_contrast(image_data: Vec<f64>, offset: f64, min: f64, max: f64) -> Vec<f64> {
    let val_range: f64 = 256 as f64 / (max - min);

    let mut new_image_data: Vec<f64> = vec![0 as f64; image_data.len()];
    for i in (0..image_data.len()).step_by(4) {
        let val: f64 = (image_data[i] - min + offset) * val_range;
        new_image_data[i] = val as f64;  // set Red
        new_image_data[i + 1] = val as f64;  // set Green
        new_image_data[i + 2] = val as f64;  // set Blue
        new_image_data[i + 3] = 255 as f64;  // set Alpha
    }

    // return new contrast
    new_image_data
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