use std::collections::{BTreeMap, BTreeSet};
use std::mem::discriminant;

use language_v2_definition::model::{
    FragmentItem, Item, KeywordItem, KeywordValue, Language, Scanner, TokenItem, TriviaItem,
    VersionSpecifier,
};

use crate::lexer::{Lexeme, LexerModel, LexicalContext, Subpattern};

pub struct LexerModelBuilder;

impl LexerModelBuilder {
    pub fn build(language: &Language) -> LexerModel {
        let contexts = Self::collect_contexts(language);
        let lexeme_kinds = Self::collect_lexeme_kinds(&contexts);

        LexerModel {
            contexts,
            lexeme_kinds,
        }
    }

    fn collect_contexts(language: &Language) -> Vec<LexicalContext> {
        let mut non_trivia = BTreeMap::<String, LexicalContext>::new();
        let mut common_trivia = Vec::<Lexeme>::new();

        for topic in language.topics() {
            let context = non_trivia
                .entry(topic.lexical_context.to_string())
                .or_insert_with(|| LexicalContext {
                    name: topic.lexical_context.to_string(),
                    lexemes: Vec::new(),
                    subpatterns: Vec::new(),
                });

            for item in &topic.items {
                match item {
                    Item::Struct { .. } => {}
                    Item::Enum { .. } => {}
                    Item::Repeated { .. } => {}
                    Item::Separated { .. } => {}
                    Item::Precedence { .. } => {}

                    Item::Trivia { item } => {
                        common_trivia.push(Self::convert_trivia(item));
                    }
                    Item::Keyword { item } => {
                        context.lexemes.extend(Self::convert_keyword(item));
                    }
                    Item::Token { item } => {
                        context.lexemes.extend(Self::convert_token(item));
                    }
                    Item::Fragment { item } => {
                        context.subpatterns.push(Self::convert_fragment(item));
                    }
                }
            }
        }

        non_trivia
            .into_values()
            .map(|mut context| {
                // Insert common trivia into each context:
                context.lexemes.extend(common_trivia.iter().cloned());
                context
            })
            .collect()
    }

    fn collect_lexeme_kinds(contexts: &Vec<LexicalContext>) -> BTreeSet<String> {
        let mut kinds = BTreeSet::new();

        for context in contexts {
            for lexeme in &context.lexemes {
                match lexeme {
                    Lexeme::Trivia { kind, .. } => {
                        kinds.insert(kind.clone());
                    }
                    Lexeme::Token { kind, .. } => {
                        kinds.insert(kind.clone());
                    }
                    Lexeme::Keyword { kind, reserved, .. } => {
                        if match reserved {
                            None => true,
                            Some(VersionSpecifier::Never) => false,
                            Some(VersionSpecifier::From { .. }) => true,
                            Some(VersionSpecifier::Till { .. }) => true,
                            Some(VersionSpecifier::Range { .. }) => true,
                        } {
                            kinds.insert(format!("{kind}_Reserved"));
                        }

                        if reserved.is_some() {
                            kinds.insert(format!("{kind}_Unreserved"));
                        }
                    }
                }
            }
        }

        kinds
    }

    fn convert_trivia(item: &TriviaItem) -> Lexeme {
        Lexeme::Trivia {
            kind: item.name.to_string(),
            regex: Self::convert_scanner(&item.scanner),
        }
    }

    fn convert_keyword(item: &KeywordItem) -> impl Iterator<Item = Lexeme> + '_ {
        item.definitions.iter().map(|def| Lexeme::Keyword {
            kind: item.name.to_string(),
            regex: Self::convert_keyword_value(&def.value),
            reserved: def.reserved.clone(),
        })
    }

    fn convert_token(item: &TokenItem) -> impl Iterator<Item = Lexeme> + '_ {
        item.definitions.iter().map(|def| Lexeme::Token {
            kind: item.name.to_string(),
            regex: Self::convert_scanner(&def.scanner),
        })
    }
    fn convert_fragment(item: &FragmentItem) -> Subpattern {
        Subpattern {
            name: item.name.to_string(),
            regex: Self::convert_scanner(&item.scanner),
        }
    }

    fn convert_keyword_value(parent: &KeywordValue) -> String {
        match parent {
            KeywordValue::Sequence { values } => values
                .iter()
                .map(|value| Self::convert_child_keyword_value(parent, value))
                .collect(),

            KeywordValue::Choice { values } => values
                .iter()
                .map(|value| Self::convert_child_keyword_value(parent, value))
                .collect::<Vec<_>>()
                .join("|"),

            KeywordValue::Optional { value } => {
                format!("{}?", Self::convert_child_keyword_value(parent, value))
            }

            KeywordValue::Atom { atom } => Self::convert_atom(atom),
        }
    }

    fn convert_child_keyword_value(parent: &KeywordValue, child: &KeywordValue) -> String {
        if discriminant(parent) != discriminant(child)
            && Self::keyword_value_precedence(child) <= Self::keyword_value_precedence(parent)
        {
            format!("({})", Self::convert_keyword_value(child))
        } else {
            Self::convert_keyword_value(child)
        }
    }

    fn keyword_value_precedence(value: &KeywordValue) -> u8 {
        match value {
            // Binary:
            KeywordValue::Sequence { .. } | KeywordValue::Choice { .. } => 1,
            // Postfix:
            KeywordValue::Optional { .. } => 2,
            // Primary:
            KeywordValue::Atom { .. } => 3,
        }
    }

    fn convert_scanner(parent: &Scanner) -> String {
        match parent {
            Scanner::Sequence { scanners } => scanners
                .iter()
                .map(|scanner| Self::convert_child_scanner(parent, scanner))
                .collect(),

            Scanner::Choice { scanners } => scanners
                .iter()
                .map(|scanner| Self::convert_child_scanner(parent, scanner))
                .collect::<Vec<_>>()
                .join("|"),

            Scanner::Optional { scanner } => {
                format!("{}?", Self::convert_child_scanner(parent, scanner))
            }

            Scanner::ZeroOrMore { scanner } => {
                format!("{}*", Self::convert_child_scanner(parent, scanner))
            }

            Scanner::OneOrMore { scanner } => {
                format!("{}+", Self::convert_child_scanner(parent, scanner))
            }

            Scanner::Not { chars } => {
                format!(
                    "[^{}]",
                    chars
                        .iter()
                        .map(|c| Self::convert_char(*c))
                        .collect::<String>()
                )
            }

            Scanner::Range {
                inclusive_start,
                inclusive_end,
            } => format!(
                "[{}-{}]",
                Self::convert_char(*inclusive_start),
                Self::convert_char(*inclusive_end)
            ),

            Scanner::Atom { atom } => Self::convert_atom(atom),

            Scanner::Fragment { reference } => {
                format!("(?&{reference})")
            }
        }
    }

    fn convert_child_scanner(parent: &Scanner, child: &Scanner) -> String {
        if discriminant(parent) != discriminant(child)
            && Self::scanner_precedence(child) <= Self::scanner_precedence(parent)
        {
            format!("({})", Self::convert_scanner(child))
        } else {
            Self::convert_scanner(child)
        }
    }

    fn scanner_precedence(scanner: &Scanner) -> u8 {
        match scanner {
            // Binary:
            Scanner::Sequence { .. } | Scanner::Choice { .. } => 1,
            // Postfix:
            Scanner::Optional { .. } | Scanner::ZeroOrMore { .. } | Scanner::OneOrMore { .. } => 2,
            // Primary:
            Scanner::Not { .. }
            | Scanner::Range { .. }
            | Scanner::Atom { .. }
            | Scanner::Fragment { .. } => 3,
        }
    }

    fn convert_atom(atom: &str) -> String {
        atom.chars().map(Self::convert_char).collect()
    }

    fn convert_char(c: char) -> String {
        match c {
            // use alphanumerics as-is:
            'a'..='z' | 'A'..='Z' | '0'..='9' => c.to_string(),
            // use punctuation as-is:
            ' ' | '_' | '-' | ',' | ';' | ':' | '!' | '"' | '/' | '\'' | '&' | '%' | '<' | '='
            | '>' | '~' => c.to_string(),
            // escape regex control characters:
            '^' | '$' | '|' | '?' | '.' | '(' | ')' | '[' | ']' | '{' | '}' | '*' | '\\' | '+' => {
                format!("\\{c}")
            }
            // escape breaks:
            '\t' => "\\t".to_string(),
            '\n' => "\\n".to_string(),
            '\r' => "\\r".to_string(),
            _ => {
                panic!("Unsupported character in lexer char: '{c}'");
            }
        }
    }
}
