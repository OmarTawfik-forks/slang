//! Example of using the bindings API.
//! Note that this API is experimental, and it will change in the future.
//! The final API will be able to compose a graph from multiple files, and resolve imports between them.

#![cfg(all(test, feature = "__experimental_bindings_api"))]

use semver::Version;

use crate::bindings;
use crate::kinds::{NonterminalKind, TerminalKind};
use crate::language::Language;

#[test]
fn test_bindings() {
    let version = Version::new(0, 8, 0);

    let source = "
      contract X {}
      contract Y is X {}
    ";

    let language = Language::new(version.clone()).unwrap();
    let output = language.parse(NonterminalKind::SourceUnit, source);
    assert!(output.is_valid());

    let mut identifiers = vec![];
    {
        let mut cursor = output.create_tree_cursor();
        while cursor.go_to_next_terminal_with_kind(TerminalKind::Identifier) {
            identifiers.push(cursor.clone());
        }
    }

    assert_eq!(identifiers.len(), 3);
    assert_eq!(identifiers[0].node().unparse(), "X"); // definition
    assert_eq!(identifiers[1].node().unparse(), "Y"); // definition
    assert_eq!(identifiers[2].node().unparse(), "X"); // reference

    let mut bindings = bindings::create(version.clone());
    bindings.add_file("input.sol", output.create_tree_cursor());

    let definitions: Vec<_> = bindings
        .all_definitions()
        .map(|h| h.get_cursor().unwrap())
        .collect();

    assert_eq!(definitions.len(), 2);
    assert_eq!(definitions[0].node().unparse(), "X"); // first contract
    assert_eq!(definitions[0].text_offset(), identifiers[0].text_offset());
    assert_eq!(definitions[1].node().unparse(), "Y"); // second contract
    assert_eq!(definitions[1].text_offset(), identifiers[1].text_offset());

    let references: Vec<_> = bindings
        .all_references()
        .map(|h| h.get_cursor().unwrap())
        .collect();

    assert_eq!(references.len(), 1);
    assert_eq!(references[0].node().unparse(), "X"); // inheritance specifier
    assert_eq!(references[0].text_offset(), identifiers[2].text_offset());
}
