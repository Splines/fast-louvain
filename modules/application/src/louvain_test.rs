use super::*;

use crate::louvain_graph::louvain_graph_samples as samples;

#[test]
fn louvain_first_test() {
    // Just a first dummy test
    let mut graph = samples::house_and_triangle_graph();

    let louvain = Louvain::new(&mut graph);
    let (hierarchy, modularities) = louvain.run();

    println!("Hierarchy: {:?}", hierarchy);
    println!("Modularities: {:?}", modularities);
}
