use louvain_domain::graph::Node;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::{louvain_graph::LouvainGraph, modularity::Modularity};

pub struct Louvain<'a> {
    graph: &'a LouvainGraph,
    modularity: Modularity<'a>,
}

impl<'a> Louvain<'a> {
    fn new(graph: &'a LouvainGraph) -> Self {
        Self {
            graph,
            modularity: Modularity::new(graph),
        }
    }

    /// TODO: split this function into smaller ones
    fn calc_one_level(&mut self) -> bool {
        let mut is_improvement_in_pass = false;
        let mut is_improvement_in_one_iteration = false; // one iteration in the current pass

        // Prepare random access to nodes
        let mut indices_shuffled: Vec<Node> = (0..self.graph.num_nodes()).collect();
        indices_shuffled.shuffle(&mut thread_rng());

        let mut new_quality = self.modularity.calc_modularity();
        loop {
            let current_quality = new_quality;

            // For each node (in random order)
            // Remove node from its current community and insert it into
            // the community of one of its neighbors, so that modularity
            // increase is maximal.
            for &node in &indices_shuffled {
                let community = self.modularity.assignment.get_community(node);

                // Remove node from old community
                // -> weight of edges from current node to other nodes
                // that are in the same community (not other nodes)
                self.modularity
                    .assignment
                    .remove_node_from_its_community(node);

                // Compute the best community to move vertex into
                // (Default choice for future insertion is the previous community)
                let mut best_community = community;
                let mut best_gain = 0.0;

                for adjacent_node in self.graph.adjacent_nodes(node) {
                    let target_community = self.modularity.assignment.get_community(adjacent_node);
                    let gain = self.modularity.gain(target_community, node);
                    if gain > best_gain {
                        best_community = community;
                        best_gain = gain;
                    }
                }

                // insert the node in the best community
                self.modularity
                    .assignment
                    .insert_node_into_community(node, best_community);

                // Improvement: if we moved the vertex to ANOTHER community
                if best_community != community {
                    is_improvement_in_one_iteration = true;
                    is_improvement_in_pass = true;
                }
            }

            // Recalculate quality
            new_quality = self.modularity.calc_modularity();

            // Do while
            // -> there is an improvement of quality
            // -> or there is an improvement of quality greater than a given epsilon
            if is_improvement_in_one_iteration || (new_quality - current_quality) <= 0.001 {
                break;
            }
        }
        is_improvement_in_pass
    }
}
