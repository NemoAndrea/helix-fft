// collection of bessel functions that are required for Helix-FFt
// written by Nemo Andrea in 2020.
// The approaches below are probably not the best way to use these functions for general use,
// so I would be careful in using these functions for your own project.

extern crate num;

// complex numbers
use num::traits::{Pow};
use std::f64::consts::PI;

// for the validity of the calculation, see http://www.mhtlab.uwaterloo.ca/courses/me755/web_chap4.pdf
pub fn J0(x: f64) -> f64 {
  let mut z = x;
  if x < 0.0 { z = -x }  // we ensure z = |x|, as J0(x) = J0(-x) anyway
  if z < 2.0 { // in domain x{0,5} we use a simple series expansion
    return J0a(x)
  }

  // else we are domain x{2, inf}, where we use a different approximation
  return J0b(x)
}


// for the validity of the calculation, see http://www.mhtlab.uwaterloo.ca/courses/me755/web_chap4.pdf
pub fn J1(x: f64) -> f64 {
  let mut z = x;
  let mut sign = 1.;
  if x < 0.0 {
    z = -x;
    sign = -1.;
  }  // we ensure z = |x|, as J1(x) = -J1(-x) anyway
  if z < 2.0 { // in domain x{0,2} we use a simple series expansion
    return sign * J1a(x)
  }

  // else we are domain x{2, inf}, where we use a different approximation
  return sign * J1b(x)
}


pub fn J0a(x: f64) -> f64 {
  1. - x.pow(2) / 4. + x.pow(4) / 64. - x.pow(6) / (2304.) + x.pow(8) / 147456. -
      x.pow(10) / 14745600. + x.pow(12) / 2123366400. - x.pow(14) / 416179814400.
}

// for 'large' values of x we use approximations described in:
// Harrison, John. "Fast and accurate Bessel function computation."
// 2009 19th IEEE Symposium on Computer Arithmetic. IEEE, 2009.

pub fn J0b(x: f64) -> f64 {
  let trigarg: f64 = x - PI / 4. - 1. / (8. * x) + 25. / (384. * x.pow(3));
  (2. / (PI * x)).sqrt() * (1. - (4. * x).pow(-2) + 53. / (512. * x.pow(4))) * trigarg.cos()
}


pub fn J1a(x: f64) -> f64 {
  x / 2. - x.pow(3) / 16. + x.pow(5) / 384. - x.pow(7) / 18432.
}

pub fn J1b(x: f64) -> f64 {
  let trigarg: f64 = x - 3. * PI / 4. + 3. / (8. * x) - 21. / (128. * x.pow(3));
  (2. / (PI * x)).sqrt() * (1. + 3. * (4. * x).pow(-2) - 99. / (512. * x.pow(4))) * trigarg.cos()
}


// computes the next bessel "Jn(x)" using the values of Jn_minus_one(x) and Jn_minus_two(x) (at same x)
// this makes use of the recurrence relation Jn(x) = 2*(n-1) / x * Jn_minus_one(x) - Jn_minus_two(x)
// as described in:
// Goldstein, M., and R. M. Thaler. "Recurrence techniques for the calculation of Bessel functions."
// Mathematics of Computation 13.66 (1959): 102-108.
pub fn nextBessel(n: u64, x: f64, Jn_minus_one: f64, Jn_minus_two: f64) -> f64 {
  let mut sign = 1.;
  let mut z = x;

  if x < 0.0 {
    sign = -1.;
    z = -x;
  };

  if n == 0 {  // we have no choice but to actually compute J0
    return J0(z)
  } else if n == 1 { // we have no choice but to actually compute J1
    return J1(z)
  }
  // for n=2 and up, we can use our recurrence relation
  return sign * (2. * ((n - 1) as f64) / z * Jn_minus_one) - Jn_minus_two
}


pub fn Jn(n: u64, x: f64) -> f64 {
  let mut sign = 1.;
  let mut z: f64 = x;
  if x < 0.0 {
    sign = -1.;
    z = -x;
  };

  if n == 0 {  // we have no choice but to actually compute J0
    return J0(x);
  } else if n == 1 { // we have no choice but to actually compute J1
    return J1(x);
  }

  let mut k = 53;
  let mut pk: f64 = 2. * (n as f64 + k as f64);
  let mut ans: f64 = pk;
  let xk: f64 = z * z;

  while k > 0 {
    pk -= 2.;
    ans = pk - xk / ans;

    k -= 1;
  }

  ans = z / ans;

  pk = 1.;
  let mut pkm1: f64 = 1. / ans;
  k = n - 1;
  let mut r: f64 = 2. * (k as f64);
  let mut pkm2: f64;

  while k > 0 {
    pkm2 = (pkm1 * r - pk * z) / z;
    pk = pkm1;
    pkm1 = pkm2;
    r -= 2.;

    k -= 1;
  }

  if pk.abs() > pkm1.abs() {
    ans = J1(z) / pk;
  } else {
    ans = J0(z) / pkm1;
  }
  return sign * ans;
}

pub fn bessel_first_max(n: u32) -> f64 {
  let bessel_max_array: Vec<f64> = vec![
    0.0,  // n=0
    1.8411845631396737,
    3.054237453259305,
    4.201189419366311,
    5.31755394435717,
    6.4156173973175346, // n=5
    7.50126727562796,
    8.577837671919578,
    9.647422850370246,
    10.711435164143216,
    11.770877850893616, // n=10
    12.82649237913127,
    13.878844191880322,
    14.928375584199912,
    15.975439867036497,
    17.020324300768433, // n=15
    18.06326599063402,
    19.104463207431817,
    20.144083640890184,
    21.182270540354093,
    22.219147365931807, //n=20
    23.25482136749967,
    24.289386377801353,
    25.322925019895663,
    26.355510471827266,
    27.387207891944033, //n=25
  ];
  if n > (bessel_max_array.len()-1) as u32 {
    // ideally you might add a function that uses some minimiser and Jn(x) to find a minimum.
    panic!("Cannot find bessel maximum for this order") // throw an error
  } else {
    bessel_max_array[n as usize]
  }
}