use crate::analytic_diffraction::Helix;
use ndarray::{Array2};
use std::f64::consts::PI;
use num:: { traits::Pow };


// function that takes a series of helix objects and builds a rasterised simulated projection image.
// the simulated image is extremely primitive, without optical abberations and considering only
// uniform material density
pub fn projection_from_helix_family(helices: Vec<Helix>, pixel_size: f64, pitch_lengths: f64, density: f64) -> Array2<f64> {
    // we figure out the largest helical pitch of the family (may not all be the same)
    let mut largest_pitch = 0f64;
    let mut largest_radius = [0f64, 0f64]; // radius, unit_size
    for helix in &helices {
        if helix.pitch() > largest_pitch { largest_pitch = helix.pitch() }
        if helix.radius > largest_radius[0] {
            largest_radius[0] = helix.radius;
            largest_radius[1] = helix.unit_size;
        }
    }

    // now we have the dimensions to draw our rasterised image at
    let mut image_width: f64 = ( largest_radius[0] + largest_radius[1] ).ceil() * 2f64 + 4.;
    let mut image_height: f64 = ( pitch_lengths * largest_pitch ).ceil();
    let mut projection:Array2<f64> = Array2::zeros(( (image_height / pixel_size) as usize,
                                             ( image_width / pixel_size) as usize ));

    let mut x:f64;
    let mut z:f64;
    for helix in &helices {
        // how many subunits will we draw? We draw one extra pitch on top and bottom of image.
        // this will guarantee (provided the subunit size  <  pitch (reasonable) we don't miss any
        // at the cost of some performance
        let num_subunits = ( image_height / helix.rise ) as u32;
        for index in 0..num_subunits {
            x = ( (index as f64 * helix.rad_per_subunit() + helix.rotation_to_rad() ).cos()
                * helix.radius + image_width / 2f64 ) / pixel_size;
            z = ( index as f64 * helix.rise + helix.offset ) / pixel_size;
            if z > (image_height - helix.unit_size) / pixel_size || z < ( helix.unit_size ) / pixel_size{
                draw_border_subunit(&mut projection, x as usize, z as usize, helix.unit_size)
            } else {
                draw_subunit(&mut projection, x as usize, z as usize, helix.unit_size / pixel_size, density)

            }

        }

    }


    return projection
}

fn draw_subunit(image: &mut Array2<f64>, x: usize, z: usize, size: f64, density: f64){
    if (x >= 0) && (x < image.ncols()) &&
        (z >= 0) && (z < image.nrows()) {
        let mut r_sq:f64;
        let mut val:f64 = 0.;
        for x_loc in 0..size.ceil() as usize {
            for z_loc in 0..size.ceil() as usize {
                r_sq = (x_loc as f64).pow(2) + (z_loc as f64).pow(2);
                if r_sq < (size).pow(2) {
                    val = 255. * density * (1. - r_sq / (size).pow(2));
                    //println!("mini val {}, and rsq {}, size {}, factor: {}",val, r_sq, size,  (1. - r_sq / (size).pow(2)));

                    image[[z + z_loc as usize, x + x_loc as usize]] += val;
                    if x_loc != 0  { image[[z + z_loc as usize, x - x_loc as usize]] += val; }
                    if z_loc != 0 { image[[z - z_loc as usize, x + x_loc as usize]] += val;
                        if x_loc != 0 { image[[z - z_loc as usize, x - x_loc as usize]] += val };
                    }
                }
            }
        }
        image[[z, x]] += 0f64;
    }
}

fn draw_border_subunit(image: &mut Array2<f64>, x: usize, z: usize, size: f64){
    if (x >= 0) && (x < image.ncols()) &&
        (z >= 0) && (z < image.nrows()) {
        image[[z, x]] += 255f64;
    }
}
