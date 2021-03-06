use image::{ImageBuffer, Rgb, RgbImage};
use rand::prelude::*;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::vector::Color;
const MAX_DEPTH : i32 = 50;

pub fn render(
    camera: &Camera,
    world: &HittableList,
    image_width: u32,
    image_height: u32,
    samples: u32,
) -> RgbImage {
    let start = std::time::Instant::now();
    let mut buffer: RgbImage = ImageBuffer::new(image_width, image_height);

    let colors: Vec<Color> = (0..image_height * image_width)
        .into_par_iter()
        .map(|index| {
            let mut rng = thread_rng();
            let x = index % image_width;
            let y = index / image_width;
            let mut color: Color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..samples {
                let rand_x: f32 = rng.gen();
                let rand_y: f32 = rng.gen();
                let u = (rand_x + x as f32) / (image_width - 1) as f32;
                let v = 1.0 - (rand_y + y as f32) / (image_height - 1) as f32;
                let ray = camera.get_ray(u, v);
                color = color + ray.color(&world, &mut rng, MAX_DEPTH);
            }
            (color / samples as f32).sqrt()
        })
        .collect();
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let index: usize = (y * image_width) as usize + x as usize;
        let color = colors.get(index).unwrap();
        *pixel = Rgb(color.to_rgb());
    }
    eprintln!("Elapsed: {:?}", start.elapsed());
    buffer
}
