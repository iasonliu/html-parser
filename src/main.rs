use std::collections::HashMap;

use winnow::{
    ascii::{alpha1, alphanumeric1, multispace0},
    combinator::{delimited, separated, separated_pair, terminated},
    PResult, Parser,
};

fn main() {
    println!("Hello, world!");
}

fn parse_key<'i>(input: &mut &'i str) -> PResult<&'i str> {
    alpha1.parse_next(input)
}

fn parse_val<'i>(input: &mut &'i str) -> PResult<&'i str> {
    delimited('"', alphanumeric1, '"').parse_next(input)
}

/// Parses something like key="val"
fn parse_attribute<'i>(input: &mut &'i str) -> PResult<(&'i str, &'i str)> {
    separated_pair(
        parse_key,
        delimited(multispace0, '=', multispace0),
        parse_val,
    )
    .parse_next(input)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Attributes<'i> {
    kvs: HashMap<&'i str, &'i str>,
}

impl<'i> Attributes<'i> {
    fn parse(input: &mut &'i str) -> PResult<Self> {
        let kvs =
            separated(0.., parse_attribute, terminated(',', multispace0)).parse_next(input)?;
        Ok(Self { kvs })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tag<'i> {
    /// Like 'div'
    tag_type: &'i str,
    attributes: Attributes<'i>,
}

impl<'i> Tag<'i> {
    /// <div width="40", height="100">
    fn parse(input: &mut &'i str) -> PResult<Self> {
        let parse_parts = (alpha1, ' ', Attributes::parse);
        let parse_tag = parse_parts.map(|(tag_type, _space_char, attributes)| Self {
            tag_type,
            attributes,
        });
        let tag = delimited('<', parse_tag, '>').parse_next(input)?;
        Ok(tag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parser_key() {
        let input = "width";
        let actual = parse_key.parse(input).unwrap();
        let expected = "width";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parser_val() {
        let input = r#""width""#;
        let actual = parse_val.parse(input).unwrap();
        let expected = "width";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parser_val1() {
        let input = r#""40""#;
        let actual = parse_val.parse(input).unwrap();
        let expected = "40";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_attribute() {
        let input = r#"width="40""#;
        let actual = parse_attribute.parse(input).unwrap();
        let expected = ("width", "40");
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_parse_attribute_with_spaces() {
        let input = r#"width =   "40""#;
        let actual = parse_attribute.parse(input).unwrap();
        let expected = ("width", "40");
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_attribute() {
        let input = r#"width="40", height = "30""#;
        let actual = Attributes::parse.parse(input).unwrap();
        let expected = Attributes {
            kvs: HashMap::from([("width", "40"), ("height", "30")]),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_tag() {
        let input = r#"<div width="40", height="100">"#;
        let actual = Tag::parse.parse(&input).unwrap();
        let expected = Tag {
            tag_type: "div",
            attributes: Attributes {
                kvs: HashMap::from([("width", "40"), ("height", "100")]),
            },
        };
        assert_eq!(actual, expected);
    }
}
