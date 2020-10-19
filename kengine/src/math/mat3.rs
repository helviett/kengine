use std::ops;
use super::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat3 {
    pub c0: Vec3,
    pub c1: Vec3,
    pub c2: Vec3,
}

impl Mat3 {
    pub fn from_cols(c0: Vec3, c1: Vec3, c2: Vec3) -> Mat3 {
        Mat3 { c0, c1, c2, }
    }

    pub fn row(&self, index: usize) -> Vec3 {
        Vec3 { x: self.c0[index], y: self.c1[index], z: self.c2[index], }
    }

    pub fn identity() -> Mat3 {
        Mat3 {
            c0: Vec3::x_axis(),
            c1: Vec3::y_axis(),
            c2: Vec3::z_axis(),
        }
    }

    pub fn zero() -> Mat3 {
        Mat3 {
            c0: Vec3::zero(),
            c1: Vec3::zero(),
            c2: Vec3::zero(),
        }
    }

    pub fn inv(&self) -> Mat3 {
        let bxc = Vec3::cross(self.c1, self.c2);
        let cxa = Vec3::cross(self.c2, self.c0);
        let axb = Vec3::cross(self.c0, self.c1);

        let inv_det = 1.0 / Vec3::dot(axb, self.c2);
        Mat3 {
            c0: Vec3::new(bxc.x * inv_det, cxa.x * inv_det, axb.x * inv_det),
            c1: Vec3::new(bxc.y * inv_det, cxa.y * inv_det, axb.y * inv_det),
            c2: Vec3::new(bxc.z * inv_det, cxa.z * inv_det, axb.z * inv_det),
        }
    }

    pub fn rot_x(rads: f32) -> Mat3 {
        let sin = rads.sin();
        let cos = rads.cos();

        Mat3 {
            c0: Vec3::x_axis(),
            c1: Vec3::new(0.0, cos, sin),
            c2: Vec3::new(0.0, -sin, cos),
        }
    }

    pub fn rot_y(rads: f32) -> Mat3 {
        let sin = rads.sin();
        let cos = rads.cos();

        Mat3 {
            c0: Vec3::new(cos, 0.0, -sin),
            c1: Vec3::y_axis(),
            c2: Vec3::new(sin, 0.0, cos),
        }
    }

    pub fn rot_z(rads: f32) -> Mat3 {
        let sin = rads.sin();
        let cos = rads.cos();

        Mat3 {
            c0: Vec3::new(cos, sin, 0.0),
            c1: Vec3::new(-sin, cos, 0.0),
            c2: Vec3::z_axis(),
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
            c0: Vec3::new(c + x * a.x, axay + s * a.z, axaz - s * a.y),
            c1: Vec3::new(axay - s * a.z, c + y * a.y, ayaz + s * a.x),
            c2: Vec3::new(axaz + s * a.y, ayaz - s * a.x, c + z * a.z),
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
            c0: Vec3::new(x * a.x + 1.0, axay, axaz),
            c1: Vec3::new(axay, y * a.y + 1.0, ayaz),
            c2: Vec3::new(axaz, ayaz, z * a.z + 1.0),
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
           c0: Vec3::new(x * a.x - 1.0, axay, axaz),
           c1: Vec3::new(axay, y * a.y - 1.0, ayaz),
           c2: Vec3::new(axaz, ayaz, z * a.z - 1.0),
        }
    }

    pub fn scale(sx: f32, sy: f32, sz:f32) -> Mat3 {
        Mat3 {
           c0: Vec3::new(sx, 0.0, 0.0),
           c1: Vec3::new(0.0, sy, 0.0),
           c2: Vec3::new(0.0, 0.0, sz),
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
            c0: Vec3::new(x * a.x + 1.0, axay, axaz),
            c1: Vec3::new(axay, y * a.y + 1.0, ayaz),
            c2: Vec3::new(axaz, ayaz, z * a.z + 1.0),
        }
    }

    pub fn skew(t: f32, a: Vec3, b: Vec3) -> Mat3 {
        let t = t.tan();
        let x = a.x * t;
        let y = a.y * t;
        let z = a.z * t;

        Mat3 {
            c0: Vec3::new(x * b.x + 1.0, y * b.x, z * b.x),
            c1: Vec3::new(x * b.y, y * b.y + 1.0, z * b.y),
            c2: Vec3::new(x * b.z, y * b.z, z * b.z + 1.0),
        }
    }
}

impl ops::Add for Mat3 {
    type Output = Self;

    fn add(self, b: Mat3) -> Mat3 {
        Mat3::from_cols(self.c0 + b.c0, self.c1 + b.c1, self.c2 + b.c2)
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
        Mat3::from_cols(
            Vec3::new(Vec3::dot(r0, m.c0), Vec3::dot(r1, m.c0), Vec3::dot(r2, m.c0)),
            Vec3::new(Vec3::dot(r0, m.c1), Vec3::dot(r1, m.c1), Vec3::dot(r2, m.c1)),
            Vec3::new(Vec3::dot(r0, m.c2), Vec3::dot(r1, m.c2), Vec3::dot(r2, m.c2)),
        )
    }
}
