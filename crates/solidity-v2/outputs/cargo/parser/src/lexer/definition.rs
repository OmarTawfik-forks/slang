use std::collections::VecDeque;

use logos::Logos;
use semver::Version;

use crate::lexer::contexts::{ContextExtras, ContextKind, ContextWrapper};
use crate::lexer::lexemes::{Lexeme, LexemeKind};

pub struct Lexer<'source> {
    language_version: Version,
    context: ContextWrapper<'source>,
    queue: VecDeque<Lexeme>,
}

impl<'source> Lexer<'source> {
    pub fn new(kind: ContextKind, source: &'source str, language_version: Version) -> Self {
        let extras = ContextExtras {
            language_version: language_version.clone(),
        };

        let context = ContextWrapper::new(kind, source, extras);

        Self {
            language_version,
            context,
            queue: VecDeque::new(),
        }
    }

    #[must_use]
    pub fn switch_kind(mut self, kind: ContextKind) -> Self {
        self.context = self.context.switch_kind(kind);
        self
    }

    pub fn next_lexeme(&mut self) -> Option<Lexeme> {
        if let Some(lexeme) = self.queue.pop_front() {
            return Some(lexeme);
        }

        let lexeme = self.context.next_lexeme()?;

        match lexeme.kind {
            LexemeKind::DecimalLiteral => self.handle_decimal_literal(lexeme),
            LexemeKind::YulIdentifier => self.handle_yul_identifier(lexeme),
            _ => Some(lexeme),
        }
    }
}

impl Lexer<'_> {
    fn handle_decimal_literal(&mut self, original: Lexeme) -> Option<Lexeme> {
        // `DecimalLiteral` included a lone dot suffix without a fractional part (e.g. `1.` and `1.e1`)
        // before `0.5.0` as part of the same terminal. Afterwards, a lone `Period` should be its own lexeme,
        // with anything after (exponents, negative sign, and digits) being separate subsequent lexemes:

        #[derive(Logos, Debug)]
        enum Splitter {
            #[regex(r#"(([0-9]+(_[0-9]+)*))(\.(([0-9]+(_[0-9]+)*)))?(((e|E)-?(([0-9]+(_[0-9]+)*))))?"#, |_| { LexemeKind::DecimalLiteral })]
            #[regex(r#"\.(([0-9]+(_[0-9]+)*))(((e|E)-?(([0-9]+(_[0-9]+)*))))?"#, |_| { LexemeKind::DecimalLiteral })]
            #[regex(r#"\."#, |_| { LexemeKind::Period })]
            #[regex(r#"(e|E)([0-9]+(_[0-9]+)*)?"#, |_| { LexemeKind::Identifier })]
            #[regex(r#"-"#, |_| { LexemeKind::Minus })]
            Lexeme(LexemeKind),
        }

        if self.language_version < Version::new(0, 5, 0) {
            return Some(original);
        }

        let original_start = original.range.start;
        let original_source = self.context.source()[original.range].as_ref();

        let mut parts = Splitter::lexer(original_source)
            .spanned()
            .map(|(lexeme, range)| {
                let kind = match lexeme {
                    Ok(Splitter::Lexeme(kind)) => kind,
                    Err(()) => LexemeKind::UNRECOGNIZED,
                };

                let range = (range.start + original_start)..(range.end + original_start);

                Lexeme { kind, range }
            });

        let first_part = parts.next();
        self.queue.extend(parts);
        first_part
    }

    fn handle_yul_identifier(&mut self, original: Lexeme) -> Option<Lexeme> {
        // `YulIdentifier` allowed periods (i.e. `foo.bar.baz`) from `0.5.8` till `0.7.0`:
        // Otherwise, they should be split into multiple `YulIdentifier` and `Period` lexemes:

        #[derive(Logos, Debug)]
        enum Splitter {
            #[regex(r#"(_|\$|[a-z]|[A-Z]|[0-9])+"#, |_| { LexemeKind::YulIdentifier })]
            #[regex(r#"\."#, |_| { LexemeKind::Period })]
            Lexeme(LexemeKind),
        }

        if Version::new(0, 5, 8) <= self.language_version
            && self.language_version < Version::new(0, 7, 0)
        {
            return Some(original);
        }

        let original_start = original.range.start;
        let original_source = self.context.source()[original.range].as_ref();

        let mut parts = Splitter::lexer(original_source)
            .spanned()
            .map(|(lexeme, range)| {
                let kind = match lexeme {
                    Ok(Splitter::Lexeme(kind)) => kind,
                    Err(()) => LexemeKind::UNRECOGNIZED,
                };

                let range = (range.start + original_start)..(range.end + original_start);

                Lexeme { kind, range }
            });

        let first_part = parts.next();
        self.queue.extend(parts);
        first_part
    }
}
