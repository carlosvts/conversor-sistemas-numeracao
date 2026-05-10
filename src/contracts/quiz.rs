use super::dto::{QuizAnswer, QuizQuestion, QuizResult};
use super::errors::QuizError;
use super::processor::{ConversionProcessor, NumericProcessingService};

#[derive(Debug, Clone)]
pub struct QuizService<S>
where
    S: NumericProcessingService,
{
    processor: S,
}

impl QuizService<ConversionProcessor> {
    pub fn new_default() -> Self {
        Self {
            processor: ConversionProcessor::new(),
        }
    }
}

impl<S> QuizService<S>
where
    S: NumericProcessingService,
{
    pub fn new(processor: S) -> Self {
        Self { processor }
    }

    pub fn processor(&self) -> &S {
        &self.processor
    }

    pub fn generate_question(&self, _difficulty: u8) -> Result<QuizQuestion, QuizError> {
        todo!("generate_question skeleton only")
    }

    pub fn submit_answer(
        &self,
        _question: &QuizQuestion,
        _answer: QuizAnswer,
    ) -> Result<QuizResult, QuizError> {
        todo!("submit_answer skeleton only")
    }
}