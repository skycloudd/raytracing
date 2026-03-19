use crate::camera::Camera;
use image::DynamicImage;

mod camera;
mod ray;

const ASPECT_RATIO: f32 = 16. / 9.;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO).max(1.) as u32;

const FOCAL_LENGTH: f32 = 1.0;
const VIEWPORT_HEIGHT: f32 = 2.0;

fn main() {
    let mut img = DynamicImage::new_rgb8(IMAGE_WIDTH, IMAGE_HEIGHT);

    let camera = Camera::new(FOCAL_LENGTH, VIEWPORT_HEIGHT);

    camera.render(&mut img);

    img.save("output.png").unwrap();
}
