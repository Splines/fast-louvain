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
        let hierarchy = CommunityAssignmentHierarchy::new();

        // first run
        let (is_improvement, next_graph) = self.run_level(&self.graph);
        if !is_improvement {
            return hierarchy;
        }

        let mut working_graph: LouvainGraph = next_graph;
        loop {
            let (is_improvement, next_graph) = self.run_level(&working_graph);
            working_graph = next_graph;
            if !is_improvement {
                break;
            }
        }

        println!("-------");
        let final_runner = LouvainLevel::new(&working_graph, 0.0);
        let final_modularity = final_runner.modularity.calc_modularity();
        println!("Final modularity: {}", final_modularity);

        // TODO: fill hierarchy
        // Pass other information to the outside, e.g. node-to-community mapping
        hierarchy
    }

    pub fn run_level(&self, next_graph: &LouvainGraph) -> (bool, LouvainGraph) {
        println!();
        println!("Run next level");

        let mut level_runner = LouvainLevel::new(next_graph, 0.0);

        let modularity = level_runner.modularity.calc_modularity();
        println!("Modularity: {}", modularity);

        let is_improvement = level_runner.optimize_one_level();
        let mut next_graph = level_runner.get_next_level_graph();
        next_graph.calc_degrees();

        println!("Node to community assignment:");
        let node_to_community = &level_runner.modularity.assignment.node_to_community;
        for (node, community) in node_to_community.iter().enumerate() {
            println!("{}->{}", node, community);
        }

        println!("New graph edges:");
        for (source, target, weight) in next_graph.edges() {
            println!("{}-{} ({})", source, target, weight);
        }

        (is_improvement, next_graph)
    }
}
