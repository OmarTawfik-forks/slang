use anyhow::Result;
use clap::{Parser, Subcommand};
use infra_utils::{commands::Command, terminal::Terminal};

#[derive(Clone, Debug, Parser)]
pub struct ChangesetController {
    #[command(subcommand)]
    command: ChangesetCommand,
}

#[derive(Clone, Debug, Subcommand)]
pub enum ChangesetCommand {
    /// Adds a new changeset entry as markdown.
    Add,
}

impl ChangesetController {
    pub fn execute(&self) -> Result<()> {
        Terminal::separator("changeset");

        return match self.command {
            ChangesetCommand::Add => add_changeset(),
        };
    }
}

fn add_changeset() -> Result<()> {
    return Command::new("changeset").arg("add").run();
}
