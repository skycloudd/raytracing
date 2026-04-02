use clap::Parser;
use color::OpaqueColor;
use glam::Vec3;
use image::DynamicImage;
use raytracer::{
    camera::{Camera, CameraConfig},
    hittable, material,
};
use std::{path::PathBuf, rc::Rc};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "output.png")]
    output: PathBuf,
    #[arg(short, long, default_value = "50")]
    depth: u32,
    #[arg(short, long, default_value = "10")]
    samples: u32,
    #[arg(short, long, default_value = "400")]
    width: u32,
}

fn main() {
    let args = Args::parse();

    let config = CameraConfig {
        samples_per_pixel: args.samples,
        max_depth: args.depth,
        defocus_angle: 0.6,
        focus_distance: 10.0,
        vfov: 20.0,
        look_from: Vec3::new(13.0, 2.0, 3.0),
        look_at: Vec3::new(0.0, 0.0, 0.0),
        v_up: Vec3::Y,
    };

    let aspect_ratio = 16.0 / 9.0;
    let image_width = args.width;
    let image_height = ((image_width as f32 / aspect_ratio) as u32).max(1);

    let image = DynamicImage::new_rgb8(image_width, image_height);

    let mut camera = Camera::new(image, config);

    let mut world = hittable::List::new();

    demo_scene(&mut world);

    camera.render(&world);

    camera.image().save(args.output).unwrap();
}

fn demo_scene(world: &mut hittable::List) {
    world.clear();

    let ground_material = Rc::new(material::Lambertian::new(OpaqueColor::new([0.5, 0.5, 0.5])));

    world.add(Box::new(hittable::Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rand::random_range(0.0..1.0);

            let center = Vec3::new(
                0.9f32.mul_add(rand::random_range(0.0..1.0), a as f32),
                0.2,
                0.9f32.mul_add(rand::random_range(0.0..1.0), b as f32),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = if choose_material < 0.7 {
                    let albedo = OpaqueColor::new([
                        rand::random_range(0.0..1.0) * rand::random_range(0.0..1.0),
                        rand::random_range(0.0..1.0) * rand::random_range(0.0..1.0),
                        rand::random_range(0.0..1.0) * rand::random_range(0.0..1.0),
                    ]);

                    Rc::new(material::Lambertian::new(albedo)) as Rc<dyn material::Material>
                } else {
                    let albedo = OpaqueColor::new([
                        rand::random_range(0.5..=1.0),
                        rand::random_range(0.5..=1.0),
                        rand::random_range(0.5..=1.0),
                    ]);

                    let fuzz = rand::random_range(0.0..0.5);

                    Rc::new(material::Metal::new(albedo, fuzz))
                };

                world.add(Box::new(hittable::Sphere::new(center, 0.2, material)));
            }
        }
    }

    let material_1 = Rc::new(material::Lambertian::new(OpaqueColor::new([
        rand::random_range(0.0..1.0) * rand::random_range(0.0..1.0),
        rand::random_range(0.0..1.0) * rand::random_range(0.0..1.0),
        rand::random_range(0.0..1.0) * rand::random_range(0.0..1.0),
    ])));

    world.add(Box::new(hittable::Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = Rc::new(material::Metal::new(OpaqueColor::new([0.7, 0.6, 0.5]), 0.0));

    world.add(Box::new(hittable::Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));
}
