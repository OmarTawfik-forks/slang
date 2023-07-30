use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use infra_utils::{commands::Command, paths::PathExtensions};

pub enum BuildTarget {
    Debug,
    ReleaseTarget(String),
}

pub struct NapiCliOutput {
    /// `index.d.ts` and `index.js`
    pub source_files: Vec<PathBuf>,
    /// `index.TARGET_NAME.node`
    pub node_binary: PathBuf,
}

pub struct NapiCli;

impl NapiCli {
    pub fn build(
        package_dir: impl AsRef<Path>,
        crate_dir: impl AsRef<Path>,
        output_dir: impl AsRef<Path>,
        cargo_executable: impl Into<String>,
        target: &BuildTarget,
    ) -> Result<NapiCliOutput> {
        let package_dir = package_dir.as_ref();
        let crate_dir = crate_dir.as_ref();
        let output_dir = output_dir.as_ref();

        let mut command = Command::new("napi");

        {
            // NAPI will fail if it was invoked with full paths.
            // It expects all arguments to be relative to the current directory.
            // Commands run from repo root by default, so let's strip it from paths.
            let package_dir = package_dir.strip_repo_root()?;
            let crate_dir = crate_dir.strip_repo_root()?;
            let output_dir = output_dir.strip_repo_root()?;

            command = command
                .args(["build", output_dir.unwrap_str()])
                .property("--config", package_dir.join("package.json").unwrap_str())
                .property("--cargo-cwd", crate_dir.unwrap_str());
        }

        command = command
            .flag("--platform")
            .flag("--no-const-enum")
            .env("CARGO", cargo_executable);

        match target {
            BuildTarget::Debug => {
                // Default
            }
            BuildTarget::ReleaseTarget(target) => {
                command = command.flag("--release").property("--target", target);
            }
        };

        command.run()?;

        let mut source_files = vec![];
        let mut node_binary = None;

        for child in output_dir.collect_children()? {
            let file_name = child.unwrap_name();

            match file_name {
                "index.js" | "index.d.ts" => {
                    source_files.push(output_dir.join(file_name));
                }
                file if file.ends_with(".node") && node_binary.is_none() => {
                    node_binary = Some(output_dir.join(file));
                }
                _ => {
                    bail!("Unexpected file {file_name:?} in {output_dir:?}");
                }
            };
        }

        assert_eq!(
            source_files.len(),
            2,
            "Missing source files in {output_dir:?}",
        );

        let node_binary =
            node_binary.with_context(|| format!("No .node file in {output_dir:?}"))?;

        return Ok(NapiCliOutput {
            source_files,
            node_binary,
        });
    }
}
