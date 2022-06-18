#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]

mod colour_funcs;
mod constants;
mod filewrite;
mod types;
use constants::{HEIGHT, MAX_ITERATIONS, WIDTH};
use num::complex::Complex;
use rayon::prelude::*;
use types::{Image, Window};

use crate::filewrite::write_tga;

fn render_mandelbrot(window: Window, output: &mut Image) {
    let Window {
        left,
        right,
        top,
        bottom,
    } = window;
    output.par_iter_mut().enumerate().for_each(|(y, row)| {
        for (x, pixel) in row.iter_mut().enumerate() {
            // Work out the point in the complex plane that
            // corresponds to this pixel in the output image.
            let c = Complex::new(
                left + (x as f64 * (right - left) / WIDTH as f64),
                top + (y as f64 * (bottom - top) / HEIGHT as f64),
            );

            let mut z = Complex::new(0.0, 0.0);

            // Iterate z = z^2 + c until z moves more than 2 units
            // away from (0, 0), or we've iterated too many times.
            let mut iterations = 0;
            while (z.norm_sqr() < 4.0) && (iterations < MAX_ITERATIONS) {
                z = z * z + c;
                iterations += 1;
            }

            if iterations == MAX_ITERATIONS {
                // z didn't escape from the circle.
                // This point is in the Mandelbrot set.
                // Colour it black.
                *pixel = 0x00_00_00;
            } else {
                // z escaped within less than MAX_ITERATIONS
                // iterations. This point isn't in the set.
                // we can use the number of iterations to
                // determine a colour for the pixel.
                *pixel = colour_funcs::wiki_modulo(iterations);
            }
        }
    });
}

fn fraction_black(image: &Image) -> f64 {
    let black_pixels = image
        .iter()
        .map(|row| row.iter().filter(|&&pixel| pixel == 0x00_00_00).count())
        .sum::<usize>() as f64;
    black_pixels / (WIDTH * HEIGHT) as f64
}

fn main() {
    println!("Please wait...");
    let mut image = Box::new([[0; WIDTH]; HEIGHT]);

    // start timing
    let start = std::time::Instant::now();

    // Compute the Mandelbrot set.
    let global_window = Window {
        left: -2.0,
        right: 1.0,
        top: 1.125,
        bottom: -1.125,
    };
    let window = global_window;

    // This zooms in on an interesting bit of detail.
    // let small_window = Window {
    //     left: -0.751085,
    //     right: -0.734975,
    //     top: 0.118378,
    //     bottom: 0.134488
    // };
    // let window = small_window;

    render_mandelbrot(window, &mut image);

    // Stop timing.
    let ms = start.elapsed().as_millis();

    println!("Computing the Mandelbrot set took {} ms.", ms);

    // Write the image to a TGA file.
    write_tga("output.tga", &image);

    println!("Done.");

    // Print the percentage of black pixels.
    let frac = fraction_black(&image);
    println!("{:.2}% of the image is black.", frac * 100.0);
    let viewport_area = (window.right - window.left) * (window.top - window.bottom);
    let black_area = frac * viewport_area;
    println!("The area of the Mandelbrot set is {:.5}.", black_area);
}
