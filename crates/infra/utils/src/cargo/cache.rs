use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::Result;
use lazy_static::lazy_static;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use semver::Version;

use crate::{
    cargo::serde::{CrateManifest, WorkspaceManifest},
    paths::PrivatePathExtensions,
};

lazy_static! {
    pub static ref CACHE: Cache = Cache::load().expect("Failed to load cache.");
}

pub struct Cache {
    pub workspace: Workspace,
    pub crates: HashMap<String, Crate>,
}

impl Cache {
    fn load() -> Result<Self> {
        let repo_root = Path::repo_root();

        let workspace = Workspace::load(&repo_root)?;

        let crates = workspace
            .members
            .par_iter()
            .map(|member| {
                let _crate = Crate::load(repo_root.join(member))?;
                let name = _crate.name.to_owned();
                return Ok((name, _crate));
            })
            .collect::<Result<_>>()?;

        return Ok(Self { workspace, crates });
    }
}

pub struct Workspace {
    pub version: Version,
    pub members: Vec<String>,
    pub dependencies: HashMap<String, Version>,
}

impl Workspace {
    fn load(repo_root: &Path) -> Result<Self> {
        let super::serde::Workspace {
            package,
            members,
            dependencies,
        } = WorkspaceManifest::load(&repo_root)?.workspace;

        let version = package.version;

        let dependencies = dependencies
            .into_iter()
            .filter_map(|(name, dependency)| Some((name, dependency.version?)))
            .collect();

        return Ok(Self {
            version,
            members,
            dependencies,
        });
    }
}

pub struct Crate {
    pub name: String,
    pub crate_dir: PathBuf,
}

impl Crate {
    fn load(crate_dir: PathBuf) -> Result<Self> {
        let super::serde::CratePackage { name } = CrateManifest::load(&crate_dir)?.package;

        return Ok(Crate { name, crate_dir });
    }
}
