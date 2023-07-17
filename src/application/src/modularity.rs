#[cfg(test)]
#[path = "./modularity_test.rs"]
mod modularity_test;

use louvain_domain::graph::Node;

use crate::{
    community_assignment::{Community, CommunityAssignment},
    louvain_graph::LouvainGraph,
};

/// Measure to evaluate how good a proposed partition of the network into
/// communities is. Modularity is in the range [-0.5, 1].
///
/// Modularity Q is maximized for divisions of a graph when many edges fall
/// within the proposed communities (intra-community edges)
/// as compared to edges between communities (inter-community edges).
///
/// Note that for the Louvain algorithm, we only need to calculate the absolute
/// modularity once for every pass when every node is in its own singleton
/// community. Then, only relative deltas of modularity are calculated and
/// added to the global value.
/// Therefore, this implementation of modularity does not allow to
/// calculate modularity for arbitrary vertex-community assignments.
pub struct Modularity<'a> {
    pub assignment: CommunityAssignment<'a>,

    graph: &'a LouvainGraph,
}

impl<'a> Modularity<'a> {
    pub fn new(graph: &'a LouvainGraph) -> Self {
        Self {
            assignment: CommunityAssignment::new(graph),
            graph,
        }
    }

    /// We assume that the node has already been removed from its community
    /// beforehand by calling the respective remove function from the
    /// CommunityAssignment.
    pub fn gain(&self, node: Node, target_community: Community) -> f64 {
        // Let m equal twice_total_weighted_degree. Then:
        // We drop a factor of 1/m here as we only need a relative measure
        // to compare, not absolute values. The correct equation would be:
        // \Delta Q = \frac{1}{m} \cdot \Bigl( k_{i,in} - \frac{\Sigma_{tot}\cdot k_i}{2m} \Bigr)
        let tot_community = self.assignment.weights_tot[target_community];
        let vertex_weighted_degree_of_edges_to_community = self
            .assignment
            .weighted_degrees_to_communities
            .get(&target_community)
            .unwrap_or_else(|| {
                panic!(
                    "Fatal: Cannot access weighted degree \
                of community {} for node {}. Are you sure you have removed \
                the node from its community before calling this function?",
                    target_community, node
                )
            });
        let weighted_node_degree = self.graph.weighted_degrees[node];

        vertex_weighted_degree_of_edges_to_community
            - (tot_community * weighted_node_degree) / self.graph.twice_total_weighted_degree
    }

    pub fn calc_modularity(&self) -> f64 {
        let mut quality: f64 = 0.0;
        let twice_total_weighted_degree = self.graph.twice_total_weighted_degree;

        // TODO: Get rid of unique_communities retrieval
        let unique_communities = self
            .assignment
            .node_to_community
            .iter()
            .collect::<std::collections::HashSet<_>>();

        for community in unique_communities {
            quality += self.assignment.weights_in[*community]
                - self.assignment.weights_tot[*community].powi(2) / twice_total_weighted_degree;
        }

        quality /= twice_total_weighted_degree;
        quality
    }
}
