use crate::colour::Colour;
use crate::material::Scatterable;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::random_unit_vector;

use std::option::Option;

#[derive(Debug)]
pub struct Lambertian{
    albedo: Colour,
}

impl Lambertian{
    pub fn new(albedo: Colour) -> Self{
        Self{albedo}
    }
}

impl Scatterable for Lambertian{
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)>{
        let mut scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((
            Ray::new(rec.point, scatter_direction),
            self.albedo,
        ))
    }
}