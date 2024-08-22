mod manifest;
#[cfg(test)]
mod public_api_snapshots;
mod workspace;

pub use workspace::{CargoWorkspace, CargoWorkspaceCommands};

#[derive(Clone, Copy, Debug, strum_macros::Display, strum_macros:: EnumIter)]
#[allow(non_camel_case_types)]
pub enum UserFacingCrate {
    // Sorted by dependency order (from dependencies to dependents):
    metaslang_cst,
    metaslang_graph_builder,
    metaslang_bindings,
    slang_solidity,
    slang_solidity_cli,
}
