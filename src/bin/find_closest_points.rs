use std::collections::BinaryHeap;

use find_closest_points::kd_tree::KdTreeNode;
use find_closest_points::nearest_neighbour::{find_k_nearest_neighbours, Neighbour};
use find_closest_points::point::{Dimension, Point};

fn main() {
    println!("generating random points");
    let mut points = vec![];
    for _ in 0..100_000 {
        points.push(Point::random());
    }

    println!("constructing tree...");
    let tree = KdTreeNode::construct_tree(points.clone());
    println!("construction finished");

    let given_point = Point::random();
    let mut heap = BinaryHeap::new();
    println!("finding nearest neighbours");
    find_k_nearest_neighbours(&Some(Box::new(tree)), &given_point, &Dimension::X, &mut heap, 10);
    let mut heap_vec = heap.into_iter().collect::<Vec<Neighbour>>();
    heap_vec.sort();
    println!("given point: {:?}", given_point);
    for neighbour in &heap_vec {
        println!("Nearest Neighbour, value: {:?}, point: {:?}", neighbour.distance, neighbour.point);
    }

    let mut neighbours = points.into_iter().map(|p| Neighbour { distance: given_point.distance_to(&p), point: p }).collect::<Vec<Neighbour>>();
    neighbours.sort();
    println!("using sort: ");
    for i in 0..10 {
        println!("Nearest Neighbour, value: {:?}, point: {:?}", neighbours[i].distance, neighbours[i].point);
    }

    assert_eq!(heap_vec, &neighbours[0..10]);
}