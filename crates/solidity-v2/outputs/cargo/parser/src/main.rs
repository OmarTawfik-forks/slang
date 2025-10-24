#![allow(unused_crate_dependencies)]

use std::time::{SystemTime, UNIX_EPOCH};

use semver::Version;
use slang_solidity::cst::LexicalContextType;
use slang_solidity::parser::{KeywordScan, Parser, ParserContext, ScannedTerminal};
use slang_solidity_v2_parser::lexer::{ContextKind, Lexer};

fn main() {
    // Largest file with trivia in mainnet shard 00:
    // https://vscode.blockscan.com/1/0x00aB290CC289F818a9E80eBaF18685E353DF16F0
    //
    // Note that pragmas and assembly blocks were removed from the file (~50 lines).
    // This eliminates the need to switch lexical contexts, as this is handled by the parser.
    // The cost to switch in both lexers is negligible compared to actual lexing time in any case.

    let source = include_str!("./input.sol");
    assert_eq!(source.lines().count(), 3301);

    // Duplicate the source to increase the size for better measurement
    let source = source.repeat(100);

    let language_version = Version::new(0, 8, 19);

    let (v1_output, v1_time) = {
        let mut stream = ParserContext::new(&source);

        let parser = Parser::create(language_version.clone()).unwrap();

        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        let output = std::iter::from_fn(|| {
            let pos1 = stream.position();
            let scan = slang_solidity::parser::Lexer::next_terminal::<LexicalContextType::Default>(
                &parser,
                &mut stream,
            )?;
            let pos2 = stream.position();
            Some((scan, pos1..pos2))
        })
        .collect::<Vec<_>>();

        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        (output, end - start)
    };

    let (v2_output, v2_time) = {
        let mut lexer = Lexer::new(ContextKind::Default, &source, language_version.clone());

        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        let output = std::iter::from_fn(|| lexer.next_lexeme()).collect::<Vec<_>>();

        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        (output, end - start)
    };

    assert_eq!(v1_output.len(), v2_output.len());

    println!("V1 time: {} ms", v1_time.as_millis());
    println!("V2 time: {} ms", v2_time.as_millis());
    println!(
        "Speedup: {:.2}x",
        v1_time.as_secs_f64() / v2_time.as_secs_f64()
    );

    for ((v1_kind, v1_range), v2) in v1_output.iter().zip(v2_output.iter()) {
        let v2_kind = format!("{:?}", v2.kind);

        match v1_kind {
            ScannedTerminal::IdentifierOrKeyword { identifier, kw } => match kw {
                KeywordScan::Absent => {
                    assert_eq!(identifier.to_string(), v2_kind);
                }
                KeywordScan::Present(kw_kind) => {
                    let v2_kind = v2_kind.strip_suffix("_Unreserved").unwrap_or_else(|| {
                        panic!("v2_kind should have _Unreserved suffix: {v2_kind}")
                    });

                    assert_eq!(kw_kind.to_string(), v2_kind);
                }
                KeywordScan::Reserved(kw_kind) => {
                    let v2_kind = v2_kind.strip_suffix("_Reserved").unwrap_or_else(|| {
                        panic!("v2_kind should have _Reserved suffix: {v2_kind}")
                    });

                    assert_eq!(kw_kind.to_string(), v2_kind);
                }
            },
            ScannedTerminal::Single(v1_kind) => {
                assert_eq!(v1_kind.to_string(), v2_kind);
            }
        }

        assert_eq!(v1_range, &v2.range);
    }

    println!("All tokens match!");
}
