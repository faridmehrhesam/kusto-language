use super::scanner::*;
use super::utilities::*;
use crate::token_parser::BOOL_LITERALS;
use crate::token_parser::{AVG_BYTES_PER_TOKEN, LexicalToken, ParseOptions, SyntaxKind};
use std::ops::Range;

pub fn parse_tokens(text: &str, options: &ParseOptions) -> Vec<LexicalToken> {
    let bytes = text.as_bytes();
    // Pre-allocate based on estimation
    let mut tokens = Vec::with_capacity((bytes.len() / AVG_BYTES_PER_TOKEN).max(1));
    let mut pos = 0;

    while let Some(token) = next_token(bytes, pos, options) {
        let is_eof = token.kind == SyntaxKind::EndOfTextToken;
        pos += token.len();
        tokens.push(token);

        if is_eof {
            break;
        }
    }

    tokens
}

fn next_token(bytes: &[u8], start: usize, options: &ParseOptions) -> Option<LexicalToken> {
    let mut pos = start;
    let trivia = parse_trivia(bytes, start).unwrap_or(start..start);
    let has_trivia = !trivia.is_empty();

    pos += trivia.end - trivia.start;

    if let Some(&byte) = peek(bytes, pos) {
        if !byte.is_ascii_alphanumeric() {
            if let Some((kind, range)) = parse_punctuation(bytes, pos) {
                return Some(LexicalToken::new(kind, trivia, range));
            }

            if is_string_literal_start_quote(byte) {
                if let Some(range) = parse_string_literal(bytes, pos) {
                    return Some(LexicalToken {
                        kind: SyntaxKind::StringLiteralToken,
                        trivia_span: trivia,
                        text_span: range,
                    });
                }
            } else if byte == b'@'
                && let Some(&next_byte) = peek(bytes, pos + 1)
                && is_string_literal_start_quote(next_byte)
            {
                if let Some(range) = parse_string_literal(bytes, pos) {
                    return Some(LexicalToken {
                        kind: SyntaxKind::StringLiteralToken,
                        trivia_span: trivia,
                        text_span: range,
                    });
                }
            } else if byte == b'#' {
                let directive_end = get_line_end(bytes, pos);
                return Some(LexicalToken {
                    kind: SyntaxKind::DirectiveToken,
                    trivia_span: trivia,
                    text_span: pos..directive_end,
                });
            } else if is_at_end(bytes, pos) {
                if has_trivia || options.always_produce_end_tokens {
                    return Some(LexicalToken::new(
                        SyntaxKind::EndOfTextToken,
                        trivia,
                        pos..pos,
                    ));
                }

                return None;
            }
        }

        if let Some((keywork_len, keyword_kind)) = get_longest_keyword(bytes, pos) {
            if let Some(&next_byte) = peek(bytes, pos + keywork_len)
                && next_byte == b'('
                && let Some(goo_kind) = get_goo_literal_kind_from_keyword_kind(keyword_kind)
                && let Some(goo_len) = scan_goo(bytes, pos + keywork_len, options)
            {
                return Some(LexicalToken {
                    kind: goo_kind,
                    trivia_span: trivia,
                    text_span: pos..pos + keywork_len + goo_len,
                });
            }

            let is_keyword = match peek(bytes, pos + keywork_len) {
                Some(&next_byte) => !is_identifier_char(next_byte),
                None => true,
            };

            if is_keyword {
                return Some(LexicalToken {
                    kind: keyword_kind,
                    trivia_span: trivia,
                    text_span: pos..pos + keywork_len,
                });
            }
        }

        if is_identifier_start_char(byte) {
            if let Some(bool_len) = parse_bool_literal(bytes, pos) {
                let is_bool = match peek(bytes, pos + bool_len) {
                    Some(&b) => !is_identifier_char(b),
                    None => true,
                };

                if is_bool {
                    return Some(LexicalToken {
                        kind: SyntaxKind::BooleanLiteralToken,
                        trivia_span: trivia,
                        text_span: pos..pos + bool_len,
                    });
                }
            }

            if let Some(raw_guid_len) = scan_raw_guid_literal(bytes, pos) {
                return Some(LexicalToken {
                    kind: SyntaxKind::RawGuidLiteralToken,
                    trivia_span: trivia,
                    text_span: pos..pos + raw_guid_len,
                });
            }

            if let Some(id_len) = scan_identifier(bytes, pos) {
                if id_len == 1
                    && (byte == b'h' || byte == b'H')
                    && let Some(&next_byte) = peek(bytes, pos + 1)
                    && (is_string_literal_start_quote(next_byte) || next_byte == b'@')
                    && let Some(range) = parse_string_literal(bytes, pos)
                {
                    return Some(LexicalToken {
                        kind: SyntaxKind::StringLiteralToken,
                        trivia_span: trivia,
                        text_span: range,
                    });
                }

                return Some(LexicalToken {
                    kind: SyntaxKind::IdentifierToken,
                    trivia_span: trivia,
                    text_span: pos..pos + id_len,
                });
            }
        } else if byte.is_ascii_digit() {
            if let Some(raw_guid_len) = scan_raw_guid_literal(bytes, pos) {
                return Some(LexicalToken {
                    kind: SyntaxKind::RawGuidLiteralToken,
                    trivia_span: trivia,
                    text_span: pos..pos + raw_guid_len,
                });
            }
            if let Some(real_len) = scan_real_literal(bytes, pos) {
                return Some(LexicalToken {
                    kind: SyntaxKind::RealLiteralToken,
                    trivia_span: trivia,
                    text_span: pos..pos + real_len,
                });
            }
            if let Some(timespan_len) = scan_timespan_literal(bytes, pos) {
                return Some(LexicalToken {
                    kind: SyntaxKind::TimespanLiteralToken,
                    trivia_span: trivia,
                    text_span: pos..pos + timespan_len,
                });
            }
            if let Some(long_len) = scan_long_literal(bytes, pos) {
                return Some(LexicalToken {
                    kind: SyntaxKind::LongLiteralToken,
                    trivia_span: trivia,
                    text_span: pos..pos + long_len,
                });
            }
            if let Some(id_len) = scan_identifier(bytes, pos) {
                return Some(LexicalToken {
                    kind: SyntaxKind::IdentifierToken,
                    trivia_span: trivia,
                    text_span: pos..pos + id_len,
                });
            }
        }
    } else {
        if has_trivia || options.always_produce_end_tokens {
            return Some(LexicalToken::new(
                SyntaxKind::EndOfTextToken,
                trivia,
                pos..pos,
            ));
        }
        return None;
    }

    Some(LexicalToken::new(
        SyntaxKind::BadToken,
        trivia,
        pos..pos + 1,
    ))
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

fn parse_punctuation(bytes: &[u8], start: usize) -> Option<(SyntaxKind, Range<usize>)> {
    let (kind, width) = match peek(bytes, start)? {
        b'(' => (SyntaxKind::OpenParenToken, 1),
        b')' => (SyntaxKind::CloseParenToken, 1),
        b'[' => (SyntaxKind::OpenBracketToken, 1),
        b']' => (SyntaxKind::CloseBracketToken, 1),
        b'{' => (SyntaxKind::OpenBraceToken, 1),
        b'}' => (SyntaxKind::CloseBraceToken, 1),
        b'|' => (SyntaxKind::BarToken, 1),
        b'.' => {
            if peek(bytes, start + 1) == Some(&b'.') {
                (SyntaxKind::DotDotToken, 2)
            } else {
                (SyntaxKind::DotToken, 1)
            }
        }
        b'+' => (SyntaxKind::PlusToken, 1),
        b'-' => (SyntaxKind::MinusToken, 1),
        b'*' => (SyntaxKind::AsteriskToken, 1),
        b'/' => (SyntaxKind::SlashToken, 1),
        b'%' => (SyntaxKind::PercentToken, 1),
        b'<' => {
            let next_byte = peek(bytes, start + 1);
            if next_byte == Some(&b'=') {
                (SyntaxKind::LessThanOrEqualToken, 2)
            } else if next_byte == Some(&b'|') {
                (SyntaxKind::LessThanBarToken, 2)
            } else if next_byte == Some(&b'>') {
                (SyntaxKind::LessThanGreaterThanToken, 2)
            } else {
                (SyntaxKind::LessThanToken, 1)
            }
        }
        b'>' => {
            if peek(bytes, start + 1) == Some(&b'=') {
                (SyntaxKind::GreaterThanOrEqualToken, 2)
            } else {
                (SyntaxKind::GreaterThanToken, 1)
            }
        }
        b'=' => {
            let next_byte = peek(bytes, start + 1);
            if next_byte == Some(&b'=') {
                (SyntaxKind::EqualEqualToken, 2)
            } else if next_byte == Some(&b'>') {
                (SyntaxKind::FatArrowToken, 2)
            } else if next_byte == Some(&b'~') {
                (SyntaxKind::EqualTildeToken, 2)
            } else {
                (SyntaxKind::EqualToken, 1)
            }
        }
        b'!' => {
            let next_byte = peek(bytes, start + 1);
            if next_byte == Some(&b'=') {
                (SyntaxKind::BangEqualToken, 2)
            } else if next_byte == Some(&b'~') {
                (SyntaxKind::BangTildeToken, 2)
            } else {
                return None;
            }
        }
        b':' => (SyntaxKind::ColonToken, 1),
        b';' => (SyntaxKind::SemicolonToken, 1),
        b',' => (SyntaxKind::CommaToken, 1),
        b'@' => {
            if let Some(&b) = peek(bytes, start + 1)
                && is_string_literal_start_quote(b)
            {
                return None;
            }
            (SyntaxKind::AtToken, 1)
        }
        b'?' => (SyntaxKind::QuestionToken, 1),
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
        for sequence in crate::token_parser::MULTI_LINE_STRING_SEQUENCES {
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
