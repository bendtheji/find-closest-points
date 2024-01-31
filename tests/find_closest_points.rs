use std::collections::BinaryHeap;
use find_closest_points::{generate_random_points, NUM_OF_NEAREST_NEIGHBOURS};
use find_closest_points::kd_tree::KdTreeNode;
use find_closest_points::nearest_neighbour::{find_k_nearest_neighbours, Neighbour};
use find_closest_points::point::{Dimension, Point};

fn get_nearest_neighbours_sorted_using_kd_tree(points: &Vec<Point>, given_point: &Point) -> Vec<Neighbour>{
    let tree = KdTreeNode::construct_tree(points.clone());
    let mut heap = BinaryHeap::new();
    find_k_nearest_neighbours(&Some(Box::new(tree)), &given_point, &Dimension::X, &mut heap, NUM_OF_NEAREST_NEIGHBOURS);
    let mut heap_vec = heap.into_iter().collect::<Vec<Neighbour>>();
    heap_vec.sort();
    heap_vec
}

#[test]
fn find_10_closest_points_within_10_points() {
    let points = generate_random_points(10);
    let given_point = Point::random();
    let output = get_nearest_neighbours_sorted_using_kd_tree(&points, &given_point);

    let mut expected = points.into_iter().map(|p| Neighbour { distance: given_point.distance_to(&p), point: p }).collect::<Vec<Neighbour>>();
    expected.sort();

    assert_eq!(output, expected);
}

#[test]
fn find_10_closest_points_within_20_points() {
    let points = generate_random_points(20);
    let given_point = Point::random();
    let output = get_nearest_neighbours_sorted_using_kd_tree(&points, &given_point);

    let mut expected = points.into_iter().map(|p| Neighbour { distance: given_point.distance_to(&p), point: p }).collect::<Vec<Neighbour>>();
    expected.sort();

    assert_eq!(output, &expected[0..10]);
}

#[test]
fn find_10_closest_points_within_50_points() {
    let points = generate_random_points(50);
    let given_point = Point::random();
    let output = get_nearest_neighbours_sorted_using_kd_tree(&points, &given_point);

    let mut expected = points.into_iter().map(|p| Neighbour { distance: given_point.distance_to(&p), point: p }).collect::<Vec<Neighbour>>();
    expected.sort();

    assert_eq!(output, &expected[0..10]);
}

#[test]
fn find_10_closest_points_within_1000_points() {
    let points = generate_random_points(1000);
    let given_point = Point::random();
    let output = get_nearest_neighbours_sorted_using_kd_tree(&points, &given_point);

    let mut expected = points.into_iter().map(|p| Neighbour { distance: given_point.distance_to(&p), point: p }).collect::<Vec<Neighbour>>();
    expected.sort();

    assert_eq!(output, &expected[0..10]);
}

#[test]
fn find_10_closest_points_within_10000_points() {
    let points = generate_random_points(10000);
    let given_point = Point::random();
    let output = get_nearest_neighbours_sorted_using_kd_tree(&points, &given_point);

    let mut expected = points.into_iter().map(|p| Neighbour { distance: given_point.distance_to(&p), point: p }).collect::<Vec<Neighbour>>();
    expected.sort();

    assert_eq!(output, &expected[0..10]);
}

#[test]
fn find_10_closest_points_within_1000000_points() {
    let points = generate_random_points(1_000_000);
    let given_point = Point::random();
    let output = get_nearest_neighbours_sorted_using_kd_tree(&points, &given_point);

    let mut expected = points.into_iter().map(|p| Neighbour { distance: given_point.distance_to(&p), point: p }).collect::<Vec<Neighbour>>();
    expected.sort();

    assert_eq!(output, &expected[0..10]);
}

#[test]
fn find_10_closest_points_within_10_000_000_points() {
    let points = generate_random_points(10_000_000);
    let given_point = Point::random();
    let output = get_nearest_neighbours_sorted_using_kd_tree(&points, &given_point);

    let mut expected = points.into_iter().map(|p| Neighbour { distance: given_point.distance_to(&p), point: p }).collect::<Vec<Neighbour>>();
    expected.sort();

    assert_eq!(output, &expected[0..10]);
}