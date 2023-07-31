use crate::io::{
    community_assignment_writer::write_node_to_community_assignment_to_file,
    graph_parser::parse_graph_from_file, hierarchy_parser::get_community_assignment_for_level,
};
use std::fs::File;

use clap::Args;
use louvain_application::louvain::Louvain;

// TODO: make epsilon_min threshold adjustable from CLI

#[derive(Args)]
pub struct LouvainArgs {
    graph_path: std::path::PathBuf,

    /// Output path for the final node-community assignment
    #[arg(short = 'o', long = "output")]
    output_path: Option<std::path::PathBuf>,

    /// Output path for the hierarchy
    #[arg(long = "hierarchy")]
    hierarchy_output_path: Option<std::path::PathBuf>,
}

pub fn run(args: &LouvainArgs) {
    let mut g = parse_graph_from_file(&args.graph_path);
    let louvain = Louvain::new(&mut g);
    let (hierarchy, modularities) = louvain.run();

    println!("Modularities: {:?}", modularities);

    if args.output_path.is_some() {
        let path = args.output_path.as_ref().unwrap();
        let node_to_community = get_community_assignment_for_level(&hierarchy, 0);
        write_node_to_community_assignment_to_file(
            &node_to_community,
            args.output_path.as_ref().unwrap(),
        );
        println!("Node-community assignment written to '{}'", path.display());
    }

    if args.hierarchy_output_path.is_some() {
        let path = args.hierarchy_output_path.as_ref().unwrap();
        let output = File::create(path).expect("Could not create hierarchy output file");
        serde_json::to_writer(&output, &hierarchy)
            .expect("Could not write hierarchy to output file");
        println!("Hierarchy written to '{}'", path.display());
    }
    println!("There are {} levels in the hierarchy", hierarchy.len());
}
