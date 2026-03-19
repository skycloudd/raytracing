use crate::{hittable::Hittable, interval::Interval, ray::Ray};
use color::{HueDirection, OpaqueColor, Srgb};
use glam::{Vec2, Vec3};
use image::{DynamicImage, GenericImage as _, Pixel, Rgba};
use indicatif::ProgressIterator as _;

#[derive(Debug)]
pub struct Camera {
    image: DynamicImage,
    center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    samples_per_pixel_sqrt: u32,
    max_depth: u32,
}

impl Camera {
    #[must_use]
    pub fn new(image: DynamicImage) -> Self {
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let samples_per_pixel_sqrt = 10;
        let max_depth = 50;

        let viewport_width = viewport_height * (image.width() as f32 / image.height() as f32);

        let camera_center = Vec3::ZERO;

        let viewport_u = Vec3::ZERO.with_x(viewport_width);
        let viewport_v = Vec3::ZERO.with_y(-viewport_height);

        let pixel_delta_u = viewport_u / image.width() as f32;
        let pixel_delta_v = viewport_v / image.height() as f32;

        let viewport_upper_left =
            camera_center - Vec3::ZERO.with_z(focal_length) - viewport_u / 2. - viewport_v / 2.;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u * pixel_delta_v);

        Self {
            image,
            center: camera_center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            samples_per_pixel_sqrt,
            max_depth,
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        for y in (0..self.image.height()).progress() {
            for x in 0..self.image.width() {
                let mut pixel_color: Option<OpaqueColor<Srgb>> = None;

                for n in 0..(self.samples_per_pixel_sqrt * self.samples_per_pixel_sqrt) {
                    let sample = Sample::new(n, self.samples_per_pixel_sqrt);

                    let ray = self.get_ray(x as f32, y as f32, sample);
                    let ray_color = ray_color(ray, self.max_depth, world);

                    pixel_color = Some(pixel_color.map_or_else(
                        || ray_color,
                        |pixel_color| pixel_color.lerp(ray_color, 0.5, HueDirection::default()),
                    ));
                }

                self.image.put_pixel(
                    x,
                    y,
                    Rgba::from(
                        pixel_color
                            .unwrap()
                            .map(|x, y, z| [x, y, z].map(linear_to_gamma))
                            .to_rgba8()
                            .to_u8_array(),
                    ),
                );
            }
        }
    }

    fn get_ray(&self, i: f32, j: f32, sample: Sample) -> Ray {
        let offset = sample_square(sample);

        let pixel_sample = self.pixel00_loc
            + ((i + offset.x) * self.pixel_delta_u)
            + ((j + offset.y) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    #[must_use]
    pub fn image(self) -> DynamicImage {
        self.image
    }
}

#[derive(Clone, Copy)]
struct Sample {
    n: u32,
    total_sqrt: u32,
}

impl Sample {
    const fn new(n: u32, total_sqrt: u32) -> Self {
        Self { n, total_sqrt }
    }
}

fn ray_color(ray: Ray, depth: u32, world: &dyn Hittable) -> OpaqueColor<Srgb> {
    if depth == 0 {
        return OpaqueColor::BLACK;
    }

    world
        .hit(ray, Interval::new(0.001, f32::INFINITY))
        .map_or_else(
            || skybox(ray),
            |hit_rec| {
                0.5 * ray_color(
                    Ray::new(hit_rec.point(), hit_rec.normal()),
                    depth - 1,
                    world,
                )
            },
        )
}

fn linear_to_gamma(x: f32) -> f32 {
    match x {
        x if x <= 0. => 0.,
        x => x.sqrt(),
    }
}

fn skybox(ray: Ray) -> OpaqueColor<Srgb> {
    let unit_direction = ray.direction().normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * OpaqueColor::WHITE + a * OpaqueColor::new([0.5, 0.7, 1.0])
}

const fn sample_square(sample: Sample) -> Vec2 {
    let x = sample.n % sample.total_sqrt;
    let y = sample.n / sample.total_sqrt;

    Vec2::new(
        (x as f32) / sample.total_sqrt as f32 + 0.5,
        (y as f32) / sample.total_sqrt as f32 + 0.5,
    )
}
