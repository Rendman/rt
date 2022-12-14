use crate::{ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub trait Hittable {

    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}