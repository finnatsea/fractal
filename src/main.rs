extern crate image;
extern crate num_complex;

use image::{ImageBuffer, Rgb};
use num_complex::Complex;
use std::path::Path;

fn main() {
    let width = 800;
    let height = 800;
    let max_iterations = 1000;

    let mut imgbuf = ImageBuffer::new(width, height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cx = (x as f64 / width as f64) * 3.5 - 2.5;
        let cy = (y as f64 / height as f64) * 3.5 - 1.75;
        let c = Complex::new(cx, cy);
        let mut z = Complex::new(0.0, 0.0);

        let mut iterations = 0;
        while iterations < max_iterations && z.norm() <= 2.0 {
            z = z * z + c;
            iterations += 1;
        }

        let color = if iterations == max_iterations {
            [0, 0, 0]
        } else {
            let v = (iterations as f64 / max_iterations as f64) * 255.0;
            [v as u8, v as u8, v as u8]
        };

        *pixel = Rgb(color);
    }

    let path = Path::new("mandelbrot.png");
    imgbuf.save(path).unwrap();
}
