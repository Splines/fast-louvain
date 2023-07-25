//! While modularity be calculated for arbitrary communities,
//! the programm actually only needs to calculate the modularity for
//! the singleton communities in the beginning of every run.
//! Then, only relative deltas of modularity are calculated and
//! added to the global value.
//! Therefore, we only provide test graphs with singleton communities here.

use super::*;

pub fn unweighted_graph() -> LouvainGraph {
    let mut g = LouvainGraph::new(4);
    g.insert_edge(0, 0, 1.0);
    g.insert_edge(0, 1, 1.0);
    g.insert_edge(1, 2, 1.0);
    g.insert_edge(2, 3, 1.0);
    g.insert_edge(3, 1, 1.0);
    g.insert_edge(3, 3, 1.0);
    g
}

pub fn weighted_graph_1() -> LouvainGraph {
    let mut g = LouvainGraph::new(4);
    g.insert_edge(0, 0, 3.0);
    g.insert_edge(0, 1, 1.0);
    g.insert_edge(1, 2, 5.0);
    g.insert_edge(2, 3, 2.5);
    g.insert_edge(3, 1, 7.0);
    g.insert_edge(3, 3, 1.0);
    g
}

pub fn weighted_graph_2() -> LouvainGraph {
    let mut g = LouvainGraph::new(10);
    g.insert_edge(0, 0, 42.0);
    g.insert_edge(0, 1, 5.0);
    g.insert_edge(0, 2, 1.0);
    g.insert_edge(0, 3, 1.0);
    g.insert_edge(1, 2, 3.0);
    g.insert_edge(1, 3, 2.0);
    g.insert_edge(2, 2, 7.0);
    g.insert_edge(2, 3, 2.0);
    g
}

pub fn house_and_triangle_graph() -> LouvainGraph {
    let mut g = LouvainGraph::new(9);
    g.insert_edge(0, 1, 1.0);
    g.insert_edge(0, 4, 1.0);
    g.insert_edge(4, 1, 1.0);
    g.insert_edge(1, 2, 1.0);
    g.insert_edge(2, 3, 1.0);
    g.insert_edge(3, 4, 1.0);
    g.insert_edge(4, 5, 1.0);
    g.insert_edge(5, 6, 1.0);
    g.insert_edge(5, 8, 1.0);
    g.insert_edge(6, 7, 1.0);
    g.insert_edge(6, 8, 1.0);
    g.insert_edge(7, 8, 1.0);
    g
}
