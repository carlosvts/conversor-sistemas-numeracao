use super::dto::{ConversionResult, MaximumValueRequest, MaximumValueResult, ParsedConversionRequest};
use super::errors::ProcessingError;

pub trait NumericProcessingService {
    fn convert(&self, request: ParsedConversionRequest) -> Result<ConversionResult, ProcessingError>;

    fn compute_maximum(
        &self,
        request: MaximumValueRequest,
    ) -> Result<MaximumValueResult, ProcessingError>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct ConversionProcessor;

impl ConversionProcessor {
    pub fn new() -> Self {
        Self
    }
}

impl NumericProcessingService for ConversionProcessor {
    fn convert(&self, _request: ParsedConversionRequest) -> Result<ConversionResult, ProcessingError> {
        todo!("convert skeleton only")
    }

    fn compute_maximum(
        &self,
        _request: MaximumValueRequest,
    ) -> Result<MaximumValueResult, ProcessingError> {
        todo!("compute_maximum skeleton only")
    }
}