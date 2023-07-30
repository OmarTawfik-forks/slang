use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::{commands::Command, terminal::Terminal};

use crate::utils::{Task, ValueEnumExtensions};

#[derive(Clone, Debug, Parser)]
pub struct TestCommand {
    #[clap(trailing_var_arg = true)]
    tasks: Vec<TestTask>,
}

impl TestCommand {
    pub fn execute(&self) -> Result<()> {
        return TestTask::execute_user_selection(&self.tasks);
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum TestTask {
    /// Runs 'cargo test' for all crates, features, and targets.
    Cargo,
    /// Runs 'test' scripts in each NPM package in the repository.
    Npm,
}

impl Task for TestTask {
    fn execute(&self) -> Result<()> {
        Terminal::separator(self.clap_name());

        return match self {
            TestTask::Cargo => test_cargo(),
            TestTask::Npm => test_npm(),
        };
    }
}

fn test_cargo() -> Result<()> {
    return Command::new("cargo")
        .arg("test")
        .flag("--no-fail-fast")
        .flag("--offline")
        .flag("--all")
        .flag("--all-targets")
        .flag("--all-features")
        .run();
}

fn test_npm() -> Result<()> {
    return Command::new("npm")
        .args(["run", "test"])
        .flag("--workspaces")
        .flag("--if-present")
        .run();
}
