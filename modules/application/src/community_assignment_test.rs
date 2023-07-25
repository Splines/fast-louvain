use super::*;

use crate::louvain_graph::louvain_graph_samples as samples;

#[test]
fn initial_assignment_in_singletons() {
    let mut g = samples::weighted_graph_1();
    g.calc_degrees();

    let assignment = CommunityAssignment::new(&g);

    assert_eq!(assignment.node_to_community, vec![0, 1, 2, 3]);
    assert_eq!(assignment.weights_in, vec![3.0, 0.0, 0.0, 1.0]);
    assert_eq!(assignment.weights_tot, vec![4.0, 13.0, 7.5, 10.5]);
}
