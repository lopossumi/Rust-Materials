use image::{ImageBuffer, Rgb, RgbImage};

mod ray;
mod vector;
mod sphere;
mod hittable;
mod hittable_list;

use ray::Ray;
use vector::Vec3;
use hittable_list::*;

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 384;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let world = HittableList::new();
        
    let mut buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let u = x as f64 / (IMAGE_WIDTH - 1) as f64;
        let v = 1.0 - (y as f64 / (IMAGE_HEIGHT - 1) as f64);
        let ray = Ray::new(
            origin,
            lower_left_corner + u * horizontal + v * vertical - origin,
        );
        let color = ray.color(&world).to_rgb();
        *pixel = Rgb(color);
    }

    match buffer.save("two_spheres.png") {
        Err(e) => eprintln!("Error writing file: {}", e),
        Ok(()) => println!("Done."),
    };
}
