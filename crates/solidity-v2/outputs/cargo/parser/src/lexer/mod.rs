#[path = "contexts.generated.rs"]
mod contexts;
mod definition;
#[path = "lexemes.generated.rs"]
mod lexemes;

#[cfg(test)]
mod tests;

/// TODO(v2): remove this temporary API once the lexer is fully integrated into the parser, and can be tested through it:
pub mod temp_sourcify;
