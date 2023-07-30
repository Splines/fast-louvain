use louvain_application::louvain_graph::LouvainGraph;
use louvain_domain::graph::Node;

use crate::io::file_reader;

pub fn parse_graph_from_file(file: &std::path::PathBuf) -> LouvainGraph {
    let mut reader = file_reader::BufReader::open(file)
        .unwrap_or_else(|_| panic!("Cannot find file '{}'", file.as_path().display()));
    let mut buffer = String::new();

    let num_lines = reader.num_non_empty_lines();
    println!("Number of lines in file: {}", num_lines);

    // We divide here as the number of edges is probably bigger than the actual
    // number of nodes in a graph. If not, the graph will resize the data
    // structures accordingly.
    let capacity_pessimistic_estimate = num_lines / 3;
    let mut g = LouvainGraph::new(capacity_pessimistic_estimate);

    let mut line_buf: &str;
    let mut line_split: std::str::Split<'_, char>;
    reader.read_line(&mut buffer); // skip header
    while let Some(line) = reader.read_line(&mut buffer) {
        line_buf = line.expect("Could not read line from file").trim();
        if line_buf.is_empty() {
            continue;
        }
        line_split = line_buf.split(',');

        let source: Node = line_split.next().unwrap().parse::<usize>().unwrap();
        let target: Node = line_split.next().unwrap().parse::<usize>().unwrap();

        let weight_res = line_split.next();
        let weight = if let Some(weight) = weight_res {
            weight.parse::<f64>().unwrap()
        } else {
            1.0
        };

        g.insert_edge(source, target, weight);
    }

    g
}
