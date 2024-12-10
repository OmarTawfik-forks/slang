use slang_solidity::bindings::BindingGraph;

pub fn setup() -> BindingGraph {
    let dependencies = super::definitions::setup();

    super::definitions::run(dependencies)
}

pub fn run(binding_graph: BindingGraph) {
    let mut reference_count = 0_usize;
    let mut resolved_references = 0_usize;

    for reference in binding_graph.all_references() {
        reference_count += 1;

        let resolution = reference.jump_to_definition();
        if resolution.is_ok() {
            resolved_references += 1;
        }
    }

    assert_eq!(reference_count, 1686, "Failed to fetch all references");

    assert_eq!(
        resolved_references, 1353,
        "Failed to resolve all references"
    );
}
