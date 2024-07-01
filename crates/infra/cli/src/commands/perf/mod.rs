use anyhow::Result;
use clap::Parser;

#[derive(Clone, Debug, Parser)]
pub struct PerfController;

impl PerfController {
    pub fn execute(&self) -> Result<()> {
        /*
         - initialize measurements dir, and attach it to command below
         - bencher run --project "470c4135-1400-4caf-9b15-676f4051a8e2" --adapter rust_iai_callgrind "cargo bench --package solidity_testing_perf --bench iai"
         - report extra measurements (list them on stdout)
         - then read and list flamegraphs on stdout
        */

        Ok(())
    }
}
