// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::collections::BTreeMap;

use crate::cst::{Cursor, Node};
use crate::parser::ParseOutput;

#[derive(Clone)]
pub struct File {
    id: String,
    parse_output: ParseOutput,

    resolved_imports: BTreeMap<usize, String>,
}

impl File {
    pub(super) fn new(id: String, parse_output: ParseOutput) -> Self {
        Self {
            id,
            parse_output,

            resolved_imports: BTreeMap::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn tree(&self) -> &Node {
        self.parse_output.tree()
    }

    pub fn create_tree_cursor(&self) -> Cursor {
        self.parse_output.create_tree_cursor()
    }

    pub(super) fn resolve_import(&mut self, import_path: &Cursor, destination_file_id: String) {
        self.resolved_imports
            .insert(import_path.node().id(), destination_file_id);
    }

    pub(super) fn resolved_import(&self, import_path: &Cursor) -> Option<&String> {
        self.resolved_imports.get(&import_path.node().id())
    }
}
