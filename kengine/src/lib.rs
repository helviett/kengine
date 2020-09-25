mod math;

#[cfg(test)]
mod tests {
    use super::math::*;
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
        let mat = Mat3::from_cols(
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(3.0, 4.0, 5.0),
            Vec3::new(3.0, 6.0, 10.0)
        );
        let inv = mat.inv();
        assert_eq!(mat * inv, Mat3::identity());
    }
}
