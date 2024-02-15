mod camera;
mod hittable;
mod interval;
mod ray;
mod sphere;
mod vec3;
use camera::Camera;
use std::sync::Arc;
use vec3::Vec3;

use crate::{hittable::HittableList, sphere::Sphere};

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;

    cam.render(&world);
}
