mod filewrite;
mod constants;
mod colour_funcs;
use constants::{WIDTH, HEIGHT, MAX_ITERATIONS};
use num::complex::Complex;
use rayon::prelude::*;

use crate::filewrite::write_tga;

struct Window {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

fn compute_mandelbrot(
    window: Window,
    output: &mut [[u32; WIDTH]; HEIGHT]
) {
    let Window { left, right, top, bottom } = window;
    output.par_iter_mut().enumerate().for_each(|(y, row)| {
        for (x, pixel) in row.iter_mut().enumerate() {
            // Work out the point in the complex plane that
            // corresponds to this pixel in the output image.
            let c = Complex::new(
                left + (x as f64 * (right - left) / WIDTH as f64),
                top + (y as f64 * (bottom - top) / HEIGHT as f64));
            
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
                *pixel = 0x000000;
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
        bottom: -1.125
    };
    compute_mandelbrot(global_window, &mut image);

    // This zooms in on an interesting bit of detail.
    // let small_window = Window {
    //     left: -0.751085, 
    //     right: -0.734975, 
    //     top: 0.118378, 
    //     bottom: 0.134488
    // };
    // compute_mandelbrot(small_window, &mut image);

    // Stop timing.
    let ms = start.elapsed().as_millis();

    println!("Computing the Mandelbrot set took {} ms.", ms);

    // Write the image to a TGA file.
    write_tga("output.tga", &image);

    println!("Done.");
}

