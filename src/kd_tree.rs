use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;

use rand::{Rng, thread_rng};

use crate::point::Point;

#[derive(Debug, Clone)]
pub struct KdTreeNode {
    point: Point,
    left: Option<Box<KdTreeNode>>,
    right: Option<Box<KdTreeNode>>,
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
    // pick a random point as pivot
    // let mut rng = thread_rng();
    // let index = rng.gen_range(0..points.len());
    // let pivot = points.swap_remove(index);
    points.sort_by(|a,b| compare_dimension(a, b, curr_dimension));
    let mid = points.len() / 2;
    let pivot = points.swap_remove(mid);
    let mut lesser = vec![];
    let mut greater = vec![];

    for point in points {
        match compare_dimension(&pivot, &point, curr_dimension) {
            // if pivot is greater than current point in that dimension, current point should be added to the lesser group
            Ordering::Greater => lesser.push(point),
            // if pivot is less than or equal to current point in dimension, add current point to greater group
            Ordering::Equal | Ordering::Less => greater.push(point),
        }
    }
    (pivot, lesser, greater)
}

fn compare_dimension(pivot: &Point, other_point: &Point, curr_dimension: u8) -> Ordering {
    match curr_dimension {
        0 => pivot.x.total_cmp(&other_point.x),
        1 => pivot.y.total_cmp(&other_point.y),
        2 => pivot.z.total_cmp(&other_point.z),
        _ => unreachable!()
    }
}

#[derive(Debug)]
pub struct Distance {
    pub value: f64,
    pub other_point: Point,
}

impl Eq for Distance {}

impl PartialEq<Self> for Distance {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl PartialOrd<Self> for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.total_cmp(&other.value)
    }
}

pub fn find_k_nearest_neighbours(curr_node: Option<Box<KdTreeNode>>, given_point: &Point, curr_dimension: u8, max_dimension: u8,
                             k_nearest_neighbours: &mut BinaryHeap<Distance>) {
    if let Some(x) = curr_node {
        let curr_node = *x;
        let curr_point = curr_node.point;
        let curr_distance = given_point.distance_to(&curr_point);

        // two cases, one is binary heap has 10 points already
        // 1) there are less than ten points inside, so we just push
        if k_nearest_neighbours.len() < 10 {
            k_nearest_neighbours.push(Distance { value: curr_distance, other_point: curr_point.clone() });
        }
        // 2) we need to check against the top, if curr node distance is shorter
        // we pop the top and push this new one in
        else {
            match k_nearest_neighbours.peek() {
                Some(x) if x.value > curr_distance => {
                    k_nearest_neighbours.pop();
                    k_nearest_neighbours.push(Distance { value: curr_distance, other_point: curr_point.clone() });
                }
                _ => {}
            }
        }

        let curr_dimension = curr_dimension % max_dimension;
        match compare_dimension(&curr_point, given_point, curr_dimension) {
            // if current node current dimension is greater than given node, go into left subtree
            Ordering::Greater => {
                find_k_nearest_neighbours(curr_node.left, given_point, curr_dimension + 1, max_dimension, k_nearest_neighbours);
                // check if we should be going into the other bounding box
                let distance_to_other_bounding_box = (curr_point.get_dimension(curr_dimension) - given_point.get_dimension(curr_dimension)).abs();
                if let Some(d) = k_nearest_neighbours.peek() {
                    if distance_to_other_bounding_box <= d.value {
                        find_k_nearest_neighbours(curr_node.right, given_point, curr_dimension + 1, max_dimension, k_nearest_neighbours);
                    }
                }
            }
            // if current node current dimension is equal or less than given node, go into right subtree
            Ordering::Equal | Ordering::Less => {
                find_k_nearest_neighbours(curr_node.right, given_point, curr_dimension + 1, max_dimension, k_nearest_neighbours);
                // check if we should be going into the other bounding box
                let distance_to_other_bounding_box = (curr_point.get_dimension(curr_dimension) - given_point.get_dimension(curr_dimension)).abs();
                if let Some(d) = k_nearest_neighbours.peek() {
                    if distance_to_other_bounding_box <= d.value {
                        find_k_nearest_neighbours(curr_node.left, given_point, curr_dimension + 1, max_dimension, k_nearest_neighbours);
                    }
                }
            }
        }
    }
}