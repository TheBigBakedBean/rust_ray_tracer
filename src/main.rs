use std::fs::File;
use std::io::prelude::*;

mod vec3;
mod colour;

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
            let pixel_colour = colour::Colour::new(
                f64::from(x) / f64::from(IMAGE_WIDTH - 1),
                f64::from(y) / f64::from(IMAGE_HEIGHT - 1),
                0.0
            );

            colour::write_color(&mut file, &pixel_colour)?;
        }
    }

    println!("\rDone.                                     ");

    Ok(())
}