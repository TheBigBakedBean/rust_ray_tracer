use std::fs::File;
use std::io::prelude::*;

// Image resolution configuration
const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

// File output configuration
const FILE_PATH: &str = "renders/ch02_gradient.ppm";

fn main() -> std::io::Result<()>{
    let mut file = File::create(FILE_PATH)?;
    
    file.write(format!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())?;

    // Iterate over pixel coordinates
    for y in 0..IMAGE_HEIGHT{
        print!("\rScanlines remaining: {} ", IMAGE_HEIGHT-y);
        for x in 0..IMAGE_WIDTH{

            // Normalization of pixel coordinates and assignment to colour channels
            let r = f64::from(x) / f64::from(IMAGE_WIDTH - 1);
            let g = f64::from(y) / f64::from(IMAGE_HEIGHT - 1);
            let b = 0.0;

            // Convert floating point color values to integer for ppm format (lossy and truncates towards zero)
            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            file.write(format!("{} {} {}", ir, ig, ib).as_bytes())?;
        }
    }

    println!("\rDone.                                     ");

    Ok(())
}