use std::collections::{HashMap, HashSet};

use louvain_domain::graph::Node;

use crate::louvain_graph::LouvainGraph;

pub type Community = usize;
pub type NodeToCommunity = Vec<Community>;

pub struct CommunityAssignment<'a> {
    pub node_to_community: NodeToCommunity,

    graph: &'a LouvainGraph,

    /// Sum of weights of edges belonging to a community c
    pub weights_in: Vec<f64>,

    /// Sum of weighted degrees of nodes in a community c (including self-loops
    /// which are only counted once, other than undirected edges). This is
    /// equivalent to: Sum of the weights of the edges incident to nodes
    /// in a communiyt c, including self-loops (including edges with both
    /// ends in community c).
    pub weights_tot: Vec<f64>,

    /// includes the own community as well
    /// only for one node at a time
    /// i.e. we only account the weights of edges from the current node
    /// not for the whole communities
    pub weighted_degrees_to_communities: HashMap<Community, f64>,
}

impl<'a> CommunityAssignment<'a> {
    pub fn new(graph: &'a LouvainGraph) -> Self {
        let mut res = Self {
            node_to_community: vec![0; graph.num_nodes()],
            graph,
            // there are graph.capacity() communities at the beginning of every run
            weights_in: vec![0.0; graph.num_nodes()],
            weights_tot: vec![0.0; graph.num_nodes()],
            weighted_degrees_to_communities: HashMap::new(),
        };

        // In the beginning, every vertex is in its own singleton community
        for v in 0..graph.num_nodes() {
            res.node_to_community[v] = v;
            res.weights_in[v] = graph.self_loop_weighted_degrees[v];
            res.weights_tot[v] = graph.weighted_degrees[v];
        }

        res
    }

    pub fn get_community(&self, node: Node) -> Community {
        self.node_to_community[node]
    }

    pub fn remove_node_from_its_community(&mut self, node: Node) {
        self.calc_weighted_degrees_for_communities(node);
        let community = self.node_to_community[node];

        // Since edges are undirected, we need to count their weights twice
        // EXCEPT FOR self-loops where i=vertex & j=vertex only appears once
        // in the sum (see formula for modularity). In other cases, we need the
        // weight of the edge for each order of arguments, thus i=vertex1 & j=vertex2
        // AND i=vertex2 & j=vertex1.
        self.weights_in[community] = *self
            .weighted_degrees_to_communities
            .entry(community)
            .or_default();
        self.weights_in[community] -= self.graph.self_loop_weighted_degrees[node];

        // Remove weighted degree of this vertex as contribution to the community
        self.weights_tot[community] -= self.graph.weighted_degrees[node];

        // Reset community assignment
        // TODO: Use other value than 0 to indicate that a node is not assigned to a community
        // (-1 not working as we use usize for community)
        // e.g. use other data structure, or give guarantees that this node
        // is never left "dangling" in the graph
        self.node_to_community[node] = 0;
    }

    /// This method assumes you have already removed the node from its old community
    /// via `removeNodeFromCommunity()`.
    pub fn insert_node_into_community(&mut self, node: Node, community: Community) {
        // see explanation in remove_node_from_its_community()
        self.weights_in[community] += 2.0
            * *self
                .weighted_degrees_to_communities
                .entry(community)
                .or_default();
        self.weights_in[community] += self.graph.self_loop_weighted_degrees[node];

        // We insert the vertex into a new community, so we need to add its
        // weighted degree to the sum of weighted degrees of the community (sigma_tot).
        self.weights_tot[community] += self.graph.weighted_degrees[node];

        // Update community assignment
        self.node_to_community[node] = community;
    }

    /// Renumbers communities to have consecutive IDs starting from 0.
    /// Returns the number of communities.
    ///
    /// # Example
    /// When the communities currently "in use" (have at least one node in it)
    /// are labeled 0,1,3, we would rename them to 0,1,2 and adjust the
    /// node-to-community assignments accordingly.
    pub fn renumber_communities(&mut self) -> usize {
        // Example
        // vertex -> community
        // 1 --> 0
        // 2 --> 1
        // 3 --> 3
        // 4 --> 3
        // 5 --> 1
        // 6 --> 1
        // 7 --> 3

        // See which of n (number nodes) possible communities are used
        // i.e. here: communities 0,1,3 are used
        let pessimistic_capacity = self.graph.num_nodes() / 2;
        let mut used_communities: HashSet<Community> = HashSet::with_capacity(pessimistic_capacity);
        for node in 0..self.graph.num_nodes() {
            used_communities.insert(self.node_to_community[node]);
        }

        // Iterate over used communities and assign consecutive ids 0,1,2,...
        // e.g. we would get: 0 -> 0, 1 -> 1, 3 -> 2 (community 3 is now called 2)
        let mut renumbered_communities: HashMap<Node, Node> =
            HashMap::with_capacity(used_communities.len());

        let mut num_communities = 0;
        for community in &used_communities {
            renumbered_communities.insert(*community, num_communities);
            num_communities += 1;
        }
        assert_eq!(num_communities, used_communities.len());

        // Remap old community to renumbered one
        for node in 0..self.graph.num_nodes() {
            self.node_to_community[node] = renumbered_communities[&self.node_to_community[node]];
        }

        num_communities
    }

    fn calc_weighted_degrees_for_communities(&mut self, node: Node) {
        // Reset weights
        self.weighted_degrees_to_communities.clear();

        // Calculate weights
        self.graph
            .adjacent_edges(node)
            .into_iter()
            .flatten()
            // Filter out self-loops as we deal with their weights separately
            // in the modularity calculation
            // TODO: further explain this comment
            .filter(|(target, _)| node != **target)
            .for_each(|(target, weight)| {
                let target_community = self.node_to_community[*target];

                // Adjust weights to adjacent communities
                // (this actually includes the own community as well)
                self.weighted_degrees_to_communities
                    .entry(target_community)
                    .and_modify(|w| *w += weight)
                    .or_insert(*weight);
            });
    }
}
