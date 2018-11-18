extern crate num;
use num::Complex;
use std::str::FromStr;

/// Parse a coordinate pair from a string.
///
/// s has to be a string of the form <left><separator><right> and will return
/// a coordinate tuple of (left, right). This can be used to map between
/// image coordinates and the Complex plane or read the bound specs given
/// by the user.
//fn parse_str<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {

//}

/// determine if c is part of the Mandelbrot set
///
/// This will be done by running at most 'i' iterations.
/// In Case this limit is reached, return None, assuming
/// c is a member. Else return the Some(i).
fn is_member(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        z = z*z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

#[test]
fn test_is_member() {
    assert_eq!(is_member(Complex {re: 1.0, im: 1.0}, 500), Some(1));
    assert_eq!(is_member(Complex {re: 0.1, im: -0.3}, 500), None);
}


/// Test some complex numbers for creating tests
fn main() {
    println!("(1.0 + 1.0i): {:?}", is_member(Complex {re: 1.0, im: 1.0}, 500));
    println!("(0.1 - 0.3i): {:?}", is_member(Complex {re: 0.1, im: -0.3}, 500));
}
