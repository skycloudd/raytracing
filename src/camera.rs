use crate::{
    hittable::Hittable,
    interval::Interval,
    random::{random_in_unit_disk, random_unit_square},
    ray::Ray,
};
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
    samples_per_pixel: u32,
    max_depth: u32,
    defocus_angle: f32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    #[must_use]
    pub fn new(image: DynamicImage, config: CameraConfig) -> Self {
        let theta = config.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * config.focus_distance;

        let viewport_width = viewport_height * (image.width() as f32 / image.height() as f32);

        let camera_center = config.look_from;

        let w = (config.look_from - config.look_at).normalize();
        let u = config.v_up.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / image.width() as f32;
        let pixel_delta_v = viewport_v / image.height() as f32;

        let viewport_upper_left =
            camera_center - config.focus_distance * w - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u * pixel_delta_v);

        let defocus_radius =
            config.focus_distance * (config.defocus_angle / 2.0).to_radians().tan();

        let defocus_disk_u = defocus_radius * u;
        let defocus_disk_v = defocus_radius * v;

        Self {
            image,
            center: camera_center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            samples_per_pixel: config.samples_per_pixel,
            max_depth: config.max_depth,
            defocus_angle: config.defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        for y in (0..self.image.height()).progress() {
            for x in 0..self.image.width() {
                let mut pixel_color: [f32; 3] = [0.0, 0.0, 0.0];

                for _ in 0..(self.samples_per_pixel) {
                    let ray = self.get_ray(x as f32, y as f32);
                    let ray_color = ray_color(&ray, self.max_depth, world);

                    pixel_color = add_colors(pixel_color, ray_color.components);
                }

                self.image.put_pixel(
                    x,
                    y,
                    Rgba::from(
                        OpaqueColor::<Srgb>::new(
                            pixel_color
                                .map(|x| x / self.samples_per_pixel as f32)
                                .map(linear_to_gamma),
                        )
                        .to_rgba8()
                        .to_u8_array(),
                    ),
                );
            }
        }
    }

    fn get_ray(&self, i: f32, j: f32) -> Ray {
        let offset = random_unit_square();

        let pixel_sample = self.pixel00_loc
            + ((i + offset.x) * self.pixel_delta_u)
            + ((j + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.sample_defocus_disk()
        };

        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_defocus_disk(&self) -> Vec3 {
        let p = random_in_unit_disk();

        self.center + (p.x * self.defocus_disk_u + p.y * self.defocus_disk_v)
    }

    #[must_use]
    pub fn image(self) -> DynamicImage {
        self.image
    }
}

fn ray_color(ray: &Ray, depth: u32, world: &dyn Hittable) -> OpaqueColor<Srgb> {
    if depth == 0 {
        return OpaqueColor::BLACK;
    }

    world
        .hit(ray, Interval::new(0.001, f32::INFINITY))
        .map_or_else(
            || skybox(ray),
            |hit_rec| match hit_rec.material().scatter(ray, &hit_rec) {
                Some((scattered, attenuation)) => OpaqueColor::new(multiply_colors(
                    attenuation.components,
                    ray_color(&scattered, depth - 1, world).components,
                )),
                None => OpaqueColor::BLACK,
            },
        )
}

fn linear_to_gamma(x: f32) -> f32 {
    match x {
        x if x <= 0.0 => 0.0,
        x => x.sqrt(),
    }
}

fn skybox(ray: &Ray) -> OpaqueColor<Srgb> {
    let unit_direction = ray.direction().normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * OpaqueColor::WHITE + a * OpaqueColor::new([0.3, 0.5, 1.0])
}

fn add_colors(x: [f32; 3], y: [f32; 3]) -> [f32; 3] {
    [x[0] + y[0], x[1] + y[1], x[2] + y[2]]
}

fn multiply_colors(x: [f32; 3], y: [f32; 3]) -> [f32; 3] {
    [x[0] * y[0], x[1] * y[1], x[2] * y[2]]
}

#[derive(Clone, Copy, Debug)]
pub struct CameraConfig {
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub defocus_angle: f32,
    pub focus_distance: f32,
    pub vfov: f32,
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub v_up: Vec3,
}
