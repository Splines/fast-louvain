// TODO: modularize the CLI and add human-readable error messages
// see https://rust-cli.github.io/book/tutorial/errors.html
// TODO: add logging to application

use crate::graph_file_parser::parse_graph_from_file;
use std::{fs::File, io::Write};

use clap::Parser;
use louvain_application::louvain::Louvain;

#[derive(Parser)]
struct Cli {
    graph_path: std::path::PathBuf,

    #[arg(short = 'o', long = "output")]
    output_path: std::path::PathBuf,
    // TODO: make epsilon_min threshold adjustable from CLI
}

pub fn run() {
    // Try it with: cargo run ./tests/graphs/weighted_graph_1.txt --output './tmp/output.txt'
    let args = Cli::parse();

    let mut g = parse_graph_from_file(&args.graph_path);
    let louvain = Louvain::new(&mut g);
    let (hierarchy, modularities) = louvain.run();

    // println!("Hierarchy: {:?}", hierarchy);
    println!("Modularities: {:?}", modularities);

    let mut output = File::create(&args.output_path).expect("Could not create output file");
    writeln!(output, "{:?}", hierarchy).expect("Could not write to output file");
    writeln!(output, "{:?}", modularities).expect("Could not write to output file");
    println!("Output written to '{}'", args.output_path.display());
}
