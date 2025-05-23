use codegen_language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::model::{Identifier, Scanner, VersionSpecifier};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct KeywordItem {
    pub name: Identifier,
    pub identifier: Identifier,

    pub definitions: Vec<KeywordDefinition>,
}

impl KeywordItem {
    pub fn is_unique(&self) -> bool {
        self.definitions
            .iter()
            .all(|definition| definition.value.collect_variations().len() == 1)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct KeywordDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VersionSpecifier>,

    /// When the keyword is reserved, i.e. can't be used in other position (e.g. as a name)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved: Option<VersionSpecifier>,

    /// Underlying keyword scanner (i.e. identifier scanner)
    pub value: KeywordValue,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub enum KeywordValue {
    Sequence { values: Vec<KeywordValue> },
    Optional { value: Box<KeywordValue> },
    Choice { values: Vec<KeywordValue> },
    Atom { atom: String },
}

impl From<KeywordValue> for Scanner {
    fn from(value: KeywordValue) -> Scanner {
        match value {
            KeywordValue::Optional { value } => Scanner::Optional {
                scanner: Box::new((*value).into()),
            },
            KeywordValue::Sequence { values } => Scanner::Sequence {
                scanners: values.into_iter().map(Into::into).collect(),
            },
            KeywordValue::Atom { atom } => Scanner::Atom { atom },
            KeywordValue::Choice { values } => Scanner::Choice {
                scanners: values.into_iter().map(Into::into).collect(),
            },
        }
    }
}

impl KeywordValue {
    /// Collects all possible variations generated by this value.
    pub fn collect_variations(&self) -> Vec<String> {
        match self {
            KeywordValue::Atom { atom } => vec![atom.to_owned()],
            KeywordValue::Optional { value } => {
                let mut results = value.collect_variations();
                results.insert(0, String::new());
                results
            }
            KeywordValue::Choice { values } => {
                values.iter().flat_map(Self::collect_variations).collect()
            }

            KeywordValue::Sequence { values } => {
                let matrix = values.iter().map(Self::collect_variations).collect_vec();

                let results_len = matrix.iter().map(Vec::len).product();
                let mut results = (0..results_len).map(|_| String::new()).collect_vec();

                let mut span = results_len;

                for variations in matrix {
                    span /= variations.len();

                    for j in 0..results_len {
                        results[j].push_str(&variations[j / span % variations.len()]);
                    }
                }

                results
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom() {
        let value = KeywordValue::Atom { atom: "foo".into() };

        assert_eq!(value.collect_variations(), vec!["foo"]);
    }

    #[test]
    fn test_optional() {
        let value = KeywordValue::Optional {
            value: KeywordValue::Atom { atom: "foo".into() }.into(),
        };

        assert_eq!(value.collect_variations(), vec!["", "foo"]);
    }

    #[test]
    fn test_choice() {
        let value = KeywordValue::Choice {
            values: vec![
                KeywordValue::Atom { atom: "foo".into() },
                KeywordValue::Atom { atom: "bar".into() },
            ],
        };

        assert_eq!(value.collect_variations(), vec!["foo", "bar"]);
    }

    #[test]
    fn test_sequence() {
        let value = KeywordValue::Sequence {
            values: vec![
                KeywordValue::Atom { atom: "foo".into() },
                KeywordValue::Atom { atom: "bar".into() },
            ],
        };

        assert_eq!(value.collect_variations(), vec!["foobar"]);
    }

    #[test]
    fn test_all() {
        let value = KeywordValue::Sequence {
            values: vec![
                KeywordValue::Atom { atom: "foo".into() },
                KeywordValue::Optional {
                    value: KeywordValue::Sequence {
                        values: vec![
                            KeywordValue::Atom { atom: "_".into() },
                            KeywordValue::Choice {
                                values: vec![
                                    KeywordValue::Atom { atom: "1".into() },
                                    KeywordValue::Atom { atom: "2".into() },
                                    KeywordValue::Atom { atom: "3".into() },
                                    KeywordValue::Atom { atom: "4".into() },
                                    KeywordValue::Atom { atom: "5".into() },
                                ],
                            },
                        ],
                    }
                    .into(),
                },
            ],
        };

        assert_eq!(
            value.collect_variations(),
            vec!["foo", "foo_1", "foo_2", "foo_3", "foo_4", "foo_5",]
        );
    }
}
