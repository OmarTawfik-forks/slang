use std::{
    fmt::Write,
    path::{Path, PathBuf},
};

use anyhow::Result;
use infra_utils::{
    cargo::CargoWorkspace, codegen::Codegen, commands::Command, paths::PathExtensions,
};

use crate::toolchains::napi::{
    cli::{BuildTarget, NapiCli, NapiCliOutput},
    config::NapiConfig,
    resolver::{NapiPackageKind, NapiResolver},
};

pub enum NapiProfile {
    /// Build only the main package for local development.
    Debug,
    /// Build all packages (all platforms) for publishing to NPM.
    #[allow(dead_code)] // TODO(OmarTawfik): Not implemented yet.
    Release,
}

pub struct NapiCompiler;

impl NapiCompiler {
    pub fn run(profile: NapiProfile) -> Result<()> {
        match profile {
            NapiProfile::Debug => {
                // Compiles the default target for local development
                let napi_output = compile_target(&BuildTarget::Debug)?;

                // Process its generated JavaScript and TypeScript files, and copy any new changes to the source folder.
                process_generated_files(&napi_output)?;

                // Compile the main cross-platform package, and copy the build node binary to it for debugging/testing.
                compile_root_package(Some(&napi_output.node_binary))?;
            }
            NapiProfile::Release => {
                // Compile all available targets for publishing to NPM.
                let node_binaries = compile_all_targets()?;

                // Compile the main cross-platform package, but without any binaries.
                compile_root_package(None)?;

                // Compile all platform-specific packages, and copy the built node binaries to them.
                compile_platform_packages(&node_binaries)?;
            }
        }

        return Ok(());
    }
}

fn compile_all_targets() -> Result<Vec<PathBuf>> {
    let targets = NapiConfig::list_all_targets()?;

    Command::new("rustup")
        .args(["target", "add"])
        .args(&targets)
        .run()?;

    let mut node_binaries = vec![];

    for target in targets {
        let output = compile_target(&BuildTarget::ReleaseTarget(target))?;

        node_binaries.push(output.node_binary);
    }

    return Ok(node_binaries);
}

fn compile_target(target: &BuildTarget) -> Result<NapiCliOutput> {
    let cargo_executable = match target {
        BuildTarget::ReleaseTarget(target) if target.contains("-windows-") => {
            CargoWorkspace::install_binary("cargo-xwin")?;
            "cargo-xwin"
        }
        _ => "cargo",
    };

    let output_dir = NapiResolver::napi_target_dir(target);

    std::fs::create_dir_all(&output_dir)?;

    return NapiCli::build(output_dir, cargo_executable, target);
}

fn process_generated_files(napi_output: &NapiCliOutput) -> Result<()> {
    let mut codegen = Codegen::write_only()?;

    for raw_file in &napi_output.source_files {
        let destination_file = NapiResolver::generated_dir().join(raw_file.unwrap_name());

        let w = &mut String::new();

        writeln!(
            w,
            "// Slang License: https://github.com/NomicFoundation/slang/blob/main/LICENSE"
        )?;

        writeln!(
            w,
            "// NAPI-RS License: https://github.com/napi-rs/napi-rs/blob/main/LICENSE"
        )?;

        writeln!(w)?;
        writeln!(w, "// @ts-nocheck")?;
        writeln!(w)?;
        writeln!(w, "{}", raw_file.read_to_string()?)?;

        codegen.write_file(destination_file, w)?;
    }

    return Ok(());
}

fn compile_root_package(node_binary: Option<&Path>) -> Result<()> {
    let package_dir = NapiResolver::package_dir();
    let output_dir = NapiResolver::npm_target_dir(&NapiPackageKind::Main);

    std::fs::create_dir_all(&output_dir)?;

    Command::new("tsc")
        .property("--project", package_dir.join("tsconfig.json").unwrap_str())
        .property("--outDir", output_dir.unwrap_str())
        .property("--declaration", "true")
        .property("--noEmit", "false")
        .run()?;

    for file_name in &["package.json", "CHANGELOG.md", "LICENSE", "README.md"] {
        let source = package_dir.join(file_name);
        let destination = output_dir.join(file_name);

        std::fs::copy(source, destination)?;
    }

    let generated_dir = NapiResolver::generated_dir();
    let output_dir = NapiResolver::generated_target_dir();

    std::fs::create_dir_all(&output_dir)?;

    for child in generated_dir.collect_children()? {
        let destination = output_dir.join(child.unwrap_name());

        std::fs::copy(child, destination)?;
    }

    if let Some(node_binary) = node_binary {
        let destination = output_dir.join(node_binary.unwrap_name());

        std::fs::copy(node_binary, destination)?;
    }

    return Ok(());
}

fn compile_platform_packages(node_binaries: &Vec<PathBuf>) -> Result<()> {
    for platform_dir in NapiResolver::platforms_dir().collect_children()? {
        let platform = platform_dir.unwrap_name();
        let package_kind = NapiPackageKind::Platform(platform.to_owned());
        let output_dir = NapiResolver::npm_target_dir(&package_kind);

        std::fs::create_dir_all(&output_dir)?;

        for file in platform_dir.collect_children()? {
            std::fs::copy(&file, output_dir.join(file.unwrap_name()))?;
        }

        let node_binary = node_binaries
            .iter()
            .find(|node_binary| node_binary.unwrap_name() == format!("index.{platform}.node"))
            .expect("Could not find node binary for platform.");

        std::fs::copy(node_binary, output_dir.join(node_binary.unwrap_name()))?;
    }

    return Ok(());
}
