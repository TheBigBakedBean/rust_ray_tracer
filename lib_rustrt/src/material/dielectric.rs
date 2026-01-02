use crate::colour::Colour;
use crate::material::Scatterable;
use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::random_f64;

use vec3math::graphics::{refract, reflect};
use vec3math::dot;

use std::option::Option;

#[derive(Debug)]
pub struct Dielectric{
    refraction_index:  f64
}
impl Dielectric{
    pub fn new(refraction_index: f64) -> Self{
        Self { refraction_index }
    }
}
impl Scatterable for Dielectric{
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)>{
        let ri = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let unit_direction = ray_in.dir.normalized();
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, ri) > random_f64() {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, ri)
        };

        Some((
            Ray::new(rec.point, direction),
            Colour::new(1.0, 1.0, 1.0),
        ))
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}