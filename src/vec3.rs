use overload::overload;
use rand::Rng;
use std::ops;

// 三维向量
#[derive(Clone, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// 创建三维向量
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    /// 长度平方
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// 长度
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// 点积
    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// 叉积
    pub fn cross(&self, rhs: &Vec3) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    /// 单位向量
    pub fn unit_vector(&self) -> Self {
        self.clone() / self.length()
    }

    /// 随机向量
    fn random() -> Vec3 {
        Vec3 {
            x: rand::random::<f64>(),
            y: rand::random::<f64>(),
            z: rand::random::<f64>(),
        }
    }

    /// 有范围的随机向量
    fn random_in_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: min + (max - min) * rand::random::<f64>(),
            y: min + (max - min) * rand::random::<f64>(),
            z: min + (max - min) * rand::random::<f64>(),
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3 {
                x: rand::thread_rng().gen_range(-1.0..1.0),
                y: rand::thread_rng().gen_range(-1.0..1.0),
                z: 0.0,
            };
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// 单位球内的随机向量
    fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_in_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                // 半径小于1.0
                return p;
            }
        }
    }

    /// 单位球内的随机单位向量
    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    /// 单位球内的随机单位向量
    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector(); // 单位球内的随机单位向量
        if on_unit_sphere.dot(normal) > 0.0 {
            // 如果点积大于0.0，说明在半球内
            return on_unit_sphere;
        }
        // 否则取反
        -on_unit_sphere
    }

    /// 判断是否接近0
    pub fn near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        (self.x.abs() < S) && (self.y.abs() < S) && (self.z.abs() < S)
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        v - 2.0 * n.dot(v) * n
    }

    /// 折射
    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-uv).dot(n).min(1.0); // 取最小值
        let r_out_perp = etai_over_etat * (uv + cos_theta * n); // 法线方向
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).sqrt() * n; // 平行方向
        r_out_parallel + r_out_perp
    }
}

// Add
overload!((a: ?Vec3) + (b: ?Vec3) -> Vec3 {
    Vec3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
});

// Sub
overload!((a: ?Vec3) - (b: ?Vec3) -> Vec3 {
    Vec3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});

// Mul
overload!((a: ?Vec3) * (b: ?Vec3) -> Vec3 {
    Vec3 {
        x: a.x * b.x,
        y: a.y * b.y,
        z: a.z * b.z,
    }
});

overload!((a: ?Vec3) * (b: ?f64) -> Vec3 {
    Vec3 {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
});

overload!((a: ?f64) * (b: ?Vec3) -> Vec3 {
    b * a
});

// Div
overload!((a: ?Vec3) / (b: ?f64) -> Vec3 {
    Vec3 {
        x: a.x / b,
        y: a.y / b,
        z: a.z / b,
    }
});

// Neg
overload!(- (a: ?Vec3) -> Vec3 {
    Vec3 {
        x: -a.x,
        y: -a.y,
        z: -a.z,
    }
});

// AddAssign
overload!((a: &mut Vec3) += (b: ?Vec3) {
    a.x += b.x;
    a.y += b.y;
    a.z += b.z;
});
