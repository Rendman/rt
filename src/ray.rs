use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn at(self, t: f64) -> Vec3 {
        self.origin + self.direction.mul_scalar(t)
    }
}

#[test]
fn test_ray() {

    let r = Ray {origin: Vec3{e0: 2.0, e1: 2.0, e2: 2.0}, direction: Vec3 {e0: 4.0, e1: 4.0, e2: 4.0}};

    assert_eq!(r.at(3.0), Vec3{e0: 14.0, e1: 14.0, e2: 14.0});
}