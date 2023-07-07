use std::vec;

use super::*;

use crate::louvain_graph::louvain_graph_samples::*;

#[test]
fn modularity_init() {
    let mut g = weighted_graph_1();
    g.calc_degrees();

    let m = Modularity::new(&g);

    assert_eq!(m.vertex_to_community, vec![0, 1, 2, 3]);
    assert_eq!(m.weights_in, vec![3.0, 0.0, 0.0, 1.0]);
    assert_eq!(m.weights_tot, vec![4.0, 13.0, 7.5, 10.5]);
}

#[test]
fn modularity_for_sample_graphs_singletons() {
    let mut graphs = vec![unweighted_graph(), weighted_graph_1(), weighted_graph_2()];
    let expected_modularities = vec![-0.06, -0.172653, 0.181818];

    for (i, g) in graphs.iter_mut().enumerate() {
        g.calc_degrees();
        let m = Modularity::new(&g);

        let m_rounded = (m.modularity() * 1e6).round() / 1e6;
        assert_eq!(m_rounded, expected_modularities[i]);
    }
}
