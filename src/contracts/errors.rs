#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    EmptyInput,
    InvalidBase(u8),
    InvalidDigit { digit: char, base: u8 },
    InvalidFormat(String),
    FractionalInputDisabled,
    PrefixConflict { hinted_base: u8, detected_base: u8 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessingError {
    InvalidRequest(String),
    UnsupportedBase(u8),
    Overflow,
    DivisionByZero,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FacadeError {
    Parse(ParseError),
    Processing(ProcessingError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchError {
    Io(String),
    Csv(String),
    InvalidHeader(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuizError {
    InvalidDifficulty(u8),
    MissingActiveQuestion,
    InvalidAnswerFormat(String),
    Processing(ProcessingError),
}