use std::ops;
use std::convert;
use std::mem;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn dot(a: Vec3, b: Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn proj(a: Vec3, b: Vec3) -> Vec3 {
        Vec3::dot(a, b) / Vec3::dot(b, b) * b
    }

    pub fn rej(a: Vec3, b: Vec3) -> Vec3 {
        a - Vec3::dot(a, b) / Vec3::dot(b, b) * b
    }

    pub fn zero() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0, }
    }

    pub fn one() -> Vec3 {
        Vec3 { x: 1.0, y: 1.0, z: 1.0, }
    }

    pub fn x_axis() -> Vec3 {
        Vec3 { x: 1.0, y: 0.0, z: 0.0, }
    }

    pub fn y_axis() -> Vec3 {
        Vec3 { x: 0.0, y: 1.0, z: 0.0, }
    }

    pub fn z_axis() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 1.0, }
    }

}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, b: Self) -> Self {
        Vec3 { x: self.x + b.x, y: self.y + b.y, z: self.z + b.z }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, b: Self) -> Self {
        Vec3 { x: self.x - b.x, y: self.y - b.y, z: self.z - b.z }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, b: Vec3) -> Vec3 {
        Vec3 { x: self * b.x, y: self * b.y, z: self * b.z }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 { 
        &self.as_ref()[i]
    }
}

impl convert::AsRef<[f32; 3]> for Vec3 {
    fn as_ref(&self) -> &[f32; 3] {
        unsafe { mem::transmute(self) }
    }
}
