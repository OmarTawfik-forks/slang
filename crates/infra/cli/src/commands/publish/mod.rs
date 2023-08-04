use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::{cargo::CargoWorkspace, commands::Command, github::GitHub, terminal::Terminal};

use crate::utils::{Task, ValueEnumExtensions};

#[derive(Clone, Debug, Parser)]
pub struct PublishCommand {
    #[clap(trailing_var_arg = true)]
    tasks: Vec<PublishTask>,
}

impl PublishCommand {
    pub fn execute(&self) -> Result<()> {
        return PublishTask::execute_user_selection(&self.tasks);
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum PublishTask {
    /// Publishes source crates to [crates.io].
    Cargo,
}

impl Task for PublishTask {
    fn execute(&self) -> Result<()> {
        Terminal::separator(self.clap_name());

        return match self {
            PublishTask::Cargo => publish_cargo(),
        };
    }
}

fn publish_cargo() -> Result<()> {
    let local_version = CargoWorkspace::local_version();
    println!("Repository version: {local_version}");

    let published_version = CargoWorkspace::published_version()?;
    println!("Published version: {published_version}");

    if local_version == &published_version {
        println!("Skipping publish, since the repository version is already published.");
        return Ok(());
    }

    let mut command = Command::new("cargo")
        .arg("publish")
        .property("--package", "slang_solidity")
        .flag("--all-features");

    if !GitHub::is_running_in_ci() {
        command = command.flag("--dry-run");
    }

    return command.run();
}
