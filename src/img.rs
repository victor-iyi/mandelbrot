use crate::{img, parser};

use image::ColorType;
use image::{png::PngEncoder, ImageResult};
use num::Complex;

use std::fs::File;

/// Given the row and column of a pixel in the output image, return the corresponding
/// point on the complex plane.
///
/// - `bounds` is a pair giving the width and height of the image in pixels.
///
/// - `pixels` is a (column, row) pair indicating a particular pixel in that image.
///
/// - The `upper_left` and `lower_right` parameters are points on the complex plane
/// designating the area our image covers.
pub fn pixel_to_point(
  bounds: (usize, usize),
  pixel: (usize, usize),
  upper_left: Complex<f64>,
  lower_right: Complex<f64>,
) -> Complex<f64> {
  let (width, height) = (
    lower_right.re - upper_left.re,
    upper_left.im - lower_right.im,
  );
  Complex {
    re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
    im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    // Why subtraction here? pixel.1 increases as we go down,
    // but the imaginary component increases as we go up.
  }
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-left
/// and lower-right corners of the pixel buffer.
pub fn render(
  pixels: &mut [u8],
  bounds: (usize, usize),
  upper_left: Complex<f64>,
  lower_right: Complex<f64>,
) {
  assert!(pixels.len() == bounds.0 * bounds.1);

  for row in 0..bounds.1 {
    for column in 0..bounds.0 {
      let point = img::pixel_to_point(bounds, (column, row), upper_left, lower_right);
      pixels[row * bounds.0 + column] = match parser::escape_time(point, 255) {
        None => 0,
        Some(count) => 255 - count as u8,
      };
    }
  }
}

/// Write the buffer `pixels`, whose dimensions are given by `bounds`,
/// to the file named `filename`.
pub fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> ImageResult<()> {
  let output = File::create(filename)?;

  let encoder = PngEncoder::new(output);
  encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::L8)?;

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_pixel_to_point() {
    assert_eq!(
      pixel_to_point(
        (100, 100),
        (25, 75),
        Complex { re: -1.0, im: 1.0 },
        Complex { re: 1.0, im: -1.0 }
      ),
      Complex { re: -0.5, im: -0.5 }
    );
  }
}
