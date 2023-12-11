use winnow::{
    ascii::{alpha1, alphanumeric1, multispace0},
    combinator::{delimited, separated_pair},
    PResult, Parser,
};

fn main() {
    println!("Hello, world!");
}

fn parse_key<'i>(input: &mut &'i str) -> PResult<&'i str> {
    let key = alpha1.parse_next(input)?;
    Ok(key)
}

fn parse_val<'i>(input: &mut &'i str) -> PResult<&'i str> {
    let val = delimited('"', alphanumeric1, '"').parse_next(input)?;
    Ok(val)
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
}
// <div windth="40" height="100">
