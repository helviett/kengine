use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
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

