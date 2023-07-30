use crate::io::graph_parser::parse_graph_from_file;
use std::fs::File;

use clap::Args;
use louvain_application::louvain::Louvain;

#[derive(Args)]
pub struct LouvainArgs {
    graph_path: std::path::PathBuf,

    #[arg(short = 'o', long = "output")]
    output_path: std::path::PathBuf,
    // TODO: make epsilon_min threshold adjustable from CLI
}

pub fn run(args: &LouvainArgs) {
    // Try it with: cargo run ./tests/graphs/weighted_graph_1.txt --output './tmp/output.txt'
    // let args = Cli::parse();

    let mut g = parse_graph_from_file(&args.graph_path);
    let louvain = Louvain::new(&mut g);
    let (hierarchy, modularities) = louvain.run();

    println!("Modularities: {:?}", modularities);

    let output = File::create(&args.output_path).expect("Could not create output file");
    serde_json::to_writer(&output, &hierarchy).expect("Could not write hierarchy to output file");
    println!("Hierarchy written to '{}'", args.output_path.display());
}
