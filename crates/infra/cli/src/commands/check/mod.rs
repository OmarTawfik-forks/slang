use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::{commands::Command, github::GitHub, terminal::Terminal};

use crate::{
    toolchains::napi::{NapiCompiler, NapiProfile},
    utils::{Task, ValueEnumExtensions},
};

#[derive(Clone, Debug, Parser)]
pub struct CheckCommand {
    #[clap(trailing_var_arg = true)]
    tasks: Vec<CheckTask>,
}

impl CheckCommand {
    pub fn execute(&self) -> Result<()> {
        return CheckTask::execute_user_selection(&self.tasks);
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum CheckTask {
    /// Runs 'cargo check' for all crates, features, and targets.
    Cargo,
    /// Checks NPM packages for any outdated codegen steps.
    Npm,
}

impl Task for CheckTask {
    fn execute(&self) -> Result<()> {
        Terminal::separator(self.clap_name());

        return match self {
            CheckTask::Cargo => check_cargo(),
            CheckTask::Npm => check_npm(),
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
