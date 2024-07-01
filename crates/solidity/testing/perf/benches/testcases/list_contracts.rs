use slang_solidity::kinds::{NonterminalKind, TerminalKind};
use slang_solidity::language::Language;
use slang_solidity::text_index::TextIndex;

use crate::common::{SOLC_VERSION, SOURCES};

pub fn run() {
    let language = Language::new(SOLC_VERSION).unwrap();

    let mut contract_names = vec![];

    for source in SOURCES {
        let parse_output = language.parse(Language::ROOT_KIND, source);

        assert!(
            parse_output.is_valid(),
            "Found parse errors: {0:?}",
            parse_output.errors(),
        );

        let mut cursor = parse_output.tree().cursor_with_offset(TextIndex::ZERO);

        while cursor.go_to_next_nonterminal_with_kind(NonterminalKind::ContractDefinition) {
            for contract_name in cursor
                .node()
                .children()
                .iter()
                .filter_map(|edge| edge.node.as_terminal_with_kind(TerminalKind::Identifier))
            {
                contract_names.push(contract_name.text.clone());
            }
        }
    }

    assert_eq!(contract_names, ["ERC20", "ERC721", "Governor"]);
}
