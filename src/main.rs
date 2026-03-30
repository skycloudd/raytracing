use color::OpaqueColor;
use glam::Vec3;
use image::DynamicImage;
use raytracer::{camera::Camera, hittable, material};
use std::rc::Rc;

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = ((image_width as f32 / aspect_ratio) as u32).max(1);

    let image = DynamicImage::new_rgb8(image_width, image_height);

    let mut camera = Camera::new(image);

    let mut world = hittable::List::new();

    let mat_ground = Rc::new(material::Lambertian::new(OpaqueColor::new([0.8, 0.8, 0.0])));
    let mat_center = Rc::new(material::Lambertian::new(OpaqueColor::new([0.1, 0.2, 0.5])));
    let mat_left = Rc::new(material::Metal::new(
        OpaqueColor::new([0.8, 0.8, 0.8]),
        0.05,
    ));
    let mat_right = Rc::new(material::Metal::new(
        OpaqueColor::new([0.8, 0.6, 0.2]),
        0.05,
    ));

    world.add(Box::new(hittable::Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.,
        mat_ground,
    )));

    world.add(Box::new(hittable::Sphere::new(
        Vec3::new(0.0, 0.0, -1.2),
        0.5,
        mat_center,
    )));

    world.add(Box::new(hittable::Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        mat_left,
    )));

    world.add(Box::new(hittable::Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        mat_right,
    )));

    camera.render(&world);

    camera.image().save("output.png").unwrap();
}
