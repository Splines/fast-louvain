#[path = "./louvain_graph_samples.rs"]
pub mod louvain_graph_samples;

#[cfg(test)]
#[path = "./louvain_graph_test.rs"]
mod louvain_graph_test;

use std::collections::{HashMap, HashSet};

use louvain_domain::graph::{EdgeWeight, Graph, Node};

pub type NodeWeightedDegree = f64;

/// An undirected graph of vertices and edges with weights.
///
/// In addition to the `Graph` struct, this struct also stores the weighted
/// degrees of the nodes and allows to precalculate them for the Louvain
/// algorithm.
///
/// Isolated nodes are not allowed in this graph, i.e. the number of nodes
/// must be equal to the capacity of the graph. If this is not the case,
/// it will panic.
///
/// For more information on the underyling graph, see the `Graph` struct
/// in the `domain` module.
#[derive(Debug)]
pub struct LouvainGraph {
    graph: Graph,
    pub weighted_degrees: Vec<NodeWeightedDegree>,
    pub self_loop_weighted_degrees: Vec<NodeWeightedDegree>,
    pub twice_total_weighted_degree: NodeWeightedDegree,
}

impl LouvainGraph {
    pub fn new(initial_capacity: usize) -> Self {
        LouvainGraph {
            graph: Graph::new(initial_capacity),
            weighted_degrees: vec![0.0; initial_capacity],
            self_loop_weighted_degrees: vec![0.0; initial_capacity],
            twice_total_weighted_degree: 0.0,
        }
    }

    pub fn num_nodes(&self) -> usize {
        self.graph.num_nodes()
    }

    pub fn insert_edge(&mut self, source: Node, target: Node, weight: EdgeWeight) {
        self.graph.insert_edge(source, target, weight);
    }

    pub fn edges(&self) -> impl Iterator<Item = (usize, usize, f64)> + '_ {
        self.graph.edges()
    }

    pub fn increase_edge_weight(&mut self, source: Node, target: Node, weight: EdgeWeight) {
        self.graph.increase_edge_weight(source, target, weight);
    }

    pub fn adjacent_edges(&self, node: Node) -> Option<&HashMap<Node, EdgeWeight>> {
        self.graph.adjacent_edges(node)
    }

    pub fn adjacent_nodes(&self, node: Node) -> HashSet<Node> {
        self.graph.adjacent_nodes(node)
    }

    /// "Finalizes the graph", i.e. checks that there are no isolated nodes
    /// (makes sure they are contiguously labeled from 0 to n-1)
    /// and calculates the weighted degrees of the nodes for later use
    /// in the Louvain algorithm.
    pub fn finalize(&mut self) {
        self.graph.finalize();
        self.calc_degrees();
    }

    /// Calculates the weighted degree of every node.
    /// Note that this method is not idempotent as the variables are not reset.
    fn calc_degrees(&mut self) {
        self.graph
            .adj
            .iter()
            .for_each(|(node, neighbor_edges_weights)| {
                // Note this also includes weights of self-loops
                let incr_weight = neighbor_edges_weights.values().sum::<EdgeWeight>();
                self.twice_total_weighted_degree += incr_weight; // weighted degree counted twice here -> each order of the argument
                self.weighted_degrees[*node] += incr_weight;

                // Also consider self-loops separately
                if neighbor_edges_weights.contains_key(node) {
                    self.self_loop_weighted_degrees[*node] += neighbor_edges_weights[node];
                }
            });
    }
}
