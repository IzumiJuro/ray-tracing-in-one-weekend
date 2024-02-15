use std::sync::Arc;

use crate::{ray::Ray, vec3::Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3, // 交点
    pub normal: Vec3, // 法线
    pub t: f64, // 光线参数
    pub front_face: bool, // 是否是正面
}

pub trait Hittable {
    /// 判断光线是否击中物体
    fn hit(&self,r: &Ray, ray_min: f64, ray_max: f64, rec: &mut HitRecord) -> bool;   
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Vec3::new(0.0,0.0,0.0),
            normal: Vec3::new(0.0,0.0,0.0),
            t: 0.0,
            front_face: false
        }
    }
    /// 设置法线
    pub fn set_face_normal(mut self, r: &Ray, outward_noraml: Vec3) {
        self.front_face = r.direction.dot(outward_noraml) < 0.0; // 判断是否是正面
        self.normal = if self.front_face {outward_noraml} else {-outward_noraml}; // 如果是正面，法线方向不变，否则取反
    }
}

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }

    pub fn add(&mut self,object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self,r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;
    

        for object in self.objects.iter() {
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}