use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::Result;
use codegen_language_definition::model::{
    BuiltIn, EnumItem, Field, Identifier, Item, KeywordItem, Language, PrecedenceItem, TokenItem,
    VersionSpecifier,
};
use indexmap::IndexMap;
use itertools::Itertools;
use semver::Version;
use serde::Serialize;
use solidity_language::SolidityDefinition;
use strum_macros::Display;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Serialize)]
struct Change {
    version: Version,
    area: Area,
    direction: Direction,
    description: String,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Display)]
enum Direction {
    ForwardsIncompatible,
    BackwardsIncompatible,
}

impl Display for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "- {version:6} | {area:10} | {direction:25} | {description}",
            version = self.version,
            area = self.area,
            direction = self.direction,
            description = self.description,
        )
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Display)]
enum Area {
    Nodes,
    Children,
    Keywords,
    Builtins,
    Bindings,
}

pub fn run() -> Result<()> {
    let language = SolidityDefinition::create();

    let mut changes = Vec::new();

    check_items(&language, &mut changes);
    check_builtins(&language, &mut changes);
    add_binding_rules_changes(&mut changes);

    changes.sort();

    let file_path = PathBuf::from("changes.csv");

    let mut file = csv::Writer::from_path(&file_path)?;
    for change in &changes {
        file.serialize(change)?;
    }
    file.flush()?;

    println!(
        "\nWrote {changes_count} changes to {file_path:?}",
        changes_count = changes.len(),
        file_path = file_path.canonicalize()?,
    );

    Ok(())
}

fn check_items(language: &Rc<Language>, changes: &mut Vec<Change>) {
    let items = language
        .items()
        .map(|item| (item.name(), item))
        .collect::<HashMap<_, _>>();

    for (name, item) in items {
        match item {
            Item::Struct { item } => {
                check_nonterminal(name, &item.enabled, changes);
                check_fields(name, &item.fields, changes);
            }
            Item::Enum { item } => {
                check_nonterminal(name, &item.enabled, changes);
                check_variants(item, changes);
            }
            Item::Repeated { item } => {
                check_nonterminal(name, &item.enabled, changes);
            }
            Item::Separated { item } => {
                check_nonterminal(name, &item.enabled, changes);
            }
            Item::Precedence { item } => {
                check_nonterminal(name, &item.enabled, changes);
                check_precedence(item, changes);
            }
            Item::Trivia { .. } => {
                // Not versioned...
            }
            Item::Keyword { item } => {
                check_keyword(item, changes);
            }
            Item::Token { item } => {
                check_token(item, changes);
            }
            Item::Fragment { .. } => {
                // not user-facing...
            }
        }
    }
}

fn check_nonterminal(
    name: &Identifier,
    enabled: &Option<VersionSpecifier>,
    changes: &mut Vec<Change>,
) {
    if let Some(from) = from(enabled) {
        changes.push(Change {
            version: from.clone(),
            area: Area::Nodes,
            direction: Direction::ForwardsIncompatible,
            description: format!("Nonterminal `{name}` is introduced."),
        });
    }

    if let Some(till) = till(enabled) {
        changes.push(Change {
            version: till.clone(),
            area: Area::Nodes,
            direction: Direction::BackwardsIncompatible,
            description: format!("Nonterminal `{name}` is deprecated."),
        });
    }
}

fn check_variants(r#enum: &EnumItem, changes: &mut Vec<Change>) {
    let name = &r#enum.name;

    for variant in &r#enum.variants {
        let reference = &variant.reference;

        if let Some(from) = from(&variant.enabled) {
            changes.push(Change {
                version: from.clone(),
                area: Area::Children,
                direction: Direction::ForwardsIncompatible,
                description: format!("Child `{reference}` of node `{name}` is introduced."),
            });
        }

        if let Some(till) = till(&variant.enabled) {
            changes.push(Change {
                version: till.clone(),
                area: Area::Children,
                direction: Direction::BackwardsIncompatible,
                description: format!("Child `{reference}` of node `{name}` is deprecated."),
            });
        }
    }
}

fn check_precedence(precedence: &PrecedenceItem, changes: &mut Vec<Change>) {
    for expression in &precedence.precedence_expressions {
        let name = &expression.name;

        let prefix = if expression.operators.len() > 1 {
            "A variation of the operator"
        } else {
            "Operator"
        };

        expression
            .operators
            .iter()
            .filter_map(|op| from(&op.enabled))
            .unique()
            .for_each(|from| {
                changes.push(Change {
                    version: from.clone(),
                    area: Area::Nodes,
                    direction: Direction::ForwardsIncompatible,
                    description: format!("{prefix} `{name}` is introduced."),
                });
            });

        expression
            .operators
            .iter()
            .filter_map(|op| till(&op.enabled))
            .unique()
            .for_each(|till| {
                changes.push(Change {
                    version: till.clone(),
                    area: Area::Nodes,
                    direction: Direction::BackwardsIncompatible,
                    description: format!("{prefix} `{name}` is deprecated."),
                });
            });

        for operator in &expression.operators {
            check_fields(name, &operator.fields, changes);
        }
    }

    for expression in &precedence.primary_expressions {
        let name = &precedence.name;
        let reference = &expression.reference;

        if let Some(from) = from(&expression.enabled) {
            changes.push(Change {
                version: from.clone(),
                area: Area::Children,
                direction: Direction::ForwardsIncompatible,
                description: format!("Child `{reference}` of node `{name}` is introduced."),
            });
        }

        if let Some(till) = till(&expression.enabled) {
            changes.push(Change {
                version: till.clone(),
                area: Area::Children,
                direction: Direction::BackwardsIncompatible,
                description: format!("Child `{reference}` of node `{name}` is deprecated."),
            });
        }
    }
}

fn check_fields(
    parent: &Identifier,
    fields: &IndexMap<Identifier, Field>,
    changes: &mut Vec<Change>,
) {
    for (label, field) in fields {
        let Field::Optional { enabled, .. } = field else {
            continue;
        };

        if let Some(from) = from(enabled) {
            changes.push(Change {
                version: from.clone(),
                area: Area::Children,
                direction: Direction::ForwardsIncompatible,
                description: format!("Child `{label}` of node `{parent}` is introduced."),
            });
        }

        if let Some(till) = till(enabled) {
            changes.push(Change {
                version: till.clone(),
                area: Area::Children,
                direction: Direction::BackwardsIncompatible,
                description: format!("Child `{label}` of node `{parent}` is deprecated."),
            });
        }
    }
}

fn check_keyword(keyword: &KeywordItem, changes: &mut Vec<Change>) {
    let name = &keyword.name;

    let prefix = if keyword.definitions.len() > 1 {
        "A variation of the keyword"
    } else {
        "Keyword"
    };

    keyword
        .definitions
        .iter()
        .filter_map(|def| from(&def.enabled))
        .unique()
        .for_each(|from| {
            changes.push(Change {
                version: from.clone(),
                area: Area::Nodes,
                direction: Direction::ForwardsIncompatible,
                description: format!("{prefix} `{name}` is introduced."),
            });
        });

    keyword
        .definitions
        .iter()
        .filter_map(|def| till(&def.enabled))
        .unique()
        .for_each(|till| {
            changes.push(Change {
                version: till.clone(),
                area: Area::Nodes,
                direction: Direction::BackwardsIncompatible,
                description: format!("{prefix} `{name}` is deprecated."),
            });
        });

    keyword
        .definitions
        .iter()
        .filter_map(|def| from(&def.reserved))
        .unique()
        .for_each(|from| {
            changes.push(Change {
                version: from.clone(),
                area: Area::Keywords,
                direction: Direction::BackwardsIncompatible,
                description: format!(
                    "{prefix} `{name}` is now reserved, and can not be used as an identifier."
                ),
            });
        });

    keyword
        .definitions
        .iter()
        .filter_map(|def| till(&def.reserved))
        .unique()
        .for_each(|till| {
            changes.push(Change {
                version: till.clone(),
                area: Area::Keywords,
                direction: Direction::ForwardsIncompatible,
                description: format!(
                    "{prefix} `{name}` is now unreserved, and can be used as an identifier."
                ),
            });
        });
}

fn check_token(token: &TokenItem, changes: &mut Vec<Change>) {
    let name = &token.name;

    let prefix = if token.definitions.len() > 1 {
        "A variation of the token"
    } else {
        "Token"
    };

    token
        .definitions
        .iter()
        .filter_map(|def| from(&def.enabled))
        .unique()
        .for_each(|from| {
            changes.push(Change {
                version: from.clone(),
                area: Area::Nodes,
                direction: Direction::ForwardsIncompatible,
                description: format!("{prefix} `{name}` is introduced."),
            });
        });

    token
        .definitions
        .iter()
        .filter_map(|def| till(&def.enabled))
        .unique()
        .for_each(|till| {
            changes.push(Change {
                version: till.clone(),
                area: Area::Nodes,
                direction: Direction::BackwardsIncompatible,
                description: format!("{prefix} `{name}` is deprecated."),
            });
        });
}

fn check_builtins(language: &Rc<Language>, changes: &mut Vec<Change>) {
    let mut add = |enabled: &Option<VersionSpecifier>, title: &str| {
        if let Some(from) = from(enabled) {
            changes.push(Change {
                version: from.clone(),
                area: Area::Builtins,
                direction: Direction::ForwardsIncompatible,
                description: format!("{title} is introduced."),
            });
        }

        if let Some(till) = till(enabled) {
            changes.push(Change {
                version: till.clone(),
                area: Area::Builtins,
                direction: Direction::BackwardsIncompatible,
                description: format!("{title} is deprecated."),
            });
        }
    };

    for definition in language.built_ins.iter().flat_map(|c| &c.definitions) {
        match definition {
            BuiltIn::BuiltInFunction { item } => {
                let title = format!("Builtin Function `{}`", &item.name);
                add(&item.enabled, &title);
            }
            BuiltIn::BuiltInVariable { item } => {
                let title = format!("Builtin Variable `{}`", &item.definition);
                add(&item.enabled, &title);
            }
            BuiltIn::BuiltInType { item } => {
                let title = format!("Builtin Type `{}`", &item.name);
                add(&item.enabled, &title);

                for function in &item.functions {
                    let title = format!(
                        "Builtin Type Function `{}({}){}` of type `{}`",
                        &function.name,
                        &function.parameters.join(", "),
                        match &function.return_type {
                            Some(v) => format!(" returns {v}"),
                            None => String::new(),
                        },
                        &item.name,
                    );
                    add(&function.enabled, &title);
                }

                for field in &item.fields {
                    let title = format!(
                        "Builtin Field `{}` of type `{}`",
                        &field.definition, &item.name,
                    );
                    add(&field.enabled, &title);
                }
            }
        }
    }
}

fn add_binding_rules_changes(changes: &mut Vec<Change>) {
    changes.extend([
        Change {
            version: Version::new(0, 5, 0),
            area: Area::Bindings,
            direction: Direction::BackwardsIncompatible,
            description: "`this` no longer works as an `address` type, deprecating member access like `this.balance`.".to_string(),
        },
        Change {
          version: Version::new(0, 7, 0),
          area: Area::Bindings,
          direction: Direction::BackwardsIncompatible,
          description: "`using` directives in contracts no apply to sub-contracts.".to_string(),
        },
        Change {
            version: Version::new(0, 5, 0),
            area: Area::Bindings,
            direction: Direction::BackwardsIncompatible,
            description: "Functions with the same name as the containing contract are no longer considered constructors.".to_string(),
        },
        Change {
            version: Version::new(0, 8, 8),
            area: Area::Builtins,
            direction: Direction::ForwardsIncompatible,
            description: "`type(Enum)` expressions now have members `min` and `max`.".to_string(),
        },
        Change {
            version: Version::new(0, 5, 0),
            area: Area::Bindings,
            direction: Direction::BackwardsIncompatible,
            description: "Scoping rules changed from Javascript-style to C99 style.".to_string(),
        },
        Change {
            version: Version::new(0, 7, 0),
            area: Area::Bindings,
            direction: Direction::BackwardsIncompatible,
            description: "Accessing slot/offset values for storage variable `foo` in Yul using `foo_slot` and `foo_offset` is deprecated.".to_string(),
        },
        Change {
            version: Version::new(0, 7, 0),
            area: Area::Builtins,
            direction: Direction::ForwardsIncompatible,
            description: "Added new `.slot` and `.offset` properties to storage variables in Yul.".to_string(),
        },
    ]);
}

fn from(specifier: &Option<VersionSpecifier>) -> Option<&Version> {
    match specifier.as_ref()? {
        VersionSpecifier::Never => None,
        VersionSpecifier::From { from } => Some(from),
        VersionSpecifier::Till { till: _ } => None,
        VersionSpecifier::Range { from, till: _ } => Some(from),
    }
}

pub fn till(specifier: &Option<VersionSpecifier>) -> Option<&Version> {
    match specifier.as_ref()? {
        VersionSpecifier::Never => None,
        VersionSpecifier::From { from: _ } => None,
        VersionSpecifier::Till { till } => Some(till),
        VersionSpecifier::Range { from: _, till } => Some(till),
    }
}
