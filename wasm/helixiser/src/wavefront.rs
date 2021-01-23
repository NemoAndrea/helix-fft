//! # Helixiser's equivalent of an image
use ndarray::Array2;
use num::complex::Complex64;
use num::traits::Pow;
use crate::utilities::luminance_to_rgba;


pub struct Wavefront {
    /// A wavefront is a 2D array of complex value where each point corresponds to a coordinate in
    /// 2D space. The array indices correspond to a point in space.
    pub values: Array2<Complex64>,
    width: usize,
    height: usize,
}

impl Wavefront {
    /// Instantiate a new Wavefront struct
    pub fn new(values: Array2<Complex64>) -> Self {
        Wavefront {
            width: values.nrows(),
            height: values.ncols(),
            values,
        }
    }

    /// rescale the (complex) values of the wavefront by a (real) scalar value.
    ///
    /// # Examples
    /// ```
    /// use assert_approx_eq::assert_approx_eq;
    /// use helixiser::wavefront::Wavefront;
    /// use num::traits::Pow;
    /// use ndarray::arr2;
    /// use num::complex::Complex64;
    /// let mut my_wavefront = Wavefront::new( arr2(&[[Complex64::new(5., 1.),
    ///                                           Complex64::new(0., 0.)],
    ///                                          [Complex64::new(0., 0.),
    ///                                           Complex64::new(4., 2.)]]));
    /// // Rescale the wavefront
    /// my_wavefront.rescale(10.5);
    ///
    /// assert_eq!(my_wavefront.values, arr2(&[[Complex64::new(52.5, 10.5),
    ///                                         Complex64::new(0., 0.)],
    ///                                        [Complex64::new(0., 0.),
    ///                                         Complex64::new(42., 21.)]]))
    ///
    /// ```
    pub fn rescale(&mut self, factor:f64) {
        self.values = &self.values * factor;
    }

    /// Convert the complex valued wavefront into (real valued) intensities.
    ///
    /// Real values are obtained by squaring the norm of the complex values.
    pub fn intensities(&self) -> Array2<f64>{
        self.values.mapv(
            |comval| (comval).norm().pow(2)
        )
    }

    /// Save wavefront intensities as image using the image crate
    ///
    /// # Panics
    /// This function uses the `image::save_buffer` function which can panic
    pub fn save_image(&self, path: &str) -> () {
        let intensities = Self::intensities(&self);
        let rgba = luminance_to_rgba(intensities.into_raw_vec());
        let img: Vec<u8> = rgba.iter().map(|val| *val as u8).collect();
        image::save_buffer(path, &img, self.width as u32, self.height as u32, image::ColorType::Rgba8).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use crate::wavefront::Wavefront;
    use num::traits::Pow;
    use ndarray::arr2;
    use num::complex::Complex64;

    #[test]
    fn complex_to_intensities() {
        let my_wavefront = Wavefront::new( arr2(&[[Complex64::new(5., 1.),
                                                                      Complex64::new(0., 0.)],
                                                                     [Complex64::new(0., 0.),
                                                                      Complex64::new(4., 2.)]]));

        let analytic_answers = vec![5f64.pow(2) + 1f64.pow(2), 0., 0.,
                                    4f64.pow(2) + 2f64.pow(2)];

        for (i, intensity) in my_wavefront.intensities().iter().enumerate() {
            assert_approx_eq!(intensity, analytic_answers[i])
        }
    }
}