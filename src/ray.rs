use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// 光线上的点
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }

    /// 创建光线
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn ray_color(&self) -> Vec3 {
        let t = self.hit_sphere(Vec3::new(0.0,0.0,-1.0), 0.5); // 球心在 (0,0,-1) 半径为 0.5
        if t > 0.0 {
            let n = (self.at(t) - Vec3::new(0.0,0.0,-1.0)).unit_vector(); // 交点到球心的向量
            return Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5; 
        }
        let unit_direction = self.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
    }

    fn hit_sphere(&self, center: Vec3, radius: f64) -> f64{
        let oc = self.origin - center; // 球心到光线起点的向量
        let a = self.direction.length_squared();  // 光线方向的长度
        let half_b = oc.dot(self.direction);
        let c = oc.length_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c; // 判别式

        if discriminant < 0.0 {
            -1.0 // 无交点
        } else {
            (-half_b - discriminant.sqrt()) / a // 返回较小的根
        }
    }
}