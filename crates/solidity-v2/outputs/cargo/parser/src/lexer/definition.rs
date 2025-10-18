use semver::Version;

use crate::lexer::contexts::{ContextExtras, ContextWrapper};
use crate::lexer::lexemes::Lexeme;
use crate::lexer::ContextKind;

pub struct Lexer<'source> {
    context: ContextWrapper<'source>,
}

impl<'source> Lexer<'source> {
    pub fn new(kind: ContextKind, source: &'source str, language_version: Version) -> Self {
        let extras = ContextExtras { language_version };
        let context = ContextWrapper::new(kind, source, extras);
        Self { context }
    }

    #[must_use]
    pub fn switch_kind(mut self, kind: ContextKind) -> Self {
        self.context = self.context.switch_kind(kind);
        self
    }

    pub fn next_lexeme(&mut self) -> Option<Lexeme> {
        self.context.next_lexeme()
    }
}
