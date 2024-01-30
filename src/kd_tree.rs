use std::cmp::Ordering;

use rand::Rng;

use crate::point::{Dimension, Point};

#[derive(Debug, Clone)]
pub struct KdTreeNode {
    pub point: Point,
    pub left: Option<Box<KdTreeNode>>,
    pub right: Option<Box<KdTreeNode>>,
}

impl KdTreeNode {
    fn new(point: Point) -> KdTreeNode {
        KdTreeNode {
            point,
            left: None,
            right: None,
        }
    }

    pub fn construct_tree(points: Vec<Point>) -> KdTreeNode {
        match construct_kd_tree(points, &Dimension::X) {
            Some(x) => *x,
            None => KdTreeNode { point: Default::default(), left: None, right: None }
        }
    }
}

fn construct_kd_tree(mut points: Vec<Point>, curr_dimension: &Dimension) -> Option<Box<KdTreeNode>> {
    // base cases
    // remaining points is 1 or 0 length
    match points.len() {
        // length is zero, then no more children to append
        0 => None,
        // length is one, append this child
        1 => {
            let point = points.swap_remove(0);
            let node = KdTreeNode::new(point);
            Some(Box::new(node))
        }
        _ => {
            // got two or more elements, we need to
            // 1) find a random middle element as a pivot
            // 2) partition the two vectors into points less than current dimension
            // and points more than current dimension
            let (pivot, lesser, greater) = partition(points, curr_dimension);
            let mut pivot = KdTreeNode::new(pivot);
            pivot.left = construct_kd_tree(lesser, &curr_dimension.turn());
            pivot.right = construct_kd_tree(greater, &curr_dimension.turn());
            Some(Box::new(pivot))
        }
    }
}

fn partition(mut points: Vec<Point>, curr_dimension: &Dimension) -> (Point, Vec<Point>, Vec<Point>) {
    let mean = calculate_mean(&points, curr_dimension);
    let mut min_diff = f64::MAX;
    let mut mean_index = 0;
    for (i, point) in points.iter().enumerate() {
        let curr_point_dimension = point.get_dimension(curr_dimension);
        let diff = (curr_point_dimension - mean).abs();
        if diff < min_diff {
            min_diff = diff;
            mean_index = i;
        }
    }
    let pivot = points.swap_remove(mean_index);

    let mut lesser = vec![];
    let mut greater = vec![];

    for point in points {
        match pivot.compare_dimension(&point, curr_dimension) {
            // if pivot is greater than current point in that dimension, current point should be added to the lesser group
            Ordering::Greater => lesser.push(point),
            // if pivot is less than or equal to current point in dimension, add current point to greater group
            Ordering::Equal | Ordering::Less => greater.push(point),
        }
    }
    (pivot, lesser, greater)
}

fn get_pivot(points: &mut Vec<Point>, curr_dimension: &Dimension) -> Point {
    let mean = calculate_mean(&points, curr_dimension);
    let mut min_diff = f64::MAX;
    let mut mean_index = 0;
    for (i, point) in points.iter().enumerate() {
        let curr_point_dimension = point.get_dimension(curr_dimension);
        let diff = (curr_point_dimension - mean).abs();
        if diff < min_diff {
            min_diff = diff;
            mean_index = i;
        }
    }
    points.swap_remove(mean_index)
}

fn calculate_mean(points: &Vec<Point>, curr_dimension: &Dimension) -> f64 {
    if points.is_empty() { return f64::default(); }
    points.iter().map(|p| p.get_dimension(curr_dimension)).sum::<f64>() / points.len() as f64
}

#[cfg(test)]
mod kd_tree_test {
    use crate::point::{Dimension, Point};

    use super::calculate_mean;

    #[test]
    fn get_x_mean_from_points() {
        let points = vec![
            Point::new(0.1, 0.2, 0.3),
            Point::new(0.2, 0.4, 0.6),
            Point::new(0.3, 0.6, 0.9),
        ];
        let output = calculate_mean(&points, &Dimension::X);
        let expected = (0.1 + 0.2 + 0.3) / 3.0;
        assert_eq!(output, expected);
    }

    #[test]
    fn get_y_mean_from_points() {
        let points = vec![
            Point::new(0.1, 0.2, 0.3),
            Point::new(0.2, 0.4, 0.6),
            Point::new(0.3, 0.6, 0.9),
        ];
        let output = calculate_mean(&points, &Dimension::Y);
        let expected = (0.2 + 0.4 + 0.6) / 3.0;
        assert_eq!(output, expected);
    }

    #[test]
    fn get_z_mean_from_points() {
        let points = vec![
            Point::new(0.1, 0.2, 0.3),
            Point::new(0.2, 0.4, 0.6),
            Point::new(0.3, 0.6, 0.9),
        ];
        let output = calculate_mean(&points, &Dimension::Z);
        let expected = (0.3 + 0.6 + 0.9) / 3.0;
        assert_eq!(output, expected);
    }

    #[test]
    fn get_mean_from_empty_vec() {
        let points = vec![];
        let output = calculate_mean(&points, &Dimension::X);
        let expected = 0.0;
        assert_eq!(output, expected);
    }
}
