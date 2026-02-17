use std::fmt::Debug;
use std::rc::Rc;

use indexmap::IndexMap;

use crate::compiler::analysis::Analysis;
use crate::compiler::utils::version_set::VersionSet;
use crate::internals::Spanned;
use crate::model::SpannedItemDiscriminants::{
    self, Enum, Fragment, Keyword, Precedence, Repeated, Separated, Struct, Token, Trivia,
};
use crate::model::{
    Identifier, SpannedBuiltIn, SpannedBuiltInField, SpannedBuiltInFunction, SpannedBuiltInType,
    SpannedEnumItem, SpannedEnumVariant, SpannedField, SpannedFragmentItem, SpannedItem,
    SpannedKeywordItem, SpannedPrecedenceExpression, SpannedPrecedenceItem,
    SpannedPrecedenceOperator, SpannedPrimaryExpression, SpannedRepeatedItem, SpannedScanner,
    SpannedSeparatedItem, SpannedStructItem, SpannedTokenDefinition, SpannedTokenItem,
    SpannedTriviaItem, SpannedTriviaParser, SpannedVersionSpecifier,
};

pub(crate) fn run(analysis: &mut Analysis) {
    let language = Rc::clone(&analysis.language);

    let mut enablement = VersionSet::new();
    enablement.add_all_versions(&analysis.language);

    // Language-level references have no context constraint:
    check_reference(
        analysis,
        None,
        &language.root_item,
        &enablement,
        &[Struct, Enum, Repeated, Separated, Precedence],
        None,
    );

    check_trivia_parser(analysis, &language.leading_trivia, &enablement);
    check_trivia_parser(analysis, &language.trailing_trivia, &enablement);

    // Walk items grouped by topic to know each item's context:
    for section in &language.sections {
        for topic in &section.topics {
            let context = &*topic.lexical_context;
            for item in &topic.items {
                check_item(analysis, item, &enablement, context);
            }
        }
    }

    for built_in_context in &language.built_ins {
        for built_in in &built_in_context.definitions {
            check_built_in(analysis, built_in, &enablement);
        }
    }
}

fn check_item(
    analysis: &mut Analysis,
    item: &SpannedItem,
    enablement: &VersionSet,
    context: &Identifier,
) {
    match item {
        SpannedItem::Struct { item } => {
            check_struct(analysis, item, enablement, context);
        }
        SpannedItem::Enum { item } => {
            check_enum(analysis, item, enablement, context);
        }
        SpannedItem::Repeated { item } => {
            check_repeated(analysis, item, enablement, context);
        }
        SpannedItem::Separated { item } => {
            check_separated(analysis, item, enablement, context);
        }
        SpannedItem::Precedence { item } => {
            check_precedence(analysis, item, enablement, context);
        }
        SpannedItem::Trivia { item } => {
            check_trivia(analysis, item, enablement, context);
        }
        SpannedItem::Keyword { item } => {
            check_keyword(analysis, item, enablement, context);
        }
        SpannedItem::Token { item } => {
            check_token(analysis, item, enablement, context);
        }
        SpannedItem::Fragment { item } => {
            check_fragment(analysis, item, enablement, context);
        }
    }
}

fn check_struct(
    analysis: &mut Analysis,
    item: &SpannedStructItem,
    enablement: &VersionSet,
    context: &Identifier,
) {
    let SpannedStructItem {
        name,
        switch_lexical_context,
        enabled,
        error_recovery: _,
        fields,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled.as_ref());

    // If this struct is a gateway, validate the annotation and use it for field context:
    if let Some(gateway_context) = switch_lexical_context {
        if **gateway_context == *context {
            analysis.errors.add(
                gateway_context,
                &Errors::UnnecessaryLexicalContext(gateway_context, context),
            );
        }

        check_gateway_fields(analysis, name, fields, &enablement, context, gateway_context);
    } else {
        check_fields(analysis, name, fields, &enablement, context);
    }
}

/// Check fields of a gateway struct: first field uses topic context, remaining fields use gateway context.
fn check_gateway_fields(
    analysis: &mut Analysis,
    source: &Identifier,
    fields: &IndexMap<Spanned<Identifier>, SpannedField>,
    enablement: &VersionSet,
    topic_context: &Identifier,
    gateway_context: &Spanned<Identifier>,
) {
    for (index, field) in fields.values().enumerate() {
        let expected_context = if index == 0 {
            topic_context
        } else {
            gateway_context
        };

        match field {
            SpannedField::Required { reference } => {
                check_reference(
                    analysis,
                    Some(source),
                    reference,
                    enablement,
                    &[
                        Struct, Enum, Repeated, Separated, Precedence, Keyword, Token,
                    ],
                    Some(expected_context),
                );
            }
            SpannedField::Optional { reference, enabled } => {
                let enablement = update_enablement(analysis, enablement, enabled.as_ref());

                check_reference(
                    analysis,
                    Some(source),
                    reference,
                    &enablement,
                    &[
                        Struct, Enum, Repeated, Separated, Precedence, Keyword, Token,
                    ],
                    Some(expected_context),
                );

                check_optional_field_allows_empty(analysis, reference);
            }
        }
    }
}

fn check_enum(
    analysis: &mut Analysis,
    item: &SpannedEnumItem,
    enablement: &VersionSet,
    context: &Identifier,
) {
    let SpannedEnumItem {
        name,
        enabled,
        variants,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled.as_ref());

    for variant in variants {
        let SpannedEnumVariant { reference, enabled } = variant;

        let enablement = update_enablement(analysis, &enablement, enabled.as_ref());

        check_reference(
            analysis,
            Some(name),
            reference,
            &enablement,
            &[
                Struct, Enum, Repeated, Separated, Precedence, Keyword, Token,
            ],
            Some(context),
        );
    }
}

fn check_repeated(
    analysis: &mut Analysis,
    item: &SpannedRepeatedItem,
    enablement: &VersionSet,
    context: &Identifier,
) {
    let SpannedRepeatedItem {
        name,
        reference,
        allow_empty: _,
        enabled,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled.as_ref());

    check_reference(
        analysis,
        Some(name),
        reference,
        &enablement,
        &[
            Struct, Enum, Repeated, Separated, Precedence, Keyword, Token,
        ],
        Some(context),
    );
}

fn check_separated(
    analysis: &mut Analysis,
    item: &SpannedSeparatedItem,
    enablement: &VersionSet,
    context: &Identifier,
) {
    let SpannedSeparatedItem {
        name,
        reference,
        separator,
        allow_empty: _,
        enabled,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled.as_ref());

    check_reference(
        analysis,
        Some(name),
        reference,
        &enablement,
        &[
            Struct, Enum, Repeated, Separated, Precedence, Keyword, Token,
        ],
        Some(context),
    );

    check_reference(
        analysis,
        Some(name),
        separator,
        &enablement,
        &[Token],
        Some(context),
    );
}

fn check_precedence(
    analysis: &mut Analysis,
    item: &SpannedPrecedenceItem,
    enablement: &VersionSet,
    context: &Identifier,
) {
    let SpannedPrecedenceItem {
        name,
        enabled,
        precedence_expressions,
        primary_expressions,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled.as_ref());

    for precedence_expression in precedence_expressions {
        let SpannedPrecedenceExpression { name: _, operators } = precedence_expression.as_ref();

        for operator in operators {
            let SpannedPrecedenceOperator {
                model: _,
                enabled,
                error_recovery: _,
                fields,
            } = operator;

            let enablement = update_enablement(analysis, &enablement, enabled.as_ref());

            check_fields(analysis, name, fields, &enablement, context);
        }
    }

    for primary_expression in primary_expressions {
        let SpannedPrimaryExpression { reference, enabled } = primary_expression;

        let enablement = update_enablement(analysis, &enablement, enabled.as_ref());

        check_reference(
            analysis,
            Some(name),
            reference,
            &enablement,
            &[
                Struct, Enum, Repeated, Separated, Precedence, Keyword, Token,
            ],
            Some(context),
        );
    }
}

fn check_fields(
    analysis: &mut Analysis,
    source: &Identifier,
    fields: &IndexMap<Spanned<Identifier>, SpannedField>,
    enablement: &VersionSet,
    context: &Identifier,
) {
    for field in fields.values() {
        match field {
            SpannedField::Required { reference } => {
                check_reference(
                    analysis,
                    Some(source),
                    reference,
                    enablement,
                    &[
                        Struct, Enum, Repeated, Separated, Precedence, Keyword, Token,
                    ],
                    Some(context),
                );
            }
            SpannedField::Optional { reference, enabled } => {
                let enablement = update_enablement(analysis, enablement, enabled.as_ref());

                check_reference(
                    analysis,
                    Some(source),
                    reference,
                    &enablement,
                    &[
                        Struct, Enum, Repeated, Separated, Precedence, Keyword, Token,
                    ],
                    Some(context),
                );

                check_optional_field_allows_empty(analysis, reference);
            }
        }
    }
}

fn check_optional_field_allows_empty(analysis: &mut Analysis, reference: &Spanned<Identifier>) {
    if let Some(target) = analysis.metadata.get_mut(&**reference) {
        match &target.item {
            SpannedItem::Repeated { item: child } => {
                if child.allow_empty.as_ref().is_some_and(|b| **b) {
                    analysis
                        .errors
                        .add(reference, &Errors::OptionalFieldAllowsEmpty);
                }
            }
            SpannedItem::Separated { item: child } => {
                if child.allow_empty.as_ref().is_some_and(|b| **b) {
                    analysis
                        .errors
                        .add(reference, &Errors::OptionalFieldAllowsEmpty);
                }
            }
            _ => {}
        }
    }
}

fn check_trivia_parser(
    analysis: &mut Analysis,
    parser: &SpannedTriviaParser,
    enablement: &VersionSet,
) {
    match parser {
        SpannedTriviaParser::Sequence { parsers } | SpannedTriviaParser::Choice { parsers } => {
            for parser in parsers {
                check_trivia_parser(analysis, parser, enablement);
            }
        }
        SpannedTriviaParser::OneOrMore { parser }
        | SpannedTriviaParser::ZeroOrMore { parser }
        | SpannedTriviaParser::Optional { parser } => {
            check_trivia_parser(analysis, parser, enablement);
        }
        SpannedTriviaParser::Trivia { reference } => {
            // Language-level trivia parsers have no context constraint:
            check_reference(analysis, None, reference, enablement, &[Trivia], None);
        }
    }
}

fn check_trivia(
    analysis: &mut Analysis,
    item: &SpannedTriviaItem,
    enablement: &VersionSet,
    context: &Identifier,
) {
    let SpannedTriviaItem { name, scanner } = item;

    check_scanner(analysis, Some(name), scanner, enablement, context);
}

fn check_keyword(
    analysis: &mut Analysis,
    item: &SpannedKeywordItem,
    enablement: &VersionSet,
    context: &Identifier,
) {
    let SpannedKeywordItem {
        name,
        identifier,
        enabled,
        definitions: _,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled.as_ref());

    check_reference(
        analysis,
        Some(name),
        identifier,
        &enablement,
        &[Token],
        Some(context),
    );
}

fn check_token(
    analysis: &mut Analysis,
    item: &SpannedTokenItem,
    enablement: &VersionSet,
    context: &Identifier,
) {
    let SpannedTokenItem {
        name,
        enabled,
        definitions,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled.as_ref());

    for definition in definitions {
        let SpannedTokenDefinition { scanner } = definition;

        check_scanner(analysis, Some(name), scanner, &enablement, context);
    }
}

fn check_fragment(
    analysis: &mut Analysis,
    item: &SpannedFragmentItem,
    enablement: &VersionSet,
    context: &Identifier,
) {
    let SpannedFragmentItem {
        name,
        enabled,
        scanner,
    } = item;

    let enablement = update_enablement(analysis, enablement, enabled.as_ref());

    check_scanner(analysis, Some(name), scanner, &enablement, context);
}

fn check_scanner(
    analysis: &mut Analysis,
    source: Option<&Identifier>,
    scanner: &SpannedScanner,
    enablement: &VersionSet,
    context: &Identifier,
) {
    match scanner {
        SpannedScanner::Sequence { scanners } | SpannedScanner::Choice { scanners } => {
            for scanner in scanners {
                check_scanner(analysis, source, scanner, enablement, context);
            }
        }
        SpannedScanner::Optional { scanner }
        | SpannedScanner::ZeroOrMore { scanner }
        | SpannedScanner::OneOrMore { scanner } => {
            check_scanner(analysis, source, scanner, enablement, context);
        }
        SpannedScanner::Not { chars: _ }
        | SpannedScanner::Range {
            inclusive_start: _,
            inclusive_end: _,
        }
        | SpannedScanner::Atom { atom: _ } => {
            // Nothing to check for now.
        }
        SpannedScanner::Fragment { reference } => {
            check_reference(
                analysis,
                source,
                reference,
                enablement,
                &[Fragment],
                Some(context),
            );
        }
    }
}

fn check_reference(
    analysis: &mut Analysis,
    source: Option<&Identifier>,
    reference: &Spanned<Identifier>,
    enablement: &VersionSet,
    expected_kinds: &[SpannedItemDiscriminants],
    expected_context: Option<&Identifier>,
) {
    let Some(target) = analysis.metadata.get_mut(&**reference) else {
        analysis
            .errors
            .add(reference, &Errors::UnknownReference(reference));
        return;
    };

    let not_defined_in = enablement.difference(&target.defined_in);
    if !not_defined_in.is_empty() {
        analysis.errors.add(
            reference,
            &Errors::InvalidReferenceVersion(reference, &target.defined_in, &not_defined_in),
        );
    }

    let actual_kind: SpannedItemDiscriminants = target.item.clone().into();
    if !expected_kinds.contains(&actual_kind) {
        analysis.errors.add(
            reference,
            &Errors::InvalidReferenceFilter(reference, &actual_kind, expected_kinds),
        );
    }

    // Check lexical context constraint:
    if let Some(expected_context) = expected_context {
        if target.lexical_context != *expected_context {
            analysis.errors.add(
                reference,
                &Errors::InvalidReferenceContext(
                    reference,
                    &target.lexical_context,
                    expected_context,
                ),
            );
        }
    }

    target.used_in.add_version_set(enablement);

    target.referenced_from.push(reference.span());

    if let Some(source) = source {
        analysis.metadata[source]
            .referenced_items
            .push((**reference).clone());
    }
}

fn check_built_in(analysis: &mut Analysis, built_in: &SpannedBuiltIn, enablement: &VersionSet) {
    match built_in {
        SpannedBuiltIn::BuiltInFunction { item } => {
            check_built_in_function(analysis, item, enablement);
        }
        SpannedBuiltIn::BuiltInType { item } => {
            check_built_in_type(analysis, item, enablement);
        }
        SpannedBuiltIn::BuiltInVariable { item } => {
            check_built_in_field(analysis, item, enablement);
        }
    }
}

fn check_built_in_function(
    analysis: &mut Analysis,
    built_in: &SpannedBuiltInFunction,
    enablement: &VersionSet,
) {
    let SpannedBuiltInFunction {
        name: _,
        return_type: _,
        parameters: _,
        enabled,
    } = built_in;

    let _ = update_enablement(analysis, enablement, enabled.as_ref());
}

fn check_built_in_type(
    analysis: &mut Analysis,
    built_in: &SpannedBuiltInType,
    enablement: &VersionSet,
) {
    let SpannedBuiltInType {
        name: _,
        fields,
        functions,
        enabled,
    } = built_in;

    let enablement = update_enablement(analysis, enablement, enabled.as_ref());

    for field in fields {
        check_built_in_field(analysis, field, &enablement);
    }
    for function in functions {
        check_built_in_function(analysis, function, &enablement);
    }
}

fn check_built_in_field(
    analysis: &mut Analysis,
    built_in: &SpannedBuiltInField,
    enablement: &VersionSet,
) {
    let SpannedBuiltInField {
        definition: _,
        enabled,
    } = built_in;

    let _ = update_enablement(analysis, enablement, enabled.as_ref());
}

fn update_enablement(
    analysis: &mut Analysis,
    existing_enablement: &VersionSet,
    new_specifier: Option<&Spanned<SpannedVersionSpecifier>>,
) -> VersionSet {
    let Some(new_specifier) = new_specifier else {
        return existing_enablement.to_owned();
    };

    let mut new_enablement = VersionSet::new();
    new_enablement.add_specifier(new_specifier, &analysis.language);

    let not_defined_in = new_enablement.difference(existing_enablement);
    if !not_defined_in.is_empty() {
        analysis
            .errors
            .add(new_specifier, &Errors::EnabledTooWide(existing_enablement));
    }

    new_enablement
}

#[derive(thiserror::Error, Debug)]
enum Errors<'err> {
    #[error("Parent scope is only enabled in '{0}'.")]
    EnabledTooWide(&'err VersionSet),
    #[error("Reference to unknown item '{0}'.")]
    UnknownReference(&'err Identifier),
    #[error("Reference '{0}' is only defined in '{1}', but not in '{2}'.")]
    InvalidReferenceVersion(&'err Identifier, &'err VersionSet, &'err VersionSet),
    #[error("Optional field points to a container that allows empty children. Should this be required instead?")]
    OptionalFieldAllowsEmpty,
    #[error("Reference '{0}' of kind '{1:?}' is not valid. Expected: {2:?}")]
    InvalidReferenceFilter(
        &'err Identifier,
        &'err SpannedItemDiscriminants,
        &'err [SpannedItemDiscriminants],
    ),
    #[error("Reference '{0}' is in context '{1}', but expected context '{2}'.")]
    InvalidReferenceContext(&'err Identifier, &'err Identifier, &'err Identifier),
    #[error("Unnecessary 'switch_lexical_context = {0}': it is the same as the topic's context '{1}'.")]
    UnnecessaryLexicalContext(&'err Identifier, &'err Identifier),
}
