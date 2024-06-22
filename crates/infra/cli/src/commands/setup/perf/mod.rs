use anyhow::Result;
use infra_utils::commands::Command;

pub fn setup_perf() -> Result<()> {
    // Currently `bencher` backend API is under development/unstable,
    // so they recommend always running with the latest CLI version from 'main' until it is stabilized.
    Command::new("cargo")
        .args(["install", "bencher_cli"])
        .property("--git", "https://github.com/bencherdev/bencher")
        .property("--branch", "main")
        .run()
}
