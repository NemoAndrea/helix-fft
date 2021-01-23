//! # The main helix object
use std::f64::consts::PI;

/// A Helix struct
#[derive(Debug, Clone)]
pub struct Helix {
    /// A Helix has a radius value, which should be positive
    pub radius: f64,
    /// The helical rise is the distance between subunits along the helix axis (i.e. z axis)
    pub rise: f64,
    /// The amount of subunits in a full turn of a helix. Can be decimal value.
    pub frequency: f64,
    /// The size of each subunit
    pub unit_size: f64,
    /// Translational offset along helix axis (i.e. z)
    pub offset: f64,
    /// Rotational offset of helix around helix axis (i.e. z)
    pub rotation: f64,
    /// Handedness (left- or righthanded) of the helix.
    pub handedness: Handedness,
}


impl Helix {
    /// Create a new Helix
    ///
    /// Directly instantiating is also permitted, as it is more human readable, so may be
    /// preferred for example code. When programmatically generating helices, it is best to use
    /// the Helix::new() approach as it panics when it encounters invalid values.
    /// # Examples
    /// ```
    /// use helixiser::helix::{Helix, Handedness};
    /// let my_helix = Helix::new(0.1, 0.4, 12., 0.1, 5.6, 180., Handedness::Right);
    /// let second = my_helix.clone();
    /// ```
    ///
    /// # Panics
    ///
    /// The new function will panic if any of radius, rise, frequency, unit_size or offset are
    /// smaller than or equal to zero.
    pub fn new(radius: f64, rise: f64, frequency: f64, unit_size: f64, offset: f64, rotation: f64, handedness: Handedness) -> Helix {
        assert!(radius > 0.);
        assert!(rise > 0.);
        assert!(frequency > 0.);
        assert!(unit_size > 0.);
        assert!(offset >= 0.);
        Helix {
            radius,
            rise,
            frequency,
            unit_size,
            offset,
            rotation,
            handedness,
        }
    }

    /// Compute the pitch of the helix. This is the distance between the helix backbone along
    /// the helix axis (i.e. z axis) after a full rotation. Does not need to be an integer
    /// multiple of the rise. Is computed as `pitch` = `rise` * `frequency`.
    ///
    /// # Examples
    /// ```
    /// use std::f64::consts::PI;
    /// # use crate::helixiser::helix::{ Handedness, Helix };
    ///
    /// let my_helix = Helix::new( 18.9, 0.55, 11.5, 0.1, 0., 0., Handedness::Right );
    /// let myPitch = my_helix.pitch();
    ///
    /// assert_eq!(myPitch, 11.5 * 0.55)
    /// ```
    pub fn pitch(&self) -> f64 {
        self.rise * self.frequency
    }

    /// Convert the helix' rotation to radians insted of the default (degrees)
    ///
    /// # Examples
    /// ```
    /// use std::f64::consts::PI;
    /// # use crate::helixiser::helix::{ Handedness, Helix };
    ///
    /// let my_helix = Helix::new ( 1.0, 0.34, 10., 0.18, 0., 90., Handedness::Right );
    /// let rotation = my_helix.rotation_to_rad();
    ///
    /// assert_eq!(rotation, PI/2f64)
    /// ```
    pub fn rotation_to_rad(&self) -> f64 {
        self.rotation * PI / 180.
    }

    /// Determine the rotational offset between subunits. This is related only to the `frequency`
    /// by: rotational offset = 2*Pi / frequency
    ///
    /// # Examples
    /// ```
    /// use std::f64::consts::PI;
    /// # use crate::helixiser::helix::{ Handedness, Helix };
    ///
    /// let my_helix = Helix::new ( 2.0, 0.34, 8., 0.1, 0., 90., Handedness::Right );
    /// let angle_per_subunit = my_helix.rad_per_subunit();
    ///
    /// assert_eq!(angle_per_subunit, PI/4f64)
    /// ```
    pub fn rad_per_subunit(&self) -> f64 { 2f64 * PI / self.frequency }
}


/// An enum to specify the handedness of an object. Can be either left or right handed.
#[derive(Debug, Copy, Clone)]
pub enum Handedness {
    Right,
    Left,
}



