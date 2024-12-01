// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::collections::{BTreeMap, BTreeSet};

use semver::Version;

use crate::compilation::{CompilationUnit, File};
use crate::cst::{Cursor, Query};
use crate::parser::{Parser, ParserInitializationError};

pub struct CompilationBuilder {
    parser: Parser,
    files: BTreeMap<String, File>,
}

#[derive(thiserror::Error, Debug)]
pub enum CompilationInitializationError {
    #[error("Unsupported language version '{0}'.")]
    UnsupportedLanguageVersion(Version),
}

impl CompilationBuilder {
    pub fn new(version: Version) -> Result<Self, CompilationInitializationError> {
        let parser = match Parser::create(version) {
            Ok(parser) => parser,
            Err(ParserInitializationError::UnsupportedLanguageVersion(version)) => {
                return Err(CompilationInitializationError::UnsupportedLanguageVersion(
                    version,
                ))
            }
        };

        Ok(Self {
            parser,
            files: BTreeMap::new(),
        })
    }

    pub fn add_file(
        &mut self,
        id: String,
        contents: &str,
    ) -> Result<AddFileResponse, AddFileError> {
        let parse_output = self.parser.parse(Parser::ROOT_KIND, contents);

        let import_strings =
            extract_imports(parse_output.create_tree_cursor()).map_err(AddFileError::Internal)?;

        let file = File::new(id.clone(), parse_output);
        self.files.insert(id, file);

        Ok(AddFileResponse { import_strings })
    }

    pub fn add_import(
        &mut self,
        file_id: &str,
        import_string: String,
        imported_file_id: String,
    ) -> Result<(), AddImportError> {
        self.files
            .get_mut(file_id)
            .ok_or_else(|| AddImportError::FileNotFound(file_id.to_owned()))?
            .add_import(import_string, imported_file_id);

        Ok(())
    }

    pub fn build(&self) -> CompilationUnit {
        CompilationUnit::new(self.files.clone())
    }
}

pub struct AddFileResponse {
    pub import_strings: BTreeSet<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum AddFileError {
    #[error("Internal Error: {0}")]
    Internal(String),
}

#[derive(thiserror::Error, Debug)]
pub enum AddImportError {
    #[error("File not found: '{0}'.")]
    FileNotFound(String),
}

fn extract_imports(cursor: Cursor) -> Result<BTreeSet<String>, String> {
    let mut import_strings = BTreeSet::new();

    for query_match in cursor.query(vec![
        Query::parse(
            "[PathImport
                path: [StringLiteral
                    @value ([DoubleQuotedStringLiteral] | [SingleQuotedStringLiteral])
                ]
            ]",
        )
        .map_err(|e| e.to_string())?,
        Query::parse(
            "[NamedImport
                path: [StringLiteral
                    @value ([DoubleQuotedStringLiteral] | [SingleQuotedStringLiteral])
                ]
            ]",
        )
        .map_err(|e| e.to_string())?,
        Query::parse(
            "[ImportDeconstruction
                path: [StringLiteral
                    @value ([DoubleQuotedStringLiteral] | [SingleQuotedStringLiteral])
                ]
            ]",
        )
        .map_err(|e| e.to_string())?,
    ]) {
        for (name, _, cursors) in query_match.captures() {
            if name != "value" {
                return Err(format!("Unexpected capture name: {name}"));
            }

            for cursor in cursors {
                let literal_value = cursor.node().unparse();

                import_strings.insert(
                    literal_value
                        .strip_prefix(|c| matches!(c, '"' | '\''))
                        .ok_or_else(|| format!("Missing starting quote: `{literal_value}`."))?
                        .strip_suffix(|c| matches!(c, '"' | '\''))
                        .ok_or_else(|| format!("Missing ending quote: `{literal_value}`."))?
                        .to_owned(),
                );
            }
        }
    }

    Ok(import_strings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::LanguageFacts;

    #[test]
    pub fn path_import() -> Result<(), String> {
        run(
            r#"
                import "foo-double";
                import "bar-double" as bar;

                import 'foo-single';
                import 'bar-single' as bar;

            "#,
            &["foo-double", "bar-double", "foo-single", "bar-single"],
        )
    }

    #[test]
    pub fn named_import() -> Result<(), String> {
        run(
            r#"
                import * as foo from "foo-double";

                import * as foo from 'foo-single';
            "#,
            &["foo-double", "foo-single"],
        )
    }

    #[test]
    pub fn import_deconstruction() -> Result<(), String> {
        run(
            r#"
                import {a, b} from "foo-double";

                import {a, b} from 'foo-single';
            "#,
            &["foo-double", "foo-single"],
        )
    }

    fn run(source: &str, expected: &[&str]) -> Result<(), String> {
        if LanguageFacts::NAME != "Solidity" {
            return Ok(()); // This API is Solidity-specific
        }

        let parser = Parser::create(Version::new(0, 8, 0)).unwrap();
        let parse_output = parser.parse(Parser::ROOT_KIND, source);

        let actual = extract_imports(parse_output.create_tree_cursor())?;
        let expected = expected.iter().map(|v| (*v).to_string()).collect();

        assert_eq!(actual, expected);

        Ok(())
    }
}
