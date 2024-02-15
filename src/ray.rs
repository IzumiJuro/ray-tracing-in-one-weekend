use crate::vec3::Vec3;

#[derive(Default)]
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
}
