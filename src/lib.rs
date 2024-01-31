//!
//! A rust library for finding the 10 nearest neighbours to a target point in a 3D space amongst a huge number of 3D points using a kd-tree.
//!
//! ## Description
//! Generate a huge number of random 3D points and build a kd-tree with those points, then use the kd-tree to find the nearest neighbours to the target point.
use crate::point::Point;

pub mod point;
pub mod kd_tree;
pub mod nearest_neighbour;

/// Const for the number of nearest neighbours that we want to find for most of our tests.
pub const NUM_OF_NEAREST_NEIGHBOURS: usize = 10;

/// Function for generating n random points.
pub fn generate_random_points(n: u32) -> Vec<Point> {
    let mut points = vec![];
    for _ in 0..n {
        points.push(Point::random());
    }
    points
}