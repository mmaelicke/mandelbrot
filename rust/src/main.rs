extern crate num;
use num::Complex;
use std::str::FromStr;


/// Convert coordinate pair to Complex
///
/// In order to transform between image coordinates and the Complex plane,
/// convert a given coordinate tuple string to a Complex number
fn coordinate_to_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        None => None,
        Some((re, im)) => Some(Complex {re, im})
    }
}

#[test]
fn test_coordinate_to_complex() {
    assert_eq!(coordinate_to_complex("5.4,6.3"), Complex {re: 5.4, im: 6.3});
    assert_eq!(coordinate_to_complex(".3,1"), Complex {re: 0.1, im: 1.0});
    assert_eq!(coordinate_to_complex(",6.66"), None);
}

/// Parse a coordinate pair from a string.
///
/// s has to be a string of the form <left><separator><right> and will return
/// a coordinate tuple of (left, right). This can be used to map between
/// image coordinates and the Complex plane or read the bound specs given
/// by the user.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>(",", ','), None);
    assert_eq!(parse_pair::<i32>("6,", ','), None);
    assert_eq!(parse_pair::<i32>(",500", ','), None);
    assert_eq!(parse_pair::<i32>("200,500", ','), (200, 500));
    assert_eq!(parse_pair::<i32>("200x500", 'x'), (200, 500));
    assert_eq!(parse_pair::<f64>("0.5x1.0", 'x'), (0.5, 1.0));

}

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
    println!("Test parsing");
    println!("20.5,40.4 => {:?}", coordinate_to_complex("20.5,40.4"));

    println!("Test Loop:");
    println!("(1.0 + 1.0i): {:?}", is_member(Complex {re: 1.0, im: 1.0}, 500));
    println!("(0.1 - 0.3i): {:?}", is_member(Complex {re: 0.1, im: -0.3}, 500));
}
