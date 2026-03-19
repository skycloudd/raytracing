use glam::Vec3;
use image::DynamicImage;
use raytracer::{camera::Camera, hittable};

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f32 / aspect_ratio).max(1.) as u32;

    let image = DynamicImage::new_rgb8(image_width, image_height);

    let mut camera = Camera::new(image);

    let mut world = hittable::List::new();

    world.add(Box::new(hittable::Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(hittable::Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.,
    )));

    camera.render(&world);

    camera.image().save("output.png").unwrap();
}
