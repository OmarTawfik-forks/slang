use std::path::Path;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::{
    cargo::CargoWorkspace,
    commands::Command,
    github::GitHub,
    paths::{FileWalker, PathExtensions},
    terminal::Terminal,
};

use crate::utils::{Task, ValueEnumExtensions};

#[derive(Clone, Debug, Parser)]
pub struct LintCommand {
    #[clap(trailing_var_arg = true)]
    tasks: Vec<LintTask>,
}

impl LintCommand {
    pub fn execute(&self) -> Result<()> {
        return LintTask::execute_user_selection(&self.tasks);
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum LintTask {
    /// Format all Rust source files.
    CargoFmt,
    /// Check for spelling issues in Markdown files.
    Cspell,
    /// Format all non-Rust source files.
    Prettier,
    /// Check for broken links in Markdown files.
    MarkdownLinkCheck,
    /// Check for violations in Markdown files.
    MarkdownLint,
    /// Check for violations in Bash files.
    Shellcheck,
    /// Check for type errors in TypeScript files.
    Tsc,
    /// Check for violations issues in Yaml files.
    Yamllint,
}

impl Task for LintTask {
    fn execute(&self) -> Result<()> {
        Terminal::separator(self.clap_name());

        return match self {
            LintTask::CargoFmt => run_cargo_fmt(),
            LintTask::Cspell => run_cspell(),
            LintTask::Prettier => run_prettier(),
            LintTask::MarkdownLinkCheck => run_markdown_link_check(),
            LintTask::MarkdownLint => run_markdown_lint(),
            LintTask::Shellcheck => run_shellcheck(),
            LintTask::Tsc => run_tsc(),
            LintTask::Yamllint => run_yamllint(),
        };
    }
}

fn run_cargo_fmt() -> Result<()> {
    let mut command = Command::new("cargo-fmt").flag("--all").flag("--verbose");

    if GitHub::is_running_in_ci() {
        command = command.flag("--check");
    }

    return command.run();
}

fn run_cspell() -> Result<()> {
    return Command::new("cspell")
        .arg("lint")
        .flag("--show-context")
        .flag("--show-suggestions")
        .flag("--dot")
        .flag("--gitignore")
        .run();
}

fn run_prettier() -> Result<()> {
    return if GitHub::is_running_in_ci() {
        Command::new("prettier").property("--check", ".").run()
    } else {
        Command::new("prettier").property("--write", ".").run()
    };
}

fn run_markdown_link_check() -> Result<()> {
    let config_file = Path::repo_path(".markdown-link-check.json");

    let markdown_files = FileWalker::from_repo_root().find(["**/*.md"])?;

    return Command::new("markdown-link-check")
        .property("--config", config_file.unwrap_str())
        .run_in_batches(markdown_files);
}

fn run_markdown_lint() -> Result<()> {
    let markdown_files = FileWalker::from_repo_root()
        .find(["**/*.md"])?
        .into_iter()
        .map(|path| {
            println!("{}", path.display());
            return path;
        });

    let mut command = Command::new("markdownlint").flag("--dot");

    if !GitHub::is_running_in_ci() {
        command = command.flag("--fix");
    }

    return command.run_in_batches(markdown_files);
}

fn run_shellcheck() -> Result<()> {
    let bash_files = FileWalker::from_repo_root()
        .find(["**/*.sh"])?
        .into_iter()
        .map(|path| {
            println!("{}", path.display());
            return path;
        });

    return Command::new("shellcheck").run_in_batches(bash_files);
}

fn run_tsc() -> Result<()> {
    let config_file = Path::repo_path("tsconfig.json");

    return Command::new("tsc")
        .property("--project", config_file.unwrap_str())
        .run();
}

fn run_yamllint() -> Result<()> {
    let config_file = Path::repo_path(".yamllint.yml");

    let yaml_files = FileWalker::from_repo_root()
        .find(["**/*.yml"])?
        .into_iter()
        .map(|path| {
            println!("{}", path.display());
            return path;
        });

    // Run the command next to the Pipfile installing it:
    let crate_dir = CargoWorkspace::locate_source_crate("infra_cli");

    return Command::new("python3")
        .property("-m", "pipenv")
        .args(["run", "yamllint"])
        .flag("--strict")
        .property("--config-file", config_file.unwrap_str())
        .current_dir(crate_dir)
        .run_in_batches(yaml_files);
}
