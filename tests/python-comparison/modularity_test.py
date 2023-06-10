import networkx as nx

# Test graph
G = nx.Graph()
G.add_edges_from([(0, 0, {"weight": 3.0}),
                  (0, 1, {"weight": 1.0}),
                  (1, 2, {"weight": 5.0}),
                  (2, 3, {"weight": 2.5}),
                  (3, 1, {"weight": 7.0}),
                  (3, 3, {"weight": 1.0}),
                  ])
edges = list(G.edges(data=True))
print(f'Edges: {edges}')
degrees = dict(G.degree(weight="weight"))
print(f'Degrees: {degrees}')

# Modularity for a given partition
partition = [{0}, {1}, {2}, {3}]
Q = nx.community.modularity(G, partition)
print(f'Modularity (singleton communities): {Q}')

partition = [{0}, {1, 3}, {2}]
Q = nx.community.modularity(G, partition)
print(f'Modularity (other communities): {Q}')
