use super::*;

#[test]
fn num_nodes() {
    let mut g = Graph::new(3);
    g.insert_edge(0, 1, 10.0);
    g.insert_edge(0, 2, 10.0);

    assert_eq!(g.adj.len(), 3);
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
#[should_panic(expected = "Node 17")]
fn access_invalid_node_in_adjacent_edges() {
    let mut g = Graph::new(3);
    g.insert_edge(0, 0, 10.0);
    g.insert_edge(0, 1, 42.0);
    g.finalize();
    g.adjacent_edges(17);
}

#[test]
fn adjacent_edges() {
    let mut g = Graph::new(4);
    g.insert_edge(0, 0, 0.0);
    g.insert_edge(0, 1, 1.0);
    g.insert_edge(0, 2, 1.5);
    g.insert_edge(2, 3, 4.2);
    g.finalize();

    assert_eq!(
        g.adjacent_edges(0).unwrap(),
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
    g.finalize();

    assert_eq!(g.adjacent_nodes(0), HashSet::from([1, 2, 3]));
}

#[test]
#[should_panic(expected = "Node 17")]
fn access_invalid_node_in_adjacent_nodes() {
    let mut g = Graph::new(3);
    g.insert_edge(0, 0, 0.0);
    g.insert_edge(0, 1, 0.0);
    g.finalize();

    g.adjacent_nodes(17);
}

#[test]
fn adjacent_nodes_does_not_contain_own_node() {
    let mut g = Graph::new(3);
    g.insert_edge(0, 0, 0.0);
    g.insert_edge(0, 1, 1.0);
    g.insert_edge(0, 2, 1.5);
    g.finalize();

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
    g.finalize();

    let mut num_edges_visited = 0;
    let mut visited_source_nodes: Vec<Node> = vec![];

    for (source, target, weight) in g.edges() {
        assert_eq!(weight, g.adjacent_edges(source).unwrap()[&target]);

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
    g.insert_edge(0, 1, 1.5);
    g.increase_edge_weight(0, 1, 1.2);
    g.finalize();

    assert_eq!(g.adjacent_edges(0).unwrap()[&0], 0.0);
    assert_eq!(g.adjacent_edges(0).unwrap()[&1], 2.7);
}

#[test]
#[should_panic(expected = "Node 2")]
fn increase_edge_weight_non_existent_source() {
    let mut g = Graph::new(4);
    g.insert_edge(0, 0, 0.0);
    g.insert_edge(0, 1, 1.5);

    g.increase_edge_weight(2, 1, 1.0);
}

#[test]
#[should_panic(expected = "Node 3")]
fn increase_edge_weight_non_existent_target() {
    let mut g = Graph::new(4);
    g.insert_edge(0, 0, 0.0);
    g.insert_edge(0, 1, 1.5);

    g.increase_edge_weight(1, 3, 1.0);
}

#[test]
#[should_panic(expected = "does not exist")]
fn increase_edge_weight_non_existent_edge() {
    let mut g = Graph::new(4);
    g.insert_edge(0, 0, 0.0);
    g.insert_edge(0, 1, 1.5);
    g.insert_edge(0, 2, 1.5);

    g.increase_edge_weight(1, 2, 1.0);
}

////////////////////// Finalization and contiguous labeling ////////////////////

#[test]
#[should_panic(expected = "at least two nodes")]
fn should_contain_at_least_two_nodes() {
    // TODO: check this already in the CLI (!)
    let mut g = Graph::new(1);
    g.insert_edge(0, 0, 0.0);

    assert_eq!(g.num_nodes(), 1);
    g.finalize();
}

#[test]
fn insert_nodes_greater_than_capacity() {
    let mut g = Graph::new(3);
    g.insert_edge(42, 0, 1.0);
    g.insert_edge(0, 43, 1.0);
    // should not panic
}

#[test]
#[should_panic(expected = "contiguous")]
fn nodes_not_contiguous() {
    let mut g = Graph::new(3);
    g.insert_edge(0, 1, 2.0);
    g.insert_edge(42, 0, 1.0);
    g.insert_edge(0, 43, 1.0);

    g.finalize();
}

#[test]
#[should_panic(expected = "missing node 2")]
fn nodes_not_contiguous_missing_node_in_error_message() {
    nodes_not_contiguous();
}

////////////////////////// Read-only attribute /////////////////////////////////

#[test]
fn read_only_false_initially() {
    let mut g = Graph::new(3);
    g.insert_edge(0, 1, 10.0);
    g.insert_edge(0, 2, 10.0);

    assert_eq!(g.is_read_only, false);
}

#[test]
#[should_panic(expected = "read-only")]
fn insert_edge_read_only() {
    let mut g = Graph::new(5);
    g.insert_edge(0, 1, 1.0);
    g.insert_edge(1, 2, 1.0);
    g.finalize();
    g.insert_edge(1, 3, 2.0);
}

#[test]
#[should_panic(expected = "read-only")]
fn increase_edge_weight_read_only() {
    let mut g = Graph::new(5);
    g.insert_edge(0, 0, 10.0);
    g.insert_edge(0, 1, 1.0);
    g.finalize();
    g.increase_edge_weight(0, 1, 1.0);
}

#[test]
#[should_panic(expected = "read-only")]
fn get_edges_not_read_only_yet() {
    let mut g = Graph::new(5);
    g.insert_edge(0, 0, 1.0);
    g.insert_edge(0, 1, 1.0);
    g.edges().count();
}

#[test]
#[should_panic(expected = "read-only")]
fn get_adjacent_edges_not_read_only_yet() {
    let mut g = Graph::new(5);
    g.insert_edge(0, 0, 1.0);
    g.insert_edge(0, 1, 1.0);
    g.adjacent_edges(0);
}

#[test]
#[should_panic(expected = "read-only")]
fn get_adjacent_nodes_not_read_only_yet() {
    let mut g = Graph::new(5);
    g.insert_edge(0, 0, 1.0);
    g.insert_edge(0, 1, 1.0);
    g.adjacent_nodes(0);
}
