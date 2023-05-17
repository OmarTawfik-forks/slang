use std::{
    cmp::{max, min},
    path::PathBuf,
    rc::Rc,
    str::Chars,
};

use codegen_utils::{
    context::CodegenContext,
    errors::{CodegenErrors, CodegenResult, Position},
};
use indexmap::IndexMap;
use serde::de::DeserializeOwned;
use yaml_rust::{
    parser::Parser as YamlParser,
    scanner::{Marker, TScalarStyle},
    Event,
};

use super::super::SourceLocation;

use super::cst::{Node, NodeField, NodeFieldRef, NodeRef};

pub trait WithSourceLocation {
    fn set_source_location(&mut self, location: SourceLocation);
}

impl<T: WithSourceLocation> WithSourceLocation for Vec<T> {
    fn set_source_location(&mut self, location: SourceLocation) {
        for (index, production) in self.iter_mut().enumerate() {
            let source_location = SourceLocation {
                path: location.path.clone(),
                node: location.node.value_at_index(index),
            };
            production.set_source_location(source_location);
        }
    }
}

pub fn parse_yaml_file<R: WithSourceLocation + DeserializeOwned>(
    codegen: &mut CodegenContext,
    path: &PathBuf,
) -> CodegenResult<R> {
    let source = codegen.read_file(&path).unwrap();

    let parser = &mut YamlParser::new(source.chars());
    assert_eq!(consume(path, parser)?.event, Event::StreamStart);
    assert_eq!(consume(path, parser)?.event, Event::DocumentStart);
    let node = parse_value(path, parser)?;
    assert_eq!(consume(path, parser)?.event, Event::DocumentEnd);
    assert_eq!(consume(path, parser)?.event, Event::StreamEnd);

    match serde_yaml::from_str::<R>(&source) {
        Ok(mut value) => {
            value.set_source_location(SourceLocation {
                path: Rc::new(path.clone()),
                node,
            });
            Ok(value)
        }
        Err(error) => {
            let range = {
                let location = error.location().unwrap();
                let position = Position::new(location.index(), location.line(), location.column());
                node.pinpoint(&position)
                    .map_or_else(|| position..position, |node| node.range().to_owned())
            };
            Err(CodegenErrors::single(&path, &range, Errors::Serde(error)))
        }
    }
}

fn parse_value(path: &PathBuf, parser: &mut YamlParser<Chars>) -> CodegenResult<NodeRef> {
    let Token {
        event: current,
        position: start,
    } = consume(path, parser)?;

    let value = match current {
        Event::Scalar(value, style, ..) => {
            let contents = match style {
                TScalarStyle::SingleQuoted => format!("'{value}'"),
                TScalarStyle::DoubleQuoted => format!("\"{value}\""),
                _ => value,
            };

            let lines: Vec<&str> = contents.lines().collect();
            let lines_count = lines.len();

            let end = if lines_count < 2 {
                let line_length = contents.chars().count();
                Position::new(
                    start.offset + line_length,
                    start.line,
                    start.column + line_length,
                )
            } else {
                let last_line_length = lines.last().unwrap().chars().count();
                Position::new(
                    start.offset + last_line_length,
                    start.line + lines_count - 1,
                    last_line_length,
                )
            };

            Node::Value { range: start..end }
        }
        Event::SequenceStart(_) => {
            let mut items = Vec::new();

            let mut start = start;
            let mut end = loop {
                if peek(path, parser)?.event == Event::SequenceEnd {
                    break consume(path, parser)?.position;
                }

                items.push(parse_value(path, parser)?);
            };

            if !items.is_empty() {
                start = min(start, items.first().unwrap().range().start);
                end = max(end, items.last().unwrap().range().end);
            }

            Node::Array {
                range: start..end,
                items,
            }
        }
        Event::MappingStart(_) => {
            let mut fields = IndexMap::new();

            let mut start = start;
            let mut end = loop {
                let property_name = match peek(path, parser)?.event {
                    Event::MappingEnd => {
                        break consume(path, parser)?.position;
                    }
                    Event::Scalar(value, ..) => value,
                    _ => unreachable!("Unexpected property name"),
                };

                let key = parse_value(path, parser)?;
                let value = parse_value(path, parser)?;
                fields.insert(property_name, NodeFieldRef::new(NodeField { key, value }));
            };

            if !fields.is_empty() {
                start = min(start, fields.first().unwrap().1.key.range().start);
                end = max(end, fields.last().unwrap().1.value.range().end);
            }

            Node::Object {
                range: start..end,
                fields,
            }
        }
        Event::Nothing
        | Event::Alias(_)
        | Event::DocumentEnd
        | Event::DocumentStart
        | Event::MappingEnd
        | Event::SequenceEnd
        | Event::StreamEnd
        | Event::StreamStart => {
            unreachable!("Unexpected event '{current:?}' at {start:?}")
        }
    };

    return Ok(NodeRef::new(value));
}

fn consume(path: &PathBuf, parser: &mut YamlParser<Chars>) -> CodegenResult<Token> {
    let token = peek(path, parser)?;
    parser.next().unwrap(); // advance the iterator
    return Ok(token);
}

fn peek(path: &PathBuf, parser: &mut YamlParser<Chars>) -> CodegenResult<Token> {
    match parser.peek() {
        Ok((event, marker)) => Ok(Token {
            event: event.to_owned(),
            position: marker_to_position(marker),
        }),
        Err(error) => {
            let position = marker_to_position(error.marker());
            let range = position..position;
            Err(CodegenErrors::single(path, &range, Errors::Parser(error)))
        }
    }
}

struct Token {
    event: Event,
    position: Position,
}

fn marker_to_position(marker: &Marker) -> Position {
    return Position {
        offset: marker.index(),
        line: marker.line(),
        column: marker.col() + 1, // parser uses 0-based columns
    };
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("{0}")]
    Parser(yaml_rust::ScanError),
    #[error("{0}")]
    Serde(serde_yaml::Error),
}
