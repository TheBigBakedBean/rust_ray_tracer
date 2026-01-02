use vec3math::{Point3, dot};
use lib_rustrt::hitable::{Hittable, HitRecord};
use lib_rustrt::ray::Ray;
use lib_rustrt::interval::Interval;
use lib_rustrt::material::Scatterable;

use std::option::Option;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Sphere{
    center: Point3,
    radius: f64,
    mat: Rc<dyn Scatterable>,
}

impl Sphere{
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Scatterable>) -> Self{
        // TODO:
        // Initialise the material pointer `mat`.

        Self {
            center: center,
            radius: f64::max(0.0, radius),
            mat
        }
    }
}

impl Hittable for Sphere{
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let offset = self.center - ray.orig;

        // Determine input variables for quadratic function
        let a = ray.dir.length_squared();
        let h = dot(&ray.dir, &offset);
        let c = offset.length_squared() - self.radius*self.radius;

        let discriminant = h*h - a*c;
        if discriminant < 0.0{
            return None;
        }

        // Square rooting discriminant here avoids doing it twice if there are 2 roots
        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let ray_at_root = ray.at(root);

        Some(HitRecord::new(
            ray_at_root,
            (ray_at_root - self.center) / self.radius,
            self.mat.clone(),
            root,
            ray,
        ))
    }
}