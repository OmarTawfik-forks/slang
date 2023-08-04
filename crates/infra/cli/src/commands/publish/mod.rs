use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::{cargo::CargoWorkspace, commands::Command, github::GitHub, terminal::Terminal};

use crate::utils::{ClapExtensions, OrderedCommand};

#[derive(Clone, Debug, Parser)]
pub struct PublishController {
    #[clap(trailing_var_arg = true)]
    commands: Vec<PublishCommand>,
}

impl PublishController {
    pub fn execute(&self) -> Result<()> {
        return PublishCommand::execute_in_order(&self.commands);
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum PublishCommand {
    /// Publishes source crates to [crates.io].
    Cargo,
}

impl OrderedCommand for PublishCommand {
    fn execute(&self) -> Result<()> {
        Terminal::separator(self.clap_name());

        return match self {
            PublishCommand::Cargo => publish_cargo(),
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
