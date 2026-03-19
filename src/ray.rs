use glam::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub const fn origin(&self) -> Vec3 {
        self.origin
    }

    pub const fn direction(&self) -> Vec3 {
        self.direction
    }
}
