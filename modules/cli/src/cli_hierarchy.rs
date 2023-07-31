use clap::Args;

use crate::io::{
    hierarchy_parser::{get_community_assignment_for_level, parse_hierarchy_from_file},
    output_writer::write_node_to_community_assignment_to_file,
};

#[derive(Args)]
pub struct HierarchyArgs {
    hierarchy_path: std::path::PathBuf,

    /// Level in the hierarchy for which the community assignment should be
    /// returned. First level in the hierarchy is level 1.
    /// If level 0 is given or no level is given at all, then the community
    /// assignment for the last level in the hierarchy is returned.
    #[arg(short = 'l', long = "level")]
    level: Option<usize>,

    /// Output path for the final node-community assignment
    #[arg(short = 's', long = "store")]
    assignment_output_path: Option<std::path::PathBuf>,
}

pub fn run(args: &HierarchyArgs) {
    let hierarchy = parse_hierarchy_from_file(&args.hierarchy_path);

    let level = args.level.unwrap_or(0);
    let node_to_community = get_community_assignment_for_level(&hierarchy, level);

    if args.assignment_output_path.is_some() {
        let path = args.assignment_output_path.as_ref().unwrap();
        write_node_to_community_assignment_to_file(&node_to_community, path);
        println!("Node-community assignment written to '{}'", path.display());
    }

    println!("Community assignment: {:?}", node_to_community);
}
