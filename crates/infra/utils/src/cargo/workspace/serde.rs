use std::{collections::HashMap, path::Path};

use anyhow::{Context, Result};
use semver::Version;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Manifest {
    pub workspace: Option<Workspace>,
    pub package: Option<Package>,
}

#[derive(Deserialize)]
pub struct Workspace {
    pub members: Vec<String>,
    pub dependencies: HashMap<String, Dependency>,
}

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Dependency {
    pub version: Option<Version>,
}

impl Manifest {
    pub fn load(crate_dir: &Path) -> Result<Self> {
        let path = crate_dir.join("Cargo.toml");

        let contents = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read manifest: {path:?}"))?;

        return toml::from_str::<Manifest>(&contents)
            .with_context(|| format!("Failed to read manifest: {path:?}"));
    }
}
