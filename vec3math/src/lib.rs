//! 3D Vector type, used for vectors, points and colours.
// This file implements basic operations and arithmetic

use std::ops::{AddAssign, MulAssign, DivAssign, SubAssign, Add, Sub, Mul, Div, Neg};
use std::fmt;

#[derive(Clone, Copy, Default, Debug)]
pub struct Vec3{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3{
    pub const fn new(x: f64, y: f64, z: f64)->Self{
        Self{x, y, z}
    }
    pub fn length_squared(&self)->f64{
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    pub fn length(&self)->f64{
        self.length_squared().sqrt()
    }

    /// Will panic if a zero vector is passed in, it is the users responsibility to ensure this
    pub fn normalized(&self)->Vec3{
        self / self.length()
    }

    pub fn near_zero(&self)->bool{
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
}

impl fmt::Display for Vec3{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

// References are used in operations to avoid cloning input values
impl Neg for &Vec3{
    type Output = Vec3;
    fn neg(self)->Self::Output{
        Vec3{
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl Add<&Vec3> for &Vec3{
    type Output = Vec3;
    fn add(self, rhs: &Vec3)->Self::Output{
        Vec3{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }    
    }
}
impl Sub<&Vec3> for &Vec3{
    type Output = Vec3;
    fn sub(self, rhs: &Vec3)->Self::Output{
        Vec3{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }    
    }
}
impl Mul<f64> for &Vec3{
    type Output = Vec3;
    fn mul(self, rhs: f64)->Self::Output{
        Vec3{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }    
    }
}
impl Div<f64> for &Vec3{
    type Output = Vec3;
    fn div(self, rhs: f64)->Self::Output{
        Vec3{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }    
    }
}
impl AddAssign<&Vec3> for Vec3{
    fn add_assign(&mut self, rhs: &Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl SubAssign<&Vec3> for Vec3{
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// Sometimes you also want versions without references
impl Neg for Vec3{
    type Output = Vec3;
    fn neg(self)->Self::Output{
        -&self
    }
}
impl Add<Vec3> for Vec3{
    type Output = Vec3;
    fn add(self, rhs: Vec3)->Self::Output{
        &self+&rhs
    }
}
impl Sub<Vec3> for Vec3{
    type Output = Vec3;
    fn sub(self, rhs: Vec3)->Self::Output{
        &self-&rhs
    }
}
impl Mul<f64> for Vec3{
    type Output = Vec3;
    fn mul(self, rhs: f64)->Self::Output{
        &self*rhs
    }
}
impl Div<f64> for Vec3{
    type Output = Vec3;
    fn div(self, rhs: f64)->Self::Output{
        &self/rhs
    }
}
impl AddAssign<Vec3> for Vec3{
    fn add_assign(&mut self, rhs: Vec3) {
        *self+=&rhs
    }
}
impl SubAssign<Vec3> for Vec3{
    fn sub_assign(&mut self, rhs: Vec3) {
        *self-=&rhs
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

// Varients of Mul with f64 as LHS
impl Mul<&Vec3> for f64{
    type Output = Vec3;
    fn mul(self, rhs: &Vec3)->Self::Output{
        rhs*self
    }
}
impl Mul<Vec3> for f64{
    type Output = Vec3;
    fn mul(self, rhs: Vec3)->Self::Output{
        rhs*self
    }
}

// More complex operations
pub fn dot(a: &Vec3, b: &Vec3) -> f64{
    a.x * b.x + a.y * b.y + a.z * b.z
}
pub fn cross(a: &Vec3, b: &Vec3) -> Vec3{
    Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3{
    *v - 2.0 * dot(v, n) * n
}

pub type Point3 = Vec3;