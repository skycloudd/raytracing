use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
};
use glam::Vec3;
use std::rc::Rc;

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    #[must_use]
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Self {
        assert!(radius >= 0.);

        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin();

        let a = ray.direction().length_squared();
        let h = ray.direction().dot(oc);
        let c = self.radius.mul_add(-self.radius, oc.length_squared());

        let discriminant = h.mul_add(h, -(a * c));

        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;

        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;

            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let hit_t = root;
        let hit_point = ray.at(hit_t);
        let outward_normal = (hit_point - self.center) / self.radius;

        Some(HitRecord::new(
            hit_point,
            hit_t,
            ray,
            outward_normal,
            self.material.clone(),
        ))
    }
}
