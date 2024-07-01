use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, process};

use anyhow::{Context, Result};
use derive_new::new;
use serde::Serialize;

use crate::commands::Command;
use crate::paths::PathExtensions;

const EXTRA_MEASUREMENTS_DIR_ENV_VAR: &str = "BENCHER_EXTRA_MEASUREMENTS_DIR";

pub type Report = HashMap<String, Benchmark>;

pub type Benchmark = HashMap<String, Measurement>;

#[derive(new, Serialize)]
pub struct Measurement {
    pub value: f64,
}

pub struct MeasurementsDir {
    path: PathBuf,
}

impl MeasurementsDir {
    pub fn create_new() -> Result<Self> {
        let path = env::temp_dir()
            .join(EXTRA_MEASUREMENTS_DIR_ENV_VAR)
            .join(process::id().to_string());

        std::fs::create_dir_all(&path)?;

        Ok(Self { path })
    }

    pub fn load_existing() -> Result<Self> {
        let path = env::var(EXTRA_MEASUREMENTS_DIR_ENV_VAR)
            .context("Measurements directory not created yet.")?
            .parse()?;

        Ok(Self { path })
    }

    pub fn add_to_command(&self, command: Command) -> Command {
        command.env(EXTRA_MEASUREMENTS_DIR_ENV_VAR, self.path.unwrap_str())
    }

    pub fn collect_reports(&self) -> Result<Vec<PathBuf>> {
        self.path.collect_children()
    }

    pub fn add_report(&self, report: &Report) -> Result<()> {
        let json = serde_json::to_string_pretty(report)?;

        self.path
            .join(format!("{0}.json", self.collect_reports()?.len()))
            .write_string(json)
    }
}
