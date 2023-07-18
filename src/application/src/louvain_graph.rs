#[path = "./louvain_graph_samples.rs"]
pub mod louvain_graph_samples;

#[cfg(test)]
#[path = "./louvain_graph_test.rs"]
mod louvain_graph_test;

use std::collections::{HashMap, HashSet};

use louvain_domain::graph::{EdgeWeight, Graph, Node};

pub type NodeWeightedDegree = f64;

#[derive(Debug)]
pub struct LouvainGraph {
    graph: Graph,
    pub weighted_degrees: Vec<NodeWeightedDegree>,
    pub self_loop_weighted_degrees: Vec<NodeWeightedDegree>,
    pub twice_total_weighted_degree: NodeWeightedDegree,
}

impl LouvainGraph {
    pub fn new(capacity: usize) -> Self {
        LouvainGraph {
            graph: Graph::new(capacity),
            weighted_degrees: vec![0.0; capacity],
            self_loop_weighted_degrees: vec![0.0; capacity],
            twice_total_weighted_degree: 0.0,
        }
    }

    pub fn num_nodes(&self) -> usize {
        self.graph.num_nodes()
    }

    pub fn adjacent_edges(&self, node: Node) -> &HashMap<Node, EdgeWeight> {
        self.graph.adjacent_edges(node)
    }

    pub fn adjacent_nodes(&self, node: Node) -> HashSet<Node> {
        self.graph.adjacent_nodes(node)
    }

    pub fn insert_edge(&mut self, source: Node, target: Node, weight: EdgeWeight) {
        self.graph.insert_edge(source, target, weight);
    }

    /// Calculates the weighted degree of every node.
    /// Note that this method is not idempotent as the variables are not reset.
    pub fn calc_degrees(&mut self) {
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
