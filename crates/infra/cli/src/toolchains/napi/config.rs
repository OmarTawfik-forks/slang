use anyhow::{Context, Result};
use serde::Deserialize;

use crate::toolchains::napi::resolver::Resolver;

#[derive(Deserialize)]
struct Package {
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
    pub fn list_all_targets() -> Result<Vec<String>> {
        let triples = Self::load_entry()?.triples;

        assert!(
            !triples.defaults,
            "Package should explicitly list targets, instead of using defaults."
        );

        return Ok(triples.additional.to_owned());
    }

    fn load_entry() -> Result<NapiEntry> {
        let package_path = Resolver::package_dir().join("package.json");

        let package_contents = std::fs::read_to_string(&package_path)
            .with_context(|| format!("Failed to read manifest: {package_path:?}"))?;

        let package = serde_json::from_str::<Package>(&package_contents)
            .with_context(|| format!("Failed to read manifest: {package_path:?}"))?;

        return package
            .napi
            .with_context(|| format!("Failed to find NAPI config section: {package_path:?}"));
    }
}
