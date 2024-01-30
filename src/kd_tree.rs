use std::cmp::Ordering;

use crate::point::Point;

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
        match construct_kd_tree(points, 3, 0) {
            Some(x) => *x,
            None => KdTreeNode { point: Default::default(), left: None, right: None }
        }
    }
}

fn construct_kd_tree(mut points: Vec<Point>, max_dimensions: u8, curr_dimension: u8) -> Option<Box<KdTreeNode>> {
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
            let curr_dimension = curr_dimension % max_dimensions;
            // got two or more elements, we need to
            // 1) find a random middle element as a pivot
            // 2) partition the two vectors into points less than current dimension
            // and points more than current dimension
            let (pivot, lesser, greater) = partition(points, curr_dimension);
            let mut pivot = KdTreeNode::new(pivot);
            pivot.left = construct_kd_tree(lesser, 3, curr_dimension + 1);
            pivot.right = construct_kd_tree(greater, 3, curr_dimension + 1);
            Some(Box::new(pivot))
        }
    }
}

fn partition(mut points: Vec<Point>, curr_dimension: u8) -> (Point, Vec<Point>, Vec<Point>) {
    // method 1: pick a random point as pivot
    // let mut rng = thread_rng();
    // let index = rng.gen_range(0..points.len());
    // let pivot = points.swap_remove(index);

    // method 2: sort in ascending order then take median
    // points.sort_by(|a, b| compare_dimension(a, b, curr_dimension));
    // let mid = points.len() / 2;
    // let pivot = points.swap_remove(mid);

    // method 3: find element closest to mean
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

fn calculate_mean(points: &Vec<Point>, curr_dimension: u8) -> f64 {
    points.iter().map(|p| p.get_dimension(curr_dimension)).sum::<f64>() / points.len() as f64
}

pub fn compare_dimension(pivot: &Point, other_point: &Point, curr_dimension: u8) -> Ordering {
    match curr_dimension {
        0 => pivot.x.total_cmp(&other_point.x),
        1 => pivot.y.total_cmp(&other_point.y),
        2 => pivot.z.total_cmp(&other_point.z),
        _ => unreachable!()
    }
}