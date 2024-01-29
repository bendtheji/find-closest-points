use std::collections::BinaryHeap;

use find_closest_points::kd_tree::{Distance, find_k_nearest_neighbours, KdTreeNode};
use find_closest_points::point::Point;

fn main() {
    let mut points = vec![];
    for i in 0..10000000 {
        points.push(Point::random());
    }
    let mut clone_points: Vec<Point> = points.clone();
    let tree = KdTreeNode::construct_tree(points);

    let given_point = Point::random();
    let mut heap = BinaryHeap::new();
    find_k_nearest_neighbours(Some(Box::new(tree)), &given_point, 0, 3, &mut heap);
    println!("given point: {:?}", given_point);
    for distance in &heap {
        println!("Distance, value: {:?}, point: {:?}", distance.value, distance.other_point);
    }

    let mut distances = clone_points.into_iter().map(|p| Distance { value: given_point.distance_to(&p), other_point: p }).collect::<Vec<Distance>>();
    distances.sort();
    println!("using sort: ");
    for i in 0..10 {
        println!("Distance, value: {:?}, point: {:?}", distances[i].value, distances[i].other_point);
    }

    let mut heap_vec = heap.into_iter().collect::<Vec<Distance>>();
    heap_vec.sort();
    assert_eq!(heap_vec, &distances[0..10]);
}