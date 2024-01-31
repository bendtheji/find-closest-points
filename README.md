# find_closest_points
A rust library for finding the 10 nearest neighbours to a target point in a 3D space amongst a huge number of 3D points using a kd-tree.

## Description
Generate a huge number of random 3D points and build a kd-tree with those points, then use the kd-tree to find the nearest neighbours to the target point.

## Example: bin/find_closest_points.rs

To see an example of the `find_closest_points` program, run the command below:
```sh
cargo run
```
This program generates 100000 3D points, then constructs a kd-tree from these points. Using the kd-tree, we generate a random target point to find the 10 nearest neighbours to the target point.

Putting the same 100000 points into another `Vec!`, computing the distance between the points and sorting by the distance, we can compare to see that the 10 nearest neighbours are the same.

## Benchmarks

Ran benchmark for finding the 10 nearest neighbours to a target point amongst 10,000,000 3D points and the results are as shown below:

| Lower Bound | Estimate | Upper Bound |
|-------------|----------|-------------|
| 945.49ns    | 998.54ns | 1.0519µs    |

To run the benchmarks, simply run the following command:
```sh
cargo bench
```

To get more detailed metrics:
```sh
cargo bench --bench find_closest_points_benchmark -- --verbose
```