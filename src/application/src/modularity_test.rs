use std::vec;

use super::*;

fn louvain_test_graph() -> LouvainGraph {
    let mut g = LouvainGraph::new(4);
    g.insert_edge(0, 0, 3.0);
    g.insert_edge(0, 1, 1.0);
    g.insert_edge(1, 2, 5.0);
    g.insert_edge(2, 3, 2.5);
    g.insert_edge(3, 1, 7.0);
    g.insert_edge(3, 3, 1.0);
    g.calc_degrees();

    g
}

#[test]
fn modularity_init() {
    let g = louvain_test_graph();
    let m = Modularity::new(&g);

    assert_eq!(m.vertex_to_community, vec![0, 1, 2, 3]);
    assert_eq!(m.weights_in, vec![3.0, 0.0, 0.0, 1.0]);
    assert_eq!(m.weights_tot, vec![4.0, 13.0, 7.5, 10.5]);
}

#[test]
fn modularity() {
    let g = louvain_test_graph();
    let m = Modularity::new(&g);

    let m_rounded = (m.modularity() * 1e6).round() / 1e6;
    assert_eq!(m_rounded, -0.172653);
}

// TODO: add more tests and calculate modularity by hand for those.
