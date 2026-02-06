use crate::token_parser::{ParseOptions, constants::MULTI_LINE_STRING_SEQUENCES};

use super::utilities::*;

pub(crate) fn scan_trivia(bytes: &[u8], start: usize) -> Option<usize> {
    let mut pos = start;

    loop {
        let before = pos;

        pos += count_while(bytes, pos, |b| b.is_ascii_whitespace());

        // line comment
        if peek(bytes, pos) == Some(&b'/') && peek(bytes, pos + 1) == Some(&b'/') {
            if let Some(next_line_start) = get_next_line_start(bytes, pos) {
                pos = next_line_start;
            } else {
                pos = bytes.len();
            }
        }

        if pos == before {
            break;
        }
    }

    if pos == start {
        None
    } else {
        Some(pos - start)
    }
}

pub(crate) fn scan_string_literal(bytes: &[u8], start: usize) -> Option<usize> {
    let mut pos = start;
    let mut byte = *peek(bytes, pos)?;

    if byte == b'h' || byte == b'H' {
        pos += 1;
        byte = *peek(bytes, pos)?;
    }

    let is_verbatim = if byte == b'@' {
        pos += 1;
        byte = *peek(bytes, pos)?;
        true
    } else {
        false
    };

    if byte == b'\'' || byte == b'"' {
        pos += 1;

        let content_len = scan_string_literal_content(bytes, pos, byte, is_verbatim)?;
        pos += content_len;

        if peek(bytes, pos) == Some(&byte) {
            pos += 1;
        } else {
            return None;
        }
    } else {
        for sequence in MULTI_LINE_STRING_SEQUENCES {
            if matches_sequence(bytes, start, sequence) {
                pos += sequence.len();
                pos += scan_multi_line_string_literal(bytes, pos, sequence);
                if matches_sequence(bytes, pos, sequence) {
                    pos += sequence.len();
                } else {
                    return None;
                }

                return Some(pos - start);
            }
        }

        return None;
    }

    Some(pos - start)
}

pub(crate) fn scan_goo(bytes: &[u8], start: usize, options: &ParseOptions) -> Option<usize> {
    let byte = *peek(bytes, start)?;
    let mut pos = start;

    if byte == b'(' {
        pos += 1;

        while let Some(&next_byte) = peek(bytes, pos)
            && next_byte != b')'
            && (options.allow_literals_with_line_breaks || !is_line_break_start(next_byte))
        {
            pos += 1;
        }

        if peek(bytes, pos) == Some(&b')') {
            pos += 1;
            return Some(pos - start);
        }
    }

    None
}

pub(crate) fn scan_string_literal_content(
    bytes: &[u8],
    start: usize,
    quote_byte: u8,
    is_verbatim: bool,
) -> Option<usize> {
    let mut pos = start;

    while let Some(&byte) = peek(bytes, pos) {
        if byte == quote_byte && is_verbatim && peek(bytes, pos + 1) == Some(&quote_byte) {
            pos += 2;
        } else if byte == b'\\' && !is_verbatim {
            let escape_len = scan_string_escape(bytes, pos)?;
            pos += escape_len;
        } else if byte == quote_byte || byte == b'\r' || byte == b'\n' {
            break;
        } else {
            pos += 1;
        }
    }

    Some(pos - start)
}

pub(crate) fn scan_multi_line_string_literal(bytes: &[u8], start: usize, sequence: &[u8]) -> usize {
    let mut pos = start;

    while !is_at_end(bytes, pos) && !matches_sequence(bytes, pos, sequence) {
        pos += 1;
    }

    pos - start
}

pub(crate) fn scan_identifier(bytes: &[u8], start: usize) -> Option<usize> {
    let mut pos = start;

    if let Some(&byte) = peek(bytes, start)
        && is_identifier_start_char(byte)
    {
        pos += 1;
        pos += count_while(bytes, pos, |&b| is_identifier_char(b))
    } else if let Some(digit_count) = scan_digits(bytes, pos)
        && let Some(next_byte) = peek(bytes, pos + digit_count)
    {
        // must have at least one letter or _ after digits
        if next_byte.is_ascii_alphabetic() || *next_byte == b'_' {
            pos += digit_count;
            pos += count_while(bytes, pos, |&b| is_identifier_char(b));
        }
    }

    if pos == start {
        None
    } else {
        Some(pos - start)
    }
}

pub(crate) fn scan_raw_guid_literal(bytes: &[u8], start: usize) -> Option<usize> {
    let mut pos = start;

    // 8 hex digits
    let eight_hex_len = scan_hex_digits(bytes, pos, Some(8))?;
    pos += eight_hex_len;

    // '-'
    if peek(bytes, pos) != Some(&b'-') {
        return None;
    }
    pos += 1;

    // 4 hex digits
    let four_hex_len = scan_hex_digits(bytes, pos, Some(4))?;
    pos += four_hex_len;

    // '-'
    if peek(bytes, pos) != Some(&b'-') {
        return None;
    }
    pos += 1;

    // 4 hex digits
    let four_hex_len = scan_hex_digits(bytes, pos, Some(4))?;
    pos += four_hex_len;

    // '-'
    if peek(bytes, pos) != Some(&b'-') {
        return None;
    }
    pos += 1;

    // 4 hex digits
    let four_hex_len = scan_hex_digits(bytes, pos, Some(4))?;
    pos += four_hex_len;

    // '-'
    if peek(bytes, pos) != Some(&b'-') {
        return None;
    }
    pos += 1;

    // 12 hex digits
    let twelve_hex_len = scan_hex_digits(bytes, pos, Some(12))?;
    pos += twelve_hex_len;

    Some(pos - start)
}

pub(crate) fn scan_real_literal(bytes: &[u8], start: usize) -> Option<usize> {
    let digit_len = scan_digits(bytes, start)?;
    let mut pos = start + digit_len;

    if peek(bytes, pos) == Some(&b'.')
        && peek(bytes, pos + 1) != Some(&b'.')
        && peek(bytes, pos + 2) != Some(&b'.')
    {
        pos += 1;

        if let Some(frac_len) = scan_digits(bytes, pos) {
            pos += frac_len;
        }

        if let Some(exp_len) = scan_exponent(bytes, pos) {
            pos += exp_len;
        }
    } else if let Some(exp_len) = scan_exponent(bytes, pos) {
        pos += exp_len;
    } else {
        return None;
    }

    if let Some(&byte) = peek(bytes, pos)
        && is_identifier_char(byte)
    {
        return None;
    }

    Some(pos - start)
}

pub(crate) fn scan_timespan_literal(bytes: &[u8], start: usize) -> Option<usize> {
    let mut len = scan_digits(bytes, start)?;
    if peek(bytes, start + len) == Some(&b'.') {
        let frac_len = scan_digits(bytes, start + len + 1)?;
        len += 1 + frac_len;
    }

    if let Some(suffix_len) = get_timespan_longest_suffix(bytes, start + len) {
        len += suffix_len;

        if let Some(&byte) = peek(bytes, start + len)
            && is_identifier_char(byte)
        {
            return None;
        }

        return Some(len);
    }

    None
}

pub(crate) fn scan_long_literal(bytes: &[u8], start: usize) -> Option<usize> {
    if let Some(hex_len) = scan_hex_integer_literal(bytes, start) {
        return Some(hex_len);
    }

    let digit_len = scan_digits(bytes, start)?;
    if let Some(&byte) = peek(bytes, start + digit_len)
        && is_identifier_char(byte)
    {
        return None;
    }

    Some(digit_len)
}

fn scan_string_escape(bytes: &[u8], start: usize) -> Option<usize> {
    if peek(bytes, start) != Some(&b'\\') {
        return None;
    }

    let mut pos = start + 1;

    match peek(bytes, pos)? {
        b'\\' | b'\'' | b'"' | b'a' | b'b' | b'f' | b'n' | b'r' | b't' | b'v' => {
            pos += 1;
        }
        b'u' => {
            pos += 1;
            let hex_len = scan_hex_digits(bytes, pos, Some(4))?;
            pos += hex_len;
        }
        b'U' => {
            pos += 1;
            let hex_len = scan_hex_digits(bytes, pos, Some(8))?;
            pos += hex_len;
        }
        b'x' => {
            pos += 1;
            let hex_len = scan_hex_digits(bytes, pos, Some(2))?;
            pos += hex_len;
        }
        _ => {
            let octal_len = scan_octal_code(bytes, pos)?;
            pos += octal_len;
        }
    }

    Some(pos - start)
}

fn scan_octal_code(bytes: &[u8], start: usize) -> Option<usize> {
    let mut pos = start;

    if let Some(&byte) = peek(bytes, pos)
        && (b'0'..=b'7').contains(&byte)
    {
        pos += 1;

        for _ in 0..2 {
            if let Some(&next_byte) = peek(bytes, pos)
                && (b'0'..=b'7').contains(&next_byte)
            {
                pos += 1;
            } else {
                break;
            }
        }

        return Some(pos - start);
    }

    None
}

fn scan_digits(bytes: &[u8], start: usize) -> Option<usize> {
    let digit_count = count_while(bytes, start, |b| b.is_ascii_digit());

    if digit_count == 0 {
        None
    } else {
        Some(digit_count)
    }
}

fn scan_hex_digits(bytes: &[u8], start: usize, count: Option<usize>) -> Option<usize> {
    let mut pos = start;

    if let Some(count_val) = count {
        for _ in 0..count_val {
            let byte = peek(bytes, pos)?;
            if !byte.is_ascii_hexdigit() {
                return None;
            }
            pos += 1;
        }

        return Some(pos - start);
    }

    let hex_len = count_while(bytes, start, |b| b.is_ascii_hexdigit());

    if hex_len == 0 { None } else { Some(hex_len) }
}

fn scan_exponent(bytes: &[u8], start: usize) -> Option<usize> {
    let mut pos = start;

    if let Some(&byte) = peek(bytes, pos)
        && (byte == b'e' || byte == b'E')
    {
        pos += 1;

        if let Some(&next_byte) = peek(bytes, pos)
            && (next_byte == b'+' || next_byte == b'-')
        {
            pos += 1;
        }

        let digit_len = scan_digits(bytes, pos)?;
        pos += digit_len;

        return Some(pos - start);
    }

    None
}

fn scan_hex_integer_literal(bytes: &[u8], start: usize) -> Option<usize> {
    let mut pos = start;

    if peek(bytes, pos) == Some(&b'0')
        && (peek(bytes, pos + 1) == Some(&b'x') || peek(bytes, pos + 1) == Some(&b'X'))
    {
        pos += 2;
        let hex_len = scan_hex_digits(bytes, pos, None)?;
        pos += hex_len;
    }

    if let Some(&byte) = peek(bytes, pos)
        && is_identifier_char(byte)
    {
        return None;
    }

    Some(pos - start)
}
