use std::{env::var, path::Path};

use anyhow::{Context, Result};
use semver::Version;
use serde::Deserialize;

use crate::{commands::Command, paths::PathExtensions};

pub struct GitHub;

impl GitHub {
    pub fn is_running_in_ci() -> bool {
        return var("CI").is_ok();
    }

    pub fn released_version() -> Result<Version> {
        #[derive(Debug, Deserialize)]
        struct Release {
            tag_name: String,
        }

        let json = Command::new("gh")
            .arg("api")
            .arg("repos/{owner}/{repo}/releases/latest")
            .evaluate()?;

        let tag_name = serde_json::from_str::<Release>(&json)?.tag_name;

        // tag_name is in the form 'v1.2.3', so remove the 'v' prefix before parsing the version:
        let version = tag_name
            .strip_prefix("v")
            .with_context(|| format!("Cannot extract version out of tag: {tag_name:#?}"))?;

        return Ok(Version::parse(version)?);
    }

    pub fn create_new_release(tag_name: impl AsRef<str>, notes: impl Into<String>) -> Result<()> {
        let notes_file = Path::repo_path("target/publish/release-notes.md");

        std::fs::create_dir_all(notes_file.unwrap_parent())?;
        std::fs::write(&notes_file, notes.into())?;

        return Command::new("gh")
            .args(["release", "create", tag_name.as_ref()])
            .property("--title", tag_name.as_ref())
            .property("--notes-file", notes_file.unwrap_str())
            .run();
    }
}
