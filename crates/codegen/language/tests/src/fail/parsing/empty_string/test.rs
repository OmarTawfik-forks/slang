#![allow(unused_crate_dependencies)]

codegen_language_macros::compile!(Language(
    name = Foo,
    binding_rules_file = "bindings/rules.msgb",
    root_item = Bar,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0"],
    sections = [Section(
        title = "Section One",
        topics = [Topic(
            title = "Topic One",
            items = [Token(
                name = Bar,
                definitions = [TokenDefinition(scanner = Atom(""))]
            )]
        )]
    )],
    built_ins = []
));

fn main() {}
