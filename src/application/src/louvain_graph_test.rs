use super::*;

#[test]
fn graph_init() {
    let g = LouvainGraph::new(3);

    assert_eq!(g.graph.num_nodes(), 0);
    assert_eq!(g.graph.adj.len(), 0);

    assert_eq!(g.weighted_degrees.len(), 3);
    assert_eq!(g.weighted_degrees[0], 0.0);
    assert_eq!(g.weighted_degrees[1], 0.0);
    assert_eq!(g.weighted_degrees[2], 0.0);

    assert_eq!(g.self_loop_weighted_degrees.len(), 3);
    assert_eq!(g.self_loop_weighted_degrees[0], 0.0);
    assert_eq!(g.self_loop_weighted_degrees[1], 0.0);
    assert_eq!(g.self_loop_weighted_degrees[2], 0.0);

    assert_eq!(g.twice_total_weighted_degree, 0.0);
}

#[test]
fn calc_degrees() {
    let mut g = LouvainGraph::new(4);
    g.insert_edge(0, 0, 3.0);
    g.insert_edge(0, 1, 1.0);
    g.insert_edge(1, 2, 5.0);
    g.insert_edge(2, 3, 2.5);
    g.insert_edge(3, 1, 7.0);
    g.insert_edge(3, 3, 1.0);
    g.calc_degrees();

    // Normal edges
    assert_eq!(g.weighted_degrees[0], 4.0);
    assert_eq!(g.weighted_degrees[1], 13.0);
    assert_eq!(g.weighted_degrees[2], 7.5);
    assert_eq!(g.weighted_degrees[3], 10.5);

    // Self loops
    assert_eq!(g.self_loop_weighted_degrees[0], 3.0);
    assert_eq!(g.self_loop_weighted_degrees[1], 0.0);
    assert_eq!(g.self_loop_weighted_degrees[2], 0.0);
    assert_eq!(g.self_loop_weighted_degrees[3], 1.0);

    // Total degree
    assert_eq!(g.twice_total_weighted_degree, 35.0);
}
