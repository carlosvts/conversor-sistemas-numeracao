use super::dto::{
    InputKind, MaximumValueRequest, ParsedConversionRequest, RawConversionInput, RawMaximumInput,
    Sign,
};
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

fn resolve_source_base<'a>(
    unsigned_str: &'a str,
    hint: Option<u8>,
    allow_prefix_detection: bool,
) -> Result<(Option<String>, u8, &'a str), ParseError> {
    if !allow_prefix_detection {
        return Ok((None, hint.unwrap_or(10), unsigned_str));
    }

    match detect_prefix(unsigned_str) {
        None => Ok((None, hint.unwrap_or(10), unsigned_str)),
        Some((prefix_base, prefix_str, rest)) => {
            if let Some(hint_base) = hint {
                if hint_base != prefix_base {
                    return Err(ParseError::PrefixConflict {
                        hinted_base: hint_base,
                        detected_base: prefix_base,
                    });
                }
            }
            Ok((Some(prefix_str), prefix_base, rest))
        }
    }
}

fn validate_base(base: u8) -> Result<(), ParseError> {
    if base >= 2 && base <= 36 {
        Ok(())
    } else {
        Err(ParseError::InvalidBase(base))
    }
}

fn validate_digits(digits: &[char], base: u8) -> Result<(), ParseError> {
    for &c in digits {
        let value: u8 = match c {
            '0'..='9' => c as u8 - b'0',
            'A'..='Z' => c as u8 - b'A' + 10,
            _ => return Err(ParseError::InvalidDigit { digit: c, base }),
        };
        if value >= base {
            return Err(ParseError::InvalidDigit { digit: c, base });
        }
    }
    Ok(())
}

fn detect_prefix(value: &str) -> Option<(u8, String, &str)> {
    let lower = value.to_ascii_lowercase();
    if lower.starts_with("0x") {
        Some((16, "0x".to_string(), &value[2..]))
    } else if lower.starts_with("0b") {
        Some((2, "0b".to_string(), &value[2..]))
    } else if lower.starts_with("0o") {
        Some((8, "0o".to_string(), &value[2..]))
    } else {
        None
    }
}

impl ConversionRequestParser for ConversionParser {
    fn parse_conversion(
        &self,
        input: RawConversionInput,
    ) -> Result<ParsedConversionRequest, ParseError> {
        // 1. Vazio
        let raw = input.raw_value.trim();
        if raw.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        // 2. Sinal
        let (sign, unsigned_str) = if raw.starts_with('-') {
            (Sign::Negative, &raw[1..])
        } else if raw.starts_with('+') {
            (Sign::Positive, &raw[1..])
        } else {
            (Sign::Positive, raw)
        };

        if unsigned_str.is_empty() {
            return Err(ParseError::EmptyInput);
        }

        // 3. Prefixo e source_base
        let (detected_prefix, source_base, value_body) = resolve_source_base(
            unsigned_str,
            input.source_base_hint,
            input.options.allow_prefix_detection,
        )?;

        validate_base(source_base)?;
        validate_base(input.target_base)?;

        if value_body.is_empty() {
            return Err(ParseError::InvalidFormat(
                "Valor vazio após prefixo".to_string(),
            ));
        }

        // 4. Parte inteira e fracionária
        let (integer_str, fractional_str, kind) = match value_body.find('.') {
            Some(dot_pos) => {
                if !input.options.allow_fractional_part {
                    return Err(ParseError::FractionalInputDisabled);
                }
                (
                    &value_body[..dot_pos],
                    &value_body[dot_pos + 1..],
                    InputKind::Fractional,
                )
            }
            None => (value_body, "", InputKind::Integer),
        };

        if integer_str.is_empty() {
            return Err(ParseError::InvalidFormat("Parte inteira vazia".to_string()));
        }

        // 5. Valida dígitos
        let integer_digits: Vec<char> = integer_str.to_ascii_uppercase().chars().collect();
        let fractional_digits: Vec<char> = fractional_str.to_ascii_uppercase().chars().collect();

        validate_digits(&integer_digits, source_base)?;
        validate_digits(&fractional_digits, source_base)?;

        // 6. Normaliza (remove zeros à esquerda)
        let normalized_int = {
            let trimmed = integer_str.trim_start_matches('0');
            if trimmed.is_empty() {
                "0".to_string()
            } else {
                trimmed.to_ascii_uppercase()
            }
        };

        let normalized_value = match kind {
            InputKind::Integer => normalized_int,
            InputKind::Fractional => {
                format!("{}.{}", normalized_int, fractional_str.to_ascii_uppercase())
            }
        };

        // 7. Monta o resultado
        Ok(ParsedConversionRequest {
            original_input: input,
            normalized_value,
            detected_source_base: source_base,
            sign,
            kind,
            detected_prefix,
            integer_digits,
            fractional_digits,
        })
    }

    fn parse_maximum_value(
        &self,
        input: RawMaximumInput,
    ) -> Result<MaximumValueRequest, ParseError> {
        validate_base(input.base)?;

        if input.digit_count == 0 {
            return Err(ParseError::InvalidFormat(
                "Número de dígitos deve ser maior que zero".to_string(),
            ));
        }

        Ok(MaximumValueRequest {
            base: input.base,
            digit_count: input.digit_count,
        })
    }
}
