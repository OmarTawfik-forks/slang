use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::cargo::CargoWorkspaceCommands;
use infra_utils::commands::Command;
use infra_utils::terminal::Terminal;
use rayon::iter::{ParallelBridge, ParallelIterator};
use strum::IntoEnumIterator;

use crate::toolchains::wasm::WasmPackage;
use crate::utils::{ClapExtensions, OrderedCommand};

#[derive(Clone, Debug, Default, Parser)]
pub struct CheckController {
    #[clap(trailing_var_arg = true)]
    commands: Vec<CheckCommand>,
}

impl CheckController {
    pub fn execute(&self) -> Result<()> {
        CheckCommand::execute_in_order(&self.commands)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum CheckCommand {
    /// Runs the build steps to (re-)generate any runtime source files.
    Sync,
    /// Run 'cargo check' for all crates, features, and targets.
    Cargo,
    /// Check NPM packages for any outdated codegen steps.
    Npm,
}

impl OrderedCommand for CheckCommand {
    fn execute(&self) -> Result<()> {
        Terminal::step(format!("check {name}", name = self.clap_name()));

        match self {
            CheckCommand::Sync => check_sync(),
            CheckCommand::Cargo => check_cargo(),
            CheckCommand::Npm => check_npm(),
        };

        Ok(())
    }
}

fn check_sync() {
    Command::new("cargo")
        .arg("check")
        .property("--package", "codegen_runtime_sync")
        .property("--package", "solidity_sync")
        .property("--package", "testlang_sync")
        .add_build_rustflags()
        .run();
}

fn check_cargo() {
    // 'cargo clippy' will run both 'cargo check', and 'clippy' lints:
    Command::new("cargo")
        .arg("clippy")
        .flag("--workspace")
        .flag("--all-features")
        .flag("--all-targets")
        .flag("--no-deps")
        .add_build_rustflags()
        .run();
}

fn check_npm() {
    WasmPackage::iter()
        .par_bridge()
        .for_each(|package| package.build().unwrap());
}
