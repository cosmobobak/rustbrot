use crate::constants::MAX_ITERATIONS;

#[allow(dead_code)]
pub const fn crazy(iterations: usize) -> u32 {
    0xFF_FF_FF - (iterations as u32 * 0xFF_FF_FF / MAX_ITERATIONS as u32)
}

#[allow(dead_code)]
pub fn log_greyscale(iterations: usize) -> u32 {
    // this function scales up low iterations, and down high iterations
    const FLOAT_MAX_ITERATIONS: f32 = MAX_ITERATIONS as f32;
    /* float_iterations is in the range 0..MAX_ITERATIONS */
    let float_iterations = iterations as f32;
    /* log_scaled is in the range 0..log(MAX_ITERATIONS) */
    let log_scaled = float_iterations.log2();
    /* scale_factor is in 0..1 */
    let scale_factor = log_scaled / FLOAT_MAX_ITERATIONS.log2();

    let blue  = 255 - (scale_factor * 255.0) as u8;
    let green = 255 - (scale_factor * 255.0) as u8;
    let red   = 255 - (scale_factor * 255.0) as u8;
    u32::from(blue) << 16 | u32::from(green) << 8 | u32::from(red)
}

#[allow(dead_code)]
pub fn log(iterations: usize) -> u32 {
    // this function scales up low iterations, and down high iterations
    const FLOAT_MAX_ITERATIONS: f32 = MAX_ITERATIONS as f32;
    /* float_iterations is in the range 0..MAX_ITERATIONS */
    let float_iterations = iterations as f32;
    /* log_scaled is in the range 0..log(MAX_ITERATIONS) */
    let log_scaled = float_iterations.log2();
    /* scale_factor is in 0..1 */
    let scale_factor = log_scaled / FLOAT_MAX_ITERATIONS.log2();

    let red_one  = 0xb8 as f32;
    let green_one = 0x4e as f32;
    let blue_one = 0x0b as f32;

    let red_two  = 0x07 as f32;
    let green_two = 0x20 as f32;
    let blue_two = 0xe3 as f32;
    
    let red = red_one.mul_add(1.0 - scale_factor, red_two * scale_factor) as u8;
    let green = green_one.mul_add(1.0 - scale_factor, green_two * scale_factor) as u8;
    let blue = blue_one.mul_add(1.0 - scale_factor, blue_two * scale_factor) as u8;
    u32::from(blue) << 16 | u32::from(green) << 8 | u32::from(red)
}

#[allow(dead_code)]
pub fn exp_greyscale(iterations: usize) -> u32 {
    // this function scales up high iterations, and down low iterations
    const FLOAT_MAX_ITERATIONS: f32 = MAX_ITERATIONS as f32;
    /* float_iterations is in the range 0..MAX_ITERATIONS */
    let float_iterations = iterations as f32;
    /* exp_scaled is in the range 0..exp(MAX_ITERATIONS) */
    let log_scaled = float_iterations.exp();
    /* scale_factor is in 0..1 */
    let scale_factor = log_scaled / FLOAT_MAX_ITERATIONS.exp();

    let blue  = 255 - (scale_factor * 255.0) as u8;
    let green = 255 - (scale_factor * 255.0) as u8;
    let red   = 255 - (scale_factor * 255.0) as u8;
    u32::from(blue) << 16 | u32::from(green) << 8 | u32::from(red)
}

#[allow(dead_code)]
pub fn edward(iterations: usize) -> u32 {
    let scale_factor = iterations as f32 / MAX_ITERATIONS as f32;
    
    let blue  = if scale_factor < 0.5 {
        255
    } else {
        let new_scalef = (scale_factor - 0.5) * 2.0;
        255 - (new_scalef * 255.0) as u8
    };
    let red   = if scale_factor > 0.5 {
        255
    } else {
        let new_scalef = scale_factor * 2.0;
        255 - (new_scalef * 255.0) as u8
    };
    let green = if scale_factor > 0.5 {
        let new_scalef = (scale_factor - 0.5) * 2.0;
        255 - (new_scalef * 255.0) as u8
    } else {
        let new_scalef = scale_factor * 2.0;
        255 - (new_scalef * 255.0) as u8
    };
    u32::from(blue) << 16 | u32::from(green) << 8 | u32::from(red)
}

#[allow(dead_code)]
pub const fn wiki_modulo(iterations: usize) -> u32 {
    #[allow(clippy::identity_op)]
    const COLOURS: [u32; 16] = [
             66 | ( 30 << 8) | ( 15 << 16), // brown 3
             25 | (  7 << 8) | ( 26 << 16), // dark violett
              9 | (  1 << 8) | ( 47 << 16), // darkest blue
              4 | (  4 << 8) | ( 73 << 16), // blue 5
              0 | (  7 << 8) | (100 << 16), // blue 4
             12 | ( 44 << 8) | (138 << 16), // blue 3
             24 | ( 82 << 8) | (177 << 16), // blue 2
             57 | (125 << 8) | (209 << 16), // blue 1
            134 | (181 << 8) | (229 << 16), // blue 0
            211 | (236 << 8) | (248 << 16), // lightest blue
            241 | (233 << 8) | (191 << 16), // lightest yellow
            248 | (201 << 8) | ( 95 << 16), // light yellow
            255 | (170 << 8) | (  0 << 16), // dirty yellow
            204 | (128 << 8) | (  0 << 16), // brown 0
            153 | ( 87 << 8) | (  0 << 16), // brown 1
            106 | ( 52 << 8) | (  3 << 16), // brown 2
    ];
    const COLOURS_LEN: usize = COLOURS.len();
    let idx = iterations % COLOURS_LEN;
    COLOURS[idx]
}