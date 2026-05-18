use super::dto::{ConversionResult, MaximumValueResult, RawConversionInput, RawMaximumInput};
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

// generico P de parser e generico S de service
impl<P, S> ConversionFacade<P, S>
where
    P: ConversionRequestParser,
    S: NumericProcessingService,
{
    pub fn new(parser: P, processor: S) -> Self {
        Self { parser, processor }
    }

    pub fn request(&self, input: RawConversionInput) -> Result<ConversionResult, FacadeError> {
        let parsed = self
            .parser
            .parse_conversion(input)
            .map_err(FacadeError::Parse)?;
        self.processor
            .convert(parsed)
            .map_err(FacadeError::Processing)
    }

    pub fn request_maximum(
        &self,
        input: RawMaximumInput,
    ) -> Result<MaximumValueResult, FacadeError> {
        let parsed = self
            .parser
            .parse_maximum_value(input)
            .map_err(FacadeError::Parse)?;
        self.processor
            .compute_maximum(parsed)
            .map_err(FacadeError::Processing)
    }
}
