// 3D Vector type, used for vectors, points and colours.
// This file implements basic operations and arithmetic

use std::ops::{AddAssign, MulAssign, DivAssign, SubAssign, Add, Sub, Mul, Div, Neg};
use std::fmt;

#[derive(Clone, Copy, Default, Debug)]
pub struct Vec3{
    pub x: f64,  // f64 is used for higher precision leading to higher quality renders
    pub y: f64,
    pub z: f64,
}
impl Vec3{
    pub fn new(x: f64, y: f64, z: f64)->Self{
        Self{x, y, z}
    }
    pub fn length_squared(&self)->f64{
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    pub fn length(&self)->f64{
        self.length_squared().sqrt()
    }

    pub fn dot(&self, b: &Vec3) -> f64{
        self.x * b.x + self.y * b.y + self.z * b.z
    }
    pub fn cross(&self, b: &Vec3) -> Vec3{
        Vec3 {
            x: self.y * b.z - self.z * b.y,
            y: self.z * b.x - self.x * b.z,
            z: self.x * b.y - self.y * b.x,
        }
    }
    pub fn normalize(&self)->Vec3{
        *self / self.length()
    }
}

impl fmt::Display for Vec3{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Neg for Vec3{
    type Output = Vec3;
    fn neg(self)->Self::Output{
        Vec3{
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl Add<Vec3> for Vec3{
    type Output = Vec3;
    fn add(self, rhs: Vec3)->Self::Output{
        Vec3{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }    
    }
}
impl Sub<Vec3> for Vec3{
    type Output = Vec3;
    fn sub(self, rhs: Vec3)->Self::Output{
        Vec3{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }    
    }
}
impl Mul<f64> for Vec3{
    type Output = Vec3;
    fn mul(self, rhs: f64)->Self::Output{
        Vec3{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }    
    }
}
impl Div<f64> for Vec3{
    type Output = Vec3;
    fn div(self, rhs: f64)->Self::Output{
        Vec3{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }    
    }
}
impl AddAssign<Vec3> for Vec3{
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl SubAssign<Vec3> for Vec3{
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl MulAssign<f64> for Vec3{
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl DivAssign<f64> for Vec3{
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

pub type Point3 = Vec3;