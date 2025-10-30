#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    binding_rules_file = "bindings/rules.msgb",
    root_item = Bar1,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0"],
    sections = [Section(
        title = "Section One",
        topics = [Topic(
            title = "Topic One",
            lexical_context = Foo,
            items = [
                Struct(
                    name = Bar1,
                    fields = (field = Required(Baz1)),
                    switch_lexical_context_on_reduce = Foo
                ),
                Struct(
                    name = Bar2,
                    fields = (field = Required(Baz2)),
                    switch_lexical_context_on_reduce = Other
                ),
                Keyword(
                    name = Baz1,
                    identifier = Identifier,
                    definitions = [KeywordDefinition(value = Atom("baz1"))],
                    switch_lexical_context_on_reduce = Foo
                ),
                Keyword(
                    name = Baz2,
                    identifier = Identifier,
                    definitions = [KeywordDefinition(value = Atom("baz2"))],
                    switch_lexical_context_on_reduce = Other
                ),
                Token(
                    name = Identifier,
                    definitions = [TokenDefinition(Atom("identifier"))]
                )
            ]
        )]
    )],
    built_ins = []
));

fn main() {}
