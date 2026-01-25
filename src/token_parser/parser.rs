use super::scanner::*;
use super::utilities::*;
use crate::token_parser::{ParseOptions, Token, TokenKind, constants::*};
use std::{ops::Range, str};

pub fn parse_tokens(text: &str, options: &ParseOptions) -> Vec<Token> {
    let bytes = text.as_bytes();
    // Pre-allocate based on estimation
    let mut tokens = Vec::with_capacity((bytes.len() / AVG_BYTES_PER_TOKEN).max(1));
    let mut pos = 0;

    while let Some((kind, trivia_span, text_span)) = next_token(bytes, pos, options) {
        let is_eof = kind == TokenKind::EndOfTextToken;
        let text_content = text[text_span.start..text_span.end].to_string();
        pos = text_span.end;
        tokens.push(Token::new(kind, trivia_span, text_span, text_content));

        if is_eof {
            break;
        }
    }

    tokens
}

fn next_token(
    bytes: &[u8],
    start: usize,
    options: &ParseOptions,
) -> Option<(TokenKind, Range<usize>, Range<usize>)> {
    let mut pos = start;
    let trivia_span = parse_trivia(bytes, start).unwrap_or(start..start);
    let has_trivia = !trivia_span.is_empty();

    pos += trivia_span.end - trivia_span.start;

    if let Some(&byte) = peek(bytes, pos) {
        if !byte.is_ascii_alphanumeric() {
            if let Some((kind, text_span)) = parse_punctuation(bytes, pos) {
                return Some((kind, trivia_span, text_span));
            }

            if is_string_literal_start_quote(byte) {
                if let Some(text_span) = parse_string_literal(bytes, pos) {
                    return Some((TokenKind::StringLiteralToken, trivia_span, text_span));
                }
            } else if byte == b'@'
                && let Some(&next_byte) = peek(bytes, pos + 1)
                && is_string_literal_start_quote(next_byte)
            {
                if let Some(text_span) = parse_string_literal(bytes, pos) {
                    return Some((TokenKind::StringLiteralToken, trivia_span, text_span));
                }
            } else if byte == b'#' {
                let directive_end = get_line_end(bytes, pos);
                return Some((TokenKind::DirectiveToken, trivia_span, pos..directive_end));
            } else if is_at_end(bytes, pos) {
                if has_trivia || options.always_produce_end_tokens {
                    return Some((TokenKind::EndOfTextToken, trivia_span, pos..pos));
                }

                return None;
            }
        }

        if let Some((keyword_len, keyword_kind)) = get_longest_keyword(bytes, pos) {
            if let Some(&next_byte) = peek(bytes, pos + keyword_len)
                && next_byte == b'('
                && let Some(goo_kind) = get_goo_literal_kind_from_keyword_kind(keyword_kind)
                && let Some(goo_len) = scan_goo(bytes, pos + keyword_len, options)
            {
                return Some((goo_kind, trivia_span, pos..pos + keyword_len + goo_len));
            }

            let is_keyword = match peek(bytes, pos + keyword_len) {
                Some(&next_byte) => !is_identifier_char(next_byte),
                None => true,
            };

            if is_keyword {
                return Some((keyword_kind, trivia_span, pos..pos + keyword_len));
            }
        }

        if is_identifier_start_char(byte) {
            if let Some(bool_len) = parse_bool_literal(bytes, pos) {
                let is_bool = match peek(bytes, pos + bool_len) {
                    Some(&b) => !is_identifier_char(b),
                    None => true,
                };

                if is_bool {
                    return Some((
                        TokenKind::BooleanLiteralToken,
                        trivia_span,
                        pos..pos + bool_len,
                    ));
                }
            }

            if let Some(raw_guid_len) = scan_raw_guid_literal(bytes, pos) {
                return Some((
                    TokenKind::RawGuidLiteralToken,
                    trivia_span,
                    pos..pos + raw_guid_len,
                ));
            }

            if let Some(id_len) = scan_identifier(bytes, pos) {
                if id_len == 1
                    && (byte == b'h' || byte == b'H')
                    && let Some(&next_byte) = peek(bytes, pos + 1)
                    && (is_string_literal_start_quote(next_byte) || next_byte == b'@')
                    && let Some(text_span) = parse_string_literal(bytes, pos)
                {
                    return Some((TokenKind::StringLiteralToken, trivia_span, text_span));
                }

                return Some((TokenKind::IdentifierToken, trivia_span, pos..pos + id_len));
            }
        } else if byte.is_ascii_digit() {
            if let Some(raw_guid_len) = scan_raw_guid_literal(bytes, pos) {
                return Some((
                    TokenKind::RawGuidLiteralToken,
                    trivia_span,
                    pos..pos + raw_guid_len,
                ));
            }
            if let Some(real_len) = scan_real_literal(bytes, pos) {
                return Some((
                    TokenKind::RealLiteralToken,
                    trivia_span,
                    pos..pos + real_len,
                ));
            }
            if let Some(timespan_len) = scan_timespan_literal(bytes, pos) {
                return Some((
                    TokenKind::TimespanLiteralToken,
                    trivia_span,
                    pos..pos + timespan_len,
                ));
            }
            if let Some(long_len) = scan_long_literal(bytes, pos) {
                return Some((
                    TokenKind::LongLiteralToken,
                    trivia_span,
                    pos..pos + long_len,
                ));
            }
            if let Some(id_len) = scan_identifier(bytes, pos) {
                return Some((TokenKind::IdentifierToken, trivia_span, pos..pos + id_len));
            }
        }
    } else {
        if has_trivia || options.always_produce_end_tokens {
            return Some((TokenKind::EndOfTextToken, trivia_span, pos..pos));
        }
        return None;
    }

    // Get the length of the UTF-8 character at pos to avoid splitting multi-byte characters
    // Use UTF-8 leading byte pattern to determine character length
    let char_len = if !is_at_end(bytes, pos) {
        let byte = bytes[pos];
        if byte & UTF8_1_BYTE_MASK == UTF8_1_BYTE_PATTERN {
            1 // ASCII (0xxxxxxx)
        } else if byte & UTF8_2_BYTE_MASK == UTF8_2_BYTE_PATTERN {
            2 // 2-byte (110xxxxx)
        } else if byte & UTF8_3_BYTE_MASK == UTF8_3_BYTE_PATTERN {
            3 // 3-byte (1110xxxx)
        } else if byte & UTF8_4_BYTE_MASK == UTF8_4_BYTE_PATTERN {
            4 // 4-byte (11110xxx)
        } else {
            1 // Invalid UTF-8, treat as single byte
        }
    } else {
        1
    };

    Some((TokenKind::BadToken, trivia_span, pos..pos + char_len))
}

fn parse_trivia(bytes: &[u8], start: usize) -> Option<Range<usize>> {
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

    if pos == start { None } else { Some(start..pos) }
}

fn parse_bool_literal(bytes: &[u8], start: usize) -> Option<usize> {
    for literal in BOOL_LITERALS {
        if matches_sequence(bytes, start, literal) {
            return Some(literal.len());
        }
    }

    None
}

fn parse_punctuation(bytes: &[u8], start: usize) -> Option<(TokenKind, Range<usize>)> {
    let (kind, width) = match peek(bytes, start)? {
        b'(' => (TokenKind::OpenParenToken, 1),
        b')' => (TokenKind::CloseParenToken, 1),
        b'[' => (TokenKind::OpenBracketToken, 1),
        b']' => (TokenKind::CloseBracketToken, 1),
        b'{' => (TokenKind::OpenBraceToken, 1),
        b'}' => (TokenKind::CloseBraceToken, 1),
        b'|' => (TokenKind::BarToken, 1),
        b'.' => {
            if peek(bytes, start + 1) == Some(&b'.') {
                (TokenKind::DotDotToken, 2)
            } else {
                (TokenKind::DotToken, 1)
            }
        }
        b'+' => (TokenKind::PlusToken, 1),
        b'-' => (TokenKind::MinusToken, 1),
        b'*' => (TokenKind::AsteriskToken, 1),
        b'/' => (TokenKind::SlashToken, 1),
        b'%' => (TokenKind::PercentToken, 1),
        b'<' => {
            let next_byte = peek(bytes, start + 1);
            if next_byte == Some(&b'=') {
                (TokenKind::LessThanOrEqualToken, 2)
            } else if next_byte == Some(&b'|') {
                (TokenKind::LessThanBarToken, 2)
            } else if next_byte == Some(&b'>') {
                (TokenKind::LessThanGreaterThanToken, 2)
            } else {
                (TokenKind::LessThanToken, 1)
            }
        }
        b'>' => {
            if peek(bytes, start + 1) == Some(&b'=') {
                (TokenKind::GreaterThanOrEqualToken, 2)
            } else {
                (TokenKind::GreaterThanToken, 1)
            }
        }
        b'=' => {
            let next_byte = peek(bytes, start + 1);
            if next_byte == Some(&b'=') {
                (TokenKind::EqualEqualToken, 2)
            } else if next_byte == Some(&b'>') {
                (TokenKind::FatArrowToken, 2)
            } else if next_byte == Some(&b'~') {
                (TokenKind::EqualTildeToken, 2)
            } else {
                (TokenKind::EqualToken, 1)
            }
        }
        b'!' => {
            let next_byte = peek(bytes, start + 1);
            if next_byte == Some(&b'=') {
                (TokenKind::BangEqualToken, 2)
            } else if next_byte == Some(&b'~') {
                (TokenKind::BangTildeToken, 2)
            } else {
                return None;
            }
        }
        b':' => (TokenKind::ColonToken, 1),
        b';' => (TokenKind::SemicolonToken, 1),
        b',' => (TokenKind::CommaToken, 1),
        b'@' => {
            if let Some(&b) = peek(bytes, start + 1)
                && is_string_literal_start_quote(b)
            {
                return None;
            }
            (TokenKind::AtToken, 1)
        }
        b'?' => (TokenKind::QuestionToken, 1),
        _ => return None,
    };

    Some((kind, start..start + width))
}

fn parse_string_literal(bytes: &[u8], start: usize) -> Option<Range<usize>> {
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

                return Some(start..pos);
            }
        }

        return None;
    }

    Some(start..pos)
}
