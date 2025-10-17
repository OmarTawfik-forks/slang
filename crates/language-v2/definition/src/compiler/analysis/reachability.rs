use std::collections::HashSet;
use std::rc::Rc;

use crate::compiler::analysis::Analysis;
use crate::compiler::version_set::VersionSet;
use crate::model::{Identifier, SpannedItem, SpannedTriviaParser};

pub(crate) fn analyze_reachability(analysis: &mut Analysis) {
    check_unreachabable_items(analysis);
    check_unused_versions(analysis);
}

fn check_unused_versions(analysis: &mut Analysis) {
    for (name, metadata) in &analysis.metadata {
        if name == &*analysis.language.root_item {
            continue;
        }

        match metadata.item {
            SpannedItem::Struct { .. }
            | SpannedItem::Enum { .. }
            | SpannedItem::Repeated { .. }
            | SpannedItem::Separated { .. }
            | SpannedItem::Precedence { .. } => {
                // check below
            }

            SpannedItem::Trivia { .. }
            | SpannedItem::Keyword { .. }
            | SpannedItem::Token { .. }
            | SpannedItem::Fragment { .. } => {
                // Skip terminals, as they are not versioned:
                continue;
            }
        }

        let unused_in = metadata.defined_in.difference(&metadata.used_in);

        if !unused_in.is_empty() {
            analysis
                .errors
                .add(&metadata.name, &Errors::UnusedVersion(name, &unused_in));
        }
    }
}

fn check_unreachabable_items(analysis: &mut Analysis) {
    let language = Rc::clone(&analysis.language);

    let mut queue = vec![&*language.root_item];

    collect_trivia(&language.leading_trivia, &mut queue);
    collect_trivia(&language.trailing_trivia, &mut queue);

    let mut visited = queue.iter().copied().collect::<HashSet<_>>();

    while let Some(name) = queue.pop() {
        for referenced_item in &analysis.metadata[name].referenced_items {
            if visited.insert(referenced_item) {
                queue.push(referenced_item);
            }
        }
    }

    for metadata in analysis.metadata.values() {
        match metadata.item {
            SpannedItem::Struct { .. }
            | SpannedItem::Enum { .. }
            | SpannedItem::Repeated { .. }
            | SpannedItem::Separated { .. }
            | SpannedItem::Precedence { .. } => {
                // check below
            }

            SpannedItem::Trivia { .. }
            | SpannedItem::Token { .. }
            | SpannedItem::Fragment { .. } => {
                // check below
            }

            SpannedItem::Keyword { .. } => {
                // Skip keywords, as they can just exist to be reserved against identifiers:
                continue;
            }
        }

        if !metadata.defined_in.is_empty() && !visited.contains(&*metadata.name) {
            analysis
                .errors
                .add(&metadata.name, &Errors::Unreachable(&metadata.name));
        }
    }
}

fn collect_trivia<'l>(parser: &'l SpannedTriviaParser, acc: &mut Vec<&'l Identifier>) {
    match parser {
        SpannedTriviaParser::Sequence { parsers } | SpannedTriviaParser::Choice { parsers } => {
            for parser in parsers {
                collect_trivia(parser, acc);
            }
        }
        SpannedTriviaParser::OneOrMore { parser }
        | SpannedTriviaParser::ZeroOrMore { parser }
        | SpannedTriviaParser::Optional { parser } => {
            collect_trivia(parser, acc);
        }
        SpannedTriviaParser::Trivia { reference } => {
            acc.push(reference);
        }
    }
}

#[derive(thiserror::Error, Debug)]
enum Errors<'err> {
    #[error("Item '{0}' is not used in versions: {1}")]
    UnusedVersion(&'err Identifier, &'err VersionSet),
    #[error("Item '{0}' is not reachable from grammar root.")]
    Unreachable(&'err Identifier),
}
