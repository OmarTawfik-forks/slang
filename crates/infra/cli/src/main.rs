mod commands;
mod toolchains;
mod utils;

use anyhow::Result;
use clap::Parser;

use crate::commands::CLI;

fn main() -> Result<()> {
    return CLI::parse().execute();
}
