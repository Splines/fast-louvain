use super::*;

#[test]
fn louvain_first_test() {
    // Just a first dummy test
    let mut graph = LouvainGraph::new(9);
    graph.insert_edge(0, 1, 1.0);
    graph.insert_edge(0, 4, 1.0);
    graph.insert_edge(4, 1, 1.0);
    graph.insert_edge(1, 2, 1.0);
    graph.insert_edge(2, 3, 1.0);
    graph.insert_edge(3, 4, 1.0);
    graph.insert_edge(4, 5, 1.0);
    graph.insert_edge(5, 6, 1.0);
    graph.insert_edge(5, 8, 1.0);
    graph.insert_edge(6, 7, 1.0);
    graph.insert_edge(6, 8, 1.0);
    graph.insert_edge(7, 8, 1.0);

    let louvain = Louvain::new(&mut graph);
    let (hierarchy, modularities) = louvain.run();

    println!("Hierarchy: {:?}", hierarchy);
    println!("Modularities: {:?}", modularities);
}
