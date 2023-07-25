use louvain_domain::graph::Node;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::{community_assignment::Community, louvain_graph::LouvainGraph, modularity::Modularity};

pub struct LouvainLevel<'a> {
    graph: &'a LouvainGraph,
    pub modularity: Modularity<'a>,

    /// Threshold for modularity improvement in one pass. If the improvement
    /// is smaller than this threshold, we stop the current iteration.
    /// Set to 0.0 to ignore this threshold.
    modularity_improvement_threshold: f64,
    should_use_threshold: bool,
}

impl<'a> LouvainLevel<'a> {
    pub fn new(graph: &'a LouvainGraph, epsilon_min: f64) -> Self {
        let should_use_threshold = epsilon_min > 0.0;
        Self {
            graph,
            modularity: Modularity::new(graph),
            modularity_improvement_threshold: epsilon_min,
            should_use_threshold,
        }
    }

    /// Phase I of Louvain: Modularity Optimization
    /// Optimizes modularity of the current graph, i.e. we calculate one level
    /// of the Louvain hierarchy.
    ///
    /// We repeatedly cycle through all nodes until there is no improvement
    /// of modularity anymore and we've reached a local maximum.
    /// For every node, we remove it from its community and move it to the
    /// community of the neighbor for which the modularity gain (delta Q)
    /// is maximal.
    ///
    /// Returns whether there was an improvement in modularity in the current
    /// level.
    pub fn optimize_one_level(&mut self) -> bool {
        let mut is_level_improvement = false;

        // Prepare random access to nodes
        let mut indices_shuffled: Vec<Node> = (0..self.graph.num_nodes()).collect();
        indices_shuffled.shuffle(&mut thread_rng());

        let mut quality = if self.should_use_threshold {
            self.modularity.calc_singleton_modularity()
        } else {
            0.0 // dummy value
        };

        let mut is_improvement_in_one_graph_traversal;
        let mut count_graph_traversals = 0;
        loop {
            is_improvement_in_one_graph_traversal = false;
            let old_quality = quality;
            for &node in &indices_shuffled {
                let improvement = self.optimize_one_node(node);
                if improvement {
                    is_improvement_in_one_graph_traversal = true;
                    is_level_improvement = true;
                }
            }
            count_graph_traversals += 1;

            // TODO: What is a good value here for "max_graph_traversals"?
            // 50 is just chosen arbitrarily in the expectation that the
            // actual number of graph traversals is much lower.
            if count_graph_traversals > 50 {
                panic!(
                    "The Louvain algorithm is somehow stuck in a local optimum
                and cannot improve modularity anymore. Please report your specific
                graph on GitHub, so we can investigate this issue."
                );
            }

            if self.should_use_threshold {
                quality = self.modularity.calc_singleton_modularity();
                if (quality - old_quality) <= self.modularity_improvement_threshold {
                    break;
                }
            }
            if !is_improvement_in_one_graph_traversal {
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
            let gain = self.modularity.gain(node, target_community);
            if gain > best_gain {
                best_community = target_community;
                best_gain = gain;
            }
        }

        best_community
    }

    /// Phase II of Louvain: Community Aggregation
    /// Aggregates the current graph into a new graph, where each node
    /// represents a community from the previous graph.
    ///
    /// In the newly created, weighted graph:
    /// - nodes are communities from Phase I (intuition: vertices from the
    ///  previous graph get merged into "super-vertices")
    /// - edge weights are the sum of weights of edges between communities
    /// - an edge within a community becomes two self-loops
    pub fn get_next_level_graph(&mut self) -> LouvainGraph {
        let num_communities = self.modularity.assignment.renumber_communities();
        let mut next_graph = LouvainGraph::new(num_communities);

        for (node, adj_node, weight) in self.graph.edges() {
            let source_community = self.modularity.assignment.get_community(node);
            let other_community = self.modularity.assignment.get_community(adj_node);

            // Edges inside communities become self-loops in the next graph.
            // The weights of these edges should be counted twice to make sure that
            // m = 1/2 \sum_i,j A_ij is constant across the multiple graphs.
            // We therefore need to double the diagonal matrix elements of the next graph
            // corresponding to the weights of self-loops.
            // However, self-loops that were self-loops in the current graph already
            // do not fall under this reasoning, as they are not counted twice
            // for each order of arguments in the sum.
            let mut new_weight = weight;
            if (source_community == other_community) && (node != adj_node) {
                new_weight *= 2.0;
            }

            let adj_edges = next_graph.adjacent_edges(source_community);
            if adj_edges.is_some() && adj_edges.unwrap().contains_key(&other_community) {
                // Edge already in the next graph -> only adjust weight
                // (There was already one node in this community connected
                // to a node in the other community.)
                next_graph.increase_edge_weight(source_community, other_community, new_weight);
            } else {
                // Edge not yet in the next graph -> Insert it
                // Communities become the new vertices
                next_graph.insert_edge(source_community, other_community, new_weight);
            }
        }

        next_graph
    }
}
