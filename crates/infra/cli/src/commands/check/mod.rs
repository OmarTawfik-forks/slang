use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::{commands::Command, github::GitHub, terminal::Terminal};

use crate::{
    extensions::{ClapExtensions, OrderedCommand},
    toolchains::napi::{NapiCompiler, NapiProfile},
};

#[derive(Clone, Debug, Parser)]
pub struct CheckController {
    #[clap(trailing_var_arg = true)]
    commands: Vec<CheckCommand>,
}

impl CheckController {
    pub fn execute(&self) -> Result<()> {
        return CheckCommand::execute_all(&self.commands);
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum CheckCommand {
    /// Runs 'cargo check' for all crates, features, and targets.
    Cargo,
    /// Checks NPM packages for any outdated codegen steps.
    Npm,
}

impl OrderedCommand for CheckCommand {
    fn execute(&self) -> Result<()> {
        Terminal::separator(self.clap_name());

        return match self {
            CheckCommand::Cargo => check_cargo(),
            CheckCommand::Npm => check_npm(),
        };
    }
}

fn check_cargo() -> Result<()> {
    let mut command = Command::new("cargo")
        .arg("check")
        .flag("--offline")
        .flag("--all")
        .flag("--all-targets")
        .flag("--all-features");

    if GitHub::is_running_in_ci() {
        command = command.property(
            "--config",
            format!(
                "build.rustflags = {rustflags}",
                rustflags = serde_json::to_string(&["--deny", "warnings"])?
            ),
        );
    }

    return command.run();
}

fn check_npm() -> Result<()> {
    return NapiCompiler::run(NapiProfile::Debug);
}
