// bfs.rs
// Implements Breadth-First Search for computing shortest paths and estimating average distances.

use rand::seq::SliceRandom;
use std::collections::{HashMap, VecDeque};
use crate::graph::Graph;

/// Estimates the average shortest path by sampling random node pairs and computing distances.

pub fn average_shortest_path(graph: &Graph, samples: usize) -> f64 {
    let mut rng = rand::thread_rng();
    let nodes = graph.nodes();
    let mut total: usize = 0;
    let mut count: usize = 0;

    for _ in 0..samples {
        let pair = nodes
            .choose_multiple(&mut rng, 2)
            .cloned()
            .collect::<Vec<_>>();
        if let Some(dist) = bfs_distance(graph, pair[0], pair[1]) {
            total += dist;
            count += 1;
        }
    }

    if count == 0 {
        return 0.0;
    }

    total as f64 / count as f64
}

/// Computes the shortest path between two nodes using BFS.

pub fn bfs_distance(graph: &Graph, start: usize, goal: usize) -> Option<usize> {
    let mut visited: HashMap<usize, usize> = HashMap::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    visited.insert(start, 0);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        let dist = visited[&node];
        if node == goal {
            return Some(dist);
        }

        if let Some(neighbors) = graph.neighbors(node) {
            for &n in neighbors {
                if !visited.contains_key(&n) {
                    visited.insert(n, dist + 1);
                    queue.push_back(n);
                }
            }
        }
    }

    None
}
