use codegen_schema::types::parser::{ParserDefinition, ParserRef};

use crate::ebnf_writer::{EBNFWritable, EBNFWriter};

impl<T: EBNFWriter> EBNFWritable<T> for ParserRef {
    fn write_ebnf(&self, name: &str, writer: &mut T) {
        writer.write_global_definition(name);
        writer.write_operator(" = ");
        self.definition.write_ebnf("", writer);
        writer.write_operator(" ;");
    }
}

impl<T: EBNFWriter> EBNFWritable<T> for ParserDefinition {
    fn write_ebnf(&self, _name: &str, w: &mut T) {
        match self {
            ParserDefinition::Choice(sub_exprs) => {
                let mut first = true;
                for sub_expr in sub_exprs {
                    if first {
                        first = false;
                    } else {
                        w.write_operator(" | ");
                    }
                    write_nested(w, self, &sub_expr.definition);
                }
            }

            ParserDefinition::DelimitedBy {
                open,
                expression,
                close,
            } => {
                w.write_global_reference(&open.reference);
                w.write_operator(" ");
                write_nested(w, &expression.definition, &expression.definition);
                w.write_operator(" ");
                w.write_global_reference(&close.reference);
            }

            ParserDefinition::OneOrMore(expr) => {
                w.write_constant("1");
                w.write_operator("…");
                w.write_operator("{ ");
                expr.definition.write_ebnf("", w);
                w.write_operator(" }");
            }

            ParserDefinition::Optional(expr) => {
                w.write_operator("[ ");
                expr.definition.write_ebnf("", w);
                w.write_operator(" ]");
            }

            ParserDefinition::Reference(name) => w.write_global_reference(name),

            ParserDefinition::Repeat {
                min,
                max,
                expression,
            } => {
                w.write_constant(&min.to_string());
                w.write_operator("…");
                w.write_constant(&max.to_string());
                w.write_operator("{ ");
                expression.definition.write_ebnf("", w);
                w.write_operator(" }");
            }

            ParserDefinition::SeparatedBy {
                expression,
                separator,
            } => {
                write_nested(w, &expression.definition, &expression.definition);
                w.write_global_reference(&separator.reference);
                w.write_operator(" { ");
                w.write_global_reference(&separator.reference);
                w.write_operator(" ");
                write_nested(w, &expression.definition, &expression.definition);
                w.write_operator(" }");
            }

            ParserDefinition::Sequence(sub_exprs) => {
                let mut first = true;
                for sub_expr in sub_exprs {
                    if first {
                        first = false;
                    } else {
                        w.write_operator(" ");
                    }
                    write_nested(w, self, &sub_expr.definition);
                }
            }

            ParserDefinition::TerminatedBy {
                expression,
                terminator,
            } => {
                write_nested(w, &expression.definition, &expression.definition);
                w.write_operator(" ");
                w.write_global_reference(&terminator.reference);
            }

            ParserDefinition::ZeroOrMore(expr) => {
                w.write_operator("{ ");
                expr.definition.write_ebnf("", w);
                w.write_operator(" }");
            }
        }
    }
}

fn write_nested<W: EBNFWriter>(
    writer: &mut W,
    parent_definition: &ParserDefinition,
    parser_definition: &ParserDefinition,
) {
    if precedence(parent_definition) < precedence(parser_definition) {
        writer.write_operator("( ");
        parser_definition.write_ebnf("", writer);
        writer.write_operator(" )");
    } else {
        parser_definition.write_ebnf("", writer);
    }
}

fn precedence(parser_definition: &ParserDefinition) -> u8 {
    match parser_definition {
        ParserDefinition::OneOrMore(..)
        | ParserDefinition::Optional(..)
        | ParserDefinition::Reference(..)
        | ParserDefinition::Repeat { .. }
        | ParserDefinition::SeparatedBy { .. }
        | ParserDefinition::TerminatedBy { .. }
        | ParserDefinition::ZeroOrMore(..) => 0,
        ParserDefinition::DelimitedBy { .. } | ParserDefinition::Sequence(..) => 1,
        ParserDefinition::Choice(..) => 2,
    }
}
