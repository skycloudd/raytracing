use crate::{hittable::Hittable, ray::Ray};
use color::{OpaqueColor, Srgb};
use glam::Vec3;
use image::{DynamicImage, GenericImage as _, Rgba};
use indicatif::ProgressIterator as _;

#[derive(Debug)]
pub struct Camera {
    image: DynamicImage,
    center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
}

impl Camera {
    #[must_use]
    pub fn new(image: DynamicImage, focal_length: f32, viewport_height: f32) -> Self {
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
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        for y in (0..self.image.height()).progress() {
            for x in 0..self.image.width() {
                let pixel_center = self.pixel00_loc
                    + (x as f32 * self.pixel_delta_u)
                    + (y as f32 * self.pixel_delta_v);

                let ray = Ray::new(self.center, pixel_center - self.center);

                let pixel_color = ray_color(ray, world);

                self.image
                    .put_pixel(x, y, Rgba::from(pixel_color.to_rgba8().to_u8_array()));
            }
        }
    }

    #[must_use]
    pub fn image(self) -> DynamicImage {
        self.image
    }
}

fn ray_color(ray: Ray, world: &dyn Hittable) -> OpaqueColor<Srgb> {
    world.hit(ray, 0., f32::INFINITY).map_or_else(
        || skybox(ray),
        |hit_rec| {
            0.5 * OpaqueColor::new([
                hit_rec.normal().x + 1.,
                hit_rec.normal().y + 1.,
                hit_rec.normal().z + 1.,
            ])
        },
    )
}

fn skybox(ray: Ray) -> OpaqueColor<Srgb> {
    let unit_direction = ray.direction().normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * OpaqueColor::WHITE + a * OpaqueColor::new([0.5, 0.7, 1.0])
}
