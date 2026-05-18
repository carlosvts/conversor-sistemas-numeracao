use super::dto::{
    ConversionResult, InputKind, MaximumValueRequest, MaximumValueResult, ParsedConversionRequest,
    Sign,
};
use super::errors::ProcessingError;
use std::collections::HashMap;

pub trait NumericProcessingService {
    fn convert(
        &self,
        request: ParsedConversionRequest,
    ) -> Result<ConversionResult, ProcessingError>;

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

// ─── Como as funcoes usam char slice, adaptei as funcoes do core para receber &[char] ────────────────

fn maximum(base: u32, num_digits: u32) -> String {
    let max = base.pow(num_digits) - 1;
    let suffix = format!("{base}^{num_digits} - 1 = ");
    suffix + &max.to_string()
}

fn bin_to_oct(bin: &[char]) -> String {
    let mut oct = String::new();
    let mut count: u8 = 0;
    let mut group: u8 = 0;

    for &b in bin.iter().rev() {
        let bit = b as u8 - b'0';
        group |= bit << count;
        count += 1;
        if count == 3 {
            oct.push_str(&group.to_string());
            count = 0;
            group = 0;
        }
    }

    if count > 0 {
        oct.push((group + b'0') as char);
    }

    oct.chars().rev().collect()
}

fn oct_to_bin(oct: &[char]) -> String {
    let mut bin = String::new();
    let mut buffer: u32 = 0;
    let mut total_bits = 0;

    for &c in oct.iter() {
        let result = c as u8 - b'0';
        buffer <<= 3;
        buffer |= result as u32;
        total_bits += 3;
    }

    for i in (0..total_bits).rev() {
        let bit = (buffer >> i) & 1;
        match bit {
            1 => bin.push('1'),
            0 => bin.push('0'),
            _ => bin.push('X'),
        };
    }
    bin
}

fn hex_to_bin(hex: &[char]) -> String {
    let mut bin = String::new();

    let mut map: HashMap<char, &str> = HashMap::new();
    map.insert('0', "0000");
    map.insert('1', "0001");
    map.insert('2', "0010");
    map.insert('3', "0011");
    map.insert('4', "0100");
    map.insert('5', "0101");
    map.insert('6', "0110");
    map.insert('7', "0111");
    map.insert('8', "1000");
    map.insert('9', "1001");
    map.insert('A', "1010");
    map.insert('B', "1011");
    map.insert('C', "1100");
    map.insert('D', "1101");
    map.insert('E', "1110");
    map.insert('F', "1111");

    for &c in hex.iter() {
        match map.get(&c) {
            Some(value) => bin.push_str(value),
            None => (),
        }
    }
    bin
}

fn bin_to_hex(bin: &[char]) -> String {
    // Reconstrói a String para reaproveitar a lógica original de padding e chunks
    let mut bin_str: String = bin.iter().collect();
    let mut hex = String::new();
    let mut chunks: Vec<String> = Vec::new();

    let mut map: HashMap<String, &str> = HashMap::new();
    map.insert("0000".to_string(), "0");
    map.insert("0001".to_string(), "1");
    map.insert("0010".to_string(), "2");
    map.insert("0011".to_string(), "3");
    map.insert("0100".to_string(), "4");
    map.insert("0101".to_string(), "5");
    map.insert("0110".to_string(), "6");
    map.insert("0111".to_string(), "7");
    map.insert("1000".to_string(), "8");
    map.insert("1001".to_string(), "9");
    map.insert("1010".to_string(), "A");
    map.insert("1011".to_string(), "B");
    map.insert("1100".to_string(), "C");
    map.insert("1101".to_string(), "D");
    map.insert("1110".to_string(), "E");
    map.insert("1111".to_string(), "F");

    let binary_lenght = bin_str.chars().count();
    if binary_lenght < 8 {
        let num_padding = 8 - binary_lenght;
        for _ in 0..num_padding {
            bin_str.insert(0, '0');
        }
    }

    let mut count: u16 = 0;
    for b in bin_str.chars() {
        hex.push(b);
        count += 1;
        if count == 4 {
            chunks.push(hex.clone());
            hex.clear();
            count = 0;
        }
    }

    if hex.len() >= 1 {
        chunks.push(hex.clone());
    }
    hex.clear();

    for chunk in chunks.iter_mut().rev() {
        if chunk.len() < 4 {
            let num_padding = 4 - chunk.len();
            for _ in 0..num_padding {
                chunk.insert(0, '0');
            }
        }
        match map.get(chunk) {
            Some(value) => hex.push_str(value),
            None => (),
        }
    }

    hex
}

// ─── Helpers: digit ↔ numeric value ─────────────────────────────────────────

fn digit_to_value(c: char) -> u64 {
    match c {
        '0'..='9' => (c as u64) - ('0' as u64),
        'A'..='Z' => (c as u64) - ('A' as u64) + 10,
        _ => 0,
    }
}

fn value_to_digit(v: u64) -> char {
    if v < 10 {
        (b'0' + v as u8) as char
    } else {
        (b'A' + (v - 10) as u8) as char
    }
}

// ─── F2: any base → decimal via positional summation ─────────────────────────

fn base_to_decimal_value(
    digits: &[char],
    source_base: u8,
    generate_trace: bool,
) -> Result<(u64, Vec<String>), ProcessingError> {
    let base = source_base as u64;
    let len = digits.len();
    let mut result: u64 = 0;
    let mut trace: Vec<String> = Vec::new();

    for (i, &c) in digits.iter().enumerate() {
        let exp = (len - 1 - i) as u32;
        let digit_val = digit_to_value(c);
        let place_value = base.checked_pow(exp).ok_or(ProcessingError::Overflow)?;
        let contribution = digit_val
            .checked_mul(place_value)
            .ok_or(ProcessingError::Overflow)?;
        result = result
            .checked_add(contribution)
            .ok_or(ProcessingError::Overflow)?;
        if generate_trace {
            trace.push(format!(
                "{} x {}^{} = {}",
                digit_val, base, exp, contribution
            ));
        }
    }

    if generate_trace {
        trace.push(format!("Result: {}", result));
    }

    Ok((result, trace))
}

// ─── F1: decimal → any base via successive divisions ─────────────────────────

fn decimal_to_any_base(
    mut n: u64,
    target_base: u8,
    generate_trace: bool,
) -> Result<(String, Vec<String>), ProcessingError> {
    if n == 0 {
        return Ok(("0".to_string(), vec![]));
    }

    let base = target_base as u64;
    let mut digits_rev: Vec<char> = Vec::new();
    let mut trace: Vec<String> = Vec::new();

    while n > 0 {
        let remainder = n % base;
        let quotient = n / base;
        digits_rev.push(value_to_digit(remainder));
        if generate_trace {
            trace.push(format!("{} / {} = {}  r {}", n, base, quotient, remainder));
        }
        n = quotient;
    }

    let result: String = digits_rev.iter().rev().collect();
    if generate_trace {
        trace.push(format!(
            "Result (reading remainders bottom to top): {}",
            result
        ));
    }

    Ok((result, trace))
}

// ─── Router: dispatches to the correct conversion strategy ───────────────────

fn convert_integer(
    digits: &[char],
    source_base: u8,
    target_base: u8,
    generate_trace: bool,
) -> Result<(String, Vec<String>), ProcessingError> {
    // F3/F4: direct bit-grouping for bin/oct/hex pairs (no decimal intermediate)
    match (source_base, target_base) {
        (2, 8) => return Ok((bin_to_oct(digits), vec![])),
        (8, 2) => return Ok((oct_to_bin(digits), vec![])),
        (2, 16) => return Ok((bin_to_hex(digits), vec![])),
        (16, 2) => return Ok((hex_to_bin(digits), vec![])),
        (8, 16) => {
            let bin = oct_to_bin(digits);
            let result = bin_to_hex(&bin.chars().collect::<Vec<char>>());
            return Ok((result, vec![]));
        }
        (16, 8) => {
            let bin = hex_to_bin(digits);
            let result = bin_to_oct(&bin.chars().collect::<Vec<char>>());
            return Ok((result, vec![]));
        }
        _ => {}
    }

    // F1: decimal → any base (successive divisions)
    if source_base == 10 {
        let (decimal_val, _) = base_to_decimal_value(digits, 10, false)?;
        return decimal_to_any_base(decimal_val, target_base, generate_trace);
    }

    // F2: any base → decimal (positional summation)
    if target_base == 10 {
        let (decimal_val, trace) = base_to_decimal_value(digits, source_base, generate_trace)?;
        return Ok((decimal_val.to_string(), trace));
    }

    // General case: source → decimal → target
    let (decimal_val, mut trace) = base_to_decimal_value(digits, source_base, generate_trace)?;
    let (result, target_trace) = decimal_to_any_base(decimal_val, target_base, generate_trace)?;
    trace.extend(target_trace);
    Ok((result, trace))
}

// ─── Implementação do trait

impl NumericProcessingService for ConversionProcessor {
    fn convert(
        &self,
        request: ParsedConversionRequest,
    ) -> Result<ConversionResult, ProcessingError> {
        let source_base = request.detected_source_base;
        let target_base = request.original_input.target_base;
        let generate_trace = request.original_input.options.generate_trace;
        let digits = &request.integer_digits;

        let (output_value, trace) =
            convert_integer(digits, source_base, target_base, generate_trace)?;

        let sign_str = if request.sign == Sign::Negative {
            "-"
        } else {
            ""
        };

        Ok(ConversionResult {
            output_value: format!("{}{}", sign_str, output_value),
            source_base,
            target_base,
            warnings: vec![],
            trace,
        })
    }

    fn compute_maximum(
        &self,
        request: MaximumValueRequest,
    ) -> Result<MaximumValueResult, ProcessingError> {
        let expression = maximum(request.base as u32, request.digit_count);

        Ok(MaximumValueResult {
            expression,
            value: String::new(), // preenchido quando houver conversão do max para a base
            base: request.base,
            digit_count: request.digit_count,
        })
    }
}
