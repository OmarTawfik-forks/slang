use std::path::Path;

use anyhow::{bail, Context, Result};

use crate::{codegen::common::formatting::format_source_file, paths::PathExtensions};

pub fn read_file(file_path: &Path) -> Result<String> {
    return std::fs::read_to_string(file_path)
        .with_context(|| format!("Cannot read source file: {file_path:?}"));
}

pub fn delete_file(file_path: &Path) -> Result<()> {
    return std::fs::remove_file(&file_path)
        .with_context(|| format!("Failed to delete source file: {file_path:?}"));
}

pub fn write_file(file_path: &Path, contents: &str) -> Result<()> {
    std::fs::create_dir_all(file_path.unwrap_parent())
        .with_context(|| format!("Cannot create parent directory of: {file_path:?}"))?;

    let formatted = format_source_file(file_path, &contents)?;

    // To respect Cargo incrementability, don't touch the file if it is already up to date.
    if file_path.exists() {
        let existing_contents = std::fs::read_to_string(file_path)
            .with_context(|| format!("Cannot read existing file: {file_path:?}"))?;

        if formatted == existing_contents {
            return Ok(());
        }
    }

    return std::fs::write(file_path, formatted)
        .with_context(|| format!("Cannot write generated file: {file_path:?}"));
}

pub fn verify_file(file_path: &Path, contents: &str) -> Result<()> {
    let formatted = format_source_file(file_path, &contents)?;

    if !file_path.exists() {
        bail!("Generated file does not exist: {file_path:?}");
    }

    let existing_contents = std::fs::read_to_string(file_path)
        .with_context(|| format!("Cannot read existing file: {file_path:?}"))?;

    similar_asserts::assert_eq!(
        formatted,
        existing_contents,
        "Generated file is out of date: {file_path:?}"
    );

    return Ok(());
}
