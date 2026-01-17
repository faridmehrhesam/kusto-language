use std::ops::Range;

const AVG_BYTES_PER_TOKEN: usize = 5;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum SyntaxKind {
    // punctuation tokens
    OpenParenToken,
    CloseParenToken,
    OpenBracketToken,
    CloseBracketToken,
    OpenBraceToken,
    CloseBraceToken,
    BarToken,
    DotToken,
    DotDotToken,
    PlusToken,
    MinusToken,
    AsteriskToken,
    SlashToken,
    PercentToken,
    LessThanToken,
    LessThanOrEqualToken,
    LessThanBarToken,
    LessThanGreaterThanToken,
    GreaterThanToken,
    GreaterThanOrEqualToken,
    EqualToken,
    EqualEqualToken,
    FatArrowToken,
    EqualTildeToken,
    BangEqualToken,
    BangTildeToken,
    ColonToken,
    SemicolonToken,
    CommaToken,
    AtToken,
    QuestionToken,

    // literal tokens
    RawGuidLiteralToken,

    // identifier
    IdentifierToken,

    // other tokens
    DirectiveToken,
    EndOfTextToken,
    BadToken,
}

#[derive(Debug, Clone, Copy)]
pub struct ParseOptions {
    pub always_produce_end_tokens: bool,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            always_produce_end_tokens: true,
        }
    }
}

impl ParseOptions {
    pub fn new(always_produce_end_tokens: bool) -> Self {
        Self {
            always_produce_end_tokens,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LexicalToken {
    pub kind: SyntaxKind,
    pub trivia_span: Range<usize>,
    pub text_span: Range<usize>,
}

impl LexicalToken {
    pub fn new(kind: SyntaxKind, trivia: Range<usize>, text: Range<usize>) -> Self {
        Self {
            kind,
            trivia_span: trivia,
            text_span: text,
        }
    }

    /// Returns total length (trivia + text)
    pub fn len(&self) -> usize {
        (self.trivia_span.end - self.trivia_span.start)
            + (self.text_span.end - self.text_span.start)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

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

fn next_token(
    bytes: &[u8],
    start: usize,
    options: &ParseOptions,
) -> Option<LexicalToken> {
    let mut pos = start;
    let trivia = parse_trivia(bytes, start).unwrap_or(start..start);
    let has_trivia = !trivia.is_empty();

    pos += trivia.end - trivia.start;

    if let Some(&byte) = peek(bytes, pos) {
        if !byte.is_ascii_alphanumeric() {
            if let Some((kind, range)) = parse_punctuation(bytes, pos) {
                return Some(LexicalToken::new(kind, trivia, range));
            }

            if byte == b'#' {
                let directive_end = get_line_end(bytes, pos);
                return Some(LexicalToken {
                    kind: SyntaxKind::DirectiveToken,
                    trivia_span: trivia,
                    text_span: pos..directive_end,
                });
            }

            if is_at_end(bytes, pos) {
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

        if is_identifier_start_char(byte) {
            if let Some(raw_guid_len) = scan_raw_guid_literal(bytes, pos) {
                return Some(LexicalToken {
                    kind: SyntaxKind::RawGuidLiteralToken,
                    trivia_span: trivia,
                    text_span: pos..pos + raw_guid_len,
                });
            }

            if let Some(id_len) = scan_identifier(bytes, pos) {
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

        pos += bytes[pos..]
            .iter()
            .take_while(|&&b| b.is_ascii_whitespace())
            .count();

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
            let next_byte = peek(bytes, start + 1);
            if let Some(&b) = next_byte {
                if is_string_literal_start_quote(b) {
                    return None;
                }
            }
            (SyntaxKind::AtToken, 1)
        }
        b'?' => (SyntaxKind::QuestionToken, 1),
        _ => return None,
    };

    Some((kind, start..start + width))
}

fn scan_identifier(bytes: &[u8], start: usize) -> Option<usize> {
    let byte = peek(bytes, start)?;
    let mut pos = start;

    if is_identifier_start_char(*byte) {
        pos += 1;
        pos += bytes[pos..]
            .iter()
            .take_while(|&&b| is_identifier_char(b))
            .count();
    } else if let Some(digit_count) = scan_digits(bytes, pos) {
        if let Some(next_byte) = peek(bytes, pos + digit_count) {
            // must have at least one one letter or _ after digits
            if next_byte.is_ascii_alphabetic() || *next_byte == b'_' {
                pos += digit_count;
                pos += bytes[pos..]
                    .iter()
                    .take_while(|&&b| is_identifier_char(b))
                    .count();
            }
        }
    }

    if pos == start {
        None
    } else {
        Some(pos - start)
    }
}

fn scan_digits(bytes: &[u8], start: usize) -> Option<usize> {
    let digit_count = bytes[start..]
        .iter()
        .take_while(|&&b| b.is_ascii_digit())
        .count();

    if digit_count == 0 {
        None
    } else {
        Some(digit_count)
    }
}

fn scan_raw_guid_literal(bytes: &[u8], start: usize) -> Option<usize> {
    let mut pos = start;

    // 8 hex digits
    let eight_hex_len = scan_hex_digits(bytes, pos, 8)?;
    pos += eight_hex_len;

    // '-'
    if peek(bytes, pos) != Some(&b'-') {
        return None;
    }
    pos += 1;

    // 4 hex digits
    let four_hex_len = scan_hex_digits(bytes, pos, 4)?;
    pos += four_hex_len;

    // '-'
    if peek(bytes, pos) != Some(&b'-') {
        return None;
    }
    pos += 1;

    // 4 hex digits
    let four_hex_len = scan_hex_digits(bytes, pos, 4)?;
    pos += four_hex_len;

    // '-'
    if peek(bytes, pos) != Some(&b'-') {
        return None;
    }
    pos += 1;

    // 4 hex digits
    let four_hex_len = scan_hex_digits(bytes, pos, 4)?;
    pos += four_hex_len;

    // '-'
    if peek(bytes, pos) != Some(&b'-') {
        return None;
    }
    pos += 1;

    // 12 hex digits
    let twelve_hex_len = scan_hex_digits(bytes, pos, 12)?;
    pos += twelve_hex_len;

    Some(pos - start)
}

fn scan_hex_digits(bytes: &[u8], start: usize, count: usize) -> Option<usize> {
    let mut pos = start;
    for _ in 0..count {
        let byte = peek(bytes, pos)?;
        if !byte.is_ascii_hexdigit() {
            return None;
        }
        pos += 1;
    }

    Some(pos - start)
}

fn get_next_line_start(bytes: &[u8], start: usize) -> Option<usize> {
    let next_start = get_next_line_break_start(bytes, start)?;
    let line_break_length = get_next_line_break_length(bytes, next_start)?;

    Some(next_start + line_break_length)
}

fn get_line_end(bytes: &[u8], start: usize) -> usize {
    start + get_line_length(bytes, start, false)
}

fn get_line_length(bytes: &[u8], start: usize, include_line_break: bool) -> usize {
    let mut pos = start;

    while let Some(&byte) = peek(bytes, pos)
        && !is_line_break_start(byte)
    {
        pos += 1;
    }

    if include_line_break
        && let Some(line_break_length) = get_next_line_break_length(bytes, start)
    {
        pos += line_break_length
    }

    pos - start
}

fn get_next_line_break_length(bytes: &[u8], start: usize) -> Option<usize> {
    let break_start = get_next_line_break_start(bytes, start)?;

    if peek(bytes, break_start) == Some(&b'\r')
        && peek(bytes, break_start + 1) == Some(&b'\n')
    {
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

#[inline(always)]
fn peek(bytes: &[u8], pos: usize) -> Option<&u8> {
    bytes.get(pos)
}

#[inline(always)]
fn is_at_end(bytes: &[u8], pos: usize) -> bool {
    pos >= bytes.len()
}

#[inline(always)]
fn is_line_break_start(byte: u8) -> bool {
    byte == b'\r' || byte == b'\n'
}

#[inline(always)]
fn is_string_literal_start_quote(byte: u8) -> bool {
    byte == b'"' || byte == b'\''
}

#[inline(always)]
fn is_identifier_start_char(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_' || byte == b'$'
}

#[inline(always)]
fn is_identifier_char(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_text<'a>(source: &'a str, range: std::ops::Range<usize>) -> &'a str {
        &source[range.start..range.end]
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        let options = ParseOptions::default();
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, SyntaxKind::EndOfTextToken);
        assert_eq!(tokens[0].len(), 0);
    }

    #[test]
    fn test_single_punctuation() {
        let input = "+";
        let options = ParseOptions::default();
        let tokens = parse_tokens(input, &options);

        // Expect: [+] [EOF]
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, SyntaxKind::PlusToken);
        assert_eq!(get_text(input, tokens[0].text_span.clone()), "+");
        assert_eq!(tokens[1].kind, SyntaxKind::EndOfTextToken);
    }

    #[test]
    fn test_multi_char_punctuation() {
        let input = "<= == => ..";
        let options = ParseOptions::default();
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens[0].kind, SyntaxKind::LessThanOrEqualToken);

        assert_eq!(tokens[1].kind, SyntaxKind::EqualEqualToken);
        assert_eq!(get_text(input, tokens[1].trivia_span.clone()), " ");

        assert_eq!(tokens[2].kind, SyntaxKind::FatArrowToken);
        assert_eq!(get_text(input, tokens[2].trivia_span.clone()), " ");

        assert_eq!(tokens[3].kind, SyntaxKind::DotDotToken);
        assert_eq!(get_text(input, tokens[3].trivia_span.clone()), " ");
    }

    #[test]
    fn test_trivia_and_comments() {
        let input = "  // this is a comment\n  +  ";
        let options = ParseOptions::default();
        let tokens = parse_tokens(input, &options);
        let plus = &tokens[0];

        assert_eq!(plus.kind, SyntaxKind::PlusToken);
        assert_eq!(
            get_text(input, plus.trivia_span.clone()),
            "  // this is a comment\n  "
        );
        assert_eq!(get_text(input, plus.text_span.clone()), "+");

        // The EOF token should capture the trailing whitespace as trivia
        let eof = &tokens[1];
        assert_eq!(eof.kind, SyntaxKind::EndOfTextToken);
        assert_eq!(get_text(input, eof.trivia_span.clone()), "  ");
    }

    #[test]
    fn test_bad_token() {
        let input = "این یک متن فارسی است"; // Non-ASCII
        let options = ParseOptions::default();
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens[0].kind, SyntaxKind::BadToken);
        assert_eq!(tokens[0].text_span.end - tokens[0].text_span.start, 1);
    }

    #[test]
    fn test_complex_punctuation_chain() {
        let input = "!=!~<|<?";
        let options = ParseOptions::default();
        let tokens = parse_tokens(input, &options);

        let kinds: Vec<SyntaxKind> = tokens.iter().map(|t| t.kind).collect();
        assert_eq!(
            kinds,
            vec![
                SyntaxKind::BangEqualToken,
                SyntaxKind::BangTildeToken,
                SyntaxKind::LessThanBarToken,
                SyntaxKind::LessThanToken,
                SyntaxKind::QuestionToken,
                SyntaxKind::EndOfTextToken,
            ]
        );
    }

    #[test]
    fn test_at_token_vs_string_literal_start() {
        let input = "@\"";
        let options = ParseOptions::default();
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens[0].kind, SyntaxKind::BadToken);

        let input_valid = "@ ";
        let tokens_valid = parse_tokens(input_valid, &options);
        assert_eq!(tokens_valid[0].kind, SyntaxKind::AtToken);
    }

    #[test]
    fn test_options_no_end_tokens() {
        let input = "+";
        let options = ParseOptions::new(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, SyntaxKind::PlusToken);
    }

    #[test]
    fn test_all_possible_punctuations() {
        let input =
            "( ) [ ] { } | . .. + - * / % < <= <| <> > >= = == => =~ != !~ : ; , @ ?";
        let options = ParseOptions::new(false);
        let tokens = parse_tokens(input, &options);
        let expected_kinds = vec![
            SyntaxKind::OpenParenToken,
            SyntaxKind::CloseParenToken,
            SyntaxKind::OpenBracketToken,
            SyntaxKind::CloseBracketToken,
            SyntaxKind::OpenBraceToken,
            SyntaxKind::CloseBraceToken,
            SyntaxKind::BarToken,
            SyntaxKind::DotToken,
            SyntaxKind::DotDotToken,
            SyntaxKind::PlusToken,
            SyntaxKind::MinusToken,
            SyntaxKind::AsteriskToken,
            SyntaxKind::SlashToken,
            SyntaxKind::PercentToken,
            SyntaxKind::LessThanToken,
            SyntaxKind::LessThanOrEqualToken,
            SyntaxKind::LessThanBarToken,
            SyntaxKind::LessThanGreaterThanToken,
            SyntaxKind::GreaterThanToken,
            SyntaxKind::GreaterThanOrEqualToken,
            SyntaxKind::EqualToken,
            SyntaxKind::EqualEqualToken,
            SyntaxKind::FatArrowToken,
            SyntaxKind::EqualTildeToken,
            SyntaxKind::BangEqualToken,
            SyntaxKind::BangTildeToken,
            SyntaxKind::ColonToken,
            SyntaxKind::SemicolonToken,
            SyntaxKind::CommaToken,
            SyntaxKind::AtToken,
            SyntaxKind::QuestionToken,
        ];

        assert_eq!(
            tokens.len(),
            expected_kinds.len(),
            "Token count mismatch. Expected {}, got {}",
            expected_kinds.len(),
            tokens.len()
        );

        for (i, expected_kind) in expected_kinds.iter().enumerate() {
            assert_eq!(
                tokens[i].kind, *expected_kind,
                "Mismatch at index {}: expected {:?}, but found {:?}",
                i, expected_kind, tokens[i].kind
            );
        }
    }

    #[test]
    fn test_directive() {
        let input = "#crp query_language=kql";
        let options = ParseOptions::new(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, SyntaxKind::DirectiveToken);
    }

    #[test]
    fn test_directive_with_other_tokens() {
        let input = " + #crp query_language=kql\n +";
        let options = ParseOptions::new(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, SyntaxKind::PlusToken);
        assert_eq!(tokens[1].kind, SyntaxKind::DirectiveToken);
        assert_eq!(tokens[2].kind, SyntaxKind::PlusToken);
    }

    #[test]
    fn test_identifier() {
        let possible_inputs = vec![
            "Column", "Column1", "Column_", "_Column", "_Column1", "_Column_", "$Column",
            "$Column1", "$Column_", "1Column", "1_",
        ];

        for input in possible_inputs {
            let options = ParseOptions::new(false);
            let tokens = parse_tokens(input, &options);

            assert_eq!(tokens.len(), 1, "{input}");
            assert_eq!(tokens[0].kind, SyntaxKind::IdentifierToken);
            assert_eq!(get_text(input, tokens[0].trivia_span.clone()), "");
            assert_eq!(get_text(input, tokens[0].text_span.clone()), input);
        }
    }

    #[test]
    fn test_raw_guid_literal() {
        let possible_inputs = vec![
            "123e4567-e89b-12d3-a456-426614174000",
            "00000000-0000-0000-0000-000000000000",
            "ffffffff-ffff-ffff-ffff-ffffffffffff",
        ];

        for input in possible_inputs {
            let options = ParseOptions::new(false);
            let tokens = parse_tokens(input, &options);

            assert_eq!(tokens.len(), 1, "{input}");
            assert_eq!(tokens[0].kind, SyntaxKind::RawGuidLiteralToken);
            assert_eq!(get_text(input, tokens[0].trivia_span.clone()), "");
            assert_eq!(get_text(input, tokens[0].text_span.clone()), input);
        }
    }
}
