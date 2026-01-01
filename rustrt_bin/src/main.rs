use lib_rustrt::camera::Camera;
use lib_rustrt::hitable_list::HittableList;
use vec3math::Point3;

use std::rc::Rc;
use std::fs::File;

use crate::sphere::Sphere;

mod sphere;

const FILE_PATH: &str = "samples/ch09_matte_grey_sphere.ppm";

fn main() -> std::io::Result<()>{
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let mut file = File::create(FILE_PATH)?;

    let cam = Camera::new(400, 16.0 / 9.0, 100, 10);

    cam.render(&world, &mut file)?;

    Ok(())
}