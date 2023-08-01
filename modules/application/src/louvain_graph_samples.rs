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
    let mut g = LouvainGraph::new(4);
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

pub fn original_louvain_paper_graph() -> LouvainGraph {
    let mut g = LouvainGraph::new(16);
    g.insert_edge(0, 2, 1.0);
    g.insert_edge(0, 3, 1.0);
    g.insert_edge(0, 4, 1.0);
    g.insert_edge(0, 5, 1.0);
    g.insert_edge(1, 2, 1.0);
    g.insert_edge(1, 4, 1.0);
    g.insert_edge(1, 7, 1.0);
    g.insert_edge(2, 4, 1.0);
    g.insert_edge(2, 5, 1.0);
    g.insert_edge(2, 6, 1.0);
    g.insert_edge(3, 7, 1.0);
    g.insert_edge(4, 10, 1.0);
    g.insert_edge(5, 7, 1.0);
    g.insert_edge(5, 11, 1.0);
    g.insert_edge(6, 7, 1.0);
    g.insert_edge(6, 11, 1.0);
    g.insert_edge(8, 9, 1.0);
    g.insert_edge(8, 10, 1.0);
    g.insert_edge(8, 11, 1.0);
    g.insert_edge(8, 14, 1.0);
    g.insert_edge(8, 15, 1.0);
    g.insert_edge(9, 12, 1.0);
    g.insert_edge(9, 14, 1.0);
    g.insert_edge(10, 11, 1.0);
    g.insert_edge(10, 12, 1.0);
    g.insert_edge(10, 13, 1.0);
    g.insert_edge(10, 14, 1.0);
    g.insert_edge(11, 13, 1.0);
    g
}

pub fn original_louvain_paper_graph_level_1() -> LouvainGraph {
    let mut g = LouvainGraph::new(4);
    g.insert_edge(0, 0, 14.0);
    g.insert_edge(0, 1, 1.0);
    g.insert_edge(0, 2, 1.0);
    g.insert_edge(0, 3, 4.0);
    g.insert_edge(1, 1, 16.0);
    g.insert_edge(1, 2, 3.0);
    g.insert_edge(2, 2, 2.0);
    g.insert_edge(2, 3, 1.0);
    g.insert_edge(3, 3, 4.0);
    g
}

pub fn original_louvain_paper_graph_level_2() -> LouvainGraph {
    let mut g = LouvainGraph::new(2);
    g.insert_edge(0, 0, 26.0);
    g.insert_edge(0, 1, 3.0);
    g.insert_edge(1, 1, 24.0);
    g
}
