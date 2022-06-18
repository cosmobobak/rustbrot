// The size of the image to generate.
const SCALEF: usize = 1;
pub const WIDTH: usize = 1920 * SCALEF;
pub const HEIGHT: usize = 1200 * SCALEF;

// The number of times to iterate before we assume that a point isn't in the
// Mandelbrot set.
// (You may need to turn this up if you zoom further into the set.)
pub const MAX_ITERATIONS: usize = 5000;
