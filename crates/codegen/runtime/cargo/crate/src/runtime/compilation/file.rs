// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::collections::BTreeMap;

use crate::cst::{Cursor, Node};
use crate::parser::ParseOutput;

#[derive(Clone)]
pub struct File {
    id: String,
    parse_output: ParseOutput,

    imports_map: BTreeMap<String, String>,
}

impl File {
    pub(super) fn new(id: String, parse_output: ParseOutput) -> Self {
        Self {
            id,
            parse_output,

            imports_map: BTreeMap::new(),
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

    pub(super) fn add_import(&mut self, import_string: String, imported_file_id: String) {
        self.imports_map.insert(import_string, imported_file_id);
    }
}
