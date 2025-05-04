// graph.rs
// Defines the Graph struct using an adjacency list and provides basic graph operations.

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Represents an undirected graph using an adjacency list.

pub struct Graph {
    pub adj: HashMap<usize, HashSet<usize>>,
}

impl Graph {
     /// Loads a graph from a text file where each line contains an edge: "u v".
    pub fn from_file(path: &str) -> Self {
        let file = BufReader::new(File::open(path).unwrap());
        let mut adj = HashMap::new();
        for line in file.lines() {
            let line = line.unwrap();
            let parts: Vec<usize> = line.split_whitespace()
                .map(|x| x.parse().unwrap()).collect();
            // Add edge in both directions since this is an undirected graph
            adj.entry(parts[0]).or_insert_with(HashSet::new).insert(parts[1]);
            adj.entry(parts[1]).or_insert_with(HashSet::new).insert(parts[0]);
        }
        Graph { adj }
    }
    /// Returns the set of neighbors for a given node.
    pub fn neighbors(&self, node: usize) -> Option<&HashSet<usize>> {
        self.adj.get(&node)
    }
    /// Returns a list of all nodes in the graph.
    pub fn nodes(&self) -> Vec<usize> {
        self.adj.keys().cloned().collect()
    }
    /// Returns the total number of nodes.
    pub fn node_count(&self) -> usize {
        self.adj.len()
    }
    /// Returns the total number of edges in the undirected graph.
    pub fn edge_count(&self) -> usize {
        self.adj.values().map(|s| s.len()).sum::<usize>() / 2
    }
}
