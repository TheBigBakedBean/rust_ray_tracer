use std::fs::File;
use std::io::prelude::*;

use crate::vec3;

pub type Colour = vec3::Vec3;

// Borrow file and colour to avoid consuming the values
pub fn write_color(file: &mut File, colour: &Colour) -> std::io::Result<()>{
    let r = colour.x;
    let g = colour.y;
    let b = colour.z;

    // Convert floating point color values to integer for ppm format (lossy and truncates towards zero)
    let ir = (255.999 * r) as i32;
    let ig = (255.999 * g) as i32;
    let ib = (255.999 * b) as i32;

    file.write(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;

    Ok(())
}