use std::vec;

use super::*;

use crate::louvain_graph::louvain_graph_samples::*;

#[test]
fn louvain_first_test() {
    // Just a first dummy test
    let mut graph = weighted_graph_1();
    let louvain = Louvain::new(&mut graph);
    let hierarchy = louvain.run();
}
