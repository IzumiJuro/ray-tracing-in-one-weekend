mod camera;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

use std::rc::Rc;

use camera::Camera;
use hittable::HittableList;
use sphere::Sphere;
use vec3::Vec3;

fn main() {
    // World
    let mut world = HittableList::new();

    let material_ground = Rc::new(material::Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(material::Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(material::Dielectric::new(1.5));
    let material_right = Rc::new(material::Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));

    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    ))); // 空心玻璃球
    world.add(Rc::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(-2.0, 2.0, 1.0);
    cam.lookat = Vec3::new(0.0, 0.0, -1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;

    cam.render(&world);
}
