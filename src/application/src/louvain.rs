#[cfg(test)]
#[path = "./louvain_test.rs"]
mod louvain_test;

use crate::{
    community_assignment::NodeToCommunity, louvain_graph::LouvainGraph, louvain_level::LouvainLevel,
};

type CommunityAssignmentHierarchy = Vec<NodeToCommunity>;

pub struct Louvain<'a> {
    graph: &'a LouvainGraph,
}

impl<'a> Louvain<'a> {
    pub fn new(graph: &'a mut LouvainGraph) -> Self {
        graph.calc_degrees();
        Self { graph }
    }

    /// Runs the Louvain algorithm on the input graph.
    ///
    /// Returns the community assignment of the nodes.
    pub fn run(&self) -> CommunityAssignmentHierarchy {
        let hierarchy: CommunityAssignmentHierarchy = Vec::with_capacity(self.graph.num_nodes());
        let mut level_runner = LouvainLevel::new(self.graph, 0.0);

        let modularity = level_runner.modularity.calc_modularity();
        println!("Modularity: {}", modularity);

        level_runner.optimize_one_level();

        let next_graph = level_runner.get_next_level_graph();
        for (source, target, weight) in next_graph.edges() {
            println!("{}-{} ({})", source, target, weight);
        }

        let node_to_community = &level_runner.modularity.assignment.node_to_community;
        // Print node to community
        for (node, community) in node_to_community.iter().enumerate() {
            println!("{}->{}", node, community);
        }

        let modularity = level_runner.modularity.calc_modularity();
        println!("Modularity: {}", modularity);

        // TODO: fill hierarchy
        // Pass other information to the outside, e.g. node-to-community mapping
        hierarchy
    }
}
