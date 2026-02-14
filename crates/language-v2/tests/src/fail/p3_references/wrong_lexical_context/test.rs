#![allow(unused_crate_dependencies)]

language_v2_macros::compile!(Language(
    name = Foo,
    root_item = One,
    leading_trivia = Sequence([]),
    trailing_trivia = Sequence([]),
    versions = ["1.0.0"],
    sections = [
        Section(
            title = "Section One",
            topics = [Topic(
                title = "Topic One",
                lexical_context = ContextA,
                items = [
                    Struct(
                        name = One,
                        lexical_context = ContextB,
                        fields = (
                            entry = Required(Two),
                            field_1 = Required(Three)
                        )
                    ),
                    Token(name = Two, definitions = [TokenDefinition(Atom("two"))]),
                    Token(name = Three, definitions = [TokenDefinition(Atom("three"))])
                ]
            )]
        ),
        Section(
            title = "Section Two",
            topics = [Topic(
                title = "Topic Two",
                lexical_context = ContextB,
                items = [Token(name = Four, definitions = [TokenDefinition(Atom("four"))])]
            )]
        )
    ],
    built_ins = []
));

fn main() {}
