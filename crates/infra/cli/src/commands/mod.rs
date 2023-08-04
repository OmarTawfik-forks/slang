mod check;
mod lint;
mod publish;
mod run;
mod setup;
mod test;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands::{
    check::CheckCommand, lint::LintCommand, publish::PublishCommand, run::RunCommand,
    setup::SetupCommand, test::TestCommand,
};

#[derive(Debug, Parser)]
pub struct CLI {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Setup toolchains and dependencies.
    Setup(SetupCommand),
    /// Runs codegen checks, and makes sure source files are up to date.
    Check(CheckCommand),
    /// Runs unit tests.
    Test(TestCommand),
    /// Lints and attempts to fix formatting, spelling, broken links, and other issues.
    Lint(LintCommand),
    /// Runs a local binary within this repository, forwarding any additional arguments along.
    Run(RunCommand),
    /// Publishes different artifacts after a successful CI build on main branch.
    Publish(PublishCommand),
}

impl CLI {
    pub fn execute(&self) -> Result<()> {
        return match &self.command {
            Command::Setup(setup_command) => setup_command.execute(),
            Command::Check(check_command) => check_command.execute(),
            Command::Test(test_command) => test_command.execute(),
            Command::Lint(lint_command) => lint_command.execute(),
            Command::Run(run_command) => run_command.execute(),
            Command::Publish(publish_command) => publish_command.execute(),
        };
    }
}
