use std::ops::Range;

use itertools::Itertools;
use slang_solidity_v2_cst::structured_cst::version_validator::{
    VersionConstraint, VersionValidationError,
};
use slang_solidity_v2_parser::ParserError;

use super::{Diagnostic, Severity};

impl Diagnostic for VersionValidationError {
    fn text_range(&self) -> Range<usize> {
        self.range.clone()
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn message(&self) -> String {
        match &self.specifier {
            VersionConstraint::From { from } => {
                format!("Not supported until version '{from}'.")
            }
            VersionConstraint::Till { till } => {
                format!("Not supported since version '{till}'.")
            }
            VersionConstraint::Range { from, till } => {
                format!("Only supported between versions '{from}' and '{till}'.")
            }
        }
    }
}

impl Diagnostic for ParserError {
    fn text_range(&self) -> Range<usize> {
        match self {
            ParserError::UnexpectedEof { offset, .. } => *offset..*offset,
            ParserError::UnexpectedTerminal { range, .. } => range.clone(),
            ParserError::ExtraTerminal { range, .. } => range.clone(),
        }
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn message(&self) -> String {
        match self {
            ParserError::UnexpectedEof { expected, .. } => {
                format!(
                    "Unexpected end of file. One of {expected_list} was expected",
                    expected_list = expected.iter().map(|e| format!("{e}")).join(", ")
                )
            }
            ParserError::UnexpectedTerminal {
                terminal, expected, ..
            } => {
                format!(
                    "Unexpected {terminal}. One of {expected_list} was expected",
                    expected_list = expected.iter().map(|e| format!("{e}")).join(", ")
                )
            }
            ParserError::ExtraTerminal { terminal, .. } => {
                format!("Unexpected {terminal}. End of file was expected")
            }
        }
    }
}
