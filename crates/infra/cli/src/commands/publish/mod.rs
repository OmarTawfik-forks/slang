use std::path::Path;

use anyhow::{bail, Result};
use clap::{Parser, ValueEnum};
use infra_utils::{
    cargo::CargoWorkspace, commands::Command, github::GitHub, paths::PathExtensions,
    terminal::Terminal,
};
use itertools::Itertools;
use markdown::{Block, Span};
use semver::Version;

use crate::extensions::{ClapExtensions, OrderedCommand};

#[derive(Clone, Debug, Parser)]
pub struct PublishController {
    #[clap(trailing_var_arg = true)]
    commands: Vec<PublishCommand>,
}

impl PublishController {
    pub fn execute(&self) -> Result<()> {
        return PublishCommand::execute_all(&self.commands);
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum PublishCommand {
    /// Publishes source crates to [crates.io].
    Cargo,
    /// Publishes a new release in the GitHub repository.
    GithubRelease,
    /// Update lock files with any newly published versions.
    LockFiles,
}

impl OrderedCommand for PublishCommand {
    fn execute(&self) -> Result<()> {
        Terminal::separator(self.clap_name());

        return match self {
            PublishCommand::Cargo => publish_cargo(),
            PublishCommand::GithubRelease => publish_github_release(),
            PublishCommand::LockFiles => publish_lock_files(),
        };
    }
}

fn publish_cargo() -> Result<()> {
    let workspace_version = CargoWorkspace::workspace_version();
    println!("Workspace_version version: {workspace_version}");

    for crate_name in CargoWorkspace::PUBLISHED_CRATES {
        let published_version = CargoWorkspace::crate_published_version(crate_name)?;
        println!("Published version: {published_version}");

        if workspace_version == &published_version {
            println!("Skipping crate, since the workspace version is already published.");
            continue;
        }

        let mut command = Command::new("cargo")
            .arg("publish")
            .property("--package", crate_name)
            .flag("--all-features");

        if !GitHub::is_running_in_ci() {
            println!("Attempting a dry run, since we are not running in CI.");
            command = command.flag("--dry-run");
        }

        command.run()?;
    }

    return Ok(());
}

fn publish_github_release() -> Result<()> {
    let workspace_version = CargoWorkspace::workspace_version();
    println!("Workspace version: {workspace_version}");

    let released_version = GitHub::released_version()?;
    println!("Latest release version: {released_version}");

    if workspace_version == &released_version {
        println!("Skipping release, since the workspace version is already published.");
        return Ok(());
    }

    let notes = extract_latest_changelogs(workspace_version, &released_version)?;
    let tag_name = format!("v{workspace_version}");

    println!("Creating release '{tag_name}' with contents:");
    println!();
    println!("{}", notes.lines().map(|l| format!("  │ {l}")).join("\n"));
    println!();

    if !GitHub::is_running_in_ci() {
        println!("Skipping release, since we are not running in CI.");
        return Ok(());
    }

    return GitHub::create_new_release(tag_name, notes);
}

fn extract_latest_changelogs(
    workspace_version: &Version,
    released_version: &Version,
) -> Result<String> {
    let changelog_contents = Path::repo_path("CHANGELOG.md").read_to_string()?;

    let mut all_blocks = markdown::tokenize(&changelog_contents).into_iter();

    // Asser that first block contains title '# changelog'
    assert_eq!(
        all_blocks.next().unwrap(),
        Block::Header(vec![Span::Text(format!("changelog"))], 1),
    );

    // H2 for workspace_version: '## 1.2.3'
    assert_eq!(
        all_blocks.next().unwrap(),
        Block::Header(vec![Span::Text(format!("{workspace_version}"))], 2),
    );

    let latest_version_blocks = all_blocks
        .take_while(|block| match block {
            // H2 for released_version: '## 1.2.3'
            Block::Header(contents, level) if *level == 2 => {
                assert_eq!(contents, &vec![Span::Text(format!("{released_version}"))]);
                return false;
            }
            // H3 for change kinds: breaking changes, features, or fixes.
            Block::Header(_, level) if *level == 3 => {
                return true;
            }
            // Individual changelog entries.
            Block::UnorderedList(_) => {
                return true;
            }
            _ => panic!("Unexpected block: {block:#?}"),
        })
        .collect::<Vec<_>>();

    return Ok(markdown::generate_markdown(latest_version_blocks));
}

fn publish_lock_files() -> Result<()> {
    let local_changes = Command::new("git")
        .arg("status")
        .flag("--short")
        .evaluate()?
        .trim()
        .to_owned();

    if !local_changes.is_empty() {
        bail!("Cannot update lock files. Found local changes:\n{local_changes}");
    }

    Command::new("npm")
        .arg("install")
        .flag("--package-lock-only")
        .run()?;

    let local_changes = Command::new("git")
        .arg("status")
        .flag("--short")
        .evaluate()?
        .trim()
        .to_owned();

    if local_changes.is_empty() {
        println!("No changes to lock files.");
        return Ok(());
    }

    if local_changes != "M package-lock.json" {
        bail!("Unexpected local changes:\n{local_changes}");
    }

    Command::new("git").arg("diff").flag("--cached").run()?;

    if !GitHub::is_running_in_ci() {
        println!("Skipping the update, since we are not running in CI.");
        return Ok(());
    }

    let remote = "origin";
    let head_branch = "infra/update-lock-files";

    Command::new("git")
        .arg("checkout")
        .property("-b", head_branch)
        .run()?;

    Command::new("git")
        .args(["add", "package-lock.json"])
        .run()?;

    Command::new("git")
        .property("-c", "user.name=github-actions")
        .property("-c", "user.email=github-actions@users.noreply.github.com")
        .arg("commit")
        .property("--message", "update lock files after release")
        .run()?;

    let base_branch = Command::new("git")
        .arg("branch")
        .flag("--show-current")
        .evaluate()?
        .trim()
        .to_owned();

    Command::new("git")
        .arg("push")
        .flag("--force")
        .property("--set-upstream", remote)
        .arg(head_branch)
        .run()?;

    Command::new("git")
        .arg("push")
        .flag("--force")
        .property("--set-upstream", remote)
        .arg(head_branch)
        .run()?;

    Command::new("gh")
        .args(["pr", "create"])
        .flag("--fill")
        .property("--base", format!("{remote}/{base_branch}"))
        .property("--head", format!("{remote}/{head_branch}"))
        .run()?;

    Command::new("git").args(["checkout", &base_branch]).run()?;

    return Ok(());
}
