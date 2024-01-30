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

    pub fn get_dimension(&self, dimension: &Dimension) -> f64 {
        match dimension {
            Dimension::X => self.x,
            Dimension::Y => self.y,
            Dimension::Z => self.z,
        }
    }

    pub fn compare_dimension(&self, other: &Point, dimension: &Dimension) -> Ordering {
        match dimension {
            Dimension::X => self.x.total_cmp(&other.x),
            Dimension::Y => self.y.total_cmp(&other.y),
            Dimension::Z => self.z.total_cmp(&other.z),
        }
    }
}

fn clamp(coordinate: f64) -> f64 {
    coordinate.max(0.0).min(1.0)
}

#[derive(Debug, PartialEq)]
pub enum Dimension {
    X,
    Y,
    Z,
}

impl Dimension {
    pub fn turn(&self) -> Self {
        use Dimension::*;
        match self {
            X => Y,
            Y => Z,
            Z => X
        }
    }
}

#[cfg(test)]
mod dimension_test{
    use super::Dimension;

    #[test]
    fn turn_x() {
        assert_eq!(Dimension::X.turn(), Dimension::Y);
    }

    #[test]
    fn turn_y() {
        assert_eq!(Dimension::Y.turn(), Dimension::Z);
    }

    #[test]
    fn turn_z() {
        assert_eq!(Dimension::Z.turn(), Dimension::X);
    }
}