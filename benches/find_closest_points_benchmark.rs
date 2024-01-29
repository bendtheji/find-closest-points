use std::collections::BinaryHeap;
use std::time::Duration;

use criterion::{BatchSize, Bencher, black_box, Criterion, criterion_group, criterion_main};

use find_closest_points::kd_tree::{Distance, find_k_nearest_neighbours, KdTreeNode};
use find_closest_points::point::Point;

fn bench_find_closest_neighbours_kd_tree(b: &mut Bencher) {
    let points = black_box(generate_10m_random_points());
    let tree = black_box(Some(Box::new(KdTreeNode::construct_tree(points))));
    let given_point = black_box(Point::random());
    b.iter_batched(|| BinaryHeap::<Distance>::new(),
                   |mut heap| find_k_nearest_neighbours(&tree,
                                                        &given_point,
                                                        0, 3, &mut heap),
                   BatchSize::SmallInput);
    // b.iter_batched(|| (points.clone(), BinaryHeap::<Distance>::new()),
    //                |(points, mut nearest)| {
    //                    let mut distances = points.into_iter().map(|p| Distance { value: given_point.distance_to(&p), other_point: p }).collect::<Vec<Distance>>();
    //                    for distance in distances {
    //                         if nearest.len() < 10 {
    //                             nearest.push(distance)
    //                         } else {
    //                             match nearest.peek() {
    //                                 Some(x) if x.value > distance.value => {
    //                                     nearest.pop();
    //                                     nearest.push(distance)
    //                                 },
    //                                 _ => {}
    //                             }
    //                         }
    //                    }
    //                }, BatchSize::SmallInput);
}

fn generate_10m_random_points() -> Vec<Point> {
    let mut points = vec![];
    for i in 0..10_000_000 {
        points.push(Point::random());
    }
    points
}

fn find_closest_points_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Find closest points");
    // group.measurement_time(Duration::new(40, 0));
    group.bench_function("Find 10 nearest neighbours using kd tree", bench_find_closest_neighbours_kd_tree);
}

criterion_group!(benches, find_closest_points_benchmark);
criterion_main!(benches);