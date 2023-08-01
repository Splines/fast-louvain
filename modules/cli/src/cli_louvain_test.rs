use super::*;

#[ignore]
#[test]
fn original_louvain_graph() {
    // For now just a convenience test to run the original graph from the paper.
    // TODO: add assert statements and test the graph directly without CLI.
    let args = LouvainArgs {
        graph_path: std::path::PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/graphs/original_paper_graph.csv"
        )),
        assignment_output_path: Some(std::path::PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tmp/data/rust_assignment.csv"
        ))),
        hierarchy_output_path: Some(std::path::PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tmp/data/rust_hierarchy.tmp"
        ))),
    };
    run(&args);
}
