use std::fs::File;
use std::io::prelude::*;

use vec3math::Vec3;

pub type Colour = Vec3;

const MIN_FLOAT_PIXEL_VALUE: f64 = 0.0;
const MAX_FLOAT_PIXEL_VALUE: f64 = 1.0;

// Borrow file and colour to avoid cloning the values
pub fn write_color(file: &mut File, colour: &Colour) -> std::io::Result<()> {
    let r = f64::clamp(colour.x, MIN_FLOAT_PIXEL_VALUE, MAX_FLOAT_PIXEL_VALUE);
    let g = f64::clamp(colour.y, MIN_FLOAT_PIXEL_VALUE, MAX_FLOAT_PIXEL_VALUE);
    let b = f64::clamp(colour.z, MIN_FLOAT_PIXEL_VALUE, MAX_FLOAT_PIXEL_VALUE);

    // Convert floating point color values to integer for ppm format (lossy and truncates towards zero)
    let ir = (255.999 * r) as i32;
    let ig = (255.999 * g) as i32;
    let ib = (255.999 * b) as i32;

    // Inserts a debug comment to ppm file if clamping occurs
    #[cfg(debug_assertions)]
    if r != colour.x || g != colour.y || b != colour.z {
        let ir_unclamped = (255.999 * colour.x) as i32;
        let ig_unclamped = (255.999 * colour.y) as i32;
        let ib_unclamped = (255.999 * colour.z) as i32;

        writeln!(file, "{} {} {} # CLAMPED! {} {} {}", ir, ig, ib, ir_unclamped, ig_unclamped, ib_unclamped)?;
    } else {
        writeln!(file, "{} {} {}", ir, ig, ib)?;
    }

    #[cfg(not(debug_assertions))]
    writeln!(file, "{} {} {}", ir, ig, ib)?;

    Ok(())
}