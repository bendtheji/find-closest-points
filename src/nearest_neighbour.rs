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
                                 k_nearest_neighbours: &mut BinaryHeap<Neighbour>, k :usize) {
    if let Some(x) = curr_node {
        let curr_node = x;
        let curr_point = &curr_node.point;
        let right_subtree = &curr_node.as_ref().right;
        let left_subtree = &curr_node.as_ref().left;
        let distance_to_other_bounding_box = (curr_point.get_dimension(curr_dimension) - given_point.get_dimension(curr_dimension)).abs();

        match given_point.compare_dimension(&curr_point, curr_dimension) {
            Ordering::Equal | Ordering::Greater => {
                find_k_nearest_neighbours(right_subtree, given_point, &curr_dimension.turn(), k_nearest_neighbours, k);
                if is_potential_nearer_neighbour(distance_to_other_bounding_box, k_nearest_neighbours, k) {
                    find_k_nearest_neighbours(left_subtree, given_point, &curr_dimension.turn(), k_nearest_neighbours, k);
                }
            }
            Ordering::Less => {
                find_k_nearest_neighbours(left_subtree, given_point, &curr_dimension.turn(), k_nearest_neighbours, k);
                if is_potential_nearer_neighbour(distance_to_other_bounding_box, k_nearest_neighbours, k) {
                    find_k_nearest_neighbours(right_subtree, given_point, &curr_dimension.turn(), k_nearest_neighbours, k);
                }
            }
        }

        let curr_distance = given_point.distance_to(&curr_point);

        if is_potential_nearer_neighbour(curr_distance, k_nearest_neighbours, k) {
            if k_nearest_neighbours.len() >= k { k_nearest_neighbours.pop(); }
            k_nearest_neighbours.push(Neighbour { distance: curr_distance, point: curr_point.clone() });
        }
    }
}

fn is_potential_nearer_neighbour(distance: f64, k_nearest_neighbours: &BinaryHeap<Neighbour>, k: usize) -> bool {
    if k_nearest_neighbours.len() < k { return true; }
    else {
        if let Some(n) = k_nearest_neighbours.peek() {
            if distance < n.distance {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod neighbours_test {
    use std::collections::BinaryHeap;
    use crate::nearest_neighbour::{is_potential_nearer_neighbour, Neighbour};
    use crate::point::Point;

    #[test]
    fn less_than_k_neighbours(){
        let neighbours = BinaryHeap::from(vec![
            Neighbour{ distance: 0.1, point: Point::new(0.1, 0.1, 0.1)}
        ]);
       let output = is_potential_nearer_neighbour(0.2, &neighbours, 2);
        assert!(output);
    }

    #[test]
    fn has_k_neighbours_and_is_potential_candidate(){
        let neighbours = BinaryHeap::from(vec![
            Neighbour{ distance: 0.1, point: Point::new(0.1, 0.1, 0.1)},
            Neighbour{ distance: 0.2, point: Point::new(0.2, 0.2, 0.2)}
        ]);
        let output = is_potential_nearer_neighbour(0.05, &neighbours, 2);
        assert!(output);
    }

    #[test]
    #[should_panic]
    fn has_k_neighbours_and_is_not_potential_candidate(){
        let neighbours = BinaryHeap::from(vec![
            Neighbour{ distance: 0.1, point: Point::new(0.1, 0.1, 0.1)},
            Neighbour{ distance: 0.2, point: Point::new(0.2, 0.2, 0.2)}
        ]);
        let output = is_potential_nearer_neighbour(0.21, &neighbours, 2);
        assert!(output);
    }
}