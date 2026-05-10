#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputKind {
    Integer,
    Fractional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessingWarning {
    FractionTruncated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConversionOptions {
    pub allow_prefix_detection: bool,
    pub allow_fractional_part: bool,
    pub uppercase_output: bool,
    pub generate_trace: bool,
    pub max_fractional_digits: u8,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            allow_prefix_detection: true,
            allow_fractional_part: true,
            uppercase_output: true,
            generate_trace: false,
            max_fractional_digits: 16,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawConversionInput {
    pub raw_value: String,
    pub source_base_hint: Option<u8>,
    pub target_base: u8,
    pub options: ConversionOptions,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedConversionRequest {
    pub original_input: RawConversionInput,
    pub normalized_value: String,
    pub detected_source_base: u8,
    pub sign: Sign,
    pub kind: InputKind,
    pub detected_prefix: Option<String>,
    pub integer_digits: Vec<char>,
    pub fractional_digits: Vec<char>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawMaximumInput {
    pub base: u8,
    pub digit_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaximumValueRequest {
    pub base: u8,
    pub digit_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversionResult {
    pub output_value: String,
    pub source_base: u8,
    pub target_base: u8,
    pub warnings: Vec<ProcessingWarning>,
    pub trace: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaximumValueResult {
    pub expression: String,
    pub value: String,
    pub base: u8,
    pub digit_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchLineError {
    pub line_index: usize,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchResult {
    pub results: Vec<ConversionResult>,
    pub errors: Vec<BatchLineError>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuizDifficulty(pub u8);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuizQuestion {
    pub value: String,
    pub source_base: u8,
    pub target_base: u8,
    pub difficulty: QuizDifficulty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuizAnswer {
    pub answer: String,
    pub response_time_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuizResult {
    pub is_correct: bool,
    pub expected_answer: String,
    pub score: u32,
}