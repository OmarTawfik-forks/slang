#[path = "contexts.generated.rs"]
mod contexts;
mod definition;
#[path = "lexemes.generated.rs"]
mod lexemes;

#[cfg(test)]
mod tests;

pub use contexts::ContextKind;
pub use definition::Lexer;

// TODO(v2): remove this API once the lexer is fully integrated into the parser, and can be tested through it:
pub mod sourcify {
    use std::rc::Rc;

    use semver::Version;
    use slang_solidity::cst::NonterminalNode;

    pub fn compare_with_v1_output(language_version: Version, tree: &Rc<NonterminalNode>) {
        //
        // infra run --bin  solidity_testing_sourcify -- test --shard-count 256 --shard-index 0
        //

        // let v2_lexer = Lexer::new(&parse_output.source, language_version);
        // let v2_lexemes: Vec<Lexeme> = v2_lexer.collect();

        // let v1_lexemes: Vec<Lexeme> = parse_output
        //     .lexemes
        //     .into_iter()
        //     .map(|lexeme_v1| Lexeme::from_v1_lexeme(lexeme_v1, &parse_output.source))
        //     .collect();

        // assert_eq!(
        //     v1_lexemes.len(),
        //     v2_lexemes.len(),
        //     "v1 and v2 lexers produced different number of lexemes"
        // );

        // for (i, (lexeme_v1, lexeme_v2)) in v1_lexemes.iter().zip(v2_lexemes.iter()).enumerate() {
        //     assert_eq!(
        //         lexeme_v1.kind, lexeme_v2.kind,
        //         "mismatched lexeme kinds at index {i}"
        //     );
        //     assert_eq!(
        //         lexeme_v1.range, lexeme_v2.range,
        //         "mismatched lexeme ranges at index {i}"
        //     );
        // }
    }
}
