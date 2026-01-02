use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::colour::Colour;

use std::option::Option;
use std::fmt::Debug;

pub mod lambertian;
pub mod metal;
pub mod dielectric;

pub trait Scatterable: Debug{
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Colour)>;
}