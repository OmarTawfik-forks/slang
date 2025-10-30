mod p1_definitions;
mod p2_version_specifiers;
mod p3_references;
mod p4_unreachabe_items;
mod p5_unused_versions;

use std::rc::Rc;

use indexmap::IndexMap;
use proc_macro2::Span;

use crate::compiler::utils::version_set::VersionSet;
use crate::internals::{ErrorsCollection, ParseOutput, Spanned};
use crate::model::{Identifier, SpannedItem, SpannedLanguage, SpannedTopic};

pub(crate) struct Analysis {
    pub errors: ErrorsCollection,
    pub language: Rc<SpannedLanguage>,
    pub metadata: IndexMap<Identifier, ItemMetadata>,
}

pub(crate) struct ItemMetadata {
    pub name: Spanned<Identifier>,
    pub item: SpannedItem,
    pub lexical_context: Identifier,

    pub defined_in: VersionSet,
    pub used_in: VersionSet,

    pub referenced_from: Vec<Span>,
    pub referenced_items: Vec<Identifier>,
}

impl Analysis {
    pub fn analyze(parse_output: ParseOutput) -> Self {
        let ParseOutput { language, errors } = parse_output;

        let mut analysis = Self {
            errors,
            language: language.into(),
            metadata: IndexMap::new(),
        };

        for pass in &[
            p1_definitions::run,
            p2_version_specifiers::run,
            p3_references::run,
            p4_unreachabe_items::run,
            p5_unused_versions::run,
        ] {
            // Early return if there are already errors, to prevent producing noise from later analysis:
            if analysis.errors.has_errors() {
                return analysis;
            }

            pass(&mut analysis);
        }

        analysis
    }
}

impl SpannedLanguage {
    fn topics(&self) -> impl Iterator<Item = &SpannedTopic> {
        self.sections.iter().flat_map(|section| &section.topics)
    }

    fn items(&self) -> impl Iterator<Item = &SpannedItem> {
        self.topics().flat_map(|topic| &topic.items)
    }
}
