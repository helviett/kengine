use std::ops;
use super::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat3 {
    e: [[f32; 3]; 3],
}

impl Mat3 {
    pub fn from_cols(col1: Vec3, col2: Vec3, col3: Vec3) -> Mat3 {
        Mat3 {
            e: [
                [col1.x, col1.y, col1.z],
                [col2.x, col2.y, col2.z],
                [col3.x, col3.y, col3.z]
            ]
        }
    }

    pub fn col(&self, index: usize) -> Vec3 {
        Vec3 { x: self.e[index][0], y: self.e[index][1], z: self.e[index][2] }
    }

    pub fn row(&self, index: usize) -> Vec3 {
        Vec3 { x: self.e[0][index], y: self.e[1][index], z: self.e[2][index], }
    }

    pub fn identity() -> Mat3 {
        Mat3 {
            e: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ]
        }
    }

    pub fn zero() -> Mat3 {
        Mat3 {
            e: [
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
            ]
        }
    }

    pub fn inv(&self) -> Mat3 {
        let a = self.col(0);
        let b = self.col(1);
        let c = self.col(2);

        let bxc = Vec3::cross(b, c);
        let cxa = Vec3::cross(c, a);
        let axb = Vec3::cross(a, b);

        let inv_det = 1.0 / Vec3::dot(axb, c);
        return Mat3 {
            e: [
                [bxc.x * inv_det, cxa.x * inv_det, axb.x * inv_det],
                [bxc.y * inv_det, cxa.y * inv_det, axb.y * inv_det],
                [bxc.z * inv_det, cxa.z * inv_det, axb.z * inv_det],
            ],
        }
    }

    pub fn rot_x(rads: f32) -> Mat3 {
        let sin = rads.sin();
        let cos = rads.cos();

        Mat3 {
            e: [
                [1.0, 0.0, 0.0],
                [0.0, cos, sin],
                [0.0, -sin, cos],
            ],
        }
    }

    pub fn rot_y(rads: f32) -> Mat3 {
        let sin = rads.sin();
        let cos = rads.cos();

        Mat3 {
            e: [
                [cos, 0.0, sin],
                [0.0, 1.0, 0.0],
                [-sin, 0.0, cos],
            ],
        }
    }

    pub fn rot_z(rads: f32) -> Mat3 {
        let sin = rads.sin();
        let cos = rads.cos();

        Mat3 {
            e: [
                [cos, -sin, 0.0],
                [sin, cos, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rot(rads: f32, a: Vec3) -> Mat3 {
        let c = rads.cos();
        let s = rads.sin();
        let d = 1.0 - c;

        let x = a.x * d;
        let y = a.y * d;
        let z = a.z * d;
        let axay = x * a.y;
        let axaz = x * a.z;
        let ayaz = y * a.z;

        Mat3 {
            e: [
                [c + x * a.x, axay + s * a.z, axaz - s * a.y],
                [axay - s * a.z, c + y * a.y, ayaz + s * a.x],
                [axaz + s * a.y, ayaz - s * a.x, c + z * a.z],
            ],
        }
    }

    pub fn reflect(a: Vec3) -> Mat3 {
        let x = -2.0 * a.x;
        let y = -2.0 * a.y;
        let z = -2.0 * a.z;
        let axay = x * a.y;
        let axaz = x * a.z;
        let ayaz = y * a.z;

        Mat3 {
            e: [
                [x * a.x + 1.0, axay, axaz],
                [axay, y * a.y + 1.0, ayaz],
                [axaz, ayaz, z * a.z + 1.0],
            ]
        }
    }

    pub fn invol(a: Vec3) -> Mat3 {
        let x = 2.0 * a.x;
        let y = 2.0 * a.y;
        let z = 2.0 * a.z;
        let axay = x * a.y;
        let axaz = x * a.z;
        let ayaz = y * a.z;

        Mat3 {
            e: [
                [x * a.x - 1.0, axay, axaz],
                [axay, y * a.y - 1.0, ayaz],
                [axaz, ayaz, z * a.z - 1.0],
            ]
        }
    }

    pub fn scale(sx: f32, sy: f32, sz:f32) -> Mat3 {
        Mat3 {
            e: [
                [sx, 0.0, 0.0],
                [0.0, sy, 0.0],
                [0.0, 0.0, sz],
            ]
        }
    }

    pub fn scale_along_dir(s: f32, a: Vec3) -> Mat3 {
        let s = s - 1.0;
        let x = a.x * s;
        let y = a.y * s;
        let z = a.z * s;
        let axay = x * a.y;
        let axaz = x * a.z;
        let ayaz = y * a.z;

        Mat3 {
            e: [
                [x * a.x + 1.0, axay, axaz],
                [axay, y * a.y + 1.0, ayaz],
                [axaz, ayaz, z * a.z + 1.0],
            ]
        }
    }

    pub fn skew(t: f32, a: Vec3, b: Vec3) -> Mat3 {
        let t = t.tan();
        let x = a.x * t;
        let y = a.y * t;
        let z = a.z * t;

        Mat3 {
            e: [
                [x * b.x + 1.0, y * b.x, z * b.x],
                [x * b.y, y * b.y + 1.0, z * b.y],
                [x * b.z, y * b.z, z * b.z + 1.0],
            ]
        }
    }
}

impl ops::Add for Mat3 {
    type Output = Self;

    fn add(self, b: Mat3) -> Mat3 {
        Mat3::from_cols(self.col(0) + b.col(0), self.col(1) + b.col(1), self.col(2) + b.col(2))
    }
}

impl ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: Vec3::dot(self.row(0), v),
            y: Vec3::dot(self.row(1), v),
            z: Vec3::dot(self.row(2), v),
        }
    }
}

impl ops::Mul<Mat3> for Mat3 {
    type Output = Self;

    fn mul(self, m: Self) -> Self {
        let r0 = self.row(0);
        let r1 = self.row(1);
        let r2 = self.row(2);
        let c0 = m.col(0);
        let c1 = m.col(1);
        let c2 = m.col(2);
        Mat3::from_cols(
            Vec3::new(Vec3::dot(r0, c0), Vec3::dot(r1, c0), Vec3::dot(r2, c0)),
            Vec3::new(Vec3::dot(r0, c1), Vec3::dot(r1, c1), Vec3::dot(r2, c1)),
            Vec3::new(Vec3::dot(r0, c2), Vec3::dot(r1, c2), Vec3::dot(r2, c2)),
        )
    }
}
