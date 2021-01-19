//! # The main helix object
use std::f64::consts::PI;

/// A struct representing a Helix
pub struct Helix {
    /// A Helix has a radius value, which should be positive
    pub radius: f64,
    /// The helical rise is the distance between subunits along the helix axis (i.e. z axis)
    pub rise: f64,
    /// The amount of subunits in a full turn of a helix. Need not be integer
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
    /// Compute the pitch of the helix. This is the distance between the helix backbone along
    /// the helix axis (i.e. z axis) after a full rotation. Does not need to be an integer
    /// multiple of the rise. Is computed as `pitch` = `rise` * `frequency`.
    ///
    /// # Examples
    /// ```
    /// use std::f64::consts::PI;
    /// use crate::helixiser::helix::{ Handedness, Helix };
    ///
    /// let myHelix = Helix {
    ///         radius: 18.9,
    ///         rise: 0.55,
    ///         frequency: 11.5,
    ///         unit_size: 0.1,
    ///         offset: 0.,
    ///         rotation: 0.,
    ///         handedness: Handedness::Right,
    /// };
    ///
    /// let myPitch = myHelix.pitch();
    ///
    /// assert_eq!(myPitch, 11.5*0.55)
    /// ```
    pub fn pitch(&self) -> f64 {
        self.rise * self.frequency
    }

    /// Convert the helix' rotation to radians insted of the default (degrees)
    ///
    /// # Examples
    /// ```
    /// use std::f64::consts::PI;
    /// use crate::helixiser::helix::{ Handedness, Helix };
    ///
    /// let myHelix = Helix {
    ///         radius: 1.,
    ///         rise: 0.34,
    ///         frequency: 10.,
    ///         unit_size: 0.18,
    ///         offset: 0.,
    ///         rotation: 90.,
    ///         handedness: Handedness::Right,
    /// };
    ///
    /// let rotation = myHelix.rotation_to_rad();
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
    /// use crate::helixiser::helix::{ Handedness, Helix };
    ///
    /// let myHelix = Helix {
    ///         radius: 2.,
    ///         rise: 0.34,
    ///         frequency: 8.0,
    ///         unit_size: 0.1,
    ///         offset: 0.,
    ///         rotation: 90.,
    ///         handedness: Handedness::Right,
    /// };
    ///
    /// let angle_per_subunit = myHelix.rad_per_subunit();
    ///
    /// assert_eq!(angle_per_subunit, PI/4f64)
    /// ```
    pub fn rad_per_subunit(&self) -> f64 { 2f64 * PI / self.frequency }
}


/// A simple enum to specify the handedness of an object. Can be either left or right handed.
#[derive(Debug, Copy, Clone)]
pub enum Handedness {
    Right,
    Left,
}



