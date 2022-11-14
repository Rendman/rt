use crate::vec3::Vec3;
use crate::hittable::{Hittable, HitRecord};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
}

impl Hittable for Sphere {

    fn hit(&self, r: crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;

        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;
       
        if discriminant < 0.0{
            return false;
        }
        
        let sqrtd = f64::sqrt(discriminant);
        let mut root = (-half_b - sqrtd)/a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center).div_scalar(self.radius);
        rec.set_face_normal(r, outward_normal);
        rec.normal = (rec.p - self.center).div_scalar(self.radius);

        return true;
    }

}