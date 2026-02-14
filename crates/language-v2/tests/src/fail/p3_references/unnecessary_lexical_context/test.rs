#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    root_item = One,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0"],
    sections = [Section(
        title = "Section One",
        topics = [Topic(
            title = "Topic One",
            lexical_context = ContextA,
            items = [
                Struct(
                    name = One,
                    lexical_context = ContextA,
                    fields = (
                        field_1 = Required(Two),
                        field_2 = Required(Two)
                    )
                ),
                Token(name = Two, definitions = [TokenDefinition(Atom("two"))])
            ]
        )]
    )],
    built_ins = []
));

fn main() {}
