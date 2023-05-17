pub mod manifest;
pub mod parser;
pub mod precedence_parser;
pub mod production;
pub mod scanner;
// pub mod validators;
pub mod yaml;

use std::{path::PathBuf, rc::Rc};

use yaml::cst::Node;

#[derive(Clone, Debug)]
pub struct SourceLocation {
    pub path: Rc<PathBuf>,
    pub node: Rc<Node>,
}
