mod changeset;
mod check;
mod lint;
mod publish;
mod run;
mod setup;
mod test;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands::{
    changeset::ChangesetController, check::CheckController, lint::LintController,
    publish::PublishController, run::RunController, setup::SetupController, test::TestController,
};

#[derive(Debug, Parser)]
pub struct AppController {
    #[command(subcommand)]
    command: AppCommand,
}

#[derive(Debug, Subcommand)]
pub enum AppCommand {
    /// Setup toolchains and dependencies.
    Setup(SetupController),
    /// Runs codegen checks, and makes sure source files are up to date.
    Check(CheckController),
    /// Runs unit tests.
    Test(TestController),
    /// Lints and attempts to fix formatting, spelling, broken links, and other issues.
    Lint(LintController),
    /// Runs a local binary within this repository, forwarding any additional arguments along.
    Run(RunController),
    /// Subcommands to create, consume, and publish changesets and changelogs.
    Changeset(ChangesetController),
    /// Publishes different artifacts after a successful CI build on main branch.
    Publish(PublishController),
}

impl AppController {
    pub fn execute(&self) -> Result<()> {
        return match &self.command {
            AppCommand::Setup(command) => command.execute(),
            AppCommand::Check(command) => command.execute(),
            AppCommand::Test(command) => command.execute(),
            AppCommand::Lint(command) => command.execute(),
            AppCommand::Run(command) => command.execute(),
            AppCommand::Changeset(command) => command.execute(),
            AppCommand::Publish(command) => command.execute(),
        };
    }
}
