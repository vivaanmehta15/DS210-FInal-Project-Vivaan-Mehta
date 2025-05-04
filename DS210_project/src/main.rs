// main.rs
// Main controller: loads graph, runs preprocessing, computes shortest paths and centralities, and prints results
mod graph;
mod bfs;
mod centrality;
mod utils;

use graph::Graph;
use bfs::average_shortest_path;
use centrality::{degree_centrality, closeness_centrality, betweenness_centrality};

/// Finds all nodes with no edges
fn find_isolated_nodes(graph: &Graph) -> Vec<usize> {
    graph.nodes()
        .into_iter()
        .filter(|&node| graph.neighbors(node).map_or(true, |n| n.is_empty()))
        .collect()
}
/// Returns the node with the highest degree (most connections)
fn find_most_connected(graph: &Graph) -> Option<(usize, usize)> {
    graph.nodes().into_iter()
        .map(|node| (node, graph.neighbors(node).map_or(0, |n| n.len())))
        .max_by_key(|&(_, degree)| degree)
}
/// Computes the average degree across all nodes in the graph
fn average_degree(graph: &Graph) -> f64 {
    let total_deg: usize = graph.nodes()
        .into_iter()
        .map(|n| graph.neighbors(n).map_or(0, |nb| nb.len()))
        .sum();
    total_deg as f64 / graph.node_count() as f64
}


fn main() {
    let graph = Graph::from_file("facebook_combined.txt");

    println!("Loaded graph with {} nodes and {} edges.", graph.node_count(), graph.edge_count());

    // Data cleaning / basic stats
    let isolated = find_isolated_nodes(&graph);
    println!("Number of isolated nodes: {}", isolated.len());

    if let Some((node, degree)) = find_most_connected(&graph) {
        println!("Most connected node: {} with degree {}", node, degree);
    }

    println!("Average degree of graph: {:.2}", average_degree(&graph));

    // Six degrees of separation
    let avg_path = average_shortest_path(&graph, 1000);
    println!("Average shortest path over 1000 samples: {:.3}", avg_path);

    // Centrality analysis
    let deg = degree_centrality(&graph);
    let close = closeness_centrality(&graph);
    let between = betweenness_centrality(&graph);

    println!("\nTop 5 Degree Centrality:");
    utils::print_top(&deg, 5);
    println!("\nTop 5 Closeness Centrality:");
    utils::print_top(&close, 5);
    println!("\nTop 5 Betweenness Centrality:");
    utils::print_top(&between, 5);
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use crate::graph::Graph;
    use crate::bfs::bfs_distance;

    #[test]
    fn test_graph_load_and_count() {
        let graph = Graph::from_file("facebook_combined.txt");
        assert_eq!(graph.node_count(), 4039);
        assert_eq!(graph.edge_count(), 88234);
    }

    #[test]
    fn test_bfs_distance_small_graph() {
        let mut adj = HashMap::new();
        adj.insert(0, HashSet::from([1, 2]));
        adj.insert(1, HashSet::from([0, 3]));
        adj.insert(2, HashSet::from([0]));
        adj.insert(3, HashSet::from([1]));

        let graph = Graph { adj };
        let dist = bfs_distance(&graph, 0, 3);
        assert_eq!(dist, Some(2));
    }
}
