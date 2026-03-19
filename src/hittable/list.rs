use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
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
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = ray_t.max();

        for object in &self.objects {
            if let Some(hit_rec) = object.hit(ray, Interval::new(ray_t.min(), closest_so_far)) {
                closest_so_far = hit_rec.t;
                hit_anything = Some(hit_rec);
            }
        }

        hit_anything
    }
}
