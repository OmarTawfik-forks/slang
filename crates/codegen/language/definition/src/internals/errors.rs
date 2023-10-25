use proc_macro2::{Span, TokenStream};
use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    message: String,
    span: Span,
}

impl Error {
    pub fn new(span: Span, message: &impl Display) -> Self {
        return Self {
            message: message.to_string(),
            span,
        };
    }

    fn to_compile_error(&self) -> TokenStream {
        return syn::Error::new(self.span, &self.message).to_compile_error();
    }
}

#[derive(Debug)]
pub struct ErrorsCollection {
    fatal: bool,
    errors: Vec<Error>,
}

impl ErrorsCollection {
    pub fn new() -> Self {
        return Self {
            fatal: false,
            errors: vec![],
        };
    }

    pub fn has_errors(&self) -> bool {
        return !self.errors.is_empty();
    }

    pub fn add(&mut self, span: Span, message: &impl Display) {
        self.errors.push(Error::new(span, message));
    }

    pub fn fatal(&mut self, span: Span, message: &impl Display) {
        self.add(span, message);
        self.fatal = true;
    }

    pub fn to_compile_errors(&self) -> TokenStream {
        return self
            .errors
            .iter()
            .map(|error| error.to_compile_error())
            .collect();
    }
}
