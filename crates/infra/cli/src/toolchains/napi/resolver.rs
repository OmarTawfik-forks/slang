use std::path::PathBuf;

use infra_utils::cargo::CargoWorkspace;

use crate::toolchains::napi::cli::BuildTarget;

pub enum NapiPackageKind {
    Main,
    Platform(String),
}

pub struct NapiResolver;

impl NapiResolver {
    pub fn crate_dir() -> &'static PathBuf {
        return CargoWorkspace::locate_source_crate("solidity_npm_crate");
    }

    pub fn package_dir() -> &'static PathBuf {
        return CargoWorkspace::locate_source_crate("solidity_npm_package");
    }

    pub fn generated_dir() -> PathBuf {
        return Self::package_dir().join("src/generated");
    }

    pub fn platforms_dir() -> PathBuf {
        return Self::package_dir().join("platforms");
    }

    pub fn napi_target_dir(target: &BuildTarget) -> PathBuf {
        return Self::package_dir().join("target/napi").join(match target {
            BuildTarget::Debug => "debug",
            BuildTarget::ReleaseTarget(target) => &target,
        });
    }

    pub fn npm_target_dir(kind: &NapiPackageKind) -> PathBuf {
        return Self::package_dir().join("target/npm").join(match kind {
            NapiPackageKind::Main => "main",
            NapiPackageKind::Platform(platform) => platform,
        });
    }

    pub fn generated_target_dir() -> PathBuf {
        return Self::npm_target_dir(&NapiPackageKind::Main).join("generated");
    }
}
