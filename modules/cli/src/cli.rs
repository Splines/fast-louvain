// TODO: modularize the CLI and add human-readable error messages
// see https://rust-cli.github.io/book/tutorial/errors.html
// TODO: add logging to application

use std::{fs::File, io::Write};

use clap::Parser;
use louvain_application::{louvain::Louvain, louvain_graph::LouvainGraph};

#[derive(Parser)]
struct Cli {
    graph_path: std::path::PathBuf,

    #[arg(short = 'o', long = "output")]
    output_path: std::path::PathBuf,
}

fn parse_graph_from_file(file: &std::path::PathBuf) -> LouvainGraph {
    let res = std::fs::read_to_string(file);
    let content = match res {
        Ok(content) => content,
        Err(_err) => {
            panic!("Cannot find file '{}'", file.as_path().display());
        }
    };

    // let num_lines = content.lines().filter(|line| !line.is_empty()).count();
    // TODO: use num_lines as initial suggestion for the capacity.
    // We cannot now the exact number of nodes in the graph before having
    // gone through it once. We should restructure the graph, so that we
    // store the maximum node id and with that check if there are isolated notes
    // instead of using the capacity.
    // TODO: for other graphs, code will fail here!
    let mut g = LouvainGraph::new(4);

    for line in content.lines() {
        let mut line_split = line.split_whitespace();

        let source = line_split.next().unwrap().parse::<usize>().unwrap();
        let target = line_split.next().unwrap().parse::<usize>().unwrap();

        let weight_res = line_split.next();
        let weight = if let Some(weight) = weight_res {
            weight.parse::<f64>().unwrap()
        } else {
            1.0
        };

        g.insert_edge(source, target, weight);
    }

    return g;
}

pub fn run() {
    // Try it with: cargo run ./tests/graphs/weighted_graph_1.txt --output './tmp/output.txt'
    let args = Cli::parse();

    let mut g = parse_graph_from_file(&args.graph_path);
    let louvain = Louvain::new(&mut g);
    let (hierarchy, modularities) = louvain.run();

    println!("Hierarchy: {:?}", hierarchy);
    println!("Modularities: {:?}", modularities);

    let mut output = File::create(&args.output_path).expect("Could not create output file");
    writeln!(output, "{:?}", hierarchy).expect("Could not write to output file");
    writeln!(output, "{:?}", modularities).expect("Could not write to output file");
    println!("Output written to '{}'", args.output_path.display());
}
