use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::kd_tree::KdTreeNode;
use crate::point::{Dimension, Point};

#[derive(Debug)]
pub struct Neighbour {
    pub distance: f64,
    pub point: Point,
}

impl Eq for Neighbour {}

impl PartialEq<Self> for Neighbour {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl PartialOrd<Self> for Neighbour {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for Neighbour {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.total_cmp(&other.distance)
    }
}

pub fn find_k_nearest_neighbours(curr_node: &Option<Box<KdTreeNode>>, given_point: &Point, curr_dimension: &Dimension,
                                 k_nearest_neighbours: &mut BinaryHeap<Neighbour>) {
    if let Some(x) = curr_node {
        let curr_node = x;
        let curr_point = &curr_node.point;
        let right_subtree = &curr_node.as_ref().right;
        let left_subtree = &curr_node.as_ref().left;

        match given_point.compare_dimension(&curr_point, curr_dimension) {
            Ordering::Equal | Ordering::Greater => {
                traverse(curr_point, given_point, curr_dimension, right_subtree,
                         left_subtree, k_nearest_neighbours);
            }
            Ordering::Less => {
                traverse(curr_point, given_point, curr_dimension, left_subtree, right_subtree, k_nearest_neighbours);
            }
        }

        let curr_distance = given_point.distance_to(&curr_point);

        // two cases, one is binary heap has 10 points already
        // 1) there are less than ten points inside, so we just push
        if k_nearest_neighbours.len() < 10 {
            k_nearest_neighbours.push(Neighbour { distance: curr_distance, point: curr_point.clone() });
        }
        // 2) we need to check against the top, if curr node distance is shorter
        // we pop the top and push this new one in
        else {
            if let Some(n) = k_nearest_neighbours.peek() {
                if curr_distance < n.distance {
                    k_nearest_neighbours.pop();
                    k_nearest_neighbours.push(Neighbour { distance: curr_distance, point: curr_point.clone() });
                }
            }
        }
    }
}

fn traverse(curr_point: &Point, given_point: &Point, curr_dimension: &Dimension,
            main_subtree: &Option<Box<KdTreeNode>>, other_subtree: &Option<Box<KdTreeNode>>,
            k_nearest_neighbours: &mut BinaryHeap<Neighbour>,
) {
    find_k_nearest_neighbours(main_subtree, given_point, &curr_dimension.turn(), k_nearest_neighbours);
    // check if we should be going into the other bounding box
    let distance_to_other_bounding_box = (curr_point.get_dimension(curr_dimension) - given_point.get_dimension(curr_dimension)).abs();
    if k_nearest_neighbours.len() < 10 {
        find_k_nearest_neighbours(other_subtree, given_point, &curr_dimension.turn(), k_nearest_neighbours);
    } else {
        if let Some(n) = k_nearest_neighbours.peek() {
            if distance_to_other_bounding_box < n.distance {
                find_k_nearest_neighbours(other_subtree, given_point, &curr_dimension.turn(), k_nearest_neighbours);
            }
        }
    }
}