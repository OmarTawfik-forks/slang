use anyhow::{bail, Result};
use clap::{Parser, Subcommand, ValueEnum};
use infra_utils::{
    cargo::CargoWorkspace,
    commands::Command,
    paths::{FileWalker, PathExtensions},
    terminal::Terminal,
};

use crate::{
    extensions::ClapExtensions,
    toolchains::napi::{NapiCli, NapiConfig, NapiResolver},
};

/*
 * This repository versions and releases all its artifacts together, generating the same changelog.
 * Unfortunately, changesets does not support combining changelogs from multiple packages into one.
 *
 * So, we let changesets bump the version of the single NPM package we ship, and generate its changelog.
 * Then our build process copies the new version and the single changelog to other packages and crates.
 *
 * Additionally, changesets can only bump versions of packages in the root workspace.
 * However, NAPI platform-specific packages cannot be added to the workspace, because other platforms will fail "npm install".
 * So we have to bump the versions over ourselves anyways.
 */

#[derive(Clone, Debug, Parser)]
pub struct ChangesetController {
    #[command(subcommand)]
    command: ChangesetCommand,
}

#[derive(Clone, Debug, Subcommand, ValueEnum)]
pub enum ChangesetCommand {
    /// Adds a new changeset entry as markdown.
    Add,
    /// Consumes existing changesets, incrementing package versions if needed.
    Consume,
}

impl ChangesetController {
    pub fn execute(&self) -> Result<()> {
        Terminal::separator(self.command.clap_name());

        return match self.command {
            ChangesetCommand::Add => add_changeset(),
            ChangesetCommand::Consume => consume_changesets(),
        };
    }
}

fn add_changeset() -> Result<()> {
    return Command::new("changeset").arg("add").run();
}

fn consume_changesets() -> Result<()> {
    let workspace_version = CargoWorkspace::workspace_version();
    println!("Workspace version: {workspace_version}");

    let main_package_version = NapiConfig::main_package_version()?;
    println!("Main package version: {main_package_version}");

    if workspace_version != &main_package_version {
        bail!("Workspace version '{workspace_version}' and main package version '{main_package_version}' do not match.");
    }

    // This command will:
    // 1) Consume/delete any changeset files currently in "$REPO_ROOT/.changeset"
    // 2) Update the CHANGELOG.md file for the NPM package.
    // 3) Bump the version in its package.json accourdingly.

    Command::new("changeset").arg("version").run()?;

    let updated_version = NapiConfig::main_package_version()?;
    println!("Updated version: {updated_version}");

    if main_package_version == updated_version {
        println!("No version changes. Skipping.");
        return Ok(());
    }

    // Update platform-specific packages:

    NapiCli::prepublish()?;

    // Format the updated package files:

    let package_dir = NapiResolver::package_dir();
    Command::new("prettier")
        .property("--write", package_dir.unwrap_str())
        .run()?;

    // Update NPM lock file:

    Command::new("npm")
        .arg("install")
        .flag("--package-lock-only")
        .run()?;

    // Update Cargo workspace:

    println!("Updating Cargo workspace version.");
    CargoWorkspace::update_workspace_version(&main_package_version, &updated_version)?;

    // Update Cargo lock file:

    Command::new("cargo")
        .arg("update")
        .flag("--workspace")
        .run()?;

    // Update other CHANGELOG files:

    let source_changelog = package_dir.join("CHANGELOG.md");

    for destination_changelog in FileWalker::from_repo_root().find(["**/CHANGELOG.md"])? {
        if source_changelog != destination_changelog {
            println!("Updating: {destination_changelog:?}");
            std::fs::copy(&source_changelog, &destination_changelog)?;
        }
    }

    return Ok(());
}
