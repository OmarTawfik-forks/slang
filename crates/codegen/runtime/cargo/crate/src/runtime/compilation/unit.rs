// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::cell::OnceCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use semver::Version;

use crate::bindings::{create_with_resolver, Bindings, PathResolver};
use crate::compilation::File;

pub struct CompilationUnit {
    language_version: Version,
    files: BTreeMap<String, Rc<File>>,
    bindings: OnceCell<Bindings>,
}

impl CompilationUnit {
    pub(super) fn new(language_version: Version, files: BTreeMap<String, Rc<File>>) -> Self {
        Self {
            language_version,
            files,
            bindings: OnceCell::new(),
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

    fn bindings(&self) -> &Bindings {
        struct Resolver;

        impl PathResolver for Resolver {
            fn resolve_path(&self, context_path: &str, path_to_resolve: &str) -> Option<String> {
                files[context_path].resolved_import(&path_to_resolve)
            }
        }

        let bindings = create_with_resolver(language_version, resolver);

        for file in files.values() {
            bindings.add_file(file);
        }

        self.bindings
            .get_or_init(|| create_with_resolver(self.language_version, PathResolver::default()))
    }
}
