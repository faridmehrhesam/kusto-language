use crate::token_parser::{ParseOptions, TokenKind, parse_tokens};
use std::ops::Range;

fn get_text(source: &str, range: Range<usize>) -> &str {
    &source[range.start..range.end]
}

#[test]
fn test_empty_string() {
    let input = "";
    let options = ParseOptions::default();
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::EndOfTextToken);
    assert_eq!(tokens[0].len(), 0);
}

#[test]
fn test_single_punctuation() {
    let input = "+";
    let options = ParseOptions::default();
    let tokens = parse_tokens(input, &options);

    // Expect: [+] [EOF]
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::PlusToken);
    assert_eq!(get_text(input, tokens[0].text_span.clone()), "+");
    assert_eq!(tokens[1].kind, TokenKind::EndOfTextToken);
}

#[test]
fn test_multi_char_punctuation() {
    let input = "<= == => ..";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens[0].kind, TokenKind::LessThanOrEqualToken);

    assert_eq!(tokens[1].kind, TokenKind::EqualEqualToken);
    assert_eq!(get_text(input, tokens[1].trivia_span.clone()), " ");

    assert_eq!(tokens[2].kind, TokenKind::FatArrowToken);
    assert_eq!(get_text(input, tokens[2].trivia_span.clone()), " ");

    assert_eq!(tokens[3].kind, TokenKind::DotDotToken);
    assert_eq!(get_text(input, tokens[3].trivia_span.clone()), " ");
}

#[test]
fn test_trivia_and_comments() {
    let input = "  // this is a comment\n  +  ";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);
    let plus = &tokens[0];

    assert_eq!(plus.kind, TokenKind::PlusToken);
    assert_eq!(
        get_text(input, plus.trivia_span.clone()),
        "  // this is a comment\n  "
    );
    assert_eq!(get_text(input, plus.text_span.clone()), "+");

    // The EOF token should capture the trailing whitespace as trivia
    let eof = &tokens[1];
    assert_eq!(eof.kind, TokenKind::EndOfTextToken);
    assert_eq!(get_text(input, eof.trivia_span.clone()), "  ");
}

#[test]
fn test_bad_token() {
    let input = "این یک متن فارسی است"; // Non-ASCII
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens[0].kind, TokenKind::BadToken);
    assert_eq!(tokens[0].text_span.end - tokens[0].text_span.start, 2);
}

#[test]
fn test_bad_token_utf8_2_byte() {
    let input = "¿"; // 2-byte UTF-8 character (U+00BF)
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens[0].kind, TokenKind::BadToken);
    assert_eq!(tokens[0].text_span.end - tokens[0].text_span.start, 2);
}

#[test]
fn test_bad_token_utf8_3_byte() {
    let input = "€"; // 3-byte UTF-8 character (U+20AC)
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens[0].kind, TokenKind::BadToken);
    assert_eq!(tokens[0].text_span.end - tokens[0].text_span.start, 3);
}

#[test]
fn test_bad_token_utf8_4_byte() {
    let input = "𝕏"; // 4-byte UTF-8 character (U+1D54F)
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens[0].kind, TokenKind::BadToken);
    assert_eq!(tokens[0].text_span.end - tokens[0].text_span.start, 4);
}

#[test]
fn test_multiple_consecutive_bad_tokens() {
    let input = "¿€𝕏"; // Mix of 2, 3, and 4-byte UTF-8 characters
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 3);

    assert_eq!(tokens[0].kind, TokenKind::BadToken);
    assert_eq!(tokens[0].text_span.end - tokens[0].text_span.start, 2);

    assert_eq!(tokens[1].kind, TokenKind::BadToken);
    assert_eq!(tokens[1].text_span.end - tokens[1].text_span.start, 3);

    assert_eq!(tokens[2].kind, TokenKind::BadToken);
    assert_eq!(tokens[2].text_span.end - tokens[2].text_span.start, 4);
}

#[test]
fn test_bad_token_mixed_with_valid_tokens() {
    let input = "¿ + €";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 3);

    assert_eq!(tokens[0].kind, TokenKind::BadToken);
    assert_eq!(tokens[0].text_span.end - tokens[0].text_span.start, 2);

    assert_eq!(tokens[1].kind, TokenKind::PlusToken);

    assert_eq!(tokens[2].kind, TokenKind::BadToken);
    assert_eq!(tokens[2].text_span.end - tokens[2].text_span.start, 3);
}

#[test]
fn test_complex_punctuation_chain() {
    let input = "!=!~<|<?";
    let options = ParseOptions::default();
    let tokens = parse_tokens(input, &options);

    let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind).collect();
    assert_eq!(
        kinds,
        vec![
            TokenKind::BangEqualToken,
            TokenKind::BangTildeToken,
            TokenKind::LessThanBarToken,
            TokenKind::LessThanToken,
            TokenKind::QuestionToken,
            TokenKind::EndOfTextToken,
        ]
    );
}

#[test]
fn test_options_no_end_tokens() {
    let input = "+";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::PlusToken);
}

#[test]
fn test_all_possible_punctuations() {
    let input = "( ) [ ] { } | . .. + - * / % < <= <| <> > >= = == => =~ != !~ : ; , @ ?";
    let options = ParseOptions::default()
        .with_always_produce_end_tokens(false)
        .with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);
    let expected_kinds = vec![
        TokenKind::OpenParenToken,
        TokenKind::CloseParenToken,
        TokenKind::OpenBracketToken,
        TokenKind::CloseBracketToken,
        TokenKind::OpenBraceToken,
        TokenKind::CloseBraceToken,
        TokenKind::BarToken,
        TokenKind::DotToken,
        TokenKind::DotDotToken,
        TokenKind::PlusToken,
        TokenKind::MinusToken,
        TokenKind::AsteriskToken,
        TokenKind::SlashToken,
        TokenKind::PercentToken,
        TokenKind::LessThanToken,
        TokenKind::LessThanOrEqualToken,
        TokenKind::LessThanBarToken,
        TokenKind::LessThanGreaterThanToken,
        TokenKind::GreaterThanToken,
        TokenKind::GreaterThanOrEqualToken,
        TokenKind::EqualToken,
        TokenKind::EqualEqualToken,
        TokenKind::FatArrowToken,
        TokenKind::EqualTildeToken,
        TokenKind::BangEqualToken,
        TokenKind::BangTildeToken,
        TokenKind::ColonToken,
        TokenKind::SemicolonToken,
        TokenKind::CommaToken,
        TokenKind::AtToken,
        TokenKind::QuestionToken,
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
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::DirectiveToken);
}

#[test]
fn test_directive_with_other_tokens() {
    let input = " + #crp query_language=kql\n +";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].kind, TokenKind::PlusToken);
    assert_eq!(tokens[1].kind, TokenKind::DirectiveToken);
    assert_eq!(tokens[2].kind, TokenKind::PlusToken);
}

#[test]
fn test_identifier() {
    let possible_inputs = vec![
        "Column", "Column1", "Column_", "_Column", "_Column1", "_Column_", "$Column", "$Column1",
        "$Column_", "1Column", "1_",
    ];

    for input in possible_inputs {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "{input}");
        assert_eq!(tokens[0].kind, TokenKind::IdentifierToken);
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
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "{input}");
        assert_eq!(tokens[0].kind, TokenKind::RawGuidLiteralToken);
        assert_eq!(get_text(input, tokens[0].trivia_span.clone()), "");
        assert_eq!(get_text(input, tokens[0].text_span.clone()), input);
    }
}

#[test]
fn test_real_literal() {
    let possible_inputs = vec![
        "1.0", "1.0e10", "1.0E10", "1.0e-10", "1.0E-10", "1.0e+10", "1.0E+10", "1.e-5", "1.E-5",
        "1.", "1e10",
    ];

    for input in possible_inputs {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "{input}");
        assert_eq!(tokens[0].kind, TokenKind::RealLiteralToken);
        assert_eq!(get_text(input, tokens[0].trivia_span.clone()), "");
        assert_eq!(get_text(input, tokens[0].text_span.clone()), input);
    }
}

#[test]
fn test_timespan_literal() {
    let possible_inputs = vec![
        "100microseconds",
        "200milliseconds",
        "300nanoseconds",
        "400microsecond",
        "500millisecond",
        "600nanosecond",
        "700microsec",
        "800millisec",
        "900nanosec",
        "10minutes",
        "20seconds",
        "30hours",
        "40micros",
        "50millis",
        "60nanos",
        "70minute",
        "80second",
        "90hour",
        "100micro",
        "200milli",
        "300nano",
        "400ticks",
        "500tick",
        "600hrs",
        "700sec",
        "800min",
        "900day",
        "1000ms",
        "1100hr",
        "1200d",
        "1300h",
        "1400m",
        "1500s",
        // with fractional part
        "1.5seconds",
    ];

    for input in possible_inputs {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "{input}");
        assert_eq!(tokens[0].kind, TokenKind::TimespanLiteralToken);
        assert_eq!(get_text(input, tokens[0].trivia_span.clone()), "");
        assert_eq!(get_text(input, tokens[0].text_span.clone()), input);
    }
}

#[test]
fn test_long_literal() {
    let possible_inputs = vec![
        "1234567890",
        "0",
        "9876543210123456789",
        "0x1A2B3C4D5E6F",
        "0XABCDEF123456",
    ];

    for input in possible_inputs {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "{input}");
        assert_eq!(tokens[0].kind, TokenKind::LongLiteralToken);
        assert_eq!(get_text(input, tokens[0].trivia_span.clone()), "");
        assert_eq!(get_text(input, tokens[0].text_span.clone()), input);
    }
}

#[test]
fn test_string_literal() {
    let possible_inputs = vec![
        r#"'Hello, World!'"#,
        r#""Hello, World!""#,
        r#"h'Hidden string'"#,
        r#"h"Hidden string""#,
        r#"H'Hidden string'"#,
        r#"H"Hidden string""#,
        r#"@'Verbatim string with ''single quotes'''"#,
        r#"@"Verbatim string with ""double quotes""""#,
        r#"'String with escape sequences: \n \t \\'"#,
        r#""String with escape sequences: \n \t \\""#,
        r#""""#,
        r#"''"#,
        r#"h"""#,
        r#"@"""#,
        r#"h@"""#,
        r#""این یک متن فارسی است""#,
        r#"```  multi
                    line
                    string
            ```"#,
        r#"~~~
                alternate
                multi
                line
            ~~~"#,
        r#"```single line```"#,
        r#"~~~single~~~"#,
    ];

    for input in possible_inputs {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "{input}");
        assert_eq!(tokens[0].kind, TokenKind::StringLiteralToken);
        assert_eq!(get_text(input, tokens[0].trivia_span.clone()), "");
        assert_eq!(get_text(input, tokens[0].text_span.clone()), input);
    }
}

#[test]
fn test_unclosed_string() {
    let inputs = vec![
        "\"unclosed string",
        "'unclosed string",
        "h\"unclosed",
        "@\"unclosed",
        "```unclosed multi line",
        "~~~unclosed alternate",
    ];

    for input in inputs {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_ne!(tokens[0].kind, TokenKind::StringLiteralToken, "{input}");
    }
}

#[test]
fn test_string_with_invalid_escape() {
    let inputs = vec![
        r#""invalid \q escape""#,
        r#""incomplete \u""#,
        r#""incomplete \x""#,
    ];

    for input in inputs {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_ne!(tokens[0].kind, TokenKind::StringLiteralToken, "{input}");
    }
}

#[test]
fn test_escape_at_eof() {
    let input = r#""string\""#;
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_ne!(tokens[0].kind, TokenKind::StringLiteralToken);
}

#[test]
fn test_string_with_valid_escape_sequences() {
    let inputs = vec![
        r#""escape: \n \t \r \\""#,
        r#""unicode: \u0041 \U00000041""#,
        r#""hex: \x41""#,
        r#""octal: \101""#,
        r#""quotes: \' \"""#,
    ];

    for input in inputs {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "{input}");
        assert_eq!(tokens[0].kind, TokenKind::StringLiteralToken, "{input}");
        assert_eq!(get_text(input, tokens[0].text_span.clone()), input);
    }
}

#[test]
fn test_verbatim_string_escaping() {
    let inputs = vec![
        r#"@'string with ''doubled'' quotes'"#,
        r#"@"string with ""doubled"" quotes""#,
        r#"h@'no backslash escape \n'"#,
    ];

    for input in inputs {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "{input}");
        assert_eq!(tokens[0].kind, TokenKind::StringLiteralToken, "{input}");
        assert_eq!(get_text(input, tokens[0].text_span.clone()), input);
    }
}

#[test]
fn test_string_terminates_at_newline() {
    let input = "\"string with\nunfinished line";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    // String should terminate at newline, making it invalid
    assert_ne!(tokens[0].kind, TokenKind::StringLiteralToken);
}

#[test]
fn test_bool_literal() {
    let possible_inputs = vec!["true", "false", "True", "False", "TRUE", "FALSE"];

    for input in possible_inputs {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "{input}");
        assert_eq!(tokens[0].kind, TokenKind::BooleanLiteralToken);
        assert_eq!(get_text(input, tokens[0].trivia_span.clone()), "");
        assert_eq!(get_text(input, tokens[0].text_span.clone()), input);
    }
}

// ============ Keyword Recognition Tests ============

#[test]
fn test_simple_keywords() {
    let test_cases = vec![
        ("let", TokenKind::LetKeyword),
        ("in", TokenKind::InKeyword),
        ("and", TokenKind::AndKeyword),
        ("or", TokenKind::OrKeyword),
        ("where", TokenKind::WhereKeyword),
        ("extend", TokenKind::ExtendKeyword),
        ("project", TokenKind::ProjectKeyword),
        ("summarize", TokenKind::SummarizeKeyword),
        ("join", TokenKind::JoinKeyword),
        ("union", TokenKind::UnionKeyword),
    ];

    for (input, expected_kind) in test_cases {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "Failed for keyword: {input}");
        assert_eq!(
            tokens[0].kind, expected_kind,
            "Wrong kind for keyword: {input}"
        );
        assert_eq!(get_text(input, tokens[0].text_span.clone()), input);
    }
}

#[test]
fn test_type_keywords() {
    let test_cases = vec![
        ("int", TokenKind::IntKeyword),
        ("long", TokenKind::LongKeyword),
        ("real", TokenKind::RealKeyword),
        ("string", TokenKind::StringKeyword),
        ("bool", TokenKind::BoolKeyword),
        ("datetime", TokenKind::DateTimeKeyword),
        ("timespan", TokenKind::TimespanKeyword),
        ("decimal", TokenKind::DecimalKeyword),
        ("dynamic", TokenKind::DynamicKeyword),
        ("guid", TokenKind::GuidKeyword),
    ];

    for (input, expected_kind) in test_cases {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "Failed for type keyword: {input}");
        assert_eq!(
            tokens[0].kind, expected_kind,
            "Wrong kind for type keyword: {input}"
        );
    }
}

#[test]
fn test_long_keywords() {
    let test_cases = vec![
        (
            "storedqueryresultcontainers",
            TokenKind::StoredQueryResultContainersKeyword,
        ),
        (
            "materialized-view-combine",
            TokenKind::MaterializedViewCombineKeyword,
        ),
        (
            "restricted_view_access",
            TokenKind::RestrictedViewAccessKeyword,
        ),
        (
            "graph-mark-components",
            TokenKind::GraphMarkComponentsKeyword,
        ),
    ];

    for (input, expected_kind) in test_cases {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "Failed for keyword: {input}");
        assert_eq!(
            tokens[0].kind, expected_kind,
            "Wrong kind for keyword: {input}"
        );
        assert_eq!(get_text(input, tokens[0].text_span.clone()), input);
    }
}

#[test]
fn test_keyword_with_special_chars() {
    let test_cases = vec![
        ("!in", TokenKind::NotInKeyword),
        ("!has", TokenKind::NotHasKeyword),
        ("!contains", TokenKind::NotBangContainsKeyword),
        ("!startswith", TokenKind::NotStartsWithKeyword),
        ("in~", TokenKind::InCsKeyword),
        ("has_any", TokenKind::HasAnyKeyword),
        ("contains_cs", TokenKind::ContainsCsKeyword2),
    ];

    for (input, expected_kind) in test_cases {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "Failed for keyword: {input}");
        assert_eq!(
            tokens[0].kind, expected_kind,
            "Wrong kind for keyword: {input}"
        );
    }
}

#[test]
fn test_keyword_boundary_detection() {
    // Keywords should not match if followed by identifier characters
    let test_cases = vec![
        ("letx", TokenKind::IdentifierToken),
        ("where_col", TokenKind::IdentifierToken),
        ("int32", TokenKind::Int32Keyword), // This is actually a different keyword
        ("datetime2", TokenKind::IdentifierToken),
    ];

    for (input, expected_kind) in test_cases {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "Failed for input: {input}");
        assert_eq!(
            tokens[0].kind, expected_kind,
            "Wrong kind for input: {input}"
        );
    }
}

#[test]
fn test_keyword_followed_by_punctuation() {
    let input = "let x = 5";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens[0].kind, TokenKind::LetKeyword);
    assert_eq!(tokens[1].kind, TokenKind::IdentifierToken);
    assert_eq!(tokens[2].kind, TokenKind::EqualToken);
    assert_eq!(tokens[3].kind, TokenKind::LongLiteralToken);
}

#[test]
fn test_longest_keyword_match() {
    // Test that longer keywords are matched before shorter ones
    let test_cases = vec![
        ("in", TokenKind::InKeyword),
        ("in~", TokenKind::InCsKeyword),
        ("has", TokenKind::HasKeyword),
        ("has_any", TokenKind::HasAnyKeyword),
        ("has_all", TokenKind::HasAllKeyword),
        ("contains", TokenKind::ContainsKeyword),
        ("contains_cs", TokenKind::ContainsCsKeyword2),
    ];

    for (input, expected_kind) in test_cases {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "Failed for input: {input}");
        assert_eq!(
            tokens[0].kind, expected_kind,
            "Wrong kind for input: {input}"
        );
        assert_eq!(get_text(input, tokens[0].text_span.clone()), input);
    }
}

// ============ Goo Literal Tests ============

#[test]
fn test_goo_int_literal() {
    let input = "int(42)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::IntLiteralToken);
    assert_eq!(get_text(input, tokens[0].text_span.clone()), "int(42)");
}

#[test]
fn test_goo_long_literal() {
    let input = "long(123456789)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::LongLiteralToken);
    assert_eq!(
        get_text(input, tokens[0].text_span.clone()),
        "long(123456789)"
    );
}

#[test]
fn test_goo_datetime_literal() {
    let input = "datetime(2024-01-01T12:00:00Z)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::DateTimeLiteralToken);
    assert_eq!(
        get_text(input, tokens[0].text_span.clone()),
        "datetime(2024-01-01T12:00:00Z)"
    );
}

#[test]
fn test_goo_timespan_literal() {
    let input = "timespan(1d)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::TimespanLiteralToken);
    assert_eq!(get_text(input, tokens[0].text_span.clone()), "timespan(1d)");
}

#[test]
fn test_goo_real_literal() {
    let input = "real(3.14159)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::RealLiteralToken);
    assert_eq!(
        get_text(input, tokens[0].text_span.clone()),
        "real(3.14159)"
    );
}

#[test]
fn test_goo_decimal_literal() {
    let input = "decimal(99.99)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::DecimalLiteralToken);
    assert_eq!(
        get_text(input, tokens[0].text_span.clone()),
        "decimal(99.99)"
    );
}

#[test]
fn test_goo_guid_literal() {
    let input = "guid(12345678-1234-1234-1234-123456789012)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::GuidLiteralToken);
    assert_eq!(
        get_text(input, tokens[0].text_span.clone()),
        "guid(12345678-1234-1234-1234-123456789012)"
    );
}

#[test]
fn test_goo_bool_literal() {
    let input = "bool(true)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::BooleanLiteralToken);
    assert_eq!(get_text(input, tokens[0].text_span.clone()), "bool(true)");
}

#[test]
fn test_goo_with_whitespace() {
    let input = "int( 42 )";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::IntLiteralToken);
    assert_eq!(get_text(input, tokens[0].text_span.clone()), "int( 42 )");
}

#[test]
fn test_goo_unclosed_paren() {
    let input = "int(42";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    // Should parse as keyword followed by open paren and number
    assert!(tokens.len() > 1);
    assert_eq!(tokens[0].kind, TokenKind::IntKeyword);
}

#[test]
fn test_goo_with_line_breaks_not_allowed() {
    let input = "int(\n42\n)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    // Should not parse as goo literal when line breaks not allowed
    assert!(tokens.len() > 1);
    assert_eq!(tokens[0].kind, TokenKind::IntKeyword);
}

#[test]
fn test_goo_with_line_breaks_allowed() {
    let input = "int(\n42\n)";
    let options = ParseOptions::default()
        .with_always_produce_end_tokens(false)
        .with_allow_literals_with_line_breaks(true);
    let tokens = parse_tokens(input, &options);

    // Should parse as goo literal when line breaks allowed
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::IntLiteralToken);
    assert_eq!(get_text(input, tokens[0].text_span.clone()), "int(\n42\n)");
}

#[test]
fn test_type_keyword_not_followed_by_paren() {
    let input = "int + 5";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens[0].kind, TokenKind::IntKeyword);
    assert_eq!(tokens[1].kind, TokenKind::PlusToken);
    assert_eq!(tokens[2].kind, TokenKind::LongLiteralToken);
}

#[test]
fn test_date_keyword_goo() {
    // date() should also create datetime literal
    let input = "date(2024-01-01)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::DateTimeLiteralToken);
}

#[test]
fn test_time_keyword_goo() {
    // time() should create timespan literal
    let input = "time(1h)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::TimespanLiteralToken);
}

#[test]
fn test_complex_query_with_keywords() {
    let input = "let x = 5; T | where x > 10 | project col1, col2 | summarize count()";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    // Verify key tokens are present
    assert_eq!(tokens[0].kind, TokenKind::LetKeyword);
    assert!(tokens.iter().any(|t| t.kind == TokenKind::WhereKeyword));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::ProjectKeyword));
    assert!(tokens.iter().any(|t| t.kind == TokenKind::SummarizeKeyword));
}

#[test]
fn test_hint_keywords() {
    let test_cases = vec![
        ("hint.remote", TokenKind::HintDotRemoteKeyword),
        ("hint.spread", TokenKind::HintDotSpreadKeyword),
        ("hint.strategy", TokenKind::HintDotStrategyKeyword),
        ("hint.concurrency", TokenKind::HintDotConcurrencyKeyword),
    ];

    for (input, expected_kind) in test_cases {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "Failed for hint keyword: {input}");
        assert_eq!(
            tokens[0].kind, expected_kind,
            "Wrong kind for hint keyword: {input}"
        );
    }
}
