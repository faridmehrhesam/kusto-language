use crate::token_parser::{KEYWORDS, SyntaxKind, TIMESPAN_SUFFIXES};

#[inline(always)]
pub(crate) fn peek(bytes: &[u8], pos: usize) -> Option<&u8> {
    bytes.get(pos)
}

#[inline(always)]
pub(crate) fn is_at_end(bytes: &[u8], pos: usize) -> bool {
    pos >= bytes.len()
}

#[inline(always)]
pub(crate) fn is_line_break_start(byte: u8) -> bool {
    byte == b'\r' || byte == b'\n'
}

#[inline(always)]
pub(crate) fn is_string_literal_start_quote(byte: u8) -> bool {
    byte == b'"' || byte == b'\'' || byte == b'`' || byte == b'~'
}

#[inline(always)]
pub(crate) fn is_identifier_start_char(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_' || byte == b'$'
}

#[inline(always)]
pub(crate) fn is_identifier_char(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

pub(crate) fn count_while<F>(bytes: &[u8], start: usize, predicate: F) -> usize
where
    F: Fn(&u8) -> bool,
{
    let mut pos = start;

    while let Some(byte) = peek(bytes, pos)
        && predicate(byte)
    {
        pos += 1;
    }

    pos - start
}

pub(crate) fn matches_sequence(bytes: &[u8], start: usize, sequence: &[u8]) -> bool {
    bytes.get(start..start + sequence.len()) == Some(sequence)
}

pub(crate) fn get_next_line_start(bytes: &[u8], start: usize) -> Option<usize> {
    let next_start = get_next_line_break_start(bytes, start)?;
    let line_break_len = get_next_line_break_len(bytes, next_start)?;

    Some(next_start + line_break_len)
}

pub(crate) fn get_line_end(bytes: &[u8], start: usize) -> usize {
    start + get_line_len(bytes, start, false)
}

pub(crate) fn get_timespan_longest_suffix(bytes: &[u8], start: usize) -> Option<usize> {
    for suffix in TIMESPAN_SUFFIXES {
        let len = suffix.len();
        if bytes.get(start..start + len) == Some(*suffix) {
            return Some(len);
        }
    }

    None
}

pub(crate) fn get_longest_keyword(bytes: &[u8], start: usize) -> Option<(usize, SyntaxKind)> {
    for keyword in KEYWORDS {
        let len = keyword.0.len();
        if bytes.get(start..start + len) == Some(keyword.0) {
            return Some((len, keyword.1));
        }
    }

    None
}

pub(crate) fn get_goo_literal_kind_from_keyword_kind(
    keyword_kind: SyntaxKind,
) -> Option<SyntaxKind> {
    match keyword_kind {
        SyntaxKind::BoolKeyword => Some(SyntaxKind::BooleanLiteralToken),
        SyntaxKind::DateTimeKeyword | SyntaxKind::DateKeyword => {
            Some(SyntaxKind::DateTimeLiteralToken)
        }
        SyntaxKind::DecimalKeyword => Some(SyntaxKind::DecimalLiteralToken),
        SyntaxKind::GuidKeyword => Some(SyntaxKind::GuidLiteralToken),
        SyntaxKind::IntKeyword | SyntaxKind::Int32Keyword => Some(SyntaxKind::IntLiteralToken),
        SyntaxKind::LongKeyword | SyntaxKind::Int64Keyword => Some(SyntaxKind::LongLiteralToken),
        SyntaxKind::RealKeyword | SyntaxKind::DoubleKeyword => Some(SyntaxKind::RealLiteralToken),
        SyntaxKind::TimeKeyword | SyntaxKind::TimespanKeyword => {
            Some(SyntaxKind::TimespanLiteralToken)
        }
        _ => None,
    }
}

fn get_line_len(bytes: &[u8], start: usize, include_line_break: bool) -> usize {
    let mut pos = start;

    while let Some(&byte) = peek(bytes, pos)
        && !is_line_break_start(byte)
    {
        pos += 1;
    }

    if include_line_break && let Some(line_break_len) = get_next_line_break_len(bytes, start) {
        pos += line_break_len
    }

    pos - start
}

fn get_next_line_break_len(bytes: &[u8], start: usize) -> Option<usize> {
    let break_start = get_next_line_break_start(bytes, start)?;

    if peek(bytes, break_start) == Some(&b'\r') && peek(bytes, break_start + 1) == Some(&b'\n') {
        Some(2)
    } else {
        Some(1)
    }
}

fn get_next_line_break_start(bytes: &[u8], start: usize) -> Option<usize> {
    if let Some(rel_pos) = bytes[start..]
        .iter()
        .position(|&b| b == b'\n' || b == b'\r')
    {
        return Some(rel_pos + start);
    }

    None
}
