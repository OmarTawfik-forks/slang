use anyhow::{Context, Result};
use infra_utils::paths::PathExtensions;
use semver::Version;
use serde::Deserialize;

use crate::toolchains::napi::resolver::NapiResolver;

#[derive(Deserialize)]
struct Package {
    version: Version,
    napi: Option<NapiEntry>,
}

#[derive(Deserialize)]
struct NapiEntry {
    triples: NapiTriples,
}

#[derive(Deserialize)]
struct NapiTriples {
    defaults: bool,
    additional: Vec<String>,
}

pub struct NapiConfig;

impl NapiConfig {
    pub fn main_package_version() -> Result<Version> {
        return Ok(load_package()?.version);
    }

    pub fn list_all_targets() -> Result<Vec<String>> {
        let triples = load_package()?
            .napi
            .context("Failed to find NAPI config section")?
            .triples;

        assert!(
            !triples.defaults,
            "Package should explicitly list targets, instead of using defaults."
        );

        return Ok(triples.additional.to_owned());
    }
}

fn load_package() -> Result<Package> {
    let package_json = NapiResolver::package_dir()
        .join("package.json")
        .read_to_string()?;

    let package = serde_json::from_str::<Package>(&package_json)?;

    return Ok(package);
}
