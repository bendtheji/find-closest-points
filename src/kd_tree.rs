use std::cmp::Ordering;

use crate::point::{Dimension, Point};

/// Struct that represents a node in the kd-tree data structure.
#[derive(Debug, Clone, PartialEq)]
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

    /// Constructs a kd-tree from a vector of `Point` objects.
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
            let (pivot, left_sub_tree, right_sub_tree) = partition(points, curr_dimension);
            let mut pivot = KdTreeNode::new(pivot);
            pivot.left = construct_kd_tree(left_sub_tree, &curr_dimension.turn());
            pivot.right = construct_kd_tree(right_sub_tree, &curr_dimension.turn());
            Some(Box::new(pivot))
        }
    }
}

fn partition(mut points: Vec<Point>, curr_dimension: &Dimension) -> (Point, Vec<Point>, Vec<Point>) {
    let pivot = get_pivot(&mut points, curr_dimension);

    let mut left_sub_tree = vec![];
    let mut right_sub_tree = vec![];

    for point in points {
        match point.compare_dimension(&pivot, curr_dimension) {
            Ordering::Equal | Ordering::Greater => right_sub_tree.push(point),
            Ordering::Less => left_sub_tree.push(point),
        }
    }
    (pivot, left_sub_tree, right_sub_tree)
}

/// We calculate the mean then find the pivot point that has the closest value
/// in the current dimension.
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

    use super::{calculate_mean, construct_kd_tree, get_pivot, KdTreeNode, partition};

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

    #[test]
    fn get_pivot_along_x_axis() {
        let mut points = vec![
            Point::new(0.1, 0.2, 0.3),
            Point::new(0.2, 0.3, 0.1),
            Point::new(0.3, 0.1, 0.2),
        ];
        let output = get_pivot(&mut points, &Dimension::X);
        let expected = Point::new(0.2, 0.3, 0.1);
        assert_eq!(output, expected);
    }

    #[test]
    fn get_pivot_along_y_axis() {
        let mut points = vec![
            Point::new(0.1, 0.2, 0.3),
            Point::new(0.2, 0.3, 0.1),
            Point::new(0.3, 0.1, 0.2),
        ];
        let output = get_pivot(&mut points, &Dimension::Y);
        let expected = Point::new(0.1, 0.2, 0.3);
        assert_eq!(output, expected);
    }

    #[test]
    fn get_pivot_along_z_axis() {
        let mut points = vec![
            Point::new(0.1, 0.2, 0.3),
            Point::new(0.2, 0.3, 0.1),
            Point::new(0.3, 0.1, 0.2),
        ];
        let output = get_pivot(&mut points, &Dimension::Z);
        let expected = Point::new(0.3, 0.1, 0.2);
        assert_eq!(output, expected);
    }

    #[test]
    fn get_pivot_between_same_values() {
        let mut points = vec![
            Point::new(0.1, 0.2, 0.3),
            Point::new(0.1, 0.3, 0.2),
        ];
        let output = get_pivot(&mut points, &Dimension::X);
        let expected = Point::new(0.1, 0.2, 0.3);
        assert_eq!(output, expected);
    }

    #[test]
    fn partition_by_x() {
        let points = vec![
            Point::new(0.1, 0.2, 0.3),
            Point::new(0.2, 0.3, 0.5),
            Point::new(0.3, 0.5, 0.6),
            Point::new(0.4, 0.1, 0.3),
        ];
        let output = partition(points, &Dimension::X);
        let expected = (
            Point::new(0.2, 0.3, 0.5),
            vec![Point::new(0.1, 0.2, 0.3)],
            vec![Point::new(0.4, 0.1, 0.3), Point::new(0.3, 0.5, 0.6)]
        );
        assert_eq!(output, expected);
    }

    #[test]
    fn partition_by_y() {
        let points = vec![
            Point::new(0.1, 0.2, 0.3),
            Point::new(0.2, 0.3, 0.5),
            Point::new(0.3, 0.5, 0.6),
            Point::new(0.4, 0.1, 0.3),
        ];
        let output = partition(points, &Dimension::Y);
        let expected = (
            Point::new(0.2, 0.3, 0.5),
            vec![Point::new(0.1, 0.2, 0.3), Point::new(0.4, 0.1, 0.3)],
            vec![Point::new(0.3, 0.5, 0.6)]
        );
        assert_eq!(output, expected);
    }

    #[test]
    fn partition_by_z() {
        let points = vec![
            Point::new(0.1, 0.2, 0.3),
            Point::new(0.2, 0.3, 0.5),
            Point::new(0.3, 0.5, 0.6),
            Point::new(0.4, 0.1, 0.3),
        ];
        let output = partition(points, &Dimension::Z);
        let expected = (
            Point::new(0.2, 0.3, 0.5),
            vec![Point::new(0.1, 0.2, 0.3), Point::new(0.4, 0.1, 0.3)],
            vec![Point::new(0.3, 0.5, 0.6)]
        );
        assert_eq!(output, expected);
    }

    #[test]
    fn partition_len_2_vec_empty_right() {
        let points = vec![
            Point::new(0.2, 0.3, 0.4),
            Point::new(0.4, 0.7, 0.9),
        ];
        let output = partition(points, &Dimension::Y);
        let expected = (
            Point::new(0.4, 0.7, 0.9),
            vec![Point::new(0.2, 0.3, 0.4)],
            vec![]
        );
        assert_eq!(output, expected);
    }

    #[test]
    fn partition_len_2_vec_empty_left() {
        let points = vec![
            Point::new(0.2, 0.3, 0.4),
            Point::new(0.4, 0.7, 0.9),
        ];
        let output = partition(points, &Dimension::Z);
        let expected = (
            Point::new(0.2, 0.3, 0.4),
            vec![],
            vec![Point::new(0.4, 0.7, 0.9)]
        );
        assert_eq!(output, expected);
    }

    #[test]
    fn build_kd_tree_empty_vec() {
        let points = vec![];
        let output = construct_kd_tree(points, &Dimension::X);
        let expected = None;
        assert_eq!(output, expected);
    }

    #[test]
    fn build_kd_tree_one_point() {
        let points = vec![Point::new(0.1, 0.1, 0.1)];
        let output = construct_kd_tree(points, &Dimension::X);
        let expected = Some(Box::new(KdTreeNode::new(Point::new(0.1, 0.1, 0.1))));
        assert_eq!(output, expected);
    }

    #[test]
    fn build_kd_tree() {
        let points = vec![
            Point::new(0.1, 0.1, 0.1),
            Point::new(0.2, 0.2, 0.2),
            Point::new(0.3, 0.3, 0.3),
        ];
        // simple kd tree
        let left_subtree = Some(Box::new(KdTreeNode::new(Point::new(0.1, 0.1, 0.1))));
        let right_subtree = Some(Box::new(KdTreeNode::new(Point::new(0.3, 0.3, 0.3))));
        let mut root = KdTreeNode::new(Point::new(0.2, 0.2, 0.2));
        root.left = left_subtree;
        root.right = right_subtree;
        let root = Some(Box::new(root));

        let output = construct_kd_tree(points, &Dimension::X);
        let expected = root;

        assert_eq!(output, expected);
    }
}

