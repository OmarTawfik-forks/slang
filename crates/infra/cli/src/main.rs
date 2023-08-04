mod commands;
mod toolchains;
mod utils;

use anyhow::Result;
use clap::Parser;

use crate::commands::AppController;

fn main() -> Result<()> {
    return AppController::parse().execute();
}
