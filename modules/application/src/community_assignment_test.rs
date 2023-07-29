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

#[test]
fn remove_node_from_its_community_singletons() {
    let mut g = samples::weighted_graph_1();
    g.calc_degrees();

    let mut assignment = CommunityAssignment::new(&g);

    assignment.remove_node_from_its_community(0);
    assert_eq!(assignment.weights_in, vec![0.0, 0.0, 0.0, 1.0]);
    assert_eq!(assignment.weights_tot, vec![0.0, 13.0, 7.5, 10.5]);
    assert_eq!(assignment.node_to_community, vec![0, 1, 2, 3]);

    assignment.remove_node_from_its_community(1);
    assert_eq!(assignment.weights_in, vec![0.0, 0.0, 0.0, 1.0]);
    assert_eq!(assignment.weights_tot, vec![0.0, 0.0, 7.5, 10.5]);
    assert_eq!(assignment.node_to_community, vec![0, 0, 2, 3]);

    assignment.remove_node_from_its_community(2);
    assert_eq!(assignment.weights_in, vec![0.0, 0.0, 0.0, 1.0]);
    assert_eq!(assignment.weights_tot, vec![0.0, 0.0, 0.0, 10.5]);
    assert_eq!(assignment.node_to_community, vec![0, 0, 0, 3]);

    assignment.remove_node_from_its_community(3);
    assert_eq!(assignment.weights_in, vec![0.0, 0.0, 0.0, 0.0]);
    assert_eq!(assignment.weights_tot, vec![0.0, 0.0, 0.0, 0.0]);
    assert_eq!(assignment.node_to_community, vec![0, 0, 0, 0]);
}

fn get_unique(vec: &Vec<usize>) -> HashSet<usize> {
    vec.clone().into_iter().collect::<HashSet<_>>()
}

#[test]
fn renumber_communities_same_outcome() {
    let mut g = LouvainGraph::new(2);
    g.insert_edge(0, 0, 2.0);
    g.insert_edge(0, 1, 1.0);

    let mut assignment = CommunityAssignment::new(&g);

    let num_communities = assignment.renumber_communities();
    assert_eq!(num_communities, 2);

    let unique_communities = get_unique(&assignment.node_to_community);
    assert_eq!(unique_communities, HashSet::from([0, 1]));

    // nodes should still be in distinct communities
    assert_ne!(
        assignment.node_to_community[0],
        assignment.node_to_community[1]
    );
}

#[test]
fn renumber_communities() {
    let mut g = LouvainGraph::new(5);
    g.insert_edge(0, 0, 1.0);
    g.insert_edge(0, 1, 1.0);
    g.insert_edge(0, 2, 1.0);
    g.insert_edge(1, 3, 1.0);
    g.insert_edge(2, 3, 1.0);
    g.insert_edge(4, 0, 1.0);
    g.insert_edge(4, 1, 1.0);

    let mut assignment = CommunityAssignment::new(&g);
    assignment.node_to_community = vec![0, 2, 2, 2, 4];

    let num_communities = assignment.renumber_communities();
    assert_eq!(num_communities, 3);

    let unique_communities = get_unique(&assignment.node_to_community);
    assert_eq!(unique_communities, HashSet::from([0, 1, 2]));

    // nodes should still be together in communities
    assert_eq!(
        assignment.node_to_community[1],
        assignment.node_to_community[2]
    );
    assert_eq!(
        assignment.node_to_community[1],
        assignment.node_to_community[3]
    );
    assert_ne!(
        assignment.node_to_community[0],
        assignment.node_to_community[1]
    );
    assert_ne!(
        assignment.node_to_community[0],
        assignment.node_to_community[4]
    );
}
