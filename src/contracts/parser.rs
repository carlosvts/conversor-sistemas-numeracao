use super::dto::{MaximumValueRequest, ParsedConversionRequest, RawConversionInput, RawMaximumInput};
use super::errors::ParseError;

pub trait ConversionRequestParser {
    fn parse_conversion(
        &self,
        input: RawConversionInput,
    ) -> Result<ParsedConversionRequest, ParseError>;

    fn parse_maximum_value(
        &self,
        input: RawMaximumInput,
    ) -> Result<MaximumValueRequest, ParseError>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ConversionParser;

impl ConversionParser {
    pub fn new() -> Self {
        Self
    }
}

impl ConversionRequestParser for ConversionParser {
    fn parse_conversion(
        &self,
        _input: RawConversionInput,
    ) -> Result<ParsedConversionRequest, ParseError> {
        todo!("parse_conversion skeleton only")
    }

    fn parse_maximum_value(
        &self,
        _input: RawMaximumInput,
    ) -> Result<MaximumValueRequest, ParseError> {
        todo!("parse_maximum_value skeleton only")
    }
}