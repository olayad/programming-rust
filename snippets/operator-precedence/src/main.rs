// Mandelbrot set generator from "Programming Rust", with I/O removed (the
// original code writes a PNG file).

use std::env;
use std::str::FromStr;
use num_complex::Complex;

fn main() {
    let bounds = (100, 200);
    let pixel = (25, 175);
    let upper_left = Complex { re: -1.0, im: 1.0 };
    let lower_right = Complex { re: 1.0, im: -1.0 };


    let(width, height) = (lower_right.re - upper_left.re,
                          upper_left.im - lower_right.im);
    println!("width:{} height:{}", width, height);
    let z = Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    };
    println!("z:{}", z);
}