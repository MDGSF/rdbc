use nom::error::ContextError;
use nom::error::{ErrorKind, ParseError};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum DbcParseError {
    #[error("bad version")]
    BadVersion,
    #[error("bad names")]
    BadNames,
    #[error("bad bit timing")]
    BadBitTiming,
    #[error("bad bit timing value")]
    BadBitTimingValue,
    #[error("bad can nodes")]
    BadCanNodes,
    #[error("bad signal")]
    BadSignal,
    #[error("bad message header")]
    BadMessageHeader,
    #[error("bad comment")]
    BadComment,
    #[error("bad network comment")]
    BadNetworkComment,
    #[error("bad node comment")]
    BadNodeComment,
    #[error("bad message comment")]
    BadMessageComment,
    #[error("bad signal comment")]
    BadSignalComment,

    #[error("bad environment variable")]
    BadEnvironmentVariable,
    #[error("bad environment variable data")]
    BadEnvironmentVariableData,
    #[error("bad environment variable comment")]
    BadEnvironmentVariableComment,

    #[error("bad signal value descriptions")]
    BadSignalValueDescriptions,
    #[error("bad environment variable value descriptions")]
    BadEnvironmentVariableValueDescriptions,

    #[error("bad attribute integer value type")]
    BadAttributeIntegerValueType,
    #[error("bad attribute hex value type")]
    BadAttributeHexValueType,
    #[error("bad attribute float value type")]
    BadAttributeFloatValueType,
    #[error("bad attribute string value type")]
    BadAttributeStringValueType,
    #[error("bad attribute enum value type")]
    BadAttributeEnumValueType,

    #[error("bad network attribute")]
    BadNetworkAttribute,
    #[error("bad node attribute")]
    BadNodeAttribute,
    #[error("bad message attribute")]
    BadMessageAttribute,
    #[error("bad signal attribute")]
    BadSignalAttribute,

    #[error("bad integer")]
    BadInt,
    #[error("bad float")]
    BadFloat,
    #[error("bad escape sequence")]
    BadEscape,
    #[error("unknown parser error")]
    Unparseable,
    #[error("invalid c identifier")]
    InvalidCIdentifier,
    #[error("invalid dbc identifier")]
    UseKeywordAsIdentifier,
    #[error("debug message")]
    DebugMsg(String),
}

impl ParseError<&str> for DbcParseError {
    // on one line, we show the error code and the input that caused it
    fn from_error_kind(input: &str, kind: ErrorKind) -> Self {
        let message = format!("{:?}:\t{:?}\n", kind, input);
        log::debug!("{}", message);
        DbcParseError::DebugMsg(message)
    }

    // if combining multiple errors, we show them one after the other
    fn append(input: &str, kind: ErrorKind, other: Self) -> Self {
        let message = format!("{}{:?}:\t{:?}\n", other, kind, input);
        log::debug!("{}", message);
        DbcParseError::DebugMsg(message)
    }

    fn from_char(input: &str, c: char) -> Self {
        let message = format!("'{}':\t{:?}\n", c, input);
        log::debug!("{}", message);
        DbcParseError::DebugMsg(message)
    }

    fn or(self, other: Self) -> Self {
        let message = format!("{}\tOR\n{}\n", self, other);
        log::debug!("{}", message);
        DbcParseError::DebugMsg(message)
    }
}

impl ContextError<&str> for DbcParseError {
    fn add_context(input: &str, ctx: &'static str, other: Self) -> Self {
        let message = format!("{}\"{}\":\t{:?}\n", other, ctx, input);
        log::debug!("{}", message);
        DbcParseError::DebugMsg(message)
    }
}
