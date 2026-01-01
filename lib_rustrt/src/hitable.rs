use std::option::Option;
use std::rc::Rc;

use vec3math::{Vec3, Point3, dot};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::Scatterable;

pub struct HitRecord{
    pub point: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Scatterable>,
    pub t: f64,
    pub front_face: bool,
}
impl HitRecord{
    pub fn new(point: Point3, outward_normal: Vec3, mat: Rc<dyn Scatterable>, t: f64, ray: &Ray) -> Self {

        // Determine if the normal is pointing inward or outward
        let front_face = dot(&ray.dir, &outward_normal) < 0.0;
        let normal = if front_face {outward_normal} else {-outward_normal};

        Self{point, normal, mat, t, front_face}
    }
}

pub trait Hittable: std::fmt::Debug{
    /// Tests if a ray has made contact with the implementor, the implementation must follow the following conditions:
    /// - Should return `Some(HitRecord)` if a collision has occured
    /// - Should return `None` if the ray doesn't collide with the implementor
    /// - If there are 2 intersections, the t value in the `HitRecord` must be the lower value
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}