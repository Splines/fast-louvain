#[cfg(test)]
#[path = "./louvain_graph_test.rs"]
mod louvain_graph_test;

use louvain_domain::graph::{EdgeWeight, Graph, Node};

pub type NodeWeightedDegree = f32;

#[derive(Debug)]
struct LouvainGraph {
    graph: Graph,
    weighted_degrees: Vec<NodeWeightedDegree>,
    self_loop_weighted_degrees: Vec<NodeWeightedDegree>,
    total_weighted_degree: NodeWeightedDegree,
}

impl LouvainGraph {
    pub fn new(num_nodes: usize) -> Self {
        LouvainGraph {
            graph: Graph::new(num_nodes),
            weighted_degrees: vec![0.0; num_nodes],
            self_loop_weighted_degrees: vec![0.0; num_nodes],
            total_weighted_degree: 0.0,
        }
    }

    pub fn insert_edge(&mut self, source: Node, target: Node, weight: EdgeWeight) {
        self.graph.insert_edge(source, target, weight);
    }

    /// Calculates the weighted degree of every node.
    fn calc_degrees(&mut self) {
        self.graph
            .adj
            .iter()
            .for_each(|(node, neighbors_with_weights)| {
                // Note this also includes weights of self-loops
                let incr_weight = neighbors_with_weights.values().sum::<EdgeWeight>();
                self.total_weighted_degree += incr_weight;
                self.weighted_degrees[*node] += incr_weight;

                // Also consider self-loops separately
                if neighbors_with_weights.contains_key(node) {
                    self.self_loop_weighted_degrees[*node] += neighbors_with_weights[node];
                }
            });
    }
}
