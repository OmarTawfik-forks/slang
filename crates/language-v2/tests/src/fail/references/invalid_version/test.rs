#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    binding_rules_file = "bindings/rules.msgb",
    root_item = Foo,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0", "2.0.0", "3.0.0"],
    sections = [Section(
        title = "Section One",
        topics = [Topic(
            title = "Topic One",
            items = [
                Struct(
                    name = Foo,
                    fields = (
                        field_1 = Optional(
                            // should have been disabled in "3.0.0"
                            reference = Bar,
                            enabled = From("2.0.0")
                        ),
                        field_2 = Optional(
                            // should have been enabled in "2.0.0"
                            reference = Bar,
                            enabled = Till("3.0.0")
                        ),
                        field_3 = Optional(
                            // should have been enabled in "2.0.0" and disabled in "3.0.0"
                            reference = Bar
                        ),
                        field_4 = Optional(
                            // correct
                            reference = Bar,
                            enabled = Range(from = "2.0.0", till = "3.0.0")
                        )
                    )
                ),
                Struct(
                    name = Bar,
                    enabled = Range(from = "2.0.0", till = "3.0.0"),
                    fields = (field = Required(Baz))
                ),
                Token(name = Baz, definitions = [TokenDefinition(Atom("baz"))])
            ]
        )]
    )],
    built_ins = []
));

fn main() {}
