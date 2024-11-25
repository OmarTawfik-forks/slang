use anyhow::{ensure, Result};
use clap::Parser;
use infra_utils::cargo::{CargoWorkspace, UserFacingCrate};
use infra_utils::commands::Command;
use strum::IntoEnumIterator;

use crate::utils::DryRun;

#[derive(Clone, Debug, Parser)]
pub struct CargoController {
    #[command(flatten)]
    dry_run: DryRun,
}

impl CargoController {
    pub fn execute(&self) -> Result<()> {
        ensure!(
            false,
            "__SLANG_CARGO_PUBLISH_TEMPORARILY_DISABLED__ (keep in sync)"
        );

        for crate_name in UserFacingCrate::iter() {
            let crate_name = crate_name.to_string();

            if needs_publishing(&crate_name) {
                run_cargo_publish(&crate_name, self.dry_run);
            }
        }

        Ok(())
    }
}

fn needs_publishing(crate_name: &str) -> bool {
    let Ok(published_version) = CargoWorkspace::published_version(crate_name) else {
        println!("No published version found for crate {crate_name}.");
        return true;
    };

    println!("Published version of {crate_name}: {published_version}");

    let local_version = CargoWorkspace::local_version().unwrap();
    println!("Local version: {local_version}");

    if local_version != published_version {
        println!("Published version of {crate_name} is out of date.");
        return true;
    }

    println!("Published {crate_name} is up to date.");
    false
}

fn run_cargo_publish(crate_name: &str, dry_run: DryRun) {
    let mut command = Command::new("cargo")
        .arg("publish")
        .property("--package", crate_name)
        .flag("--all-features");

    if dry_run.get() {
        command = command.flag("--dry-run");
    }

    command.run();
}
