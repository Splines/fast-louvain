use louvain_application::{
    community_assignment::NodeToCommunity, louvain::CommunityAssignmentHierarchy,
};
use std::io::Write;

pub fn write_node_to_community_assignment_to_file(
    node_to_community: &NodeToCommunity,
    file: &std::path::PathBuf,
) {
    let output = std::fs::File::create(file).expect("Could not open output file");
    writeln!(&output, "node,community").expect("Could not write header to file");
    for (node, community) in node_to_community.iter().enumerate() {
        writeln!(&output, "{},{}", node, community).expect("Could not write assignment to file");
    }
}

pub fn write_hierarchy_and_modularity_to_file(
    hierarchy: &CommunityAssignmentHierarchy,
    modularities: &Vec<f64>,
    file: &std::path::PathBuf,
) {
    let output = std::fs::File::create(file).expect("Could not open output file");

    // Hierarchy (node-to-community assignments)
    serde_json::to_writer(&output, &hierarchy).expect("Could not write hierarchy to output file");

    // newline
    writeln!(&output, "").unwrap();

    // Modularities
    // TODO: right now we don't do anything with the modularities later on
    // e.g. parsing them from the file and printing them to the console
    serde_json::to_writer(&output, &modularities)
        .expect("Could not write modularities to output file");
}
