use glam::Vec3;
use image::DynamicImage;
use raytracer::{camera::Camera, hittable};

const ASPECT_RATIO: f32 = 16. / 9.;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO).max(1.) as u32;

const FOCAL_LENGTH: f32 = 1.0;
const VIEWPORT_HEIGHT: f32 = 2.0;

fn main() {
    let mut img = DynamicImage::new_rgb8(IMAGE_WIDTH, IMAGE_HEIGHT);

    let camera = Camera::new(IMAGE_WIDTH, IMAGE_HEIGHT, FOCAL_LENGTH, VIEWPORT_HEIGHT);

    let mut world = hittable::List::new();

    world.add(Box::new(hittable::Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(hittable::Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.,
    )));

    camera.render(&mut img, &world);

    img.save("output.png").unwrap();
}
