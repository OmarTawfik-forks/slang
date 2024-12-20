use std::path::{Path, PathBuf};

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;

use crate::toolchains::pipenv::PipEnv;
use crate::utils::DryRun;

pub struct Mkdocs;

impl Mkdocs {
    pub fn check() -> Result<()> {
        mkdocs().arg("build").flag("--clean").flag("--strict").run();

        let site_dir = site_dir();
        let package_dir = CargoWorkspace::locate_source_crate("solidity_npm_package")?;

        Command::new("typedoc")
            .property("--options", package_dir.join("typedoc.mjs").unwrap_string())
            .arg(package_dir.join("src/generated/index.mts").unwrap_string())
            // only needed if multiple entries
            // .property(
            //     "--basePath",
            //     package_dir.join("src/generated/").unwrap_string(),
            // )
            .property("--readme", "none")
            .flag("--cleanOutputDir")
            .property("--emit", "both")
            .property(
                "--json",
                site_dir
                    .join("user-guide/npm-package/api-reference/api.json")
                    .unwrap_string(),
            )
            .property(
                "--out",
                site_dir
                    .join("user-guide/npm-package/api-reference")
                    .unwrap_string(),
            )
            .run();

        Ok(())
    }

    pub fn watch() {
        // _MKDOCS_WATCH_PORT_ | keep in sync with the port number defined in "$REPO_ROOT/.devcontainer/devcontainer.json"
        const PORT: usize = 5353;

        mkdocs()
            .arg("serve")
            .flag("--clean")
            .flag("--watch-theme")
            .property("--dev-addr", format!("localhost:{PORT}"))
            .run();
    }

    pub fn publish_main_branch(dry_run: DryRun) {
        fetch_latest_remote();

        let mut command = mike().args(["deploy", "main"]);

        if !dry_run.get() {
            command = command.flag("--push");
        }

        command.run();
    }

    pub fn publish_latest_release(dry_run: DryRun) -> Result<()> {
        fetch_latest_remote();

        let version = CargoWorkspace::local_version()?.to_string();

        if mike().args(["list", &version]).evaluate().is_ok() {
            println!("Version '{version}' is already published.");
            return Ok(());
        }

        let mut command = mike()
            .args(["deploy", &version, "latest"])
            .flag("--update-aliases");

        if !dry_run.get() {
            command = command.flag("--push");
        }

        command.run();

        Ok(())
    }
}

fn fetch_latest_remote() {
    Command::new("git")
        .args(["fetch", "origin", "gh-pages"])
        .property("--depth", "1")
        .run();
}

fn mkdocs() -> Command {
    PipEnv::run("mkdocs").current_dir(Path::repo_path("documentation"))
}

fn mike() -> Command {
    PipEnv::run("mike").current_dir(Path::repo_path("documentation"))
}

fn site_dir() -> PathBuf {
    // _SLANG_MKDOCS_DOCUMENTATION_SITE_DIR_ (keep in sync)
    Path::repo_path("documentation/target/site")
}
