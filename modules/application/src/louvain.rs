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
    /// Returns
    /// - the hierarchy of community assignments of the nodes
    /// - the modularity of each level
    pub fn run(&self) -> (CommunityAssignmentHierarchy, Vec<f64>) {
        let mut hierarchy = CommunityAssignmentHierarchy::new();
        let mut modularities: Vec<f64> = Vec::new();

        let mut is_first_run = true;
        let mut working_graph: Option<LouvainGraph> = Option::None;

        loop {
            let (is_improvement, mut next_graph, modularity) = if is_first_run {
                is_first_run = false;
                self.run_level(self.graph, &mut hierarchy)
            } else {
                self.run_level(
                    &working_graph.expect("Working graph must have been set by now"),
                    &mut hierarchy,
                )
            };

            next_graph.calc_degrees();
            working_graph = Some(next_graph);
            modularities.push(modularity);

            if !is_improvement {
                break;
            }
        }

        (hierarchy, modularities)
    }

    /// Runs one level of the Louvain algorithm on the input graph.
    ///
    /// Returns
    /// - whether there was an improvement in modularity in the current level
    /// - the next graph
    /// - the modularity of the current level.
    ///
    /// Typically, there are only a few (e.g. 5) passes, i.e. this function
    /// is only run a few times, until there is no improvement in modularity
    /// anymore. This, of course, heavily depends on the input graph, its
    /// size and the underlying community structure.
    ///
    /// "Level" might also be called one Louvain "pass".
    pub fn run_level(
        &self,
        graph: &LouvainGraph,
        hierarchy: &mut CommunityAssignmentHierarchy,
    ) -> (bool, LouvainGraph, f64) {
        let mut level_runner = LouvainLevel::new(graph, 0.0);
        let modularity = level_runner.modularity.calc_modularity();

        // Phase I: Modularity Optimization
        let is_improvement = level_runner.optimize_one_level();

        // Phase II: Community Aggregation
        let next_graph = level_runner.get_next_level_graph();

        let node_to_community = &level_runner.modularity.assignment.node_to_community;
        hierarchy.push(node_to_community.clone());

        (is_improvement, next_graph, modularity)
    }
}
