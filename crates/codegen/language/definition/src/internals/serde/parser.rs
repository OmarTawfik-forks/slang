use crate::internals::{ErrorsCollection, Spanned};
use indexmap::IndexMap;
use proc_macro2::{Delimiter, TokenStream};
use syn::{
    bracketed, parenthesized,
    parse::{Parse, ParseStream},
    token::{Bracket, Paren},
    Ident, LitBool, LitChar, LitInt, LitStr, Token,
};

pub struct ParseResult {
    value: Value,
    errors: ErrorsCollection,
}

pub enum Value {
    Map {
        fields: IndexMap<Spanned<String>, Spanned<Value>>,
    },
    NamedValue {
        name: Spanned<String>,
        value: Spanned<Box<Value>>,
    },
    Sequence {
        values: Vec<Spanned<Value>>,
    },

    Bool {
        value: bool,
    },
    Char {
        value: char,
    },
    Identifier {
        value: String,
    },
    String {
        value: String,
    },
    Usize {
        value: usize,
    },

    Skipped,
}

impl Parse for ParseResult {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut errors = ErrorsCollection::new();

        let value = match parse_value(input, &mut errors) {
            Ok(value) => value,
            Err(error) => {
                errors.fatal(error.span(), &error.to_string());
                Value::Skipped
            }
        };

        return Ok(Self { value, errors });
    }
}

fn parse_value(input: ParseStream, errors: &mut ErrorsCollection) -> syn::Result<Value> {
    if input.peek(Ident) {
        if input.peek2(Token![=]) {
            let fields = parse_fields(input, errors)?;
            return Ok(Value::Map { fields });
        }

        if input.peek2(Paren) {
            let name = Spanned::new(input.span(), input.parse::<Ident>()?.to_string());
            let value = Spanned::new(
                input.span(),
                Box::new(delimited(
                    Delimiter::Parenthesis,
                    input,
                    errors,
                    parse_value,
                )),
            );
            return Ok(Value::NamedValue { name, value });
        }

        let value = input.parse::<Ident>()?.to_string();
        return Ok(Value::Identifier { value });
    }

    if input.peek(Paren) {
        return Ok(delimited(
            Delimiter::Parenthesis,
            input,
            errors,
            parse_value,
        ));
    }

    if input.peek(Bracket) {
        return Ok(delimited(Delimiter::Bracket, input, errors, parse_sequence));
    }

    if input.peek(LitBool) {
        let value = input.parse::<LitBool>()?.value;
        return Ok(Value::Bool { value });
    }

    if input.peek(LitChar) {
        let value = input.parse::<LitChar>()?.value();
        return Ok(Value::Char { value });
    }

    if input.peek(LitStr) {
        let value = input.parse::<LitStr>()?.value();
        return Ok(Value::String { value });
    }

    if input.peek(LitInt) {
        let value = input.parse::<LitInt>()?.base10_parse::<usize>()?;
        return Ok(Value::Usize { value });
    }

    return Err(input.error(Errors::ExpectedValue));
}

fn parse_sequence(input: ParseStream, errors: &mut ErrorsCollection) -> syn::Result<Value> {
    let mut values = vec![];

    while !input.is_empty() {
        values.push(Spanned::new(input.span(), parse_value(input, errors)?));

        if !input.is_empty() {
            let comma = input.parse::<Token![,]>()?;

            if input.is_empty() {
                errors.add(comma.span, &Errors::TrailingComma);
            }
        }
    }

    return Ok(Value::Sequence { values });
}

fn parse_fields(
    input: ParseStream,
    errors: &mut ErrorsCollection,
) -> syn::Result<IndexMap<Spanned<String>, Spanned<Value>>> {
    let mut fields = IndexMap::new();

    while !input.is_empty() {
        let key = Spanned::new(input.span(), input.parse::<Ident>()?.to_string());

        input.parse::<Token![=]>()?;

        let value = Spanned::new(input.span(), parse_value(input, errors)?);

        if !input.is_empty() {
            let comma = input.parse::<Token![,]>()?;

            if input.is_empty() {
                errors.add(comma.span, &Errors::TrailingComma);
            }
        }

        if fields.contains_key(&key) {
            errors.fatal(key.span(), &Errors::DuplicateMapKey);
        } else {
            fields.insert(key, value);
        }
    }

    return Ok(fields);
}

fn delimited(
    delimiter: Delimiter,
    input: ParseStream,
    errors: &mut ErrorsCollection,
    inner_parser: impl FnOnce(ParseStream, &mut ErrorsCollection) -> syn::Result<Value>,
) -> Value {
    let inner_input;
    match delimiter {
        Delimiter::Bracket => bracketed!(inner_input in input),
        Delimiter::Parenthesis => parenthesized!(inner_input in input),
        _ => {
            unreachable!("Only brackets and parenthesis are supported.")
        }
    };

    return match inner_parser(&inner_input, errors) {
        Ok(value) => value,
        Err(error) => {
            // consume the rest of 'inner_input' so that we don't end up
            // reporting an extra/unnecessary "unexpected token" error:
            inner_input.parse::<TokenStream>().ok();

            errors.fatal(error.span(), &error.to_string());
            Value::Skipped
        }
    };
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("Expected a value.")]
    ExpectedValue,
    #[error("Duplicate map key.")]
    DuplicateMapKey,
    #[error("Trailing commas are not allowed.")]
    TrailingComma,
}
