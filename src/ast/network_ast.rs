use super::bit_timing::parser_bit_timing;
use super::bit_timing::BitTiming;
use super::common_parsers::*;
use super::env_var_value_descriptions::parser_env_var_value_descriptions;
use super::env_var_value_descriptions::EnvironmentVariableValueDescriptions;
use super::error::DbcParseError;
use super::message::*;
use super::new_symbols::parser_new_symbols;
use super::new_symbols::NewSymbols;
use super::nodes::parser_nodes;
use super::nodes::Nodes;
use super::signal_value_descriptions::parser_signal_value_descriptions;
use super::signal_value_descriptions::SignalValueDescriptions;
use super::value_tables::*;
use super::version::parser_version;
use super::version::Version;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub struct NetworkAst {
    // VERSION "xxx"
    pub version: Version,

    // NS_:
    pub new_symbols: NewSymbols,

    // BS_:
    pub bit_timing: Option<BitTiming>,

    // BU_:
    pub nodes: Nodes,

    // VAL_TABLE_
    pub value_tables: Option<Vec<ValueTable>>,

    // BO_
    pub messages: Vec<Message>,

    // VAL_ message_id signal_name [value_descriptions];
    pub signal_value_descriptions: Vec<SignalValueDescriptions>,

    // VAL_ env_var_name [value_descriptions];
    pub env_var_value_descriptions: Vec<EnvironmentVariableValueDescriptions>,
}

impl fmt::Display for NetworkAst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}\n", self.version)?;
        writeln!(f, "{}", self.new_symbols)?;
        if let Some(bc) = &self.bit_timing {
            writeln!(f, "{}", bc)?;
        }
        writeln!(f, "{}", self.nodes)?;
        for message in &self.messages {
            writeln!(f, "{}", message)?;
        }
        Ok(())
    }
}

pub fn dbc_value(input: &str) -> IResult<&str, NetworkAst, DbcParseError> {
    map(
        multispacey(tuple((
            multispacey(parser_version),
            multispacey(parser_new_symbols),
            multispacey(parser_bit_timing),
            multispacey(parser_nodes),
            multispacey(parser_value_tables),
            multispacey(many0(parser_dbc_message)),
            multispacey(many0(parser_signal_value_descriptions)),
            multispacey(many0(parser_env_var_value_descriptions)),
        ))),
        |(
            version,
            new_symbols,
            bit_timing,
            nodes,
            value_tables,
            messages,
            signal_value_descriptions,
            env_var_value_descriptions,
        )| NetworkAst {
            version,
            new_symbols,
            bit_timing,
            nodes,
            value_tables,
            messages,
            signal_value_descriptions,
            env_var_value_descriptions,
        },
    )(input)
}

pub fn parse_dbc(input: &str) -> Result<NetworkAst, DbcParseError> {
    let (_remain, result) = all_consuming(dbc_value)(input).map_err(|nom_err| {
        log::error!("nom_err: {}", nom_err);
        match nom_err {
            nom::Err::Incomplete(_) => unreachable!(),
            nom::Err::Error(e) => e,
            nom::Err::Failure(e) => e,
        }
    })?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::signal;
    use crate::ast::value_descriptions::ValueDescriptionItem;
    use crate::ast::value_descriptions::ValueDescriptions;

    #[test]
    fn test_dbc_01() {
        assert_eq!(
            parse_dbc(
                r#"VERSION "1.0"


NS_:
    BS_
    CM_

BS_:
BU_: ABS DRS_MM5_10

BO_ 117 DRS_RX_ID0: 8 ABS

BO_ 112 MM5_10_TX1: 8 DRS_MM5_10
 SG_ Yaw_Rate : 0|16@1+ (0.005,-163.84) [-163.84|163.83] "°/s"  ABS
 SG_ AY1 : 32|16@1+ (0.000127465,-4.1768) [-4.1768|4.1765] "g"  ABS

"#
            ),
            Ok(NetworkAst {
                version: Version("1.0".into()),
                new_symbols: NewSymbols(vec!["BS_".into(), "CM_".into()]),
                bit_timing: Some(BitTiming { value: None }),
                nodes: Nodes(vec!["ABS".into(), "DRS_MM5_10".into()]),
                value_tables: None,
                messages: vec![
                    Message {
                        header: MessageHeader {
                            id: 117,
                            name: "DRS_RX_ID0".into(),
                            size: 8,
                            transmitter: "ABS".into(),
                        },
                        signals: vec![],
                    },
                    Message {
                        header: MessageHeader {
                            id: 112,
                            name: "MM5_10_TX1".into(),
                            size: 8,
                            transmitter: "DRS_MM5_10".into(),
                        },
                        signals: vec![
                            signal::Signal {
                                name: "Yaw_Rate".into(),
                                multiplexer: None,
                                start_bit: 0,
                                size: 16,
                                byte_order: signal::ByteOrder::LittleEndian,
                                value_type: signal::ValueType::Unsigned,
                                factor: 0.005,
                                offset: -163.84,
                                min: Some(-163.84),
                                max: Some(163.83),
                                unit: Some("°/s".into()),
                                receivers: Some(vec!["ABS".into()]),
                            },
                            signal::Signal {
                                name: "AY1".into(),
                                multiplexer: None,
                                start_bit: 32,
                                size: 16,
                                byte_order: signal::ByteOrder::LittleEndian,
                                value_type: signal::ValueType::Unsigned,
                                factor: 0.000127465,
                                offset: -4.1768,
                                min: Some(-4.1768),
                                max: Some(4.1765),
                                unit: Some("g".into()),
                                receivers: Some(vec!["ABS".into()]),
                            }
                        ],
                    },
                ],
                signal_value_descriptions: vec![],
                env_var_value_descriptions: vec![],
            }),
        );
    }

    #[test]
    fn test_dbc_02() {
        assert_eq!(
            parse_dbc(
                r#"VERSION "1.0"


NS_:
    BS_
    CM_

BS_:
BU_: ABS DRS_MM5_10

VAL_TABLE_ ABS_fault_info 2 "active faults stored" 1 "inactive faults stored" 0 "no faults stored" ;
VAL_TABLE_ vt_WheelSpeedQualifier 5 "InvalidUnderVoltage" 4 "NotCalculated" 3 "ReducedMonitored" 2 "Faulty" 1 "Normal" 0 "NotInitialized" ;


BO_ 117 DRS_RX_ID0: 8 ABS

BO_ 112 MM5_10_TX1: 8 DRS_MM5_10
 SG_ Yaw_Rate : 0|16@1+ (0.005,-163.84) [-163.84|163.83] "°/s"  ABS
 SG_ AY1 : 32|16@1+ (0.000127465,-4.1768) [-4.1768|4.1765] "g"  ABS


VAL_ 2147487969 Value1 3 "Three" 2 "Two" 1 "One" 0 "Zero" ;
VAL_ 2147487969 Value0 2 "Value2" 1 "Value1" 0 "Value0" ;

VAL_ RWEnvVar_wData 2 "Value2" 1 "Value1" 0 "Value0" ;
VAL_ WriteOnlyEnvVar 2 "Value2" 1 "Value1" 0 "Value0" ;
VAL_ ReadOnlyEnvVar 2 "Value2" 1 "Value1" 0 "Value0" ;
"#
            ),
            Ok(NetworkAst {
                version: Version("1.0".into()),
                new_symbols: NewSymbols(vec!["BS_".into(), "CM_".into()]),
                bit_timing: Some(BitTiming { value: None }),
                nodes: Nodes(vec!["ABS".into(), "DRS_MM5_10".into()]),
                value_tables: Some(vec![
                    ValueTable {
                        name: "ABS_fault_info".to_string(),
                        value_descriptions: ValueDescriptions {
                            values: vec![
                                ValueDescriptionItem {
                                    num: 2,
                                    str: "active faults stored".to_string()
                                },
                                ValueDescriptionItem {
                                    num: 1,
                                    str: "inactive faults stored".to_string()
                                },
                                ValueDescriptionItem {
                                    num: 0,
                                    str: "no faults stored".to_string()
                                }
                            ]
                        }
                    },
                    ValueTable {
                        name: "vt_WheelSpeedQualifier".to_string(),
                        value_descriptions: ValueDescriptions {
                            values: vec![
                                ValueDescriptionItem {
                                    num: 5,
                                    str: "InvalidUnderVoltage".to_string()
                                },
                                ValueDescriptionItem {
                                    num: 4,
                                    str: "NotCalculated".to_string()
                                },
                                ValueDescriptionItem {
                                    num: 3,
                                    str: "ReducedMonitored".to_string()
                                },
                                ValueDescriptionItem {
                                    num: 2,
                                    str: "Faulty".to_string()
                                },
                                ValueDescriptionItem {
                                    num: 1,
                                    str: "Normal".to_string()
                                },
                                ValueDescriptionItem {
                                    num: 0,
                                    str: "NotInitialized".to_string()
                                }
                            ]
                        }
                    }
                ]),
                messages: vec![
                    Message {
                        header: MessageHeader {
                            id: 117,
                            name: "DRS_RX_ID0".into(),
                            size: 8,
                            transmitter: "ABS".into(),
                        },
                        signals: vec![],
                    },
                    Message {
                        header: MessageHeader {
                            id: 112,
                            name: "MM5_10_TX1".into(),
                            size: 8,
                            transmitter: "DRS_MM5_10".into(),
                        },
                        signals: vec![
                            signal::Signal {
                                name: "Yaw_Rate".into(),
                                multiplexer: None,
                                start_bit: 0,
                                size: 16,
                                byte_order: signal::ByteOrder::LittleEndian,
                                value_type: signal::ValueType::Unsigned,
                                factor: 0.005,
                                offset: -163.84,
                                min: Some(-163.84),
                                max: Some(163.83),
                                unit: Some("°/s".into()),
                                receivers: Some(vec!["ABS".into()]),
                            },
                            signal::Signal {
                                name: "AY1".into(),
                                multiplexer: None,
                                start_bit: 32,
                                size: 16,
                                byte_order: signal::ByteOrder::LittleEndian,
                                value_type: signal::ValueType::Unsigned,
                                factor: 0.000127465,
                                offset: -4.1768,
                                min: Some(-4.1768),
                                max: Some(4.1765),
                                unit: Some("g".into()),
                                receivers: Some(vec!["ABS".into()]),
                            }
                        ],
                    },
                ],
                signal_value_descriptions: vec![
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
                    },
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
                    },
                ],
                env_var_value_descriptions: vec![
                    EnvironmentVariableValueDescriptions {
                        env_var_name: "RWEnvVar_wData".to_string(),
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
                    },
                    EnvironmentVariableValueDescriptions {
                        env_var_name: "WriteOnlyEnvVar".to_string(),
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
                    },
                    EnvironmentVariableValueDescriptions {
                        env_var_name: "ReadOnlyEnvVar".to_string(),
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
                    },
                ],
            }),
        );
    }
}
