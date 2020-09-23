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
}
