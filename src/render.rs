use image::{ImageBuffer, Rgb, RgbImage};
use rand::prelude::*;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::vector::Color;

pub fn render(camera: &Camera, world: &HittableList, image_width: u32, image_height: u32, samples: u32) -> RgbImage {
    let start = std::time::Instant::now();
    let mut buffer: RgbImage = ImageBuffer::new(image_width, image_height);
    let mut rng = thread_rng();
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let mut color: Color = Color::new(0.0, 0.0, 0.0);
        for _s in 0..samples {
            let rand_x: f64 = rng.gen();
            let rand_y: f64 = rng.gen();
            let u = (rand_x + x as f64) / (image_width - 1) as f64;
            let v = 1.0 - (rand_y + y as f64) / (image_height - 1) as f64;
            let ray = camera.get_ray(u, v);
            color = color + ray.color(&world);
        }
        color = color / samples as f64;
        *pixel = Rgb(color.to_rgb());
    }
    eprintln!("Elapsed: {:?}", start.elapsed());
    buffer
}
