use anyhow::{bail, Result};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;

pub fn setup_perf() -> Result<()> {
    match Command::new("valgrind").flag("--version").evaluate() {
        Ok(output) if output.starts_with("valgrind-") => {
            // Valgrind is available
        }
        other => {
            bail!("valgrind needs to be installed to run perf tests:\n{other:?}");
        }
    };

    CargoWorkspace::install_binary("iai-callgrind-runner")?;

    // Currently `bencher` backend API is under development/unstable,
    // so they recommend always running with the latest CLI version from 'main' until it is stabilized.
    // Otherwise, we would have pinned it in `Cargo.toml` and used `CargoWorkspace::install_binary()` instead.
    Command::new("cargo")
        .args(["install", "bencher_cli"])
        .property("--git", "https://github.com/bencherdev/bencher")
        .property("--branch", "main")
        .run()?;

    Ok(())
}
