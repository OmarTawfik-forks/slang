//! This model abstracts away some details from our language definition in order to
//! facilitate generation of CST structures
//!
//! Note: This is a copy of the model in v1 (crates/codegen/generator/src/ir/model.rs) with
//! some small changes

// TODO(v2):
// - Collect the sizes of terminals and nonterminals, this should allow us to optimize space usage
//   in particular, terminals with size 1 can be represented as `()` (or anything 0 sized) and
//   terminals with size N can be represented with an enum

use std::collections::{BTreeMap, BTreeSet};

use indexmap::IndexMap;
use language_v2_definition::model;
use serde::ser::SerializeMap;
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct StructuredCstModel {
    /// Terminal nodes.
    pub terminals: BTreeSet<model::Identifier>,

    /// Nonterminal nodes that are a fixed size group of potentially different nodes
    /// ie a struct
    pub sequences: BTreeMap<model::Identifier, Sequence>,

    /// Nonterminal nodes that are a choice between other nodes
    /// ie an enum
    pub choices: BTreeMap<model::Identifier, Choice>,

    /// Nonterminal nodes that are an unbounded collections of nodes of the same type
    /// ie a vector
    pub collections: BTreeMap<model::Identifier, Collection>,
}

#[derive(Clone, Serialize)]
pub struct Sequence {
    pub fields: Vec<Field>,
    pub enabled: model::VersionSpecifier,
    pub has_versioned_descendant: bool,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum NodeType {
    Nonterminal(model::Identifier),
    Terminal(model::Identifier),
}

#[derive(Clone, Serialize)]
pub struct Field {
    pub label: model::Identifier,
    pub r#type: NodeType,
    pub is_optional: bool,
    pub enabled: model::VersionSpecifier,
}

#[allow(clippy::struct_field_names)]
#[derive(Clone, Serialize)]
pub struct Choice {
    pub variants: Vec<ChoiceVariant>,
    pub enabled: model::VersionSpecifier,
    pub has_versioned_descendant: bool,
}

#[derive(Clone, Serialize)]
pub struct ChoiceVariant {
    pub node_type: NodeType,
    pub enabled: model::VersionSpecifier,
}

#[derive(Clone, Serialize)]
pub struct Collection {
    pub item_type: NodeType,
    pub enabled: model::VersionSpecifier,
    pub has_versioned_descendant: bool,
}

impl NodeType {
    pub fn as_identifier(&self) -> &model::Identifier {
        match self {
            NodeType::Nonterminal(identifier) | NodeType::Terminal(identifier) => identifier,
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Terminal(_))
    }
}

impl PartialEq<model::Identifier> for NodeType {
    fn eq(&self, other: &model::Identifier) -> bool {
        match self {
            NodeType::Nonterminal(identifier) | NodeType::Terminal(identifier) => {
                identifier == other
            }
        }
    }
}

impl Serialize for NodeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(3))?;
        let (identifier, kind, is_terminal) = match self {
            NodeType::Nonterminal(identifier) => (identifier, "Nonterminal", false),
            NodeType::Terminal(identifier) => (identifier, "Terminal", true),
        };
        map.serialize_entry("name", identifier)?;
        map.serialize_entry("kind", kind)?;
        map.serialize_entry("is_terminal", &is_terminal)?;
        map.end()
    }
}

// Construction
impl StructuredCstModel {
    pub fn from_language(language: &model::Language) -> Self {
        let mut builder = StructuredCstModelBuilder::create(language);
        builder.compute_has_versioned_descendant();

        Self {
            terminals: builder.terminals,
            sequences: builder.sequences,
            choices: builder.choices,
            collections: builder.collections,
        }
    }

    pub fn from_model(model: &Self) -> Self {
        Self {
            terminals: model.terminals.clone(),
            sequences: model.sequences.clone(),
            choices: model.choices.clone(),
            collections: model.collections.clone(),
        }
    }
}

struct StructuredCstModelBuilder {
    pub terminals: BTreeSet<model::Identifier>,
    pub sequences: BTreeMap<model::Identifier, Sequence>,
    pub choices: BTreeMap<model::Identifier, Choice>,
    pub collections: BTreeMap<model::Identifier, Collection>,
}

impl StructuredCstModelBuilder {
    fn create(language: &model::Language) -> Self {
        let mut builder = Self {
            terminals: BTreeSet::new(),
            sequences: BTreeMap::new(),
            choices: BTreeMap::new(),
            collections: BTreeMap::new(),
        };

        // First pass: collect all terminals:
        builder.collect_terminals(language);

        // Second pass: use them to build nonterminals:
        builder.collect_nonterminals(language);

        builder
    }

    fn collect_terminals(&mut self, language: &model::Language) {
        for item in language.items() {
            match item {
                model::Item::Struct { .. }
                | model::Item::Enum { .. }
                | model::Item::Repeated { .. }
                | model::Item::Separated { .. }
                | model::Item::Precedence { .. } => {
                    // These items are nonterminals.
                }
                model::Item::Trivia { .. } => {
                    // Trivia items are skipped by the parser.
                }
                model::Item::Keyword { item } => {
                    self.terminals.insert(item.name.clone());
                }
                model::Item::Token { item } => {
                    self.terminals.insert(item.name.clone());
                }
                model::Item::Fragment { .. } => {
                    // These items are inlined.
                }
            }
        }
    }

    fn collect_nonterminals(&mut self, language: &model::Language) {
        for item in language.items() {
            match item {
                model::Item::Struct { item } => {
                    self.add_struct_item(item);
                }
                model::Item::Enum { item } => {
                    self.add_enum_item(item);
                }
                model::Item::Repeated { item } => {
                    self.add_repeated_item(item);
                }
                model::Item::Separated { item } => {
                    self.add_separated_item(item);
                }
                model::Item::Precedence { item } => {
                    self.add_precedence_item(item);

                    for expr in &item.precedence_expressions {
                        self.add_precedence_expression(&item.name, expr);
                    }
                }
                model::Item::Trivia { .. }
                | model::Item::Keyword { .. }
                | model::Item::Token { .. } => {
                    // These items are terminals.
                }
                model::Item::Fragment { .. } => {
                    // These items are inlined.
                }
            }
        }
    }

    fn find_node_type(&self, identifier: &model::Identifier) -> NodeType {
        if self.terminals.contains(identifier) {
            NodeType::Terminal(identifier.clone())
        } else {
            NodeType::Nonterminal(identifier.clone())
        }
    }

    fn add_struct_item(&mut self, item: &model::StructItem) {
        let parent_type = item.name.clone();
        let fields: Vec<_> = self.convert_fields(&item.fields).collect();
        let enabled = item.enabled.clone().unwrap_or_default();

        self.sequences.insert(
            parent_type,
            Sequence {
                fields,
                enabled,
                has_versioned_descendant: false, // computed later
            },
        );
    }

    fn add_enum_item(&mut self, item: &model::EnumItem) {
        let parent_type = item.name.clone();
        let enabled = item.enabled.clone().unwrap_or_default();

        let variants = item
            .variants
            .iter()
            .map(|variant| ChoiceVariant {
                node_type: self.find_node_type(&variant.reference),
                enabled: variant.enabled.clone().unwrap_or_default(),
            })
            .collect();

        self.choices.insert(
            parent_type,
            Choice {
                variants,
                enabled,
                has_versioned_descendant: false, // computed later
            },
        );
    }

    fn add_repeated_item(&mut self, item: &model::RepeatedItem) {
        let parent_type = item.name.clone();
        let item_type = self.find_node_type(&item.reference);
        let enabled = item.enabled.clone().unwrap_or_default();

        self.collections.insert(
            parent_type,
            Collection {
                item_type,
                enabled,
                has_versioned_descendant: false, // computed later
            },
        );
    }

    fn add_separated_item(&mut self, item: &model::SeparatedItem) {
        let parent_type = item.name.clone();
        let item_type = self.find_node_type(&item.reference);
        let enabled = item.enabled.clone().unwrap_or_default();

        self.collections.insert(
            parent_type,
            Collection {
                item_type,
                enabled,
                has_versioned_descendant: false, // computed later
            },
        );
    }

    fn add_precedence_item(&mut self, item: &model::PrecedenceItem) {
        let parent_type = item.name.clone();
        let enabled = item.enabled.clone().unwrap_or_default();

        let precedence_variants =
            item.precedence_expressions
                .iter()
                .map(|expression| ChoiceVariant {
                    node_type: self.find_node_type(&expression.name),
                    enabled: model::VersionSpecifier::Always,
                });

        let primary_variants = item
            .primary_expressions
            .iter()
            .map(|expression| ChoiceVariant {
                node_type: self.find_node_type(&expression.reference),
                enabled: expression.enabled.clone().unwrap_or_default(),
            });

        let variants = precedence_variants.chain(primary_variants).collect();

        self.choices.insert(
            parent_type,
            Choice {
                variants,
                enabled,
                has_versioned_descendant: false, // computed later
            },
        );
    }

    fn add_precedence_expression(
        &mut self,
        base_name: &model::Identifier,
        expression: &model::PrecedenceExpression,
    ) {
        let parent_type = expression.name.clone();

        // TODO: The precedence operators is too complex, and we're making a lot
        // of assumptions everywere.
        // We should make it simpler, in particular it should be correct by construction

        let operator_fields = if expression.operators.len() == 1 {
            // If there's a single operator, we just use its fields directly
            self.convert_fields(&expression.operators[0].fields)
                .collect::<Vec<_>>()
        } else {
            // If there are multiple operators, we create a choice between them
            // They must have a single required field labeled "operator"
            //
            // Note: We use a set to avoid duplicate variants, in particular exponentiation
            // uses the same operator symbol for right and left associative versions, although they live in different
            // versions
            let variants: BTreeSet<_> = expression
                .operators
                .iter()
                .map(|operator| {
                    assert!(
                        operator.fields.len() == 1,
                        "Multiple operators with multiple fields are not supported"
                    );

                    let (ident, field) = operator.fields.first().unwrap();
                    assert_eq!(
                        ident,
                        &model::Identifier::from("operator"),
                        "Operator field must be labeled 'operator'"
                    );
                    if let model::Field::Required { reference } = field {
                        self.find_node_type(reference)
                    } else {
                        panic!("Operator field must be required");
                    }
                })
                .collect();

            let ident =
                model::Identifier::from(format!("{}_{}_Operator", base_name, expression.name));

            // Insert the created choice
            self.choices.insert(
                ident.clone(),
                Choice {
                    variants: variants
                        .into_iter()
                        .map(|node_type| ChoiceVariant {
                            node_type,
                            enabled: model::VersionSpecifier::Always,
                        })
                        .collect(),
                    enabled: model::VersionSpecifier::Always,
                    has_versioned_descendant: false, // computed later
                },
            );

            // The only field we care about then is a reference to that special operator
            vec![Field {
                label: ident.clone(),
                r#type: NodeType::Nonterminal(ident),
                is_optional: false,
                enabled: model::VersionSpecifier::Always,
            }]
        };

        let operand = |label: model::PredefinedLabel| Field {
            label: label.as_ref().into(),
            r#type: NodeType::Nonterminal(base_name.clone()),
            is_optional: false,
            enabled: model::VersionSpecifier::Always,
        };

        let mut fields = vec![];

        // All operators should have the same structure (validated at compile-time),
        // So let's pick up the first one to generate the types:
        match expression.operators[0].model {
            model::OperatorModel::Prefix => {
                fields.extend(operator_fields);
                fields.push(operand(model::PredefinedLabel::Operand));
            }
            model::OperatorModel::Postfix => {
                fields.push(operand(model::PredefinedLabel::Operand));
                fields.extend(operator_fields);
            }
            model::OperatorModel::BinaryLeftAssociative
            | model::OperatorModel::BinaryRightAssociative => {
                fields.push(operand(model::PredefinedLabel::LeftOperand));
                fields.extend(operator_fields);
                fields.push(operand(model::PredefinedLabel::RightOperand));
            }
        }

        self.sequences.insert(
            parent_type,
            Sequence {
                fields,
                enabled: model::VersionSpecifier::Always,
                has_versioned_descendant: false, // computed later
            },
        );
    }

    /// Compute `has_versioned_descendant` for all nodes using a fixpoint iteration.
    /// A node has a versioned descendant if:
    /// - It has a non-Always `enabled` specifier, or
    /// - Any of its fields/variants have a non-Always `enabled`, or
    /// - Any of its children (transitively) have `has_versioned_descendant == true`.
    fn compute_has_versioned_descendant(&mut self) {
        let is_versioned =
            |spec: &model::VersionSpecifier| !matches!(spec, model::VersionSpecifier::Always);

        loop {
            let mut changed = false;

            let sequence_keys: Vec<_> = self.sequences.keys().cloned().collect();
            for key in sequence_keys {
                let seq = &self.sequences[&key];
                if seq.has_versioned_descendant {
                    continue;
                }
                let dominated = is_versioned(&seq.enabled)
                    || seq.fields.iter().any(|f| {
                        is_versioned(&f.enabled) || self.child_has_versioned_descendant(&f.r#type)
                    });
                if dominated {
                    self.sequences
                        .get_mut(&key)
                        .unwrap()
                        .has_versioned_descendant = true;
                    changed = true;
                }
            }

            let choice_keys: Vec<_> = self.choices.keys().cloned().collect();
            for key in choice_keys {
                let choice = &self.choices[&key];
                if choice.has_versioned_descendant {
                    continue;
                }
                let dominated = is_versioned(&choice.enabled)
                    || choice.variants.iter().any(|v| {
                        is_versioned(&v.enabled)
                            || self.child_has_versioned_descendant(&v.node_type)
                    });
                if dominated {
                    self.choices.get_mut(&key).unwrap().has_versioned_descendant = true;
                    changed = true;
                }
            }

            let collection_keys: Vec<_> = self.collections.keys().cloned().collect();
            for key in collection_keys {
                let coll = &self.collections[&key];
                if coll.has_versioned_descendant {
                    continue;
                }
                let dominated = is_versioned(&coll.enabled)
                    || self.child_has_versioned_descendant(&coll.item_type);
                if dominated {
                    self.collections
                        .get_mut(&key)
                        .unwrap()
                        .has_versioned_descendant = true;
                    changed = true;
                }
            }

            if !changed {
                break;
            }
        }
    }

    fn child_has_versioned_descendant(&self, node_type: &NodeType) -> bool {
        match node_type {
            NodeType::Terminal(_) => false,
            NodeType::Nonterminal(id) => {
                if let Some(seq) = self.sequences.get(id) {
                    seq.has_versioned_descendant
                } else if let Some(choice) = self.choices.get(id) {
                    choice.has_versioned_descendant
                } else if let Some(coll) = self.collections.get(id) {
                    coll.has_versioned_descendant
                } else {
                    false
                }
            }
        }
    }

    fn convert_fields<'a>(
        &'a self,
        fields: &'a IndexMap<model::Identifier, model::Field>,
    ) -> impl Iterator<Item = Field> + 'a {
        fields.iter().map(|(label, field)| {
            let (reference, is_optional, enabled) = match field {
                model::Field::Required { reference } => {
                    (reference, false, model::VersionSpecifier::Always)
                }
                model::Field::Optional { reference, enabled } => {
                    (reference, true, enabled.clone().unwrap_or_default())
                }
            };
            let r#type = self.find_node_type(reference);

            Field {
                label: label.clone(),
                r#type,
                is_optional,
                enabled,
            }
        })
    }
}
