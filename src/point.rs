use std::cmp::Ordering;

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
        ((self.x - other.x).powi(2)
            + (self.y - other.y).powi(2)
            + (self.z - other.z).powi(2)).sqrt()
    }

    pub fn distance_sq(&self, other: &Point) -> f64 {
        (self.x - other.x).powi(2)
            + (self.y - other.y).powi(2)
            + (self.z - other.z).powi(2)
    }

    pub fn get_dimension(&self, dimension: u8) -> f64 {
        match dimension {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => unreachable!()
        }
    }

    pub fn compare_dimension(&self, other: &Point, dimension: u8) -> Ordering {
        match dimension {
            0 => self.x.total_cmp(&other.x),
            1 => self.y.total_cmp(&other.y),
            2 => self.z.total_cmp(&other.z),
            _ => unreachable!()
        }
    }
}

fn clamp(coordinate: f64) -> f64 {
    coordinate.max(0.0).min(1.0)
}
