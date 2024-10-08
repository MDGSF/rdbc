use super::common_parsers::*;
use super::error::DbcParseError;
use super::value_descriptions::parser_value_descriptions;
use super::value_descriptions::ValueDescriptions;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt;

/// VAL_ message_id signal_name [value_descriptions];
/// VAL_ 2147487969 Value1 3 "Three" 2 "Two" 1 "One" 0 "Zero" ;
/// VAL_ 2147487969 Value0 2 "Value2" 1 "Value1" 0 "Value0" ;
#[derive(PartialEq, Debug, Clone)]
pub struct SignalValueDescriptions {
    pub message_id: u32,
    pub signal_name: String,
    pub value_descriptions: ValueDescriptions,
}

impl fmt::Display for SignalValueDescriptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VAL_ {} {} {};",
            self.message_id, self.signal_name, self.value_descriptions
        )
    }
}

pub fn parser_signal_value_descriptions(
    input: &str,
) -> IResult<&str, SignalValueDescriptions, DbcParseError> {
    let res = map(
        tuple((
            multispacey(tag("VAL_")),
            spacey(parser_message_id),
            spacey(parser_signal_name),
            spacey(parser_value_descriptions),
            spacey(tag(";")),
            many0(line_ending),
        )),
        |(_, message_id, signal_name, value_descriptions, _, _)| SignalValueDescriptions {
            message_id,
            signal_name: signal_name.to_string(),
            value_descriptions,
        },
    )(input);

    match res {
        Ok((remain, val)) => Ok((remain, val)),
        Err(e) => {
            log::trace!("parse signal value descriptions failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadSignalValueDescriptions))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::value_descriptions::ValueDescriptionItem;

    #[test]
    fn test_parser_signal_value_descriptions_01() {
        assert_eq!(
            parser_signal_value_descriptions(
                r#"VAL_ 2147487969 Value1 3 "Three" 2 "Two" 1 "One" 0 "Zero" ;"#
            ),
            Ok((
                "",
                SignalValueDescriptions {
                    message_id: 2147487969,
                    signal_name: "Value1".to_string(),
                    value_descriptions: ValueDescriptions {
                        values: vec![
                            ValueDescriptionItem {
                                num: 3,
                                str: "Three".to_string()
                            },
                            ValueDescriptionItem {
                                num: 2,
                                str: "Two".to_string()
                            },
                            ValueDescriptionItem {
                                num: 1,
                                str: "One".to_string()
                            },
                            ValueDescriptionItem {
                                num: 0,
                                str: "Zero".to_string()
                            }
                        ]
                    }
                }
            )),
        );
    }

    #[test]
    fn test_signal_value_descriptions_string_01() {
        assert_eq!(
            SignalValueDescriptions {
                message_id: 2147487969,
                signal_name: "Value0".to_string(),
                value_descriptions: ValueDescriptions {
                    values: vec![
                        ValueDescriptionItem {
                            num: 2,
                            str: "Value2".to_string()
                        },
                        ValueDescriptionItem {
                            num: 1,
                            str: "Value1".to_string()
                        },
                        ValueDescriptionItem {
                            num: 0,
                            str: "Value0".to_string()
                        }
                    ]
                }
            }
            .to_string(),
            r#"VAL_ 2147487969 Value0 2 "Value2" 1 "Value1" 0 "Value0";"#,
        );
    }
}
