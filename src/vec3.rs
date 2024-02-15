use overload::overload;
use std::ops;

#[derive(Clone, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

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
