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
fn insert_edge() {
    let mut g = Graph::new(3);
    g.insert_edge(0, 1, 1.0);
    g.insert_edge(0, 2, 3.0);
    g.insert_edge(1, 2, 2.0);

    assert_eq!(g.adj[&0].len(), 2);
    assert_eq!(g.adj[&1].len(), 2);
    assert_eq!(g.adj[&2].len(), 2);

    assert_eq!(g.adj[&0], HashMap::from([(1, 1.0), (2, 3.0)]));
    assert_eq!(g.adj[&1], HashMap::from([(0, 1.0), (2, 2.0)]));
    assert_eq!(g.adj[&2], HashMap::from([(0, 3.0), (1, 2.0)]));
}

#[test]
#[should_panic(expected = "already exists")]
fn insert_edge_twice() {
    let mut g = Graph::new(3);
    g.insert_edge(0, 1, 1.0);
    g.insert_edge(0, 1, 3.0);
}

#[test]
#[should_panic(expected = "already exists")]
fn insert_edge_twice_undirected() {
    let mut g = Graph::new(3);
    g.insert_edge(0, 1, 1.0);
    g.insert_edge(1, 0, 3.0);
}

#[test]
#[should_panic(expected = "already exists")]
fn insert_self_loop_twice() {
    let mut g = Graph::new(1);
    g.insert_edge(0, 0, 1.0);
    g.insert_edge(0, 0, 3.0);
}

#[test]
fn self_loops() {
    let mut g = Graph::new(1);
    g.insert_edge(0, 0, 1.0);

    assert_eq!(g.num_nodes(), 1);
    assert_eq!(g.adj[&0].len(), 1);
    assert_eq!(g.adj[&0], HashMap::from([(0, 1.0)]));
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

#[test]
fn adjacent_edges() {
    let mut g = Graph::new(4);
    g.insert_edge(0, 0, 0.0);
    g.insert_edge(0, 1, 1.0);
    g.insert_edge(0, 2, 1.5);
    g.insert_edge(2, 3, 4.2);

    assert_eq!(
        g.adjacent_edges(0),
        &HashMap::from([(0, 0.0), (1, 1.0), (2, 1.5)])
    );
}

#[test]
fn adjacent_nodes() {
    let mut g = Graph::new(4);
    g.insert_edge(0, 0, 0.0);
    g.insert_edge(0, 1, 1.0);
    g.insert_edge(0, 2, 1.5);
    g.insert_edge(2, 3, 4.2);
    g.insert_edge(3, 0, 7.2);

    assert_eq!(g.adjacent_nodes(0), HashSet::from([1, 2, 3]));
}

#[test]
fn adjacent_nodes_does_not_contain_own_node() {
    let mut g = Graph::new(3);
    g.insert_edge(0, 0, 0.0);
    g.insert_edge(0, 1, 1.0);
    g.insert_edge(0, 2, 1.5);

    // no node 0 in the result
    assert_eq!(g.adjacent_nodes(0), HashSet::from([1, 2]));
}

#[test]
fn iterate_over_edges() {
    let mut g = Graph::new(4);
    g.insert_edge(0, 0, 0.0);
    g.insert_edge(0, 1, 1.0);
    g.insert_edge(2, 0, 1.5);
    g.insert_edge(2, 3, 42.0);
    g.insert_edge(2, 2, 1.34);

    let mut num_edges_visited = 0;
    let mut visited_source_nodes: Vec<Node> = vec![];

    for (source, target, weight) in g.edges() {
        assert_eq!(weight, g.adjacent_edges(source)[&target]);

        num_edges_visited += 1;
        visited_source_nodes.push(source);
    }

    // only edges with source <= target should be visited
    // no guarantee on the order of edges though
    visited_source_nodes.sort();
    assert_eq!(visited_source_nodes, vec![0, 0, 0, 2, 2]);
    assert_eq!(num_edges_visited, 5);
}

#[test]
fn increase_edge_weight() {
    let mut g = Graph::new(4);
    g.insert_edge(0, 0, 0.0);
    g.insert_edge(0, 2, 1.5);

    g.increase_edge_weight(0, 2, 1.2);
    g.increase_edge_weight(0, 3, 42.0);

    assert_eq!(g.adjacent_edges(0)[&0], 0.0);
    assert_eq!(g.adjacent_edges(0)[&2], 2.7);
}
