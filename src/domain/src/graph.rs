use std::collections::HashMap;

pub type Node = usize;

#[derive(Debug)]
pub struct Graph {
    edges: HashMap<Node, Vec<Node>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: Node, to: Node) {
        self.edges.entry(from).or_insert_with(Vec::new).push(to);
    }

    pub fn neighbors(&self, node_id: Node) -> Option<&Vec<Node>> {
        return self.edges.get(&node_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbors() {
        let mut g = Graph::new();

        g.add_edge(1, 2);
        g.add_edge(1, 3);
        g.add_edge(2, 3);
        g.add_edge(3, 4);

        assert_eq!(g.neighbors(1), Some(&vec![2, 3]));
        assert_eq!(g.neighbors(2), Some(&vec![3]));
        assert_eq!(g.neighbors(3), Some(&vec![4]));
        assert_eq!(g.neighbors(4), None);
    }
}
