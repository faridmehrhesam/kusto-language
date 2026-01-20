use crate::token_parser::{ParseOptions, SyntaxKind, parse_tokens};

fn get_text(source: &str, range: std::ops::Range<usize>) -> &str {
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
fn test_options_no_end_tokens() {
    let input = "+";
    let options = ParseOptions::new(false);
    let tokens = parse_tokens(input, &options);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, SyntaxKind::PlusToken);
}

#[test]
fn test_all_possible_punctuations() {
    let input = "( ) [ ] { } | . .. + - * / % < <= <| <> > >= = == => =~ != !~ : ; , @ ?";
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
        "Column", "Column1", "Column_", "_Column", "_Column1", "_Column_", "$Column", "$Column1",
        "$Column_", "1Column", "1_",
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

#[test]
fn test_real_literal() {
    let possible_inputs = vec![
        "1.0", "1.0e10", "1.0E10", "1.0e-10", "1.0E-10", "1.0e+10", "1.0E+10", "1.e-5", "1.E-5",
        "1.", "1e10",
    ];

    for input in possible_inputs {
        let options = ParseOptions::new(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "{input}");
        assert_eq!(tokens[0].kind, SyntaxKind::RealLiteralToken);
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
        let options = ParseOptions::new(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "{input}");
        assert_eq!(tokens[0].kind, SyntaxKind::TimespanLiteralToken);
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
        let options = ParseOptions::new(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "{input}");
        assert_eq!(tokens[0].kind, SyntaxKind::LongLiteralToken);
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
        let options = ParseOptions::new(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "{input}");
        assert_eq!(tokens[0].kind, SyntaxKind::StringLiteralToken);
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
        let options = ParseOptions::new(false);
        let tokens = parse_tokens(input, &options);

        assert_ne!(tokens[0].kind, SyntaxKind::StringLiteralToken, "{input}");
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
        let options = ParseOptions::new(false);
        let tokens = parse_tokens(input, &options);

        assert_ne!(tokens[0].kind, SyntaxKind::StringLiteralToken, "{input}");
    }
}

#[test]
fn test_escape_at_eof() {
    let input = r#""string\""#;
    let options = ParseOptions::new(false);
    let tokens = parse_tokens(input, &options);

    assert_ne!(tokens[0].kind, SyntaxKind::StringLiteralToken);
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
        let options = ParseOptions::new(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "{input}");
        assert_eq!(tokens[0].kind, SyntaxKind::StringLiteralToken, "{input}");
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
        let options = ParseOptions::new(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "{input}");
        assert_eq!(tokens[0].kind, SyntaxKind::StringLiteralToken, "{input}");
        assert_eq!(get_text(input, tokens[0].text_span.clone()), input);
    }
}

#[test]
fn test_string_terminates_at_newline() {
    let input = "\"string with\nunfinished line";
    let options = ParseOptions::new(false);
    let tokens = parse_tokens(input, &options);

    // String should terminate at newline, making it invalid
    assert_ne!(tokens[0].kind, SyntaxKind::StringLiteralToken);
}

#[test]
fn test_bool_literal() {
    let possible_inputs = vec!["true", "false", "True", "False", "TRUE", "FALSE"];

    for input in possible_inputs {
        let options = ParseOptions::new(false);
        let tokens = parse_tokens(input, &options);

        assert_eq!(tokens.len(), 1, "{input}");
        assert_eq!(tokens[0].kind, SyntaxKind::BooleanLiteralToken);
        assert_eq!(get_text(input, tokens[0].trivia_span.clone()), "");
        assert_eq!(get_text(input, tokens[0].text_span.clone()), input);
    }
}
