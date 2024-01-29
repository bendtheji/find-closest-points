use std::cmp;

use rand::{Rng, thread_rng};

#[derive(Clone, Debug, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            x: clamp(x),
            y: clamp(y),
            z: clamp(z),
        }
    }

    pub fn random() -> Point {
        let mut rng = thread_rng();
        Point::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powf(2.0)
            + (self.y - other.y).powf(2.0)
            + (self.z - other.z).powf(2.0)).sqrt()
    }

    pub fn get_dimension(&self, dimension: u8) -> f64 {
        match dimension {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => unreachable!()
        }
    }
}

fn clamp(coordinate: f64) -> f64 {
    coordinate.max(0.0).min(1.0)
}
