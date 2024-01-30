use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::kd_tree::KdTreeNode;
use crate::point::{Dimension, Point};

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

pub fn find_k_nearest_neighbours(curr_node: &Option<Box<KdTreeNode>>, given_point: &Point, curr_dimension: &Dimension,
                                 k_nearest_neighbours: &mut BinaryHeap<Distance>) {
    if let Some(x) = curr_node {
        let curr_node = x;
        let curr_point = &curr_node.point;

        match given_point.compare_dimension(&curr_point, curr_dimension) {
            Ordering::Equal | Ordering::Greater => {
                find_k_nearest_neighbours(&curr_node.as_ref().right, given_point, &curr_dimension.turn(), k_nearest_neighbours);
                // check if we should be going into the other bounding box
                let distance_to_other_bounding_box = (curr_point.get_dimension(curr_dimension) - given_point.get_dimension(curr_dimension)).abs();
                if k_nearest_neighbours.len() < 10 {
                    find_k_nearest_neighbours(&curr_node.as_ref().left, given_point, &curr_dimension.turn(), k_nearest_neighbours);
                } else {
                    if let Some(d) = k_nearest_neighbours.peek() {
                        if distance_to_other_bounding_box < d.value {
                            find_k_nearest_neighbours(&curr_node.as_ref().left, given_point, &curr_dimension.turn(), k_nearest_neighbours);
                        }
                    }
                }
            }
            Ordering::Less => {
                find_k_nearest_neighbours(&curr_node.as_ref().left, given_point, &curr_dimension.turn(), k_nearest_neighbours);
                // check if we should be going into the other bounding box
                let distance_to_other_bounding_box = (curr_point.get_dimension(curr_dimension) - given_point.get_dimension(curr_dimension)).abs();
                if k_nearest_neighbours.len() < 10 {
                    find_k_nearest_neighbours(&curr_node.as_ref().right, given_point, &curr_dimension.turn(), k_nearest_neighbours);
                } else {
                    if let Some(d) = k_nearest_neighbours.peek() {
                        if distance_to_other_bounding_box < d.value {
                            find_k_nearest_neighbours(&curr_node.as_ref().right, given_point, &curr_dimension.turn(), k_nearest_neighbours);
                        }
                    }
                }
            }
        }

        let curr_distance = given_point.distance_to(&curr_point);

        // two cases, one is binary heap has 10 points already
        // 1) there are less than ten points inside, so we just push
        if k_nearest_neighbours.len() < 10 {
            k_nearest_neighbours.push(Distance { value: curr_distance, other_point: curr_point.clone() });
        }
        // 2) we need to check against the top, if curr node distance is shorter
        // we pop the top and push this new one in
        else {
            if let Some(d) = k_nearest_neighbours.peek() {
                if curr_distance < d.value {
                    k_nearest_neighbours.pop();
                    k_nearest_neighbours.push(Distance { value: curr_distance, other_point: curr_point.clone() });
                }
            }
        }
    }
}