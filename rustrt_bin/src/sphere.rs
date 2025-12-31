use vec3math::{Point3, dot};
use lib_rustrt::hitable::{Hittable, HitRecord};
use lib_rustrt::ray::Ray;
use std::option::Option;

#[derive(Debug, Default, Clone, Copy)]
pub struct Sphere{
    center: Point3,
    radius: f64,
}

impl Sphere{
    pub fn new(center: Point3, radius: f64) -> Self{
        Self {
            center: center,
            radius: f64::max(0.0, radius)
        }
    }
}

impl Hittable for Sphere{
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
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

        let root = (h - sqrtd) / a;
        if root <= ray_tmin || root >= ray_tmax {
            let root = (h + sqrtd) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return None;
            }
        }

        let ray_at_root = ray.at(root);

        Some(HitRecord::new(
            ray_at_root,
            (ray_at_root - self.center) / self.radius,
            root,
            ray,
        ))
    }
}