#[cfg(test)]
#[path = "./graph_test.rs"]
mod graph_test;

use std::collections::HashMap;

pub type Node = usize;
pub type EdgeWeight = f64;

/// "Adj" stands for "adjacent".
/// Edges are stored in a HashMap where the key is the node id and
/// the value is a vector of tuples of the form (neighbor, weight).
pub type Adj = HashMap<Node, HashMap<Node, EdgeWeight>>;

/// An undirected graph of vertices and edges with weights.
///
/// Mathematically, a graph is a tuple G = (V, E) of vertices V and edges E.
/// We use an adjacency list representation of our graph, i.e.,
/// a map from nodes to their respective neighbors (including weights)
/// as this information is retrieved frequently in the Louvain algorithm.
///
/// Nodes are contiguously labeled from 0 to n-1.
/// The Graph expects that all of its initial capacity is used and then
/// the graph is not altered anymore at all.
#[derive(Debug)]
pub struct Graph {
    pub adj: Adj,
    pub capacity: usize,
}

impl Graph {
    pub fn new(capacity: usize) -> Self {
        Graph {
            adj: HashMap::with_capacity(capacity),
            capacity: capacity,
        }
    }

    pub fn num_nodes(&self) -> usize {
        self.adj.len()
    }

    pub fn insert_edge(&mut self, source: Node, target: Node, weight: EdgeWeight) {
        let is_self_loop = self.is_self_loop(&source, &target);

        // Checks
        self.check_node_exists(&source);
        if !is_self_loop {
            self.check_node_exists(&target);
        }

        self.check_edge_does_not_exist(source, target);

        // Graph is undirected, so we add the edge in both directions
        let neighbors = self.adj.entry(source).or_insert(HashMap::new());
        neighbors.insert(target, weight);
        if !is_self_loop {
            let neighbors = self.adj.entry(target).or_insert(HashMap::new());
            neighbors.insert(source, weight);
        }
    }

    pub fn adjacent_edges(&self, node: Node) -> &HashMap<Node, EdgeWeight> {
        let res = self.adj.get(&node);
        match res {
            Some(neighbors) => neighbors,
            None => panic!("Node {} does not exist in graph", node),
        }
    }

    fn is_self_loop(&self, source: &Node, target: &Node) -> bool {
        source == target
    }

    fn check_node_exists(&self, node: &Node) {
        if *node >= self.capacity {
            panic!("Node {} does not exist in graph", node);
        }
    }

    fn check_edge_does_not_exist(&self, source: Node, target: Node) {
        if self.adj.contains_key(&source) {
            if self.adj[&source].contains_key(&target) {
                panic!("Edge ({} <-> {}) already exists in graph", source, target);
            }
        }
        // No need to check the other way around as edges are undirected
    }
}
