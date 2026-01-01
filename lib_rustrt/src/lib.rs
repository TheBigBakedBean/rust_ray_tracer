// Submodules

pub mod colour;
pub mod hitable_list;
pub mod hitable;
pub mod ray;
pub mod interval;

// Constants

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

// Utility Function

pub const fn degrees_to_radians(degrees: f64) -> f64{
    degrees * PI / 180.0
}