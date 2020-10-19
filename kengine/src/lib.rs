mod math;

#[cfg(test)]
mod tests {
    use super::math::*;
    use std::f32::consts::PI;

    #[test]
    fn vec3_tests() {
        let mut v = Vec3 { x: 1.0, y: 1.0, z: 1.0 };
        assert!(v.x == 1.0 && v.y == 1.0 && v.z == 1.0);
        let x = Vec3 { x: 1.0, y: 0.0, z: 0.0, };
        let y = Vec3 { x: 0.0, y: 1.0, z: 0.0, };
        let z = Vec3 { x: 0.0, y: 0.0, z: 1.0, };
        assert_eq!(Vec3::dot(x, y), 0.0);
        v = Vec3 { y: 1.0, ..x };
        assert_eq!(Vec3::proj(v, x), x);
        assert_eq!(Vec3::rej(v, x), y);
        assert_eq!(Vec3::cross(x, y), z);
    }

    #[test]
    fn mat3_tests() {
        let eps: f32 = 1e-4;
        let mat = Mat3::from_cols(
            Vec3::one(),
            Vec3::new(3.0, 4.0, 5.0),
            Vec3::new(3.0, 6.0, 10.0)
        );
        let inv = mat.inv();
        assert_eq!(mat * inv, Mat3::identity());
        // transformations
        assert!(vec3_approx_eq(Mat3::rot_x(PI / 2.0) * Vec3::y_axis(), Vec3::z_axis(), eps));
        assert!(vec3_approx_eq(Mat3::rot_z(PI / 2.0) * Vec3::x_axis(), Vec3::y_axis(), eps));
        assert!(vec3_approx_eq(Mat3::rot_y(PI / 2.0) * Vec3::z_axis(), Vec3::x_axis(), eps));
        assert!(vec3_approx_eq(Mat3::rot(PI / 2.0, Vec3::x_axis()) * Vec3::y_axis(), Vec3::z_axis(), eps));
        assert!(vec3_approx_eq(Mat3::rot(PI / 2.0, Vec3::z_axis()) * Vec3::x_axis(), Vec3::y_axis(), eps));
        assert!(vec3_approx_eq(Mat3::rot(PI / 2.0, Vec3::y_axis()) * Vec3::z_axis(), Vec3::x_axis(), eps));
        assert!(vec3_approx_eq(Mat3::rot_y(PI / 2.0) * Mat3::rot_x(PI / 2.0) * Vec3::y_axis(), Vec3::x_axis(), eps));
        assert!(vec3_approx_eq(Mat3::reflect(Vec3::x_axis()) * Vec3::new(1.0, 1.0, 0.0), Vec3::new(-1.0, 1.0, 0.0), eps));
        assert!(vec3_approx_eq(Mat3::invol(Vec3::x_axis()) * Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, -1.0, 0.0), eps));
        assert!(vec3_approx_eq(Mat3::scale(2.0, 2.0, 2.0) * Vec3::new(0.5, 0.5, 0.5), Vec3::one(), eps));
        assert!(vec3_approx_eq(Mat3::scale_along_dir(2.0, Vec3::x_axis()) * Vec3::one(), Vec3::new(2.0, 1.0, 1.0), eps));
        assert!(vec3_approx_eq(Mat3::skew(PI / 4.0, Vec3::x_axis(), Vec3::y_axis()) * Vec3::y_axis(), Vec3::new(1.0, 1.0, 0.0), eps));
    }

    fn vec3_approx_eq(a: Vec3, b: Vec3, eps: f32) -> bool {
        (a.x - b.x).abs() < eps && (a.y - b.y).abs() < eps && (a.z - b.z).abs() < eps
    }
}
