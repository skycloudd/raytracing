use crate::{IMAGE_HEIGHT, IMAGE_WIDTH, ray::Ray};
use color::{OpaqueColor, Srgb};
use glam::Vec3;
use image::{DynamicImage, GenericImage as _, Rgba};
use indicatif::ProgressIterator as _;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    // focal_length: f32,
    // viewport_height: f32,
    // viewport_width: f32,
    camera_center: Vec3,
    // viewport_u: Vec3,
    // viewport_v: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    // viewport_upper_left: Vec3,
    pixel00_loc: Vec3,
}

impl Camera {
    pub fn new(focal_length: f32, viewport_height: f32) -> Self {
        let viewport_width = viewport_height * (IMAGE_WIDTH as f32 / IMAGE_HEIGHT as f32);

        let camera_center = Vec3::ZERO;

        let viewport_u = Vec3::ZERO.with_x(viewport_width);
        let viewport_v = Vec3::ZERO.with_y(-viewport_height);

        let pixel_delta_u = viewport_u / IMAGE_WIDTH as f32;
        let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f32;

        let viewport_upper_left =
            camera_center - Vec3::ZERO.with_z(focal_length) - viewport_u / 2. - viewport_v / 2.;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u * pixel_delta_v);

        Self {
            // focal_length,
            // viewport_height,
            // viewport_width,
            camera_center,
            // viewport_u,
            // viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            // viewport_upper_left,
            pixel00_loc,
        }
    }

    pub fn render(&self, img: &mut DynamicImage) {
        for y in (0..IMAGE_HEIGHT).progress() {
            for x in 0..IMAGE_WIDTH {
                let pixel_center = self.pixel00_loc
                    + (x as f32 * self.pixel_delta_u)
                    + (y as f32 * self.pixel_delta_v);

                let ray = Ray::new(self.camera_center, pixel_center - self.camera_center);

                let pixel_color = self.ray_color(ray);

                img.put_pixel(x, y, Rgba::from(pixel_color.to_rgba8().to_u8_array()));
            }
        }
    }

    pub fn ray_color(&self, ray: Ray) -> OpaqueColor<Srgb> {
        if let Some(hit_t) = hit_sphere(Vec3::new(0., 0., -1.), 0.5, ray) {
            let sphere_normal = (ray.at(hit_t) - Vec3::new(0., 0., -1.)).normalize();

            0.5 * OpaqueColor::new([
                sphere_normal.x + 1.,
                sphere_normal.y + 1.,
                sphere_normal.z + 1.,
            ])
        } else {
            let unit_direction = ray.direction().normalize();
            let a = 0.5 * (unit_direction.y + 1.0);
            (1.0 - a) * OpaqueColor::WHITE + a * OpaqueColor::new([0.5, 0.7, 1.0])
        }
    }
}

fn hit_sphere(center: Vec3, radius: f32, ray: Ray) -> Option<f32> {
    // let oc = center - ray.origin();

    // let a = ray.direction().dot(ray.direction());
    // let b = -2. * ray.direction().dot(oc);
    // let c = oc.dot(oc) - radius * radius;

    // let discriminant = b * b - 4. * a * c;

    // (discriminant >= 0.).then(|| (-b - discriminant.sqrt()) / (2. * a))

    let oc = center - ray.origin();

    let a = ray.direction().length_squared();
    let h = ray.direction().dot(oc);
    let c = oc.length_squared() - radius * radius;

    let discriminant = h * h - a * c;

    (discriminant >= 0.).then(|| (h - discriminant.sqrt()) / a)
}
