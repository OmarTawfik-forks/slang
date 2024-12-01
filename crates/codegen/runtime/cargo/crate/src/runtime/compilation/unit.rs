// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::collections::BTreeMap;
use std::rc::Rc;

use semver::Version;

use crate::compilation::File;

pub struct CompilationUnit {
    language_version: Version,

    files: BTreeMap<String, Rc<File>>,
}

impl CompilationUnit {
    pub(super) fn new(language_version: Version, files: BTreeMap<String, Rc<File>>) -> Self {
        Self {
            language_version,
            files,
        }
    }

    pub fn language_version(&self) -> &Version {
        &self.language_version
    }

    pub fn files(&self) -> Vec<Rc<File>> {
        self.files.values().cloned().collect()
    }

    pub fn file(&self, id: &str) -> Option<Rc<File>> {
        self.files.get(id).cloned()
    }
}
