use std::rc::Rc;

use crate::{
    interval::Interval,
    material::{Lambertian, Material},
    ray::Ray,
    vec3::Vec3,
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,               // 交点
    pub normal: Vec3,          // 法线
    pub t: f64,                // 光线参数
    pub front_face: bool,      // 是否是正面
    pub mat: Rc<dyn Material>, // 材质
}

pub trait Hittable {
    /// 判断光线是否击中物体
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    /// 设置法线
    pub fn set_face_normal(&mut self, r: &Ray, outward_noraml: &Vec3) {
        self.front_face = r.direction.dot(outward_noraml) < 0.0; // 判断是否是正面
        self.normal = if self.front_face {
            outward_noraml.clone()
        } else {
            -outward_noraml.clone()
        }; // 如果是正面，法线方向不变，否则取反
    }
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let temp_rec = &mut HitRecord {
            p: Vec3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
            mat: Rc::new(Lambertian::new(Vec3::default())),
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
