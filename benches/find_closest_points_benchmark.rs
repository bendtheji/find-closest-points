use std::collections::BinaryHeap;

use criterion::{BatchSize, Bencher, black_box, Criterion, criterion_group, criterion_main};

use find_closest_points::kd_tree::KdTreeNode;
use find_closest_points::nearest_neighbour::{Distance, find_k_nearest_neighbours};
use find_closest_points::point::{Dimension, Point};

fn bench_find_closest_neighbours_kd_tree(b: &mut Bencher) {
    let points = black_box(generate_10m_random_points());
    let tree = black_box(Some(Box::new(KdTreeNode::construct_tree(points))));
    let given_point = black_box(Point::random());
    b.iter_batched(|| BinaryHeap::<Distance>::new(),
                   |mut heap| find_k_nearest_neighbours(&tree,
                                                        &given_point,
                                                        &Dimension::X, &mut heap),
                   BatchSize::SmallInput);
}

fn generate_10m_random_points() -> Vec<Point> {
    let mut points = vec![];
    for _ in 0..10_000_000 {
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