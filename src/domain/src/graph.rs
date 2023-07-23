#[cfg(test)]
#[path = "./graph_test.rs"]
mod graph_test;

use std::collections::{HashMap, HashSet};

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
    capacity: usize,
}

impl Graph {
    pub fn new(capacity: usize) -> Self {
        Graph {
            adj: HashMap::with_capacity(capacity),
            capacity,
        }
    }

    pub fn num_nodes(&self) -> usize {
        self.adj.len()
    }

    pub fn insert_edge(&mut self, source: Node, target: Node, weight: EdgeWeight) {
        let is_self_loop = self.is_self_loop(source, target);

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

    /// Returns an iterator over all edges of the graph.
    ///
    /// Makes sure that each edge is only visited once. Only edges
    /// with source <= target are visited. The order in which edges
    /// are visited is not specified.
    pub fn edges(&self) -> impl Iterator<Item = (Node, Node, EdgeWeight)> + '_ {
        self.adj.iter().flat_map(|(source, neighbors)| {
            neighbors
                .iter()
                // avoid visiting the same edge twice
                .filter(move |(target, _)| source <= target)
                .map(move |(target, weight)| (*source, *target, *weight))
        })
    }

    /// Increases the weight of an edge.
    ///
    /// If the edge does not exist, nothing happens.
    pub fn increase_edge_weight(&mut self, source: Node, target: Node, weight_delta: EdgeWeight) {
        self.adj.entry(source).and_modify(|neighbors| {
            neighbors.entry(target).and_modify(|w| {
                *w += weight_delta;
            });
        });
    }

    /// Returns the adjacent edges of a node.
    ///
    /// This might include self-loops.
    pub fn adjacent_edges(&self, node: Node) -> &HashMap<Node, EdgeWeight> {
        let res = self.adj.get(&node);
        match res {
            Some(neighbors) => neighbors,
            None => panic!("Node {} does not exist in graph", node),
        }
    }

    /// Returns the adjacent nodes of a node.
    ///
    /// This does not include the node itself if it has a self-loop.
    pub fn adjacent_nodes(&self, node: Node) -> HashSet<Node> {
        let neighbors = self.adjacent_edges(node);
        neighbors
            .keys()
            .filter(|&other| !self.is_self_loop(node, *other))
            .copied() // no worries, we use usize for "Node"
            .collect()
    }

    fn is_self_loop(&self, source: Node, target: Node) -> bool {
        source == target
    }

    fn check_node_exists(&self, node: &Node) {
        if *node >= self.capacity {
            panic!("Node {} does not exist in graph", node);
        }
    }

    fn check_edge_does_not_exist(&self, source: Node, target: Node) {
        if !self.adj.contains_key(&source) {
            return;
        }
        if self.adj[&source].contains_key(&target) {
            panic!("Edge ({} <-> {}) already exists in graph", source, target);
        }
        // No need to check the other way around as edges are undirected
    }
}
