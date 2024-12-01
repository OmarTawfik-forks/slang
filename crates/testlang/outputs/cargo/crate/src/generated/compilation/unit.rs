// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::collections::BTreeMap;

use crate::compilation::File;

pub struct CompilationUnit {
    files: BTreeMap<String, File>,
}

impl CompilationUnit {
    pub(super) fn new(files: BTreeMap<String, File>) -> Self {
        Self { files }
    }

    pub fn files(&self) -> &BTreeMap<String, File> {
        &self.files
    }
}
