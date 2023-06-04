#[cfg(test)]
#[path = "./graph_test.rs"]
mod graph_test;

use std::collections::HashMap;

pub type Node = usize;
pub type EdgeWeight = f32;
pub type NodeWeightedDegree = f32;

/// "Adj" stands for "adjacent".
/// Edges are stored in a HashMap where the key is the node id and
/// the value is a vector of tuples of the form (neighbor, weight).
pub type Adj = HashMap<Node, Vec<(Node, EdgeWeight)>>;

/// An undirected graph of vertices and edges with weights.
///
/// Mathematically, a graph is a tuple G = (V, E) of vertices V and edges E.
/// We use an adjacency list representation of our graph, i.e.,
/// a map from nodes to their respective neighbors (including weights)
/// as this information is retrieved frequently in the Louvain algorithm.
///
/// Nodes are contiguously labels from 0 to n-1.
#[derive(Debug)]
pub struct Graph {
    pub adj: Adj,
    capacity: usize,
    weighted_degrees: HashMap<Node, NodeWeightedDegree>,
    total_weighted_degree: NodeWeightedDegree,
}

impl Graph {
    pub fn new(capacity: usize) -> Self {
        Graph {
            adj: HashMap::with_capacity(capacity),
            capacity: capacity,
            weighted_degrees: HashMap::with_capacity(capacity),
            total_weighted_degree: 0.0,
        }
    }

    pub fn num_nodes(&self) -> usize {
        self.adj.len()
    }

    pub fn insert_edge(&mut self, source: Node, target: Node, weight: EdgeWeight) {
        // Check for self loops
        let is_self_loop = source == target;

        // Check if nodes exist
        if source >= self.capacity {
            panic!("Node {} does not exist in graph", source);
        }
        if !is_self_loop && target >= self.capacity {
            panic!("Node {} does not exist in graph", target);
        }

        // Graph is undirected, so we add the edge in both directions
        let neighbors = self.adj.entry(source).or_insert(Vec::new());
        neighbors.push((target, weight));
        if (!is_self_loop) {
            let neighbors = self.adj.entry(target).or_insert(Vec::new());
            neighbors.push((source, weight));
        }
    }

    pub fn adjacent_edges(&self, node: Node) -> &Vec<(Node, EdgeWeight)> {
        let res = self.adj.get(&node);
        match res {
            Some(neighbors) => neighbors,
            None => panic!("Node {} does not exist in graph", node),
        }
    }

    fn calc_degrees(&self) {
        // for (node, neighbors) in &self.adj {
        //     for (neighbor, weight) in neighbors {
        //         self.weightedDegrees[node] += weight;
        //     }
        // }
    }
}
