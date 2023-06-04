use super::*;

#[test]
fn graph_metadata() {
    let mut g = Graph::new(3);
    g.insert_edge(0, 1, 10.0);

    // Capacity
    assert_eq!(g.adj.capacity(), 3);
    assert_eq!(g.adj.capacity(), g.capacity);

    // Num nodes
    assert_eq!(g.adj.len(), 2);
    assert_eq!(g.adj.len(), g.num_nodes());
}

#[test]
fn test_insert_edge() {
    let mut g = Graph::new(3);
    g.insert_edge(0, 1, 1.0);
    g.insert_edge(0, 2, 3.0);
    g.insert_edge(1, 2, 2.0);

    assert_eq!(g.adj[&0].len(), 2);
    assert_eq!(g.adj[&1].len(), 2);
    assert_eq!(g.adj[&2].len(), 2);
    assert_eq!(g.adj[&0], vec![(1, 1.0), (2, 3.0)]);
    assert_eq!(g.adj[&1], vec![(0, 1.0), (2, 2.0)]);
    assert_eq!(g.adj[&2], vec![(0, 3.0), (1, 2.0)]);
}

#[test]
fn self_loops() {
    let mut g = Graph::new(1);
    g.insert_edge(0, 0, 1.0);

    assert_eq!(g.num_nodes(), 1);
    assert_eq!(g.adj[&0].len(), 1);
    assert_eq!(g.adj[&0], vec![(0, 1.0)]);
}

#[test]
#[should_panic(expected = "Node 0")]
fn access_invalid_node() {
    let g = Graph::new(3);
    g.adjacent_edges(0);
}

#[test]
#[should_panic(expected = "Node 3")]
fn insert_invalid_source_in_edge() {
    let mut g = Graph::new(3);
    g.insert_edge(3, 0, 1.0);
}

#[test]
#[should_panic(expected = "Node 3")]
fn insert_invalid_target_in_edge() {
    let mut g = Graph::new(3);
    g.insert_edge(0, 3, 1.0);
}
