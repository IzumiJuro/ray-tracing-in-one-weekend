mod vec3;
mod color;
fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {} ",image_height - j);
        for i in 0..image_width {
            let r = (i as f64) / (image_width as f64 - 1.0);
            let g = (j as f64) / (image_height as f64 - 1.0);
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{ir} {ig} {ib}")
        }
    }
}