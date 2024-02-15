use std::{f64::INFINITY, rc::Rc};

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Lambertian,
    ray::Ray,
    vec3::Vec3,
};
use rand::Rng;
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    image_height: usize,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: Default::default(),
            center: Vec3::default(),
            pixel00_loc: Vec3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, world, self.max_depth);
                }
                write_color(pixel_color, self.samples_per_pixel);
            }
        }
        eprintln!("\rDone.                 ")
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize; // 图像高度
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        }; // 确保至少为 1

        self.center = Vec3::new(0.0, 0.0, 0.0); // 相机中心

        let focal_length = 1.0; // 焦距
        let viewport_height = 2.0; // 视口高度
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64); // 视口宽度

        // Calculate the vector3 across the horizontal and down the vertical viewport
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0); // 水平视口
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0); // 垂直视口

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = &viewport_u / self.image_width as f64; // 水平像素间隔
        self.pixel_delta_v = &viewport_v / self.image_height as f64; // 垂直像素间隔

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            &self.center - viewport_u / 2.0 - viewport_v / 2.0 - Vec3::new(0.0, 0.0, focal_length); // 视口左上角
        self.pixel00_loc = viewport_upper_left + (&self.pixel_delta_u + &self.pixel_delta_v) / 2.0;
        // 像素00位置
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let pixel_center =
            &self.pixel00_loc + (&self.pixel_delta_u * i as f64) + (&self.pixel_delta_v * j as f64);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center.clone();
        let ray_direction = pixel_sample - &self.center;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + rand::thread_rng().gen_range(0.0..1.0);
        let py = -0.5 + rand::thread_rng().gen_range(0.0..1.0);
        px * &self.pixel_delta_u + py * &self.pixel_delta_v
    }

    /// 光线颜色
    pub fn ray_color(&self, r: &Ray, world: &dyn Hittable, depth: usize) -> Vec3 {
        let rec = &mut HitRecord {
            p: Vec3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
            mat: Rc::new(Lambertian::new(Vec3::default())),
        }; // 交点
        if depth == 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        if world.hit(r, Interval::new(0.001, INFINITY), rec) {
            // 如果光线击中物体
            let mut scattered = Ray::default();
            let mut attenuation = Vec3::default();
            if rec.mat.scatter(r, rec, &mut attenuation, &mut scattered) {
                // 如果材质发生散射
                return attenuation * self.ray_color(&scattered, world, depth - 1);
            }
            return Vec3::new(0.0, 0.0, 0.0);
            // 返回法线颜色
        }
        let unit_direction = r.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
    }
}

pub fn write_color(pixel_color: Vec3, samples_per_pixel: usize) {
    let (mut r, mut g, mut b) = (pixel_color.x, pixel_color.y, pixel_color.z);

    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    static INTENSITY: Interval = Interval {
        min: 0.0,
        max: 0.999,
    };

    let ir = (256.0 * INTENSITY.clamp(r)) as usize;
    let ig = (256.0 * INTENSITY.clamp(g)) as usize;
    let ib = (256.0 * INTENSITY.clamp(b)) as usize;

    println!("{ir} {ig} {ib}")
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}
