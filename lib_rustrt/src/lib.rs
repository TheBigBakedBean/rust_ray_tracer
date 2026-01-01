use rand::random_range;

// Submodules

pub mod colour;
pub mod hitable_list;
pub mod hitable;
pub mod ray;
pub mod interval;
pub mod camera;

// Constants

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

// Utility Function

pub const fn degrees_to_radians(degrees: f64) -> f64{
    degrees * PI / 180.0
}

/// Returns a random `f64` between `0.0` (inclusive) and `1.0` (exclusive)
pub fn random_f64()->f64{
    random_range(0.0..1.0)
}

/// Returns a random `f64` between `min` (inclusive) and `max` (exclusive)
pub fn random_f64_range(min: f64, max: f64)->f64{
    random_range(min..max)
}

pub fn random_vec3()->vec3math::Vec3{
    vec3math::Vec3::new(random_f64(), random_f64(), random_f64())
}

pub fn random_vec3_range(min: f64, max: f64)->vec3math::Vec3{
    vec3math::Vec3::new(random_f64_range(min, max), random_f64_range(min, max), random_f64_range(min, max))
}