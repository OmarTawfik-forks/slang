use std::{collections::HashMap, path::Path};

use anyhow::{Context, Result};
use semver::Version;
use serde::{de::DeserializeOwned, Deserialize};

use crate::paths::PathExtensions;

#[derive(Deserialize)]
pub struct WorkspaceManifest {
    pub workspace: Workspace,
}

#[derive(Deserialize)]
pub struct Workspace {
    pub package: WorkspacePackage,
    pub members: Vec<String>,
    pub dependencies: HashMap<String, Dependency>,
}

#[derive(Deserialize)]
pub struct WorkspacePackage {
    pub version: Version,
}

#[derive(Deserialize)]
pub struct CrateManifest {
    pub package: CratePackage,
}

#[derive(Deserialize)]
pub struct CratePackage {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Dependency {
    pub version: Option<Version>,
}

impl WorkspaceManifest {
    pub fn load(crate_dir: &Path) -> Result<Self> {
        return load_manifest::<Self>(crate_dir);
    }
}

impl CrateManifest {
    pub fn load(crate_dir: &Path) -> Result<Self> {
        return load_manifest::<Self>(crate_dir);
    }
}

fn load_manifest<T: DeserializeOwned>(crate_dir: &Path) -> Result<T> {
    let manifest_path = crate_dir.join("Cargo.toml");

    return toml::from_str::<T>(&manifest_path.read_to_string()?)
        .with_context(|| format!("Failed to deserialize manifest: {manifest_path:?}"));
}
