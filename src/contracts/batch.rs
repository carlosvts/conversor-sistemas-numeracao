use super::dto::BatchResult;
use super::errors::BatchError;
use super::facade::ConversionFacade;
use super::parser::{ConversionParser, ConversionRequestParser};
use super::processor::{ConversionProcessor, NumericProcessingService};

#[derive(Debug, Clone)]
pub struct BatchService<P, S>
where
    P: ConversionRequestParser,
    S: NumericProcessingService,
{
    facade: ConversionFacade<P, S>,
}

impl BatchService<ConversionParser, ConversionProcessor> {
    pub fn new_default() -> Self {
        Self {
            facade: ConversionFacade::new_default(),
        }
    }
}

impl<P, S> BatchService<P, S>
where
    P: ConversionRequestParser,
    S: NumericProcessingService,
{
    pub fn new(facade: ConversionFacade<P, S>) -> Self {
        Self { facade }
    }

    pub fn facade(&self) -> &ConversionFacade<P, S> {
        &self.facade
    }

    pub fn process_csv_path(&self, _path: &str) -> Result<BatchResult, BatchError> {
        todo!("process_csv_path skeleton only")
    }
}