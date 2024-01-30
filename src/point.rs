use std::cmp::Ordering;

use rand::{Rng, thread_rng};

#[derive(Clone, Debug, Default, PartialEq)]
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

    pub fn get_dimension(&self, dimension: &Dimension) -> f64 {
        match dimension {
            Dimension::X => self.x,
            Dimension::Y => self.y,
            Dimension::Z => self.z,
        }
    }

    pub fn compare_dimension(&self, other: &Point, dimension: &Dimension) -> Ordering {
        self.get_dimension(dimension).total_cmp(&other.get_dimension(dimension))
    }
}

fn clamp(coordinate: f64) -> f64 {
    coordinate.max(0.0).min(1.0)
}

#[cfg(test)]
mod point_test {
    use std::cmp::Ordering;

    use super::{clamp, Dimension, Point};

    #[test]
    fn clamp_lowest_val() {
        let output = clamp(-0.01);
        let expected = 0.0;
        assert_eq!(output, expected);
    }

    #[test]
    fn clamp_highest_val() {
        let output = clamp(1.01);
        let expected = 1.0;
        assert_eq!(output, expected);
    }

    #[test]
    fn clamp_valid_val() {
        let output = clamp(0.73);
        let expected = 0.73;
        assert_eq!(output, expected);
    }

    #[test]
    fn point_too_far_back() {
        let output = Point::new(-0.1, -0.1, -0.1);
        let expected = Point::new(0.0, 0.0, 0.0);
        assert_eq!(output, expected);
    }

    #[test]
    fn point_too_far_front() {
        let output = Point::new(1.1, 1.1, 1.1);
        let expected = Point::new(1.0, 1.0, 1.0);
        assert_eq!(output, expected);
    }

    #[test]
    fn valid_point() {
        let output = Point::new(0.5, 0.5, 0.5);
        let expected = Point::new(0.5, 0.5, 0.5);
        assert_eq!(output, expected);
    }

    #[test]
    fn distance_to() {
        let point_one = Point::new(0.0, 0.0, 0.0);
        let point_two = Point::new(0.2, 0.3, 0.6);
        let output = point_one.distance_to(&point_two);
        let expected = 0.7;
        assert_eq!(output, expected);
    }

    #[test]
    fn get_x_dimension() {
        let point = Point::new(0.1, 0.2, 0.3);
        let output = point.get_dimension(&Dimension::X);
        let expected = 0.1;
        assert_eq!(output, expected);
    }

    #[test]
    fn get_y_dimension() {
        let point = Point::new(0.1, 0.2, 0.3);
        let output = point.get_dimension(&Dimension::Y);
        let expected = 0.2;
        assert_eq!(output, expected);
    }

    #[test]
    fn get_z_dimension() {
        let point = Point::new(0.1, 0.2, 0.3);
        let output = point.get_dimension(&Dimension::Z);
        let expected = 0.3;
        assert_eq!(output, expected);
    }

    #[test]
    fn compare_x_dimension() {
        let point_one = Point::new(0.2, 0.4, 0.6);
        let point_two = Point::new(0.1, 0.2, 0.3);

        let output = point_one.compare_dimension(&point_two, &Dimension::X);
        let expected = Ordering::Greater;
        assert_eq!(output, expected);
    }

    #[test]
    fn compare_y_dimension() {
        let point_one = Point::new(0.2, 0.4, 0.6);
        let point_two = Point::new(0.1, 0.2, 0.3);

        let output = point_one.compare_dimension(&point_two, &Dimension::Y);
        let expected = Ordering::Greater;
        assert_eq!(output, expected);
    }

    #[test]
    fn compare_z_dimension() {
        let point_one = Point::new(0.2, 0.4, 0.6);
        let point_two = Point::new(0.1, 0.2, 0.3);

        let output = point_one.compare_dimension(&point_two, &Dimension::Z);
        let expected = Ordering::Greater;
        assert_eq!(output, expected);
    }
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
mod dimension_test {
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