use crate::token_parser::{
    KeywordKind, LiteralKind, ParseOptions, PunctuationKind, TokenKind, parse_tokens,
};

#[test]
fn test_empty_string() {
    let input = "";
    let options = ParseOptions::default();
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], TokenKind::EndOfFile);
}

#[test]
fn test_single_punctuation() {
    let input = "+";
    let options = ParseOptions::default();
    let tokens = parse_tokens(input, &options);

    // Expect: [+] [EOF]
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], TokenKind::Punctuation(PunctuationKind::Plus));
    assert_eq!(tokens[1], TokenKind::EndOfFile);
}

#[test]
fn test_multi_char_punctuation() {
    let input = "<= == => ..";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(
        tokens[0],
        TokenKind::Punctuation(PunctuationKind::LessThanOrEqual)
    );
    assert_eq!(
        tokens[1],
        TokenKind::Punctuation(PunctuationKind::EqualEqual)
    );
    assert_eq!(tokens[2], TokenKind::Punctuation(PunctuationKind::FatArrow));
    assert_eq!(tokens[3], TokenKind::Punctuation(PunctuationKind::DotDot));
}

#[test]
fn test_trivia_and_comments() {
    let input = "  // this is a comment\n  +  ";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens[0], TokenKind::Punctuation(PunctuationKind::Plus));
    assert_eq!(tokens[1], TokenKind::EndOfFile);
}

#[test]
fn test_bad_token() {
    let input = "این یک متن فارسی است"; // Non-ASCII
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens[0], TokenKind::Bad("ا".to_string()));
}

#[test]
fn test_bad_token_utf8_2_byte() {
    let input = "¿"; // 2-byte UTF-8 character (U+00BF)
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens[0], TokenKind::Bad("¿".to_string()));
}

#[test]
fn test_bad_token_utf8_3_byte() {
    let input = "€"; // 3-byte UTF-8 character (U+20AC)
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens[0], TokenKind::Bad("€".to_string()));
}

#[test]
fn test_bad_token_utf8_4_byte() {
    let input = "𝕏"; // 4-byte UTF-8 character (U+1D54F)
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens[0], TokenKind::Bad("𝕏".to_string()));
}

#[test]
fn test_multiple_consecutive_bad_tokens() {
    let input = "¿€𝕏"; // Mix of 2, 3, and 4-byte UTF-8 characters
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], TokenKind::Bad("¿".to_string()));
    assert_eq!(tokens[1], TokenKind::Bad("€".to_string()));
    assert_eq!(tokens[2], TokenKind::Bad("𝕏".to_string()));
}

#[test]
fn test_bad_token_mixed_with_valid_tokens() {
    let input = "¿ + €";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 3);

    assert_eq!(tokens[0], TokenKind::Bad("¿".to_string()));
    assert_eq!(tokens[1], TokenKind::Punctuation(PunctuationKind::Plus));
    assert_eq!(tokens[2], TokenKind::Bad("€".to_string()));
}

#[test]
fn test_complex_punctuation_chain() {
    let input = "!=!~<|<?";
    let options = ParseOptions::default();
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 6);
    assert_eq!(
        tokens[0],
        TokenKind::Punctuation(PunctuationKind::BangEqual)
    );
    assert_eq!(
        tokens[1],
        TokenKind::Punctuation(PunctuationKind::BangTilde)
    );
    assert_eq!(
        tokens[2],
        TokenKind::Punctuation(PunctuationKind::LessThanBar)
    );
    assert_eq!(tokens[3], TokenKind::Punctuation(PunctuationKind::LessThan));
    assert_eq!(tokens[4], TokenKind::Punctuation(PunctuationKind::Question));
    assert_eq!(tokens[5], TokenKind::EndOfFile);
}

#[test]
fn test_options_no_end_tokens() {
    let input = "+";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], TokenKind::Punctuation(PunctuationKind::Plus));
}

#[test]
fn test_all_possible_punctuations() {
    let input = "( ) [ ] { } | . .. + - * / % < <= <| <> > >= = == => =~ != !~ : ; , @ ?";
    let options = ParseOptions::default()
        .with_always_produce_end_tokens(false)
        .with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);
    let expected_kinds = vec![
        TokenKind::Punctuation(PunctuationKind::OpenParen),
        TokenKind::Punctuation(PunctuationKind::CloseParen),
        TokenKind::Punctuation(PunctuationKind::OpenBracket),
        TokenKind::Punctuation(PunctuationKind::CloseBracket),
        TokenKind::Punctuation(PunctuationKind::OpenBrace),
        TokenKind::Punctuation(PunctuationKind::CloseBrace),
        TokenKind::Punctuation(PunctuationKind::Bar),
        TokenKind::Punctuation(PunctuationKind::Dot),
        TokenKind::Punctuation(PunctuationKind::DotDot),
        TokenKind::Punctuation(PunctuationKind::Plus),
        TokenKind::Punctuation(PunctuationKind::Minus),
        TokenKind::Punctuation(PunctuationKind::Asterisk),
        TokenKind::Punctuation(PunctuationKind::Slash),
        TokenKind::Punctuation(PunctuationKind::Percent),
        TokenKind::Punctuation(PunctuationKind::LessThan),
        TokenKind::Punctuation(PunctuationKind::LessThanOrEqual),
        TokenKind::Punctuation(PunctuationKind::LessThanBar),
        TokenKind::Punctuation(PunctuationKind::LessThanGreaterThan),
        TokenKind::Punctuation(PunctuationKind::GreaterThan),
        TokenKind::Punctuation(PunctuationKind::GreaterThanOrEqual),
        TokenKind::Punctuation(PunctuationKind::Equal),
        TokenKind::Punctuation(PunctuationKind::EqualEqual),
        TokenKind::Punctuation(PunctuationKind::FatArrow),
        TokenKind::Punctuation(PunctuationKind::EqualTilde),
        TokenKind::Punctuation(PunctuationKind::BangEqual),
        TokenKind::Punctuation(PunctuationKind::BangTilde),
        TokenKind::Punctuation(PunctuationKind::Colon),
        TokenKind::Punctuation(PunctuationKind::Semicolon),
        TokenKind::Punctuation(PunctuationKind::Comma),
        TokenKind::Punctuation(PunctuationKind::At),
        TokenKind::Punctuation(PunctuationKind::Question),
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
            tokens[i], *expected_kind,
            "Mismatch at index {}: expected {:?}, but found {:?}",
            i, expected_kind, tokens[i]
        );
    }
}

#[test]
fn test_directive() {
    let input = "#crp query_language=kql";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], TokenKind::Directive(input.to_string()));
}

#[test]
fn test_directive_with_other_tokens() {
    let input = " + #crp query_language=kql\n +";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], TokenKind::Punctuation(PunctuationKind::Plus));
    assert_eq!(
        tokens[1],
        TokenKind::Directive("#crp query_language=kql".to_string())
    );
    assert_eq!(tokens[2], TokenKind::Punctuation(PunctuationKind::Plus));
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
        assert_eq!(tokens[0], TokenKind::Identifier(input.to_string()));
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
        assert_eq!(
            tokens[0],
            TokenKind::Literal(LiteralKind::RawGuid(input.to_string()))
        );
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
        assert_eq!(
            tokens[0],
            TokenKind::Literal(LiteralKind::Real(input.to_string()))
        );
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
        assert_eq!(
            tokens[0],
            TokenKind::Literal(LiteralKind::Timespan(input.to_string()))
        );
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
        assert_eq!(
            tokens[0],
            TokenKind::Literal(LiteralKind::Long(input.to_string()))
        );
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
        assert_eq!(
            tokens[0],
            TokenKind::Literal(LiteralKind::String(input.to_string()))
        );
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

        assert_ne!(
            tokens[0],
            TokenKind::Literal(LiteralKind::String(input.to_string())),
            "{input}"
        );
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

        assert_ne!(
            tokens[0],
            TokenKind::Literal(LiteralKind::String(input.to_string())),
            "{input}"
        );
    }
}

#[test]
fn test_escape_at_eof() {
    let input = r#""string\""#;
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_ne!(
        tokens[0],
        TokenKind::Literal(LiteralKind::String(input.to_string()))
    );
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
        assert_eq!(
            tokens[0],
            TokenKind::Literal(LiteralKind::String(input.to_string())),
            "{input}"
        );
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
        assert_eq!(
            tokens[0],
            TokenKind::Literal(LiteralKind::String(input.to_string())),
            "{input}"
        );
    }
}

#[test]
fn test_string_terminates_at_newline() {
    let input = "\"string with\nunfinished line";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    // String should terminate at newline, making it invalid
    assert_ne!(
        tokens[0],
        TokenKind::Literal(LiteralKind::String(input.to_string()))
    );
}

#[test]
fn test_bool_literal() {
    let possible_inputs = vec!["true", "false", "True", "False", "TRUE", "FALSE"];

    for input in possible_inputs {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "{input}");
        assert_eq!(
            tokens[0],
            TokenKind::Literal(LiteralKind::Boolean(input.to_string()))
        );
    }
}

// ============ Keyword Recognition Tests ============

#[test]
fn test_simple_keywords() {
    let test_cases = vec![
        ("let", TokenKind::Keyword(KeywordKind::Let)),
        ("in", TokenKind::Keyword(KeywordKind::In)),
        ("and", TokenKind::Keyword(KeywordKind::And)),
        ("or", TokenKind::Keyword(KeywordKind::Or)),
        ("where", TokenKind::Keyword(KeywordKind::Where)),
        ("extend", TokenKind::Keyword(KeywordKind::Extend)),
        ("project", TokenKind::Keyword(KeywordKind::Project)),
        ("summarize", TokenKind::Keyword(KeywordKind::Summarize)),
        ("join", TokenKind::Keyword(KeywordKind::Join)),
        ("union", TokenKind::Keyword(KeywordKind::Union)),
    ];

    for (input, expected_kind) in test_cases {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "Failed for keyword: {input}");
        assert_eq!(tokens[0], expected_kind, "Wrong kind for keyword: {input}");
    }
}

#[test]
fn test_type_keywords() {
    let test_cases = vec![
        ("int", TokenKind::Keyword(KeywordKind::Int)),
        ("int32", TokenKind::Keyword(KeywordKind::Int32)),
        ("long", TokenKind::Keyword(KeywordKind::Long)),
        ("real", TokenKind::Keyword(KeywordKind::Real)),
        ("string", TokenKind::Keyword(KeywordKind::String)),
        ("bool", TokenKind::Keyword(KeywordKind::Bool)),
        ("datetime", TokenKind::Keyword(KeywordKind::DateTime)),
        ("timespan", TokenKind::Keyword(KeywordKind::Timespan)),
        ("decimal", TokenKind::Keyword(KeywordKind::Decimal)),
        ("dynamic", TokenKind::Keyword(KeywordKind::Dynamic)),
        ("guid", TokenKind::Keyword(KeywordKind::Guid)),
    ];

    for (input, expected_kind) in test_cases {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "Failed for type keyword: {input}");
        assert_eq!(
            tokens[0], expected_kind,
            "Wrong kind for type keyword: {input}"
        );
    }
}

#[test]
fn test_long_keywords() {
    let test_cases = vec![
        (
            "storedqueryresultcontainers",
            TokenKind::Keyword(KeywordKind::StoredQueryResultContainers),
        ),
        (
            "materialized-view-combine",
            TokenKind::Keyword(KeywordKind::MaterializedViewCombine),
        ),
        (
            "restricted_view_access",
            TokenKind::Keyword(KeywordKind::RestrictedViewAccess),
        ),
        (
            "graph-mark-components",
            TokenKind::Keyword(KeywordKind::GraphMarkComponents),
        ),
    ];

    for (input, expected_kind) in test_cases {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "Failed for keyword: {input}");
        assert_eq!(tokens[0], expected_kind, "Wrong kind for keyword: {input}");
    }
}

#[test]
fn test_keyword_with_special_chars() {
    let test_cases = vec![
        ("!in", TokenKind::Keyword(KeywordKind::NotIn)),
        ("!has", TokenKind::Keyword(KeywordKind::NotHas)),
        (
            "!contains",
            TokenKind::Keyword(KeywordKind::NotBangContains),
        ),
        (
            "!startswith",
            TokenKind::Keyword(KeywordKind::NotStartsWith),
        ),
        ("in~", TokenKind::Keyword(KeywordKind::InCs)),
        ("has_any", TokenKind::Keyword(KeywordKind::HasAny)),
        ("contains_cs", TokenKind::Keyword(KeywordKind::ContainsCs2)),
    ];

    for (input, expected_kind) in test_cases {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "Failed for keyword: {input}");
        assert_eq!(tokens[0], expected_kind, "Wrong kind for keyword: {input}");
    }
}

#[test]
fn test_keyword_boundary_detection() {
    // Keywords should not match if followed by identifier characters
    let test_cases = vec![
        ("letx", TokenKind::Identifier("letx".to_string())),
        ("where_col", TokenKind::Identifier("where_col".to_string())),
        ("int32", TokenKind::Keyword(KeywordKind::Int32)), // This is actually a different keyword
        ("datetime2", TokenKind::Identifier("datetime2".to_string())),
    ];

    for (input, expected_kind) in test_cases {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "Failed for input: {input}");
        assert_eq!(tokens[0], expected_kind, "Wrong kind for input: {input}");
    }
}

#[test]
fn test_keyword_followed_by_punctuation() {
    let input = "let x = 5";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens[0], TokenKind::Keyword(KeywordKind::Let));
    assert_eq!(tokens[1], TokenKind::Identifier("x".to_string()));
    assert_eq!(tokens[2], TokenKind::Punctuation(PunctuationKind::Equal));
    assert_eq!(
        tokens[3],
        TokenKind::Literal(LiteralKind::Long("5".to_string()))
    );
}

#[test]
fn test_longest_keyword_match() {
    // Test that longer keywords are matched before shorter ones
    let test_cases = vec![
        ("in", TokenKind::Keyword(KeywordKind::In)),
        ("in~", TokenKind::Keyword(KeywordKind::InCs)),
        ("has", TokenKind::Keyword(KeywordKind::Has)),
        ("has_any", TokenKind::Keyword(KeywordKind::HasAny)),
        ("has_all", TokenKind::Keyword(KeywordKind::HasAll)),
        ("contains", TokenKind::Keyword(KeywordKind::Contains)),
        ("contains_cs", TokenKind::Keyword(KeywordKind::ContainsCs2)),
    ];

    for (input, expected_kind) in test_cases {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "Failed for input: {input}");
        assert_eq!(tokens[0], expected_kind, "Wrong kind for input: {input}");
    }
}

// ============ Goo Literal Tests ============

#[test]
fn test_goo_int_literal() {
    let input = "int(42)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0],
        TokenKind::Literal(LiteralKind::Int(input.to_string()))
    );
}

#[test]
fn test_goo_long_literal() {
    let input = "long(123456789)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0],
        TokenKind::Literal(LiteralKind::Long(input.to_string()))
    );
}

#[test]
fn test_goo_datetime_literal() {
    let input = "datetime(2024-01-01T12:00:00Z)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0],
        TokenKind::Literal(LiteralKind::DateTime(input.to_string()))
    );
}

#[test]
fn test_goo_timespan_literal() {
    let input = "timespan(1d)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0],
        TokenKind::Literal(LiteralKind::Timespan(input.to_string()))
    );
}

#[test]
fn test_goo_real_literal() {
    let input = "real(3.14159)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0],
        TokenKind::Literal(LiteralKind::Real(input.to_string()))
    );
}

#[test]
fn test_goo_decimal_literal() {
    let input = "decimal(99.99)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0],
        TokenKind::Literal(LiteralKind::Decimal(input.to_string()))
    );
}

#[test]
fn test_goo_guid_literal() {
    let input = "guid(12345678-1234-1234-1234-123456789012)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0],
        TokenKind::Literal(LiteralKind::Guid(input.to_string()))
    );
}

#[test]
fn test_goo_bool_literal() {
    let input = "bool(true)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0],
        TokenKind::Literal(LiteralKind::Boolean(input.to_string()))
    );
}

#[test]
fn test_goo_with_whitespace() {
    let input = "int( 42 )";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0],
        TokenKind::Literal(LiteralKind::Int(input.to_string()))
    );
}

#[test]
fn test_goo_unclosed_paren() {
    let input = "int(42";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    // Should parse as keyword followed by open paren and number
    assert!(tokens.len() > 1);
    assert_eq!(tokens[0], TokenKind::Keyword(KeywordKind::Int));
}

#[test]
fn test_goo_with_line_breaks_not_allowed() {
    let input = "int(\n42\n)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    // Should not parse as goo literal when line breaks not allowed
    assert!(tokens.len() > 1);
    assert_eq!(tokens[0], TokenKind::Keyword(KeywordKind::Int));
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
    assert_eq!(
        tokens[0],
        TokenKind::Literal(LiteralKind::Int(input.to_string()))
    );
}

#[test]
fn test_type_keyword_not_followed_by_paren() {
    let input = "int + 5";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens[0], TokenKind::Keyword(KeywordKind::Int));
    assert_eq!(tokens[1], TokenKind::Punctuation(PunctuationKind::Plus));
    assert_eq!(
        tokens[2],
        TokenKind::Literal(LiteralKind::Long("5".to_string()))
    );
}

#[test]
fn test_date_keyword_goo() {
    // date() should also create datetime literal
    let input = "date(2024-01-01)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0],
        TokenKind::Literal(LiteralKind::DateTime(input.to_string()))
    );
}

#[test]
fn test_time_keyword_goo() {
    // time() should create timespan literal
    let input = "time(1h)";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(
        tokens[0],
        TokenKind::Literal(LiteralKind::Timespan(input.to_string()))
    );
}

#[test]
fn test_complex_query_with_keywords() {
    let input = "let x = 5; T | where x > 10 | project col1, col2 | summarize count()";
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    let tokens = parse_tokens(input, &options);

    // Verify key tokens are present
    assert_eq!(tokens[0], TokenKind::Keyword(KeywordKind::Let));
    assert!(
        tokens
            .iter()
            .any(|t| *t == TokenKind::Keyword(KeywordKind::Where))
    );
    assert!(
        tokens
            .iter()
            .any(|t| *t == TokenKind::Keyword(KeywordKind::Project))
    );
    assert!(
        tokens
            .iter()
            .any(|t| *t == TokenKind::Keyword(KeywordKind::Summarize))
    );
}

#[test]
fn test_hint_keywords() {
    let test_cases = vec![
        (
            "hint.remote",
            TokenKind::Keyword(KeywordKind::HintDotRemote),
        ),
        (
            "hint.spread",
            TokenKind::Keyword(KeywordKind::HintDotSpread),
        ),
        (
            "hint.strategy",
            TokenKind::Keyword(KeywordKind::HintDotStrategy),
        ),
        (
            "hint.concurrency",
            TokenKind::Keyword(KeywordKind::HintDotConcurrency),
        ),
    ];

    for (input, expected_kind) in test_cases {
        let options = ParseOptions::default().with_always_produce_end_tokens(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "Failed for hint keyword: {input}");
        assert_eq!(
            tokens[0], expected_kind,
            "Wrong kind for hint keyword: {input}"
        );
    }
}
