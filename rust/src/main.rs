extern crate num;
extern crate image;

use num::Complex;
use std::str::FromStr;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use image::ColorType;
use image::png::PNGEncoder;

/// Write the file
///
/// Write the buffer into the file as a PNG image. The shape of the buffer is
/// given by bounds in pixel
fn write_image(filename: &str, buffer: &[u8], bounds: (usize, usize))
    -> Result<(), std::io::Error> {
    // create the output file
    let file = File::create(filename)?;

    // create the encode
    let encoder = PNGEncoder::new(file);
    encoder.encode(&buffer, bounds.0 as u32, bounds.1 as u32,
                   ColorType::Gray(8))?;

    // return Result
    Ok(())
}

/// Render the Mandelbrot set
///
/// maps each pixel in the buffer onto the Complex plane, given the needed
/// bounds for the image. Does apply the actual calculation function.
fn render(buffer: &mut [u8],
          bounds: (usize, usize),
          upper_left: Complex<f64>,
          lower_right: Complex<f64>) {
    // make sure the buffer fits bounds
    assert_eq!(buffer.len(), bounds.0 * bounds.1);

    // apply is_member to each pixel in the buffer
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            // transform the point
            let poi = pixel_to_point(bounds, (column, row),
                                     upper_left, lower_right);

            // match, 255 is the range of greyscale
            buffer[row * bounds.0 + column] =
            match is_member(poi, 255) {
                None => 0,
                Some(count) => 255 - count as u8
            };
        }
    }
}


/// Convert image pixel to Complex number
///
/// bounds specifies the size of the image in pixel and pixel is the pixel
/// that shall be converted to a Complex number. upper_left and lower_right
/// are the bounding points of the Complex plane, where the image should be
/// mapped to.
fn pixel_to_point(bounds: (usize, usize),
                  pixel: (usize, usize),
                  upper_left: Complex<f64>,
                  lower_right: Complex<f64>)
    -> Complex<f64> {
    // get width and height of the image
    let (width, height) = (lower_right.re - upper_left.re,
                           upper_left.im - lower_right.im);

    // return the Complex number.
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 100), (25, 75),
                              Complex {re: -1.0, im: 1.0},
                              Complex {re: 1.0, im: -1.0}),
               Complex {re: -0.5, im: -0.5})
}

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
    assert_eq!(coordinate_to_complex("5.4,6.3"), Some(Complex {re: 5.4, im: 6.3}));
    assert_eq!(coordinate_to_complex(".3,1"), Some(Complex {re: 0.1, im: 1.0}));
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
    assert_eq!(parse_pair::<i32>("200,500", ','), Some((200, 500)));
    assert_eq!(parse_pair::<i32>("200x500", 'x'), Some((200, 500)));
    assert_eq!(parse_pair::<f64>("0.5x1.0", 'x'), Some((0.5, 1.0)));

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


fn main() {
    // get command line args
    let args: Vec<String> = std::env::args().collect();

    // check number of arguments
    if args.len() != 5 {
        writeln!(std::io::stderr(), "Usage: mandelbrot FILE SIZE UPPERLEFT LOWERRIGHT")
            .unwrap();
        writeln!(std::io::stderr(), "Example: {} mandel.png 1000x750 -1.2,0.35 -1,0.2",
                 args[0]).unwrap();

        // exit
        std::process::exit(1);
    }

    // Program was called correctly
    // Parse the arguments
    let bounds = parse_pair(&args[2], 'x')
        .expect("Error parsing image dimensions.");
    let upper_left = coordinate_to_complex(&args[3])
        .expect("Error parsing upper left corner.");
    let lower_right = coordinate_to_complex(&args[4])
        .expect("Error parsing lower right corner.");

    // create the buffer vector and fill with zeros
    let mut buffer = vec![0; bounds.0 * bounds.1];

    // do some benchmarking
    let now = Instant::now();
    // render the image
    // this is the part that takes some time
    render(&mut buffer, bounds, upper_left, lower_right);
    let wall_time = now.elapsed();
    println!("Calculation: {}.{} seconds.",
             wall_time.as_secs(), wall_time.subsec_millis());

    // write the buffer to image
    write_image(&args[1], &buffer, bounds)
        .expect("Error writing png file");
}
