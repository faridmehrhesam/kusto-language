use std::ops::Range;

mod parser;
mod scanner;
mod utilities;

#[cfg(test)]
mod tests;

pub use parser::parse_tokens;

const AVG_BYTES_PER_TOKEN: usize = 5;
const MULTI_LINE_STRING_SEQUENCES: &[&[u8]] = &[b"```", b"~~~"];
const BOOL_LITERALS: &[&[u8]] = &[b"true", b"false", b"True", b"False", b"TRUE", b"FALSE"];
const TIMESPAN_SUFFIXES: &[&[u8]] = &[
    b"microseconds",
    b"milliseconds",
    b"nanoseconds",
    b"microsecond",
    b"millisecond",
    b"nanosecond",
    b"microsec",
    b"millisec",
    b"nanosec",
    b"minutes",
    b"seconds",
    b"hours",
    b"micros",
    b"millis",
    b"nanos",
    b"minute",
    b"second",
    b"hour",
    b"micro",
    b"milli",
    b"nano",
    b"ticks",
    b"tick",
    b"hrs",
    b"sec",
    b"min",
    b"day",
    b"ms",
    b"hr",
    b"d",
    b"h",
    b"m",
    b"s",
];

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
    BooleanLiteralToken,
    LongLiteralToken,
    RealLiteralToken,
    TimespanLiteralToken,
    RawGuidLiteralToken,
    StringLiteralToken,

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
    kind: SyntaxKind,
    trivia_span: Range<usize>,
    text_span: Range<usize>,
}

impl LexicalToken {
    pub fn new(kind: SyntaxKind, trivia: Range<usize>, text: Range<usize>) -> Self {
        Self {
            kind,
            trivia_span: trivia,
            text_span: text,
        }
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }

    pub fn trivia_span(&self) -> &Range<usize> {
        &self.trivia_span
    }

    pub fn text_span(&self) -> &Range<usize> {
        &self.text_span
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
