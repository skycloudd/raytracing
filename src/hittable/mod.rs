use crate::{interval::Interval, ray::Ray};
use core::fmt::Debug;
use glam::Vec3;

mod list;
mod sphere;

pub use list::List;
pub use sphere::Sphere;

pub trait Hittable: Debug {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord>;
}

#[derive(Debug)]
pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    fn new(point: Vec3, t: f32, ray: Ray, outward_normal: Vec3) -> Self {
        let mut rec = Self {
            point,
            normal: Vec3::ZERO,
            t,
            front_face: false,
        };

        rec.set_face_normal(ray, outward_normal);

        rec
    }

    fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }

    #[must_use]
    pub const fn normal(&self) -> Vec3 {
        self.normal
    }
}
