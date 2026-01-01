use crate::colour::Colour;
use crate::random_unit_vector;
use crate::material::Scatterable;
use crate::ray::Ray;
use crate::hitable::HitRecord;

use vec3math::reflect;

use std::option::Option;

#[derive(Debug)]
pub struct Metal{
    albedo: Colour,
    fuzz: f64
}
impl Metal{
    pub fn new(albedo: Colour, fuzz: f64) -> Self{
        Self{albedo, fuzz}
    }
}
impl Scatterable for Metal{
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)>{
        let reflected = reflect(&ray_in.dir, &rec.normal);
        let reflected = reflected.normalized() + (self.fuzz * random_unit_vector());
        Some((
            Ray::new(rec.point, reflected),
            self.albedo,
        ))
    }
}