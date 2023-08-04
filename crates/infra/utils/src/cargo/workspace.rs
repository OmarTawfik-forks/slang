use std::{env::var, path::PathBuf};

use anyhow::{Context, Result};
use regex::Regex;
use semver::Version;

use crate::{cargo::cache::CACHE, commands::Command};

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

    pub fn local_version() -> &'static Version {
        return &CACHE.workspace.version;
    }

    pub fn published_version() -> Result<Version> {
        // Expected Output from 'cargo search slang_solidity':
        //
        // slang_solidity = "1.2.3" # description
        //
        // Extract and parse the version (middle part).

        let output = Command::new("cargo")
            .args(["search", "slang_solidity"])
            .evaluate()?;

        let (_full, [version]) = Regex::new(r#"^slang_solidity = "(\d+\.\d+\.\d+)" *#"#)?
            .captures(&output)
            .with_context(|| format!("Failed to extract version:\n{output}"))?
            .extract();

        return Version::parse(version)
            .with_context(|| format!("Failed to parse version: '{version}'"));
    }
}
