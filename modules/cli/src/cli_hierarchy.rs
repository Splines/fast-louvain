use clap::Args;

use crate::io::{
    community_assignment_writer::write_node_to_community_assignment_to_file,
    hierarchy_parser::{get_community_assignment_for_level, parse_hierarchy_from_file},
};

#[derive(Args)]
pub struct HierarchyArgs {
    hierarchy_path: std::path::PathBuf,

    #[arg(short = 'l', long = "level")]
    level: Option<usize>,

    #[arg(short = 'o', long = "output")]
    output_path: Option<std::path::PathBuf>,
}

pub fn run(args: &HierarchyArgs) {
    let hierarchy = parse_hierarchy_from_file(&args.hierarchy_path);

    let level = args.level.unwrap_or(0);
    let node_to_community = get_community_assignment_for_level(&hierarchy, level);

    if args.output_path.is_some() {
        write_node_to_community_assignment_to_file(
            &node_to_community,
            args.output_path.as_ref().unwrap(),
        );
    }
    println!(
        "Node-community assignment written to '{}'",
        &args.hierarchy_path.display()
    );
    println!("Community assignment: {:?}", node_to_community);
}
