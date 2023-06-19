#[cfg(test)]
#[path = "./modularity_test.rs"]
mod modularity_test;

use louvain_domain::graph::Node;

use crate::louvain_graph::LouvainGraph;

pub type Community = usize;
pub type VertexToCommunity = Vec<Community>;

/// Measure to evaluate how good a proposed partition of the network into
/// communities is. Modularity is in the range [-0.5, 1].
///
/// Modularity Q is maximized for divisions of a graph when many edges fall
/// within the proposed communities (intra-community edges)
/// as compared to edges between communities (inter-community edges).
///
/// Note that for the Louvain algorithm, we only need to calculate the absolute
/// modularity once for every pass when every node is in its own singleton
/// community. Then, only relative deltas of modularity are calculated and
/// added to the global value.
/// Therefore, this implementation of modularity does not allow to
/// calculate modularity for arbitrary vertex-community assignments.
struct Modularity<'a> {
    graph: &'a LouvainGraph,
    vertex_to_community: VertexToCommunity,
    /// Sum of weights of edges belonging to a community c
    weights_in: Vec<f64>,
    /// Sum of weighted degrees of nodes in a community c (including self-loops
    /// which are only counted once, other than undirected edges). This is
    /// equivalent to: Sum of the weights of the edges incident to nodes
    /// in a communiyt c, including self-loops (including edges with both
    /// ends in community c).
    weights_tot: Vec<f64>,
}

impl<'a> Modularity<'a> {
    pub fn new(graph: &'a LouvainGraph) -> Self {
        let mut modularity = Modularity {
            graph,
            vertex_to_community: vec![0; graph.capacity()],
            // there are graph.capacity() communities at the beginning of every run
            weights_in: vec![0.0; graph.capacity()],
            weights_tot: vec![0.0; graph.capacity()],
        };

        // In the beginning, every vertex is in its own singleton community
        for v in 0..graph.capacity() {
            modularity.vertex_to_community[v] = v;
            modularity.weights_in[v] = graph.self_loop_weighted_degrees[v];
            modularity.weights_tot[v] = graph.weighted_degrees[v];
        }

        modularity
    }

    pub fn remove(
        &mut self,
        v: Node,
        community: usize,
        weighted_degree_of_edges_in_community: f64,
    ) {
        // Since edges are undirected, we need to count their weights twice
        // EXCEPT FOR self-loops where i=vertex & j=vertex only appears once
        // in the sum (see formula for modularity). In other cases, we need the
        // weight of the edge for each order of arguments, thus i=vertex1 & j=vertex2
        // AND i=vertex2 & j=vertex1.
        self.weights_in[community] -= 2.0 * weighted_degree_of_edges_in_community;
        self.weights_in[community] -= self.graph.self_loop_weighted_degrees[v];

        // Remove weighted degree of this vertex as contribution to the community
        self.weights_tot[community] -= self.graph.weighted_degrees[v];

        // Reset community assignment
        self.vertex_to_community[v] = 0;
    }

    pub fn insert(
        &mut self,
        v: Node,
        community: usize,
        vertex_weighted_degree_of_edges_to_community: f64,
    ) {
        // see expalanation in remove()
        self.weights_in[community] += 2.0 * vertex_weighted_degree_of_edges_to_community;
        self.weights_in[community] += self.graph.self_loop_weighted_degrees[v];

        // We insert the vertex into a new community, so we need to add its
        // weighted degree to the sum of weighted degrees of the community (sigma_tot).
        self.weights_tot[community] += self.graph.weighted_degrees[v];

        self.vertex_to_community[v] = community;
    }

    pub fn gain(
        &self,
        community: usize,
        vertex_weighted_degree_of_edges_to_community: f64,
        weighted_degree_vertex: f64,
    ) -> f64 {
        // Let m equal twice_total_weighted_degree. Then:
        // We drop a factor of 1/m here as we only need a relative measure
        // to compare, not absolute values. The correct equation would be:
        // \Delta Q = \frac{1}{m} \cdot \Bigl( k_{i,in} - \frac{\Sigma_{tot}\cdot k_i}{2m} \Bigr)
        let tot_community = self.weights_tot[community];
        let twice_total_weighted_degree = self.graph.twice_total_weighted_degree * 2.0;
        vertex_weighted_degree_of_edges_to_community
            - (tot_community * weighted_degree_vertex) / twice_total_weighted_degree
    }

    pub fn modularity(&self) -> f64 {
        let mut quality: f64 = 0.0;
        let twice_total_weighted_degree = self.graph.twice_total_weighted_degree;

        // TODO: Get rid of unique_communities retrieval
        let unique_communities = self
            .vertex_to_community
            .iter()
            .collect::<std::collections::HashSet<_>>();

        for community in unique_communities {
            quality += self.weights_in[*community]
                - self.weights_tot[*community].powi(2) / twice_total_weighted_degree;
        }

        quality /= twice_total_weighted_degree;
        quality
    }
}
