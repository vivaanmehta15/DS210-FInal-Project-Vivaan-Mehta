use std::collections::{HashMap, VecDeque};
use crate::graph::Graph;

pub fn degree_centrality(graph: &Graph) -> HashMap<usize, usize> {
    graph.adj.iter().map(|(&k, v)| (k, v.len())).collect()
}

pub fn closeness_centrality(graph: &Graph) -> HashMap<usize, f64> {
    let mut result = HashMap::new();
    for &node in &graph.nodes() {
        let mut queue = VecDeque::new();
        let mut dist = HashMap::new();
        queue.push_back(node);
        dist.insert(node, 0);
        while let Some(current) = queue.pop_front() {
            let d = dist[&current];
            if let Some(neighbors) = graph.neighbors(current) {
                for &n in neighbors {
                    if !dist.contains_key(&n) {
                        dist.insert(n, d + 1);
                        queue.push_back(n);
                    }
                }
            }
        }
        let sum_dist: usize = dist.values().sum();
        if sum_dist > 0 {
            result.insert(node, (dist.len() - 1) as f64 / sum_dist as f64);
        }
    }
    result
}

pub fn betweenness_centrality(graph: &Graph) -> HashMap<usize, f64> {
    let mut betweenness = HashMap::new();
    for &s in &graph.nodes() {
        let mut stack = Vec::new();
        let mut pred: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut sigma = HashMap::new();
        let mut dist = HashMap::new();

        sigma.insert(s, 1);
        dist.insert(s, 0);
        let mut queue = VecDeque::new();
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            let d = dist[&v];
            for &w in graph.neighbors(v).unwrap_or(&Default::default()) {
                if !dist.contains_key(&w) {
                    queue.push_back(w);
                    dist.insert(w, d + 1);
                }
                if dist[&w] == d + 1 {
                    sigma.entry(w).or_insert(0);
                    sigma.insert(w, sigma[&w] + sigma[&v]);
                    pred.entry(w).or_insert_with(Vec::new).push(v);
                }
            }
        }

        let mut delta = HashMap::new();
        for &v in &stack {
            delta.insert(v, 0.0);
        }

        while let Some(w) = stack.pop() {
            if let Some(p_vec) = pred.get(&w) {
                for &v in p_vec {
                    let c = (sigma[&v] as f64 / sigma[&w] as f64) * (1.0 + delta[&w]);
                    *delta.get_mut(&v).unwrap() += c;
                }
            }
            if w != s {
                *betweenness.entry(w).or_insert(0.0) += delta[&w];
            }
        }
    }

    for val in betweenness.values_mut() {
        *val /= 2.0;
    }

    betweenness
}
