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

## Time and Space Complexity

#### Constructing kd-tree
Constructing the kd-tree involves finding the mean value over a dimension from the list of points, and then finding a pivot point which is closest to the mean value in that dimension.

The overall time complexity of finding the mean value and the pivot point  is O(N), where N is the number of points.

As we go down into building the subtrees, because we use the mean to partition the points accordingly into the left and right subtrees, the number of points is halved in each partition. Therefore the overall time complexity for constructing the kd-tree is O(NlogN).

#### Finding k nearest neighbours
Finding k nearest neighbours involves going down the kd-tree to the bounded area (leaf node) where the target point is contained then adding the distance between the leaf node and the target point to a max heap which we will use to store k nearest neighbours so far.

Then as we go up the kd-tree to the root node, we want to check whether the distance between the target point and the sibling node's bounded area is of a smaller distance than the distance at the top of our max heap. If it is, then we consider traversing through the sibling subtree to look for potential candidates. If not, we skip the subtree.

Traversing through the potential subtrees that contain our potential nearest neighbours will take an overall time complexity of O(logN) time complexity. The time complexity taken to add each candidate to our max heap is O(logk), where k is 10 in our scenario. 

Overall, time complexity for finding k nearest neighbours in our kd-tree is O(logN).


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