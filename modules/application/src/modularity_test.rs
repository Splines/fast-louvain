use std::vec;

use super::*;

use crate::louvain_graph::louvain_graph_samples as samples;

#[test]
fn modularity_for_sample_graphs_singletons() {
    let mut graphs = vec![
        samples::unweighted_graph(),
        samples::weighted_graph_1(),
        samples::weighted_graph_2(),
        samples::original_louvain_paper_graph(),
        samples::original_louvain_paper_graph_level_1(),
        samples::original_louvain_paper_graph_level_2(),
    ];
    let expected_modularities = vec![-0.06, -0.172653, 0.181818, -0.071429, 0.346301, 0.392219];

    for (i, g) in graphs.iter_mut().enumerate() {
        g.finalize();
        let m = Modularity::new(&g);

        let modularity_rounded = (m.calc_singleton_modularity() * 1e6).round() / 1e6;
        assert_eq!(modularity_rounded, expected_modularities[i]);
    }
}

#[test]
fn modularity_gain() {
    let mut g = samples::house_and_triangle_graph();
    g.finalize();
    let mut m = Modularity::new(&g);

    // Setup specific community assignment as it could be encountered in a real run
    // see https://splines.github.io/fast-louvain/louvain/algorithm.html#i-modularity-optimization
    m.assignment.remove_node_from_its_community(1);
    m.assignment.insert_node_into_community(1, 0);
    m.assignment.remove_node_from_its_community(6);
    m.assignment.insert_node_into_community(6, 5);
    m.assignment.remove_node_from_its_community(7);
    m.assignment.insert_node_into_community(7, 5);
    m.assignment.remove_node_from_its_community(8);
    m.assignment.insert_node_into_community(8, 5);
    assert_eq!(
        m.assignment.node_to_community,
        vec![0, 0, 2, 3, 4, 5, 5, 5, 5]
    );

    // Now calculate gain when moving node 4 into neighboring communities
    m.assignment.remove_node_from_its_community(4);

    let target_communities = vec![0, 3, 5];
    let expected_gains = vec![0.097, 0.056, -0.069];

    for (i, &target_community) in target_communities.iter().enumerate() {
        let mut gain = m.gain(4, target_community);
        // This factor does not appear in the gain calculation as a factor
        // does not change the relative ordering of gains.
        // However, here we compare the absolute values of the gains.
        gain = gain / (0.5 * g.twice_total_weighted_degree);
        let gain_rounded = (gain * 1e3).round() / 1e3;
        assert_eq!(gain_rounded, expected_gains[i]);
    }
}
