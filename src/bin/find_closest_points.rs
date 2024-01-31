use std::collections::BinaryHeap;

use find_closest_points::kd_tree::KdTreeNode;
use find_closest_points::nearest_neighbour::{find_k_nearest_neighbours, Neighbour};
use find_closest_points::{generate_random_points, NUM_OF_NEAREST_NEIGHBOURS};
use find_closest_points::point::{Dimension, Point};

fn main() {
    println!("generating 100_000 random points");
    let points = generate_random_points(100_000);

    println!("constructing tree...");
    let tree = KdTreeNode::construct_tree(points.clone());
    println!("construction finished");

    println!("finding nearest neighbours");
    let given_point = Point::random();
    let mut heap = BinaryHeap::new();
    find_k_nearest_neighbours(&Some(Box::new(tree)), &given_point, &Dimension::X, &mut heap, NUM_OF_NEAREST_NEIGHBOURS);
    let mut heap_vec = heap.into_iter().collect::<Vec<Neighbour>>();
    heap_vec.sort();
    println!("Target point: {:?}\n", given_point);
    println!("List of 10 nearest neighbours using kd-tree: ");
    for neighbour in &heap_vec {
        println!("value: {:?}, point: {:?}", neighbour.distance, neighbour.point);
    }
    println!();

    let mut neighbours = points.into_iter().map(|p| Neighbour { distance: given_point.distance_to(&p), point: p }).collect::<Vec<Neighbour>>();
    neighbours.sort();
    println!("List of 10 nearest neighbours using sort: ");
    for i in 0..10 {
        println!("value: {:?}, point: {:?}", neighbours[i].distance, neighbours[i].point);
    }

    assert_eq!(heap_vec, &neighbours[0..NUM_OF_NEAREST_NEIGHBOURS]);
}