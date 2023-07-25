#[cfg(test)]
#[path = "./louvain_test.rs"]
mod louvain_test;

use crate::{
    community_assignment::NodeToCommunity, louvain_graph::LouvainGraph, louvain_level::LouvainLevel,
};

type CommunityAssignmentHierarchy = Vec<NodeToCommunity>;

pub struct Louvain<'a> {
    graph: &'a mut LouvainGraph,
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

        let mut is_first_run = true;
        let mut working_graph: Option<LouvainGraph> = Option::None;
        loop {
            let (is_improvement, mut next_graph) = if is_first_run {
                is_first_run = false;
                self.run_level(self.graph, &hierarchy)
            } else {
                self.run_level(
                    &working_graph.expect("Next graph must have been set by now"),
                    &hierarchy,
                )
            };
            next_graph.calc_degrees();
            working_graph = Some(next_graph);

            if !is_improvement {
                break;
            }
        }

        // println!("-------");
        // let final_runner = LouvainLevel::new(&working_graph, 0.0);
        // let final_modularity = final_runner.modularity.calc_modularity();
        // println!("Final modularity: {}", final_modularity);

        // TODO: fill hierarchy
        // Pass other information to the outside, e.g. node-to-community mapping
        hierarchy
    }

    pub fn run_level(
        &self,
        graph: &LouvainGraph,
        hierarchy: &CommunityAssignmentHierarchy,
    ) -> (bool, LouvainGraph) {
        println!();
        println!("Run next level");

        let mut level_runner = LouvainLevel::new(graph, 0.0);

        let modularity = level_runner.modularity.calc_modularity();
        println!("Modularity: {}", modularity);

        let is_improvement = level_runner.optimize_one_level();
        let mut next_graph = level_runner.get_next_level_graph();

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
