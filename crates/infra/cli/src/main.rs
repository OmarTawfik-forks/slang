mod commands;
mod extensions;
mod toolchains;

use anyhow::Result;
use clap::Parser;

use crate::commands::AppController;

fn main() -> Result<()> {
    return AppController::parse().execute();
}
