use louvain_application::{
    community_assignment::NodeToCommunity, louvain::CommunityAssignmentHierarchy,
};

use super::file_reader;

pub fn parse_hierarchy_from_file(file: &std::path::PathBuf) -> CommunityAssignmentHierarchy {
    let mut reader = file_reader::BufReader::open(file)
        .unwrap_or_else(|_| panic!("Cannot find file '{}'", file.as_path().display()));

    let mut buffer = String::new();
    reader.read_line(&mut buffer);

    let hierarchy: CommunityAssignmentHierarchy =
        serde_json::from_str(&buffer).expect("Could not parse hierarchy from file");
    if hierarchy.get(0).is_none() {
        panic!("Hierarchy should at least contain one level");
    }
    hierarchy
}

/// Returns the community assignment for the given level in the hierarchy.
///
/// If level = 0, the community assignment for the last level in the hierarchy
/// is returned.
pub fn get_community_assignment_for_level(
    hierarchy: &CommunityAssignmentHierarchy,
    level: usize,
) -> NodeToCommunity {
    if level > hierarchy.len() {
        panic!("Level {} does not exist in hierarchy", level);
    }

    let mut node_to_community = hierarchy[0].clone();
    for (i, hierarchy_level) in hierarchy.iter().skip(1).enumerate() {
        if (i + 1) == level {
            break;
        }

        for community in node_to_community.iter_mut() {
            *community = hierarchy_level[*community];
        }
    }

    node_to_community
}
