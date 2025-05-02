use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Graph {
    pub adj: HashMap<usize, HashSet<usize>>,
}

impl Graph {
    pub fn from_file(path: &str) -> Self {
        let file = BufReader::new(File::open(path).unwrap());
        let mut adj = HashMap::new();
        for line in file.lines() {
            let line = line.unwrap();
            let parts: Vec<usize> = line.split_whitespace()
                .map(|x| x.parse().unwrap()).collect();
            adj.entry(parts[0]).or_insert_with(HashSet::new).insert(parts[1]);
            adj.entry(parts[1]).or_insert_with(HashSet::new).insert(parts[0]);
        }
        Graph { adj }
    }

    pub fn neighbors(&self, node: usize) -> Option<&HashSet<usize>> {
        self.adj.get(&node)
    }

    pub fn nodes(&self) -> Vec<usize> {
        self.adj.keys().cloned().collect()
    }

    pub fn node_count(&self) -> usize {
        self.adj.len()
    }

    pub fn edge_count(&self) -> usize {
        self.adj.values().map(|s| s.len()).sum::<usize>() / 2
    }
}
