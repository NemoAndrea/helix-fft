// collection of bessel functions that are required for Helix-FFt
// written by Nemo Andrea in 2020.
// The approaches below are probably not the best way to use these functions for general use,
// so I would be careful in using these functions for your own project.

extern crate num;  // complex numbers
use num::traits::{ Pow };
use std::f64::consts::PI;

// for the validity of the calculation, see http://www.mhtlab.uwaterloo.ca/courses/me755/web_chap4.pdf
pub fn J0(x:f64) -> f64{
  let mut z = x;
  if x < 0.0 { z = -x }  // we ensure z = |x|, as J0(x) = J0(-x) anyway
  if z < 2.0 { // in domain x{0,2} we use a simple series expansion
    return 1. - z.pow(2) / 4. + z.pow(4) / 64.
  }

  // else we are domain x{2, inf}, where we use a different approximation
  return (1. / (PI * z)).sqrt() * ( P0(z) * (z.cos() + z.sin()) - Q0(z) * ( - z.cos() + z.sin()) )
}

// for the validity of the calculation, see http://www.mhtlab.uwaterloo.ca/courses/me755/web_chap4.pdf
pub fn J1(x:f64) -> f64{
  let mut z = x;
  let mut sign = 1.;
  if x < 0.0 {
    z = -x;
    sign = -1.;
  }  // we ensure z = |x|, as J1(x) = -J1(-x) anyway
  if z < 2.0 { // in domain x{0,2} we use a simple series expansion
    return sign * ( z/2. - z.pow(3)/16. + z.pow(5)/384.  )
  }
  return sign * (1. / (PI * z)).sqrt() * ( P1(z)*(z.sin() - z.cos()) - Q1(z)*(z.sin() + z.cos()) )
}


// computes the next bessel "Jn(x)" using the values of Jn_minus_one(x) and Jn_minus_two(x) (at same x)
// this makes use of the recurrence relation Jn(x) = 2*(n-1) / x * Jn_minus_one(x) - Jn_minus_two(x)
// as described in:
// Goldstein, M., and R. M. Thaler. "Recurrence techniques for the calculation of Bessel functions."
// Mathematics of Computation 13.66 (1959): 102-108.
pub fn nextBessel (n: u64, x:f64 , Jn_minus_one: f64, Jn_minus_two: f64) -> f64 {
  if n==0 {  // we have no choice but to actually compute J0
    return J0(x)
  } else if n==1 { // we have no choice but to actually compute J1
    return J1(x)
  }
  // for n=2 and up, we can use our recurrence relation
  return 2. * ( (n-1) as f64 ) / x * Jn_minus_one - Jn_minus_two
}

// for definitions of Q0, Q1, P0, P1 see http://www.mhtlab.uwaterloo.ca/courses/me755/web_chap4.pdf
//TODO: get rid of fractions (pointless computations; they are constants)

//https://www.wolframalpha.com/input/?i=%283%2F%28128x%5E2%29%29+*+%281+-+5%5E2*7%5E2%2F%2812*64*x%5E2%29++*+%281-9%5E2*11%5E2%2F%2830*64*x%5E2%29%29%29
fn P0(x:f64) -> f64 {
  return 1. - 3. / (128. * x.pow(2)) + 1225. / (32768. * x.pow(4)) - 800415. / (4194304. * x.pow(6))
}

//https://www.wolframalpha.com/input/?i=-1%2F%288x%29+*+%281+-+3%5E2*5%5E2%2F%283*2*64*x%5E2%29+*+%281-+7%5E2*9%5E2%2F%285*4*64*x%5E2%29%29
fn Q0(x:f64) -> f64 {
  return -1./(8. * x) + 19200. / (262144. * x.pow(3)) - 59535. / (262144. * x.pow(5))
}

//https://www.wolframalpha.com/input/?i=1+%2B+15%2F%28128*x%5E2%29+*+%281+-+21*45+%2F+%2812*64*x%5E2%29+*+%281+-+77*117+%2F+%286*5*64*x%5E2%29+%29+%29
fn P1(x:f64) -> f64 {
  return 1. + 15./ (128. * x.pow(2)) - 4725./(32768. * x.pow(4)) + 2837835./ (4194304. * x.pow(6))
}

//https://www.wolframalpha.com/input/?i=3%2F%288*x%29+*+%281+-+35%2F%282*64*x%5E2%29+*+%281+-+45*77%2F%284*5*64*x%5E2%29++%29+%29
fn Q1(x:f64) -> f64 {
  return 1./(8. * x) - 26880. / (262144. * x.pow(3)) + 72765. / (262144. * x.pow(5))
}