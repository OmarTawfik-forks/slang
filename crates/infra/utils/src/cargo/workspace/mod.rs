mod cache;
mod serde;

use std::{env::var, path::PathBuf};

use anyhow::{Context, Result};

use crate::{cargo::workspace::cache::CACHE, commands::Command};

pub struct CargoWorkspace;

impl CargoWorkspace {
    pub fn install_binary(crate_name: &str) -> Result<()> {
        let version = CACHE
            .workspace
            .dependencies
            .get(crate_name)
            .with_context(|| format!("Cannot find dependency '{crate_name}' in workspace."))?
            .to_string();

        return Command::new("cargo")
            .args(["install", crate_name])
            .property("--version", &version)
            .run();
    }

    pub fn is_running_inside_build_scripts() -> bool {
        return var("TARGET").is_ok();
    }

    pub fn locate_source_crate(name: &str) -> &PathBuf {
        return &CACHE
            .crates
            .get(name)
            .with_context(|| format!("Failed to find crate: {name}"))
            .unwrap()
            .crate_dir;
    }
}
