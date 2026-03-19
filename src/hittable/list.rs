use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

#[derive(Debug, Default)]
pub struct List {
    objects: Vec<Box<dyn Hittable>>,
}

impl List {
    #[must_use]
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for List {
    fn hit(&self, ray: Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if let Some(hit_rec) = object.hit(ray, ray_tmin, closest_so_far) {
                closest_so_far = hit_rec.t;
                hit_anything = Some(hit_rec);
            }
        }

        hit_anything
    }
}
