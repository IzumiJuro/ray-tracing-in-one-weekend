use crate::{hittable::Hittable, vec3::Vec3};

struct Sphere {
    center: Vec3,
    radius: f64
}

impl Hittable for Sphere {
    fn hit(&self,r: &crate::ray::Ray, ray_min: f64, ray_max: f64, rec: &mut crate::hittable::HitRecord) -> bool {
        let oc = r.origin - self.center; // A - C
        let a = r.direction.length_squared();
        let half_b = r.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        } 
        let sqrtd = discriminant.sqrt();

        let root = (-half_b - sqrtd) / a;
        if root <= ray_min || root >= ray_max {
            let root = (-half_b + sqrtd) / a;
            if root <= ray_min || root >= ray_max {
                return false;
            }
        }   

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal =  (rec.p - self.center) / self.radius; // 单位法线
        rec.set_face_normal(r, outward_normal); // 设置法线
        
        true
    }
}