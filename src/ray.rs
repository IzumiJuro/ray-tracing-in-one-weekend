use std::f64::INFINITY;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    vec3::Vec3,
};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// 光线上的点
    pub fn at(&self, t: f64) -> Vec3 {
        &self.origin + &self.direction * t
    }

    /// 创建光线
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    /// 光线颜色
    pub fn ray_color(&self, world: &dyn Hittable) -> Vec3 {
        let rec = &mut HitRecord {
            p: Vec3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
        }; // 交点
        if world.hit(self, Interval::new(0.0, INFINITY), rec) {
            // 如果光线击中物体
            return 0.5 * (&rec.normal + Vec3::new(1.0, 1.0, 1.0)); // 返回法线颜色
        }
        let unit_direction = self.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    }
}
