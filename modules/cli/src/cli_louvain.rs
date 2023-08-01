#[cfg(test)]
#[path = "./cli_louvain_test.rs"]
mod cli_louvain_test;

use crate::io::{
    graph_parser::parse_graph_from_file,
    hierarchy_parser::get_community_assignment_for_level,
    output_writer::{
        write_hierarchy_and_modularity_to_file, write_node_to_community_assignment_to_file,
    },
};

use clap::Args;
use louvain_application::louvain::Louvain;

// TODO: make epsilon_min threshold adjustable from CLI

#[derive(Args)]
pub struct LouvainArgs {
    graph_path: std::path::PathBuf,

    /// Output path for the final node-community assignment
    #[arg(short = 's', long = "store")]
    assignment_output_path: Option<std::path::PathBuf>,

    /// Output path for the hierarchy and modularity values. You can use this
    /// intermediary file as input for the `hierarchy` command.
    #[arg(short = 'h', long = "hierarchy")]
    hierarchy_output_path: Option<std::path::PathBuf>,
}

pub fn run(args: &LouvainArgs) {
    let mut g = parse_graph_from_file(&args.graph_path);
    let louvain = Louvain::new(&mut g);
    let (hierarchy, modularities) = louvain.run();

    println!("Modularities: {:?}", modularities);

    if args.assignment_output_path.is_some() {
        let path = args.assignment_output_path.as_ref().unwrap();
        let node_to_community = get_community_assignment_for_level(&hierarchy, 0);
        write_node_to_community_assignment_to_file(&node_to_community, path);
        println!("Node-community assignment written to '{}'", path.display());
    }

    if args.hierarchy_output_path.is_some() {
        let path = args.hierarchy_output_path.as_ref().unwrap();
        write_hierarchy_and_modularity_to_file(&hierarchy, &modularities, path);
        println!("Hierarchy written to '{}'", path.display());
    }

    println!("There are {} levels in the hierarchy", hierarchy.len());
}
