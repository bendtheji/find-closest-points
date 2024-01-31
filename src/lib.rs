use crate::point::Point;

pub mod point;
pub mod kd_tree;
pub mod nearest_neighbour;

pub const NUM_OF_NEAREST_NEIGHBOURS: usize = 10;
pub fn generate_random_points(n: u32) -> Vec<Point> {
    let mut points = vec![];
    for _ in 0..n {
        points.push(Point::random());
    }
    points
}