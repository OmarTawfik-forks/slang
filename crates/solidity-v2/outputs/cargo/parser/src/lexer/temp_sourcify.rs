use semver::Version;
use slang_solidity::cst::{Cursor, Node, NonterminalKind, TextRange, TextRangeExtensions};
use slang_solidity::diagnostic::{Diagnostic, Severity};

use crate::lexer::contexts::ContextKind;
use crate::lexer::definition::Lexer;

pub struct Error {
    pub message: String,
    pub text_range: TextRange,
}

impl Diagnostic for Error {
    fn text_range(&self) -> TextRange {
        self.text_range.clone()
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn message(&self) -> String {
        self.message.clone()
    }
}

pub struct Comparator<'source> {
    lexer: Lexer<'source>,
    cursor: Cursor,
    errors: Vec<Error>,
}

impl<'source> Comparator<'source> {
    pub fn compare_with_v1_output(
        language_version: Version,
        source: &'source str,
        root_cursor: Cursor,
    ) -> Vec<Error> {
        let mut instance = Self {
            lexer: Lexer::new(ContextKind::Solidity, source, language_version),
            cursor: root_cursor,
            errors: vec![],
        };

        instance.check_node();

        while let Some(lexeme) = instance.lexer.next_lexeme() {
            instance.add_error(format!(
                "Lexer v2 produced an extra lexeme at the end of input: {lexeme:?}",
            ));
        }

        instance.errors
    }

    fn check_node(&mut self) {
        match self.cursor.node() {
            Node::Nonterminal(nonterminal) => {
                match nonterminal.kind {
                    NonterminalKind::Pragma => {
                        self.lexer.switch_context(ContextKind::Pragma);
                    }
                    NonterminalKind::AssemblyStatement => {
                        self.lexer.switch_context(ContextKind::Yul);
                    }
                    _ => {}
                }

                if self.cursor.go_to_first_child() {
                    self.check_node();

                    while self.cursor.go_to_next_sibling() {
                        self.check_node();
                    }

                    self.cursor.go_to_parent();
                }

                match nonterminal.kind {
                    NonterminalKind::Pragma | NonterminalKind::AssemblyStatement => {
                        self.lexer.switch_context(ContextKind::Solidity);
                    }
                    _ => {}
                }
            }

            Node::Terminal(terminal) => match self.lexer.next_lexeme() {
                None => {
                    self.add_error(
                        "V2 lexer stopped producing lexemes, but v1 tree still has more terminals"
                            .to_string(),
                    );
                }
                Some(lexeme) => {
                    let v1_range = self.cursor.text_range().utf8();
                    let v2_range = lexeme.range;

                    let v1_kind = terminal.kind.as_ref();
                    let v2_kind: &'static str = lexeme.kind.into();
                    let v2_kind = v2_kind.strip_suffix("_Reserved").unwrap_or(v2_kind);
                    let v2_kind = v2_kind.strip_suffix("_Unreserved").unwrap_or(v2_kind);

                    let v1_output = format!("{v1_kind:?} @ {v1_range:?}");
                    let v2_output = format!("{v2_kind:?} @ {v2_range:?}");

                    if v1_output != v2_output {
                        self.add_error(format!("V2 lexer produced a lexeme '{v2_output}', but v1 tree has terminal '{v1_output}'"));
                    }
                }
            },
        }
    }

    fn add_error(&mut self, message: String) {
        self.errors.push(Error {
            message,
            text_range: self.cursor.text_range().clone(),
        });
    }
}
