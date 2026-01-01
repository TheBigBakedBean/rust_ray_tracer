use crate::colour::Colour;
use crate::material::Scatterable;
use crate::ray::Ray;
use crate::hitable::HitRecord;

use vec3math::{Vec3, reflect};

use std::option::Option;

#[derive(Debug)]
pub struct Metal{
    albedo: Colour,
}
impl Metal{
    pub fn new(albedo: Colour) -> Self{
        Self{albedo}
    }
}
impl Scatterable for Metal{
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)>{
        let reflected = reflect(&ray_in.dir, &rec.normal);
        Some((
            Ray::new(rec.point, reflected),
            self.albedo,
        ))
    }
}