use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::{commands::Command, terminal::Terminal};

use crate::extensions::{ClapExtensions, OrderedCommand};

#[derive(Clone, Debug, Parser)]
pub struct TestController {
    #[clap(trailing_var_arg = true)]
    commands: Vec<TestCommand>,
}

impl TestController {
    pub fn execute(&self) -> Result<()> {
        return TestCommand::execute_all(&self.commands);
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum TestCommand {
    /// Runs 'cargo test' for all crates, features, and targets.
    Cargo,
    /// Runs 'test' scripts in each NPM package in the repository.
    Npm,
}

impl OrderedCommand for TestCommand {
    fn execute(&self) -> Result<()> {
        Terminal::separator(self.clap_name());

        return match self {
            TestCommand::Cargo => test_cargo(),
            TestCommand::Npm => test_npm(),
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
