import networkx as nx

GRAPH_FILE = './tests/graphs/original_paper_graph.csv'
ASSIGNMENT_FILE = './tmp/data/rust_assignment.csv'


def read_graph_from_csv(graph_file: str) -> nx.Graph:
    G = nx.Graph()

    with open(graph_file, 'r') as f:
        f.readline()  # skip header
        for line in f:
            parts = line.strip().split(',')
            if len(parts) > 2:
                G.add_edge(int(parts[0]), int(parts[1]),
                           weight=float(parts[2]))
            else:
                G.add_edge(int(parts[0]), int(parts[1]), weight=1.0)

    return G


def read_assignment_from_csv(assignment_file: str) -> list:
    node_to_community = []

    with open(assignment_file, 'r') as f:
        f.readline()  # skip header
        for line in f:
            community = int(line.strip().split(',')[1])
            node_to_community.append(community)

    return node_to_community


def assignment_to_partition(node_to_community: list) -> list[set]:
    num_unique_communities = len(set(node_to_community))
    partition = [set() for _ in range(num_unique_communities)]

    for node, community in enumerate(node_to_community):
        partition[community].add(node)

    return partition


def main():
    print('⚠ This program does not perform any checks and assumes a happy code execution path.')
    print('⚠ It also assumes that the nodes are labeled contiguously from 0 to n-1.')

    G = read_graph_from_csv(GRAPH_FILE)
    print(f'Graph: {G}')

    node_to_community = read_assignment_from_csv(ASSIGNMENT_FILE)
    partition = assignment_to_partition(node_to_community)
    print(f'Partition: {partition}')

    # Modularity for that partition
    Q = nx.community.modularity(G, partition)
    print(f'Modularity: {Q}')


if __name__ == '__main__':
    main()
