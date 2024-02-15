use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = &r.origin - &self.center; // A - C

        let a = r.direction.length_squared();
        let half_b = r.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (&rec.p - &self.center) / self.radius; // 单位法线
        rec.set_face_normal(r, &outward_normal); // 设置法线

        true
    }
}
