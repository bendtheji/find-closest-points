use std::collections::BinaryHeap;

use find_closest_points::kd_tree::KdTreeNode;
use find_closest_points::point::{Dimension, Point};
use find_closest_points::nearest_neighbour::{Distance, find_k_nearest_neighbours};

fn main() {
    println!("generating random points");
    let mut points = vec![];
    for _ in 0..30 {
        points.push(Point::random());
    }

    println!("constructing tree...");
    let tree = KdTreeNode::construct_tree(points.clone());
    println!("construction finished");

    let given_point = Point::random();
    let mut heap = BinaryHeap::new();
    println!("finding nearest neighbours");
    find_k_nearest_neighbours(&Some(Box::new(tree)), &given_point, &Dimension::X, &mut heap);
    let mut heap_vec = heap.into_iter().collect::<Vec<Distance>>();
    heap_vec.sort();
    println!("given point: {:?}", given_point);
    for distance in &heap_vec {
        println!("Distance, value: {:?}, point: {:?}", distance.value, distance.other_point);
    }

    let mut distances = points.into_iter().map(|p| Distance { value: given_point.distance_to(&p), other_point: p }).collect::<Vec<Distance>>();
    distances.sort();
    println!("using sort: ");
    for i in 0..10 {
        println!("Distance, value: {:?}, point: {:?}", distances[i].value, distances[i].other_point);
    }

    assert_eq!(heap_vec, &distances[0..10]);
}