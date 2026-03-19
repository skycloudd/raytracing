#[derive(Debug)]
pub struct Interval {
    min: f32,
    max: f32,
}

impl Interval {
    pub const EMPTY: Self = Self::new(f32::INFINITY, f32::NEG_INFINITY);
    pub const UNIVERSE: Self = Self::new(f32::NEG_INFINITY, f32::INFINITY);

    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub const fn min(&self) -> f32 {
        self.min
    }

    pub const fn max(&self) -> f32 {
        self.max
    }
}
