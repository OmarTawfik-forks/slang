use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use semver::Version;

use crate::{cargo::workspace::serde::Manifest, paths::PrivatePathExtensions};

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
    pub members: Vec<String>,
    pub dependencies: HashMap<String, Version>,
}

impl Workspace {
    fn load(repo_root: &Path) -> Result<Self> {
        let super::serde::Workspace {
            members,
            dependencies,
        } = Manifest::load(&repo_root)?
            .workspace
            .context("Cannot find a [workspace] table in the root manifest.")?;

        let dependencies = dependencies
            .into_iter()
            .filter_map(|(name, dependency)| Some((name, dependency.version?)))
            .collect();

        return Ok(Self {
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
        let super::serde::Package { name } =
            Manifest::load(&crate_dir)?.package.with_context(|| {
                format!("Cannot find a [package] table in the manifest of: {crate_dir:?}")
            })?;

        return Ok(Crate { name, crate_dir });
    }
}
