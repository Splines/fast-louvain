use clap::Args;

use crate::io::hierarchy_parser::{get_community_assignment_for_level, parse_hierarchy_from_file};

#[derive(Args)]
pub struct HierarchyArgs {
    hierarchy_path: std::path::PathBuf,

    #[arg(short = 'l', long = "level")]
    level: usize,
}

pub fn run(args: &HierarchyArgs) {
    let hierarchy = parse_hierarchy_from_file(&args.hierarchy_path);
    let community_assignment = get_community_assignment_for_level(&hierarchy, args.level);
    println!("Community assignment: {:?}", community_assignment);
}
