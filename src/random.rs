use glam::{Vec2, Vec3};

pub fn random_unit_square() -> Vec2 {
    let x = rand::random_range(-0.5..0.5);
    let y = rand::random_range(-0.5..0.5);

    Vec2::new(x, y)
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::new(
            rand::random_range(-1.0..=1.0),
            rand::random_range(-1.0..=1.0),
            rand::random_range(-1.0..=1.0),
        );

        let length_sq = p.length_squared();

        if 1.0e-160 < length_sq && length_sq <= 1.0 {
            return p / length_sq.sqrt();
        }
    }
}

pub fn random_in_unit_disk() -> Vec2 {
    loop {
        let p = Vec2::new(
            rand::random_range(-1.0..=1.0),
            rand::random_range(-1.0..=1.0),
        );

        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
