use super::dto::{ConversionResult, RawConversionInput, RawMaximumInput};
use super::errors::FacadeError;
use super::parser::{ConversionParser, ConversionRequestParser};
use super::processor::{ConversionProcessor, NumericProcessingService};

#[derive(Debug, Clone)]
pub struct ConversionFacade<P, S>
where
    P: ConversionRequestParser,
    S: NumericProcessingService,
{
    parser: P,
    processor: S,
}

impl ConversionFacade<ConversionParser, ConversionProcessor> {
    pub fn new_default() -> Self {
        Self {
            parser: ConversionParser::new(),
            processor: ConversionProcessor::new(),
        }
    }
}

impl<P, S> ConversionFacade<P, S>
where
    P: ConversionRequestParser,
    S: NumericProcessingService,
{
    pub fn new(parser: P, processor: S) -> Self {
        Self { parser, processor }
    }

    pub fn parser(&self) -> &P {
        &self.parser
    }

    pub fn processor(&self) -> &S {
        &self.processor
    }

    pub fn request(&self, _input: RawConversionInput) -> Result<ConversionResult, FacadeError> {
        todo!("request skeleton only")
    }

    pub fn request_maximum(&self, _input: RawMaximumInput) -> Result<super::dto::MaximumValueResult, FacadeError> {
        todo!("request_maximum skeleton only")
    }
}