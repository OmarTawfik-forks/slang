use std::{collections::HashMap, env::var, path::Path};

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use infra_utils::{
    commands::Command,
    github::GitHub,
    paths::{FileWalker, PathExtensions},
    terminal::Terminal,
};
use serde::Deserialize;

use crate::utils::{Task, ValueEnumExtensions};

#[derive(Clone, Debug, Parser)]
pub struct SetupCommand {
    #[clap(trailing_var_arg = true)]
    tasks: Vec<SetupTask>,
}

impl SetupCommand {
    pub fn execute(&self) -> Result<()> {
        return SetupTask::execute_user_selection(&self.tasks);
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum SetupTask {
    /// Configures repository for building in isolation.
    Workspace,
    /// Installs Cargo dependencies.
    Cargo,
    /// Install NPM dependencies.
    Npm,
    /// Install Pipenv dependencies.
    Pipenv,
}

impl Task for SetupTask {
    fn execute(&self) -> Result<()> {
        Terminal::separator(self.clap_name());

        return match self {
            SetupTask::Workspace => setup_workspace(),
            SetupTask::Cargo => setup_cargo(),
            SetupTask::Npm => setup_npm(),
            SetupTask::Pipenv => setup_pipenv(),
        };
    }
}

fn setup_workspace() -> Result<()> {
    if GitHub::is_running_in_ci() {
        // Mark the repository as a safe directory in git:
        // See https://github.com/actions/checkout/issues/766
        let github_workspace = var("GITHUB_WORKSPACE")?;

        Command::new("git")
            .arg("config")
            .flag("--global")
            .args(["--add", "safe.directory", github_workspace.as_str()])
            .run()?;
    } else {
        // Warms up IDE tools in case it was a fresh installation:
        Command::new("rust-analyzer").flag("--version").run()?;
        Command::new("rust-src").flag("--version").run()?;
    }

    return Ok(());
}

fn setup_cargo() -> Result<()> {
    return if GitHub::is_running_in_ci() {
        Command::new("cargo").arg("fetch").flag("--locked").run()
    } else {
        Command::new("cargo").arg("fetch").run()
    };
}

fn setup_npm() -> Result<()> {
    return if GitHub::is_running_in_ci() {
        Command::new("npm").arg("install").flag("--ci").run()
    } else {
        Command::new("npm").arg("install").run()
    };
}

fn setup_pipenv() -> Result<()> {
    #[derive(Deserialize)]
    struct Pipfile {
        packages: HashMap<String, String>,
    }

    // Use the top-level `Pipfile` to find the correct version of `pipenv` to install.
    let pip_file: Pipfile = {
        let file_path = Path::repo_path("Pipfile");
        let contents = std::fs::read_to_string(file_path)?;

        toml::from_str(&contents)?
    };

    // This should be a value like "==YYYY.MM.DD"
    let version = pip_file
        .packages
        .get("pipenv")
        .context("Failed to find 'pipenv' in 'Pipfile' packages.")?;

    // pip3 install "pipenv==YYYY.MM.DD"
    Command::new("pip3")
        .arg("install")
        .arg(format!("pipenv{version}"))
        .run()?;

    // Each Python project has its own Pipfile at the root of the project.
    // Find all of them, and run 'pipenv install' in each directory.
    for pip_file in FileWalker::from_repo_root().find(["**/Pipfile"])? {
        let project_directory = pip_file.unwrap_parent();

        let mut command = Command::new("python3")
            .property("-m", "pipenv")
            .arg("install")
            .current_dir(project_directory);

        if GitHub::is_running_in_ci() {
            command = command.flag("--deploy");
        }

        command.run()?;
    }

    return Ok(());
}
