#[cfg(test)]
#[path = "./louvain_test.rs"]
mod louvain_test;

use crate::{
    community_assignment::NodeToCommunity, louvain_graph::LouvainGraph, louvain_level::LouvainLevel,
};

pub type CommunityAssignmentHierarchy = Vec<NodeToCommunity>;

pub struct Louvain<'a> {
    graph: &'a mut LouvainGraph,
}

impl<'a> Louvain<'a> {
    pub fn new(graph: &'a mut LouvainGraph) -> Self {
        graph.finalize();
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
            let (is_improvement, next_graph) = if is_first_run {
                is_first_run = false;
                self.run_level(self.graph, &mut hierarchy, &mut modularities)
            } else {
                self.run_level(
                    &working_graph.expect("Working graph must have been set by now"),
                    &mut hierarchy,
                    &mut modularities,
                )
            };

            if !is_improvement {
                break;
            }

            working_graph = next_graph;
        }

        (hierarchy, modularities)
    }

    /// Runs one level of the Louvain algorithm on the input graph.
    ///
    /// Returns
    /// - whether there was an improvement in modularity in the current level
    /// - the next graph
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
        modularities: &mut Vec<f64>,
    ) -> (bool, Option<LouvainGraph>) {
        let mut level_runner = LouvainLevel::new(graph, 0.0);
        let modularity = level_runner.modularity.calc_singleton_modularity();

        // Store modularity of previous hierarchy.
        // For the first run, we get the modularity of the original graph.
        // In the end, the modularities vector has length 1 more than the
        // hierarchy vector.
        modularities.push(modularity);

        // Phase I: Modularity Optimization
        let is_improvement = level_runner.optimize_one_level();

        if !is_improvement {
            return (false, None);
        }

        // Phase II: Community Aggregation
        let mut next_graph = level_runner.get_next_level_graph();
        next_graph.finalize();

        // We do not store the hierarchy for the original graph
        // because "level 0" is trivial (every node is in its own community).
        let node_to_community = &level_runner.modularity.assignment.node_to_community;
        hierarchy.push(node_to_community.clone());

        (is_improvement, Some(next_graph))
    }
}
