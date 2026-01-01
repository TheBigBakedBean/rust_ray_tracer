use lib_rustrt::camera::Camera;
use lib_rustrt::hitable_list::HittableList;
use lib_rustrt::material::{lambertian::Lambertian, metal::Metal};
use lib_rustrt::colour::Colour;

use vec3math::Point3;

use std::rc::Rc;
use std::fs::File;

use crate::sphere::Sphere;

mod sphere;

const FILE_PATH: &str = "renders/ch10_multimaterial_spheres.ppm";

fn main() -> std::io::Result<()>{
    let material_ground = Rc::new(Lambertian::new(Colour::new(0.3, 0.8, 0.3)));
    let material_center = Rc::new(Lambertian::new(Colour::new(0.8, 0.1, 0.1)));
    let material_left = Rc::new(Metal::new(Colour::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Colour::new(0.8, 0.6, 0.2)));

    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    )));

    let mut file = File::create(FILE_PATH)?;

    let cam = Camera::new(
        400,
        16.0 / 9.0,
        100,
        10
    );

    cam.render(&world, &mut file)?;

    Ok(())
}