mod serializer;

use anyhow::Result;
use rayon::iter::{ParallelBridge, ParallelIterator};
use strum::IntoEnumIterator;

use crate::cargo::public_api_snapshots::serializer::Snapshot;
use crate::cargo::{CargoWorkspace, UserFacingCrate};
use crate::codegen::CodegenFileSystem;
use crate::paths::PathExtensions;

#[test]
fn public_api_snapshots() {
    UserFacingCrate::iter()
        .filter(|&crate_name| has_library_target(crate_name))
        .par_bridge()
        .for_each(|crate_name| generate_public_api(crate_name).unwrap());
}

fn generate_public_api(crate_name: UserFacingCrate) -> Result<()> {
    let crate_dir = CargoWorkspace::locate_source_crate(crate_name.to_string())?;

    let json = rustdoc_json::Builder::default()
        .manifest_path(crate_dir.join("Cargo.toml"))
        .all_features(true)
        .toolchain(env!("RUST_NIGHTLY_VERSION"))
        .build()?
        .read_to_string()?;

    let _crate = serde_json::from_str::<rustdoc_types::Crate>(&json)?;
    let output = Snapshot::create(&_crate)?;

    let output_path = crate_dir.join("rustdoc/public_api_snapshot.rs");

    let mut fs = CodegenFileSystem::new(&crate_dir)?;

    fs.write_file(output_path, output)
}

fn has_library_target(crate_name: UserFacingCrate) -> bool {
    match crate_name {
        UserFacingCrate::metaslang_cst => true,
        UserFacingCrate::metaslang_graph_builder => true,
        UserFacingCrate::metaslang_bindings => true,
        UserFacingCrate::slang_solidity => true,
        UserFacingCrate::slang_solidity_cli => false,
    }
}
