use louvain_domain::graph::Node;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::{community_assignment::Community, louvain_graph::LouvainGraph, modularity::Modularity};

pub struct Louvain<'a> {
    graph: &'a LouvainGraph,
    modularity: Modularity<'a>,

    /// Threshold for modularity improvement in one pass. If the improvement
    /// is smaller than this threshold, we stop the current iteration.
    /// Set to 0.0 to ignore this threshold.
    modularity_improvement_threshold: f64,
    should_use_threshold: bool,
}

impl<'a> Louvain<'a> {
    fn new(graph: &'a LouvainGraph, epsilon_min: f64) -> Self {
        let should_use_threshold = epsilon_min > 0.0;
        Self {
            graph,
            modularity: Modularity::new(graph),
            modularity_improvement_threshold: epsilon_min,
            should_use_threshold,
        }
    }

    /// Optimizes modularity of the current graph, i.e. we calculate one level
    /// of the Louvain hierarchy.
    ///
    /// Returns whether there was an improvement in modularity in the current
    /// pass.
    ///
    /// This corresponds to Phase 1 (Modularity optimization)
    /// of one pass of the Louvain algorithm.
    fn optimize_one_level(&mut self) -> bool {
        let mut is_level_improvement = false;

        // Prepare random access to nodes
        let mut indices_shuffled: Vec<Node> = (0..self.graph.num_nodes()).collect();
        indices_shuffled.shuffle(&mut thread_rng());

        let mut quality = if self.should_use_threshold {
            self.modularity.calc_modularity()
        } else {
            0.0 // dummy value
        };

        let mut is_improvement_in_one_graph_traversal = false;
        let mut count_graph_traversals = 0;
        loop {
            let old_quality = quality;
            for &node in &indices_shuffled {
                let improvement = self.optimize_one_node(node);
                if improvement {
                    is_improvement_in_one_graph_traversal = true;
                    is_level_improvement = true;
                }
            }
            count_graph_traversals += 1;

            if self.should_use_threshold {
                quality = self.modularity.calc_modularity();
                if (quality - old_quality) <= self.modularity_improvement_threshold {
                    break;
                }
            }
            if is_improvement_in_one_graph_traversal {
                break;
            }
        }

        is_level_improvement
    }

    /// Optimizes modularity for one node by removing it from its current
    /// community and inserting it into the community of one of its neighbors,
    /// so that modularity increase is maximal.
    ///
    /// Returns whether there was an improvement in modularity, i.e.
    /// whether the node was moved to another community or stayed in its
    /// previous community.
    fn optimize_one_node(&mut self, node: Node) -> bool {
        let prev_community = self.modularity.assignment.get_community(node);

        self.modularity
            .assignment
            .remove_node_from_its_community(node);

        let new_community = self.find_best_community_to_move_into(node, prev_community);

        self.modularity
            .assignment
            .insert_node_into_community(node, new_community);

        new_community != prev_community
    }

    /// Computes the best community to move the current node into.
    ///
    /// The default choice for the "best community" is the previous community
    /// of the node if no improvement can be made otherwise (by moving into
    /// a community of an adjacent node).
    ///
    /// If there are multiple communities with the same gain, we choose
    /// the first one we encounter.
    fn find_best_community_to_move_into(&self, node: Node, prev_community: Community) -> Community {
        let mut best_community = prev_community;
        let mut best_gain = 0.0;

        for adj_node in self.graph.adjacent_nodes(node) {
            let target_community = self.modularity.assignment.get_community(adj_node);
            let gain = self.modularity.gain(target_community, node);
            if gain > best_gain {
                best_community = prev_community;
                best_gain = gain;
            }
        }

        best_community
    }
}
