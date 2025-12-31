use vec3math::{Vec3, Point3};

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray{
    pub orig: Point3,
    pub dir: Vec3,
}
impl Ray{
    pub fn new(orig: Point3, dir: Vec3) -> Self{
        Self{orig, dir}
    }

    // Returns the location of the ray at a given time
    pub fn at(&self, t: f64) -> Point3{
        self.orig + self.dir * t
    }
}