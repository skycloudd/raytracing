use crate::{hittable::HitRecord, ray::Ray};
use color::{OpaqueColor, Srgb};
use core::fmt::Debug;
use glam::Vec3;

mod lambertian;
mod metal;

pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material: Debug {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(Ray, OpaqueColor<Srgb>)>;
}

fn reflect(vec: Vec3, normal: Vec3) -> Vec3 {
    vec - 2.0 * vec.dot(normal) * normal
}
