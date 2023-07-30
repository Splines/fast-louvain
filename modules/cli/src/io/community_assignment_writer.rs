use louvain_application::community_assignment::NodeToCommunity;
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
