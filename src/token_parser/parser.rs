use crate::token_parser::types::Span;
use crate::token_parser::types::TokenStream;

use super::scanner::*;
use super::utilities::*;
use super::{KeywordKind, LiteralKind, ParseOptions, PunctuationKind, TokenKind, constants::*};

pub fn parse_tokens<'a>(source: &'a str, options: &ParseOptions) -> TokenStream<'a> {
    let bytes = source.as_bytes();
    // Pre-allocate based on estimation
    let estimated_tokens = (bytes.len() / AVG_BYTES_PER_TOKEN + 1).min(bytes.len());
    let mut tokens = Vec::with_capacity(estimated_tokens);
    let mut pos = 0;

    while let Some((kind, len)) = next_token(bytes, pos, options) {
        let is_eof = kind == TokenKind::EndOfFile;
        pos += len;
        tokens.push(kind);

        if is_eof {
            break;
        }
    }

    tokens.shrink_to_fit();

    TokenStream { tokens, source }
}

fn next_token(bytes: &[u8], start: usize, options: &ParseOptions) -> Option<(TokenKind, usize)> {
    let mut pos = start;
    let trivia_len = scan_trivia(bytes, start).unwrap_or(0);
    let has_trivia = trivia_len > 0;

    pos += trivia_len;

    if let Some(&byte) = peek(bytes, pos) {
        if !byte.is_ascii_alphanumeric() {
            if let Some((kind, punc_len)) = parse_punctuation(bytes, pos) {
                return Some((TokenKind::Punctuation(kind), trivia_len + punc_len));
            }

            if is_string_literal_start_quote(byte) {
                if let Some(string_len) = scan_string_literal(bytes, pos) {
                    let span = Span {
                        start: pos,
                        end: pos + string_len,
                    };
                    return Some((
                        TokenKind::Literal(LiteralKind::String(span)),
                        trivia_len + span.len(),
                    ));
                }
            } else if byte == b'@'
                && let Some(&next_byte) = peek(bytes, pos + 1)
                && is_string_literal_start_quote(next_byte)
            {
                if let Some(string_len) = scan_string_literal(bytes, pos) {
                    let span = Span {
                        start: pos,
                        end: pos + string_len,
                    };
                    return Some((
                        TokenKind::Literal(LiteralKind::String(span)),
                        trivia_len + span.len(),
                    ));
                }
            } else if byte == b'#' {
                let directive_end = get_line_end(bytes, pos);
                let span = Span {
                    start: pos,
                    end: directive_end,
                };
                return Some((TokenKind::Directive(span), trivia_len + span.len()));
            } else if is_at_end(bytes, pos) {
                if has_trivia || options.always_produce_end_tokens {
                    return Some((TokenKind::EndOfFile, trivia_len));
                }

                return None;
            }
        }

        if let Some((keyword_len, keyword_kind)) = get_longest_keyword(bytes, pos) {
            if let Some(&next_byte) = peek(bytes, pos + keyword_len)
                && next_byte == b'('
                && is_goo_literal_kind(&keyword_kind)
                && let Some(goo_len) = scan_goo(bytes, pos + keyword_len, options)
            {
                let span = Span {
                    start: pos,
                    end: pos + goo_len + keyword_len,
                };
                let full_len = trivia_len + span.len();
                return match keyword_kind {
                    KeywordKind::Bool => {
                        Some((TokenKind::Literal(LiteralKind::Boolean(span)), full_len))
                    }
                    KeywordKind::DateTime | KeywordKind::Date => {
                        Some((TokenKind::Literal(LiteralKind::DateTime(span)), full_len))
                    }
                    KeywordKind::Decimal => {
                        Some((TokenKind::Literal(LiteralKind::Decimal(span)), full_len))
                    }
                    KeywordKind::Guid => {
                        Some((TokenKind::Literal(LiteralKind::Guid(span)), full_len))
                    }
                    KeywordKind::Int | KeywordKind::Int32 => {
                        Some((TokenKind::Literal(LiteralKind::Int(span)), full_len))
                    }
                    KeywordKind::Long | KeywordKind::Int64 => {
                        Some((TokenKind::Literal(LiteralKind::Long(span)), full_len))
                    }
                    KeywordKind::Real | KeywordKind::Double => {
                        Some((TokenKind::Literal(LiteralKind::Real(span)), full_len))
                    }
                    KeywordKind::Time | KeywordKind::Timespan => {
                        Some((TokenKind::Literal(LiteralKind::Timespan(span)), full_len))
                    }
                    _ => None,
                };
            }

            let is_keyword = match peek(bytes, pos + keyword_len) {
                Some(&next_byte) => !is_identifier_char(next_byte),
                None => true,
            };

            if is_keyword {
                return Some((TokenKind::Keyword(keyword_kind), trivia_len + keyword_len));
            }
        }

        if is_identifier_start_char(byte) {
            if let Some(bool_len) = parse_bool_literal(bytes, pos) {
                let is_bool = match peek(bytes, pos + bool_len) {
                    Some(&b) => !is_identifier_char(b),
                    None => true,
                };

                if is_bool {
                    let span = Span {
                        start: pos,
                        end: pos + bool_len,
                    };
                    return Some((
                        TokenKind::Literal(LiteralKind::Boolean(span)),
                        trivia_len + bool_len,
                    ));
                }
            }

            if let Some(raw_guid_len) = scan_raw_guid_literal(bytes, pos) {
                let span = Span {
                    start: pos,
                    end: pos + raw_guid_len,
                };
                return Some((
                    TokenKind::Literal(LiteralKind::RawGuid(span)),
                    trivia_len + raw_guid_len,
                ));
            }

            if let Some(id_len) = scan_identifier(bytes, pos) {
                if id_len == 1
                    && (byte == b'h' || byte == b'H')
                    && let Some(&next_byte) = peek(bytes, pos + 1)
                    && (is_string_literal_start_quote(next_byte) || next_byte == b'@')
                    && let Some(string_len) = scan_string_literal(bytes, pos)
                {
                    let span = Span {
                        start: pos,
                        end: pos + string_len,
                    };
                    return Some((
                        TokenKind::Literal(LiteralKind::String(span)),
                        trivia_len + span.len(),
                    ));
                }

                let text_span = Span {
                    start: pos,
                    end: pos + id_len,
                };
                return Some((TokenKind::Identifier(text_span), trivia_len + id_len));
            }
        } else if byte.is_ascii_digit() {
            if let Some(raw_guid_len) = scan_raw_guid_literal(bytes, pos) {
                let span = Span {
                    start: pos,
                    end: pos + raw_guid_len,
                };
                return Some((
                    TokenKind::Literal(LiteralKind::RawGuid(span)),
                    trivia_len + raw_guid_len,
                ));
            }
            if let Some(real_len) = scan_real_literal(bytes, pos) {
                let span = Span {
                    start: pos,
                    end: pos + real_len,
                };
                return Some((
                    TokenKind::Literal(LiteralKind::Real(span)),
                    trivia_len + span.len(),
                ));
            }
            if let Some(timespan_len) = scan_timespan_literal(bytes, pos) {
                let span = Span {
                    start: pos,
                    end: pos + timespan_len,
                };
                return Some((
                    TokenKind::Literal(LiteralKind::Timespan(span)),
                    trivia_len + span.len(),
                ));
            }
            if let Some(long_len) = scan_long_literal(bytes, pos) {
                let span = Span {
                    start: pos,
                    end: pos + long_len,
                };
                return Some((
                    TokenKind::Literal(LiteralKind::Long(span)),
                    trivia_len + span.len(),
                ));
            }
            if let Some(id_len) = scan_identifier(bytes, pos) {
                let span = Span {
                    start: pos,
                    end: pos + id_len,
                };
                return Some((TokenKind::Identifier(span), trivia_len + span.len()));
            }
        }
    } else {
        if has_trivia || options.always_produce_end_tokens {
            return Some((TokenKind::EndOfFile, trivia_len));
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

    let text_span = Span {
        start: pos,
        end: pos + char_len,
    };

    Some((TokenKind::Bad(text_span), trivia_len + text_span.len()))
}

fn parse_bool_literal(bytes: &[u8], start: usize) -> Option<usize> {
    for literal in BOOL_LITERALS {
        if matches_sequence(bytes, start, literal) {
            return Some(literal.len());
        }
    }

    None
}

fn parse_punctuation(bytes: &[u8], start: usize) -> Option<(PunctuationKind, usize)> {
    let (kind, width) = match peek(bytes, start)? {
        b'(' => (PunctuationKind::OpenParen, 1),
        b')' => (PunctuationKind::CloseParen, 1),
        b'[' => (PunctuationKind::OpenBracket, 1),
        b']' => (PunctuationKind::CloseBracket, 1),
        b'{' => (PunctuationKind::OpenBrace, 1),
        b'}' => (PunctuationKind::CloseBrace, 1),
        b'|' => (PunctuationKind::Bar, 1),
        b'.' => {
            if peek(bytes, start + 1) == Some(&b'.') {
                (PunctuationKind::DotDot, 2)
            } else {
                (PunctuationKind::Dot, 1)
            }
        }
        b'+' => (PunctuationKind::Plus, 1),
        b'-' => (PunctuationKind::Minus, 1),
        b'*' => (PunctuationKind::Asterisk, 1),
        b'/' => (PunctuationKind::Slash, 1),
        b'%' => (PunctuationKind::Percent, 1),
        b'<' => {
            let next_byte = peek(bytes, start + 1);
            if next_byte == Some(&b'=') {
                (PunctuationKind::LessThanOrEqual, 2)
            } else if next_byte == Some(&b'|') {
                (PunctuationKind::LessThanBar, 2)
            } else if next_byte == Some(&b'>') {
                (PunctuationKind::LessThanGreaterThan, 2)
            } else {
                (PunctuationKind::LessThan, 1)
            }
        }
        b'>' => {
            if peek(bytes, start + 1) == Some(&b'=') {
                (PunctuationKind::GreaterThanOrEqual, 2)
            } else {
                (PunctuationKind::GreaterThan, 1)
            }
        }
        b'=' => {
            let next_byte = peek(bytes, start + 1);
            if next_byte == Some(&b'=') {
                (PunctuationKind::EqualEqual, 2)
            } else if next_byte == Some(&b'>') {
                (PunctuationKind::FatArrow, 2)
            } else if next_byte == Some(&b'~') {
                (PunctuationKind::EqualTilde, 2)
            } else {
                (PunctuationKind::Equal, 1)
            }
        }
        b'!' => {
            let next_byte = peek(bytes, start + 1);
            if next_byte == Some(&b'=') {
                (PunctuationKind::BangEqual, 2)
            } else if next_byte == Some(&b'~') {
                (PunctuationKind::BangTilde, 2)
            } else {
                return None;
            }
        }
        b':' => (PunctuationKind::Colon, 1),
        b';' => (PunctuationKind::Semicolon, 1),
        b',' => (PunctuationKind::Comma, 1),
        b'@' => {
            if let Some(&b) = peek(bytes, start + 1)
                && is_string_literal_start_quote(b)
            {
                return None;
            }
            (PunctuationKind::At, 1)
        }
        b'?' => (PunctuationKind::Question, 1),
        _ => return None,
    };

    Some((kind, width))
}
