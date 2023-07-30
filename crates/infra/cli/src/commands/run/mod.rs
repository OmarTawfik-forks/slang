use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::{cargo::CargoWorkspace, commands::Command, terminal::Terminal};

use crate::utils::ValueEnumExtensions;

#[derive(Clone, Debug, Parser)]
pub struct RunCommand {
    binary: RunBinary,

    #[clap(trailing_var_arg = true)]
    args: Vec<String>,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum RunBinary {
    /// The public 'slang_solidity' crate shapped to Cargo users.
    #[clap(name = "slang_solidity")]
    SlangSolidity,

    /// Runs smoke tests of the Solidity parser against source files from the Sanctuary repositories.
    #[clap(name = "solidity_testing_smoke")]
    SolidityTestingSmoke,
}

impl RunCommand {
    pub fn execute(&self) -> Result<()> {
        let crate_name = self.binary.clap_name();
        let crate_dir = CargoWorkspace::locate_source_crate(&crate_name);

        Terminal::separator(&crate_name);

        return Command::new("cargo")
            .arg("run")
            .property("--bin", &crate_name)
            .arg("--")
            .args(&self.args)
            // Execute in the crate dir, to make use of a local './target' dir if it exists:
            .current_dir(crate_dir)
            .run();
    }
}
