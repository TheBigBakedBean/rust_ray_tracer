use std::fs::File;
use std::io::prelude::*;

mod vec3;
mod colour;
mod ray;

// File output configuration
const FILE_PATH: &str = "renders/ch04_gradient.ppm";

// Image resolution configuration
const IMAGE_WIDTH: i32 = 400;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

// Camera configuration
const FOCAL_LENGTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const CAMERA_CENTER: vec3::Point3 = vec3::Point3::new(0.0, 0.0, 0.0);

fn ray_color(ray: &ray::Ray) -> colour::Colour{
    let normalized_direction = ray.dir.normalized();
    let a = 0.5*(normalized_direction.y + 1.0);
    colour::Colour::new(1.0, 1.0, 1.0) * (1.0 - a) + (colour::Colour::new(0.5, 0.7, 1.0) * a)
}

fn main() -> std::io::Result<()>{

    // Calculate image height, ensuring it's at least 1
    let image_height = (f64::from(IMAGE_WIDTH) / ASPECT_RATIO) as i32;
    let image_height = if image_height < 1 {1} else {image_height};

    let viewport_width = VIEWPORT_HEIGHT * ASPECT_RATIO;

    // Horizontal and Vertical vectors along the edges of the viewport that intersect at the origin
    let viewport_horizontal_vector = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_vertical_vector = vec3::Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);

    // Horizontal and Vertical vectors between adjacent pixels
    let pixel_delta_horizontal = viewport_horizontal_vector / IMAGE_WIDTH.into();
    let pixel_delta_vertical = viewport_vertical_vector / image_height.into();

    // Calculate the upper left corner of the viewport and the 0,0 pixel
    let viewport_upper_left = CAMERA_CENTER - vec3::Vec3::new(0.0, 0.0, FOCAL_LENGTH) - viewport_horizontal_vector / 2.0 - viewport_vertical_vector / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_horizontal + pixel_delta_vertical) * 0.5;

    let mut file = File::create(FILE_PATH)?;
    
    file.write(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, image_height).as_bytes())?;

    // Iterate over pixel coordinates
    for y in 0..image_height{
        print!("\rScanlines remaining: {} ", image_height - y);
        for x in 0..IMAGE_WIDTH{
            let pixel_center = pixel00_loc + (pixel_delta_horizontal * x.into()) + (pixel_delta_vertical * y.into());
            let ray_direction = pixel_center - CAMERA_CENTER;
            let ray = ray::Ray::new(CAMERA_CENTER, ray_direction);

            let pixel_colour = ray_color(&ray);
            colour::write_color(&mut file, &pixel_colour)?;
        }
    }

    println!("\rDone.                                     ");

    Ok(())
}