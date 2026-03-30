use crate::{hittable::HitRecord, material::Material, random::random_unit_vector, ray::Ray};
use color::{OpaqueColor, Srgb};

#[derive(Debug)]
pub struct Lambertian {
    albedo: OpaqueColor<Srgb>,
}

impl Lambertian {
    #[must_use]
    pub const fn new(albedo: OpaqueColor<Srgb>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_rec: &HitRecord) -> Option<(Ray, OpaqueColor<Srgb>)> {
        let mut scatter_direction = hit_rec.normal() + random_unit_vector();

        if scatter_direction.try_normalize().is_none() {
            scatter_direction = hit_rec.normal();
        }

        Some((Ray::new(hit_rec.point(), scatter_direction), self.albedo))
    }
}
