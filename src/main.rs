mod vec3;
mod colors;
mod ray;
mod hittable;
mod sphere;
use vec3::Vec3;
use colors::write_color;
use ray::Ray;



fn main() {
    // Image
    let aspect_radio = 16.0 / 9.0; // 图像宽高比
    let image_width = 400; // 图像宽度

    // Calculate the image height 计算图像高度，确保至少为 1。
    let image_height = image_width / aspect_radio as i32; // 图像高度
    let image_height = if image_height < 1 { 1 } else { image_height }; // 确保至少为 1

    // Camera

    let focal_length = 1.0; // 焦距
    let viewport_height = 2.0; // 视口高度
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64); // 视口宽度
    let camera_center = Vec3::new(0.0, 0.0, 0.0); // 相机中心

    // Calculate the vector3 across the horizontal and down the vertical viewport
    let viewport_u = Vec3::new(viewport_width,0.0 ,0.0); // 水平视口
    let viewport_v = Vec3::new(0.0,-viewport_height,0.0); // 垂直视口

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64; // 水平像素间隔
    let pixel_delta_v = viewport_v / image_height as f64; // 垂直像素间隔

    // Calculate the location of the upper left pixel
    let viewport_upper_left = camera_center - viewport_u / 2.0 - viewport_v / 2.0 - Vec3::new(0.0,0.0,focal_length); // 视口左上角
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0; // 像素00位置
    
    
    

    println!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {} ",image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = r.ray_color();
            write_color(pixel_color);
        }
    }
    eprintln!("\rDone.                 ")
}
