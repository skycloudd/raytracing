use crate::{
    hittable::HitRecord,
    material::{Material, reflect},
    random::random_unit_vector,
    ray::Ray,
};
use color::{OpaqueColor, Srgb};

#[derive(Debug)]
pub struct Metal {
    albedo: OpaqueColor<Srgb>,
    fuzz: f32,
}

impl Metal {
    #[must_use]
    pub const fn new(albedo: OpaqueColor<Srgb>, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(Ray, OpaqueColor<Srgb>)> {
        let reflected = reflect(ray.direction(), hit_rec.normal()).normalize()
            + self.fuzz * random_unit_vector();

        let scattered = Ray::new(hit_rec.point(), reflected);

        (scattered.direction().dot(hit_rec.normal()) > 0.0).then_some((scattered, self.albedo))
    }
}
