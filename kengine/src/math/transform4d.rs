use super::mat3::Mat3;
use super::vec3::Vec3;
use std::ops;
use std::mem;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform4D {
    e: [[f32; 4]; 4],
}

impl Transform4D {
    pub fn from_mat_vec(transform: Mat3, translation: Vec3) -> Transform4D {
        let c0 = &transform.c0;
        let c1 = &transform.c1;
        let c2 = &transform.c2;
        Transform4D {
            e: [
                [c0.x, c0.y, c0.z, 0.0],
                [c1.x, c1.y, c1.z, 0.0],
                [c2.x, c2.y, c2.z, 0.0],
                [translation.x, translation.y, translation.z, 1.0],
            ],
        }
    }

    pub fn new(
        n00: f32, n01: f32, n02: f32, n03: f32,
        n10: f32, n11: f32, n12: f32, n13: f32,
        n20: f32, n21: f32, n22: f32, n23: f32,
    ) -> Transform4D {
        Transform4D {
            e: [
                [n00, n10, n20, 0.0],
                [n01, n11, n21, 0.0],
                [n02, n12, n22, 0.0],
                [n03, n13, n23, 1.0],
            ],
        }
    }

    pub fn inv(&self) -> Transform4D {
        let a = *unsafe { &*(self.e[0].as_ptr() as *const Vec3) };
        let b = *unsafe { &*(self.e[1].as_ptr() as *const Vec3) };
        let c = *unsafe { &*(self.e[2].as_ptr() as *const Vec3) };
        let d = *unsafe { &*(self.e[3].as_ptr() as *const Vec3) };

        let s = Vec3::cross(a, b);
        let t = Vec3::cross(c, d);

        let inv_det = 1.0f32 / Vec3::dot(s, c);

        let s = inv_det * s;
        let t = inv_det * t;
        let v = inv_det * c;

        let r0 = Vec3::cross(b, v);
        let r1 = Vec3::cross(v, a);

        Transform4D::new(
            r0.x, r0.y, r0.z, -Vec3::dot(b, t),
            r1.x, r1.y, r1.z, Vec3::dot(a, t),
            s.x, s.y, s.z, -Vec3::dot(d, s)
        )
    }
}

impl ops::Mul<Vec3> for Transform4D {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0][0] * v.x + self.e[1][0] * v.y + self.e[2][0] * v.z + self.e[3][0],
            self.e[0][1] * v.x + self.e[1][1] * v.y + self.e[2][1] * v.z + self.e[3][1],
            self.e[0][2] * v.x + self.e[1][2] * v.y + self.e[2][2] * v.z + self.e[3][2],
        )
    }
}

impl ops::Mul<Transform4D> for Transform4D {
    type Output = Transform4D;

    fn mul(self, t: Transform4D) -> Self::Output {
        Transform4D::new(
            self.e[0][0] * t.e[0][0] + self.e[1][0] * t.e[0][1] + self.e[2][0] * t.e[0][2],
            self.e[0][0] * t.e[1][0] + self.e[1][0] * t.e[1][1] + self.e[2][0] * t.e[1][2],
            self.e[0][0] * t.e[2][0] + self.e[2][0] * t.e[2][1] + self.e[2][0] * t.e[2][2],
            self.e[0][0] * t.e[3][0] + self.e[1][0] * t.e[3][1] + self.e[2][0] * t.e[3][2] + self.e[3][0],
            self.e[0][1] * t.e[0][0] + self.e[1][1] * t.e[0][1] + self.e[2][1] * t.e[0][2],
            self.e[0][1] * t.e[1][0] + self.e[1][1] * t.e[1][1] + self.e[2][1] * t.e[1][2],
            self.e[0][1] * t.e[2][0] + self.e[2][1] * t.e[2][1] + self.e[2][1] * t.e[2][2],
            self.e[0][1] * t.e[3][0] + self.e[1][1] * t.e[3][1] + self.e[2][1] * t.e[3][2] + self.e[3][1],
            self.e[0][2] * t.e[0][0] + self.e[1][2] * t.e[0][1] + self.e[2][2] * t.e[0][2],
            self.e[0][2] * t.e[1][0] + self.e[1][2] * t.e[1][1] + self.e[2][2] * t.e[1][2],
            self.e[0][2] * t.e[2][0] + self.e[2][2] * t.e[2][1] + self.e[2][2] * t.e[2][2],
            self.e[0][2] * t.e[3][0] + self.e[1][2] * t.e[3][1] + self.e[2][2] * t.e[3][2] + self.e[3][2],
        )
    }
}
