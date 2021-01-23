//! # Analytic diffraction image generator
//!
//! Functions that build a diffraction image of a helical structure using an analytic solution

use crate::bessel_utils::{ Jn };
use crate::helix::Helix;
use crate::wavefront::Wavefront;

use ndarray::{Array2, Array1, Array, ArrayView};
use std::f64::consts::PI;
use num:: {Complex, complex::Complex64, traits::Pow };


/// Computes Analytic diffraction pattern for helix object
///
/// returns a Wavefront struct.
/// ```
/// # use helixiser::helix::{ Helix, Handedness };
/// # use helixiser::diffraction_analytic::diff_analytic;
/// use helixiser::wavefront::Wavefront;
/// use num::complex::Complex64;
/// use ndarray::Array2;
///
///
/// // it is also possible to use Helix::new()
/// let strand_1 = Helix {
///         radius: 1.,
///         rise: 0.34,
///         frequency: 10.,
///         unit_size: 0.18,
///         offset: 0.,
///         rotation: 0.,
///         handedness: Handedness::Right,
/// };
///
/// let strand_2 = Helix {
///        rotation: 143.,
///         ..strand_1  // copy remaining fields over from strand 1
/// };
/// let complex_img: Wavefront = diff_analytic(vec![strand_1, strand_2], 6, 2, 0.01, 512);
/// ```
pub fn diff_analytic(helices: Vec<Helix>, n_range: u8, m_range: u8, scale: f64, raster_size: u32) -> Wavefront {
    let num_pix: usize = (raster_size * raster_size) as usize;

    let mut m_vals: Vec<f64> = Vec::new();
    for i in 0..=m_range {
        if i == 0 { m_vals.push(i as f64) } else {
            // add +m and -m
            m_vals.push(i as f64);
            m_vals.push(-(i as f64));
        }
    };

    // intialise the 2D array that represents our image.
    let mut image: Array2<Complex64> = Array::zeros((raster_size as usize, raster_size as usize));

    // get a set of coordinates [-rastersize/2 to rastersize/2]
    let coords: Array1<f64> = Array::range(0., raster_size as f64, 1 as f64) - (raster_size / 2) as f64;

    let mut z_line: f64;  // the layerline coordinate
    let mut z_draw_coord: f64;  // rounded value of z_line (for drawing in image)
    let mut bessel: Array1<f64> = Array::zeros(raster_size as usize);
    let mut Ufac: Array1<Complex64> = Array::zeros(raster_size as usize);

    let mut phi_j: f64;  // phase change due to angular offset helix
    let mut phi_z: f64;  // phase change due to offset along helix axis (z)

    //loop over different helices
    for helix in &helices {
        let pitch: f64 = helix.pitch();
        let midpoint: f64 = raster_size as f64 / 2.;
        phi_j = helix.rotation_to_rad(); // rotation due to rotation

        // compute analytic FFT solution and modify <image>
        for n in 0..=n_range {
            // we compute the bessel function (depends only on n, not m)
            for index in 0..raster_size as usize {
                bessel[index] = Jn(n.into(), 2. * PI * coords[index].abs() * helix.radius * scale);
            }

            for m in &m_vals {
                // we determine the layerline position
                z_line = (n as f64) / (pitch * scale) + m / (helix.rise * scale);
                z_draw_coord = z_line.round();  // get the rounded value, for plotting

                phi_z = 2. * PI * z_line * helix.offset * scale;
                for index in 0..raster_size as usize {
                    Ufac[index] = bessel[index] * (Complex::new(0.0, (n as f64) * (PI / 2. - phi_j) + phi_z)).exp();
                }

                //web_sys::console::log_1(&format!("N is {n}, M is {m}, and z_line is {z}", n=n, m=m, z=z_line).into());

                // check if we are still within the limits of the rasterised image
                // if so, add the layerline amplitudes (Ufac)
                if z_draw_coord > -(raster_size as f64) / 2. && z_draw_coord < (raster_size as f64) / 2. {
                    let mut line = image.slice_mut(s![(midpoint + z_draw_coord) as usize, ..]);
                    line += &ArrayView::from(&Ufac);
                    if n != 0 {
                        let mut line = image.slice_mut(s![(midpoint - z_draw_coord) as usize, ..]);
                        line += &ArrayView::from(&Ufac);
                    }
                }
            }
        }
    }
    return Wavefront::new(image );
}

