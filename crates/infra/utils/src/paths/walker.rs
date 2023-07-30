use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use anyhow::{ensure, Context, Result};
use globwalk::{FileType, GlobWalkerBuilder};
use itertools::Itertools;

use crate::paths::{PathExtensions, PrivatePathExtensions};

pub struct FileWalker {
    root_dir: PathBuf,
}

impl FileWalker {
    pub fn from_repo_root() -> Self {
        return Self::from_directory(Path::repo_root());
    }

    pub fn from_directory(root_dir: impl Into<PathBuf>) -> Self {
        return Self {
            root_dir: root_dir.into(),
        };
    }

    pub fn find_all(&self) -> Result<impl IntoIterator<Item = PathBuf>> {
        return self.find(["**/*"]);
    }

    pub fn find<P>(&self, patterns: impl AsRef<[P]>) -> Result<impl IntoIterator<Item = PathBuf>>
    where
        P: AsRef<str> + Debug,
    {
        let mut patterns = patterns
            .as_ref()
            .into_iter()
            .map(|pattern| pattern.as_ref())
            .collect_vec();

        ensure!(!patterns.is_empty(), "No patterns were provided.");

        let default_patterns = default_ignore_patterns()?;

        for pattern in &default_patterns {
            patterns.push(pattern.as_ref());
        }

        let builder = GlobWalkerBuilder::from_patterns(&self.root_dir, &patterns)
            .case_insensitive(false) // ensure case sensitive search
            .contents_first(false) // produce each directory first before its children
            .file_type(FileType::FILE) // only files (skip directories and symlinks)
            .follow_links(false)
            .min_depth(0)
            .max_depth(usize::MAX)
            .sort_by(|a, b| a.file_name().cmp(b.file_name())); // produce files in sorted alphabetical order

        let walker = builder
            .build()
            .with_context(|| format!("Failed to build glob walker with patterns: {patterns:#?}"))?
            .map(|entry| entry.unwrap().into_path());

        return Ok(walker);
    }
}

fn default_ignore_patterns() -> Result<Vec<String>> {
    let ignore_file = Path::repo_path(".gitignore");

    return std::fs::read_to_string(&ignore_file)
        .with_context(|| format!("Failed to ignore file: {ignore_file:?}"))?
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            assert_eq!(
                line,
                line.trim(),
                "Line contains something other than a pattern.",
            );

            return Ok(format!("!{line}")); // invert pattern to exclude instead:
        })
        .collect();
}
