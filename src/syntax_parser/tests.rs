use super::{expr_parsers::*, lit_parsers::*, types::*};
use crate::token_parser::{ParseOptions, TokenStream, parse_tokens};
use chumsky::prelude::*;

fn parse_tokens_no_eof<'a>(input: &'a str) -> TokenStream<'a> {
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    parse_tokens(input, &options)
}

#[test]
fn test_additive_multiplicative_precedence() {
    let token_stream = parse_tokens_no_eof("1 + 2 * 3");
    let result = additive_expr(token_stream.source).parse(&token_stream.tokens);
    assert!(!result.has_errors());
    let expr = result.into_output().expect("expected additive expr");

    assert_eq!(
        expr,
        ExprKind::BinOp {
            left: Box::new(ExprKind::Literal(LitExprKind::Long(1))),
            op: BinOpKind::Add,
            right: Box::new(ExprKind::BinOp {
                left: Box::new(ExprKind::Literal(LitExprKind::Long(2))),
                op: BinOpKind::Multiply,
                right: Box::new(ExprKind::Literal(LitExprKind::Long(3))),
            })
        }
    );
}

#[test]
fn test_relational_before_equality() {
    let token_stream = parse_tokens_no_eof("1 < 2 == 3");
    let result = equality_expr(token_stream.source).parse(&token_stream.tokens);
    assert!(!result.has_errors());
    let expr = result.into_output().expect("expected equality expr");

    assert_eq!(
        expr,
        ExprKind::BinOp {
            left: Box::new(ExprKind::BinOp {
                left: Box::new(ExprKind::Literal(LitExprKind::Long(1))),
                op: BinOpKind::LessThan,
                right: Box::new(ExprKind::Literal(LitExprKind::Long(2))),
            }),
            op: BinOpKind::Equal,
            right: Box::new(ExprKind::Literal(LitExprKind::Long(3))),
        }
    );
}

#[test]
fn test_logical_and_before_or() {
    let token_stream = parse_tokens_no_eof("true and false or true");
    let result = logical_or_expr(token_stream.source).parse(&token_stream.tokens);
    assert!(!result.has_errors());
    let expr = result.into_output().expect("expected logical or expr");

    assert_eq!(
        expr,
        ExprKind::BinOp {
            left: Box::new(ExprKind::BinOp {
                left: Box::new(ExprKind::Literal(LitExprKind::Boolean(true))),
                op: BinOpKind::And,
                right: Box::new(ExprKind::Literal(LitExprKind::Boolean(false))),
            }),
            op: BinOpKind::Or,
            right: Box::new(ExprKind::Literal(LitExprKind::Boolean(true))),
        }
    );
}

#[test]
fn test_lit_expr_variants() {
    let cases = [
        ("true", ExprKind::Literal(LitExprKind::Boolean(true))),
        ("123", ExprKind::Literal(LitExprKind::Long(123))),
        ("1.5", ExprKind::Literal(LitExprKind::Real(1.5))),
    ];

    for (input, expected) in cases {
        let token_stream = parse_tokens_no_eof(input);
        let result = lit_expr(token_stream.source).parse(&token_stream.tokens);
        assert!(!result.has_errors(), "input: {input}");
        let expr = result.into_output().expect("expected literal expr");
        assert_eq!(expr, expected, "input: {input}");
    }
}

#[test]
fn test_boolean_lit_parser() {
    let token_stream = parse_tokens_no_eof("false");
    let result = boolean_lit(token_stream.source).parse(&token_stream.tokens);
    assert!(!result.has_errors());
    let lit = result.into_output().expect("expected boolean literal");
    assert_eq!(lit, LitExprKind::Boolean(false));
}

#[test]
fn test_long_lit_parser() {
    let token_stream = parse_tokens_no_eof("42");
    let result = long_lit(token_stream.source).parse(&token_stream.tokens);
    assert!(!result.has_errors());
    let lit = result.into_output().expect("expected long literal");
    assert_eq!(lit, LitExprKind::Long(42));
}

#[test]
fn test_real_lit_parser() {
    let token_stream = parse_tokens_no_eof("3.25");
    let result = real_lit(token_stream.source).parse(&token_stream.tokens);
    assert!(!result.has_errors());
    let lit = result.into_output().expect("expected real literal");
    assert_eq!(lit, LitExprKind::Real(3.25));
}

#[test]
fn test_string_lit_parser() {
    let token_stream = parse_tokens_no_eof("'hello'");
    let result = string_lit(token_stream.source).parse(&token_stream.tokens);
    assert!(!result.has_errors());
    let lit = result.into_output().expect("expected string literal");
    assert_eq!(lit, LitExprKind::String("'hello'".to_string()));
}

#[test]
fn test_string_lit_concat_parser() {
    let token_stream = parse_tokens_no_eof("'a' \"b\"");
    let result = string_lit(token_stream.source).parse(&token_stream.tokens);
    assert!(!result.has_errors());
    let lit = result
        .into_output()
        .expect("expected concatenated string literal");
    assert_eq!(lit, LitExprKind::String("'a'\"b\"".to_string()));
}

#[test]
fn test_multiplicative_ops() {
    let cases = [
        (
            "2 * 3",
            BinOpKind::Multiply,
            ExprKind::Literal(LitExprKind::Long(2)),
            ExprKind::Literal(LitExprKind::Long(3)),
        ),
        (
            "8 / 4",
            BinOpKind::Divide,
            ExprKind::Literal(LitExprKind::Long(8)),
            ExprKind::Literal(LitExprKind::Long(4)),
        ),
        (
            "9 % 5",
            BinOpKind::Modulo,
            ExprKind::Literal(LitExprKind::Long(9)),
            ExprKind::Literal(LitExprKind::Long(5)),
        ),
    ];

    for (input, op, left, right) in cases {
        let token_stream = parse_tokens_no_eof(input);
        let result = multiplicative_expr(token_stream.source).parse(&token_stream.tokens);
        assert!(!result.has_errors(), "input: {input}");
        let expr = result.into_output().expect("expected multiplicative expr");
        assert_eq!(
            expr,
            ExprKind::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            },
            "input: {input}"
        );
    }
}

#[test]
fn test_additive_ops() {
    let cases = [
        (
            "1 + 2",
            BinOpKind::Add,
            ExprKind::Literal(LitExprKind::Long(1)),
            ExprKind::Literal(LitExprKind::Long(2)),
        ),
        (
            "5 - 3",
            BinOpKind::Subtract,
            ExprKind::Literal(LitExprKind::Long(5)),
            ExprKind::Literal(LitExprKind::Long(3)),
        ),
    ];

    for (input, op, left, right) in cases {
        let token_stream = parse_tokens_no_eof(input);
        let result = additive_expr(token_stream.source).parse(&token_stream.tokens);
        assert!(!result.has_errors(), "input: {input}");
        let expr = result.into_output().expect("expected additive expr");
        assert_eq!(
            expr,
            ExprKind::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            },
            "input: {input}"
        );
    }
}

#[test]
fn test_relational_ops() {
    let cases = [
        ("1 < 2", BinOpKind::LessThan),
        ("1 <= 2", BinOpKind::LessThanOrEqual),
        ("1 > 2", BinOpKind::GreaterThan),
        ("1 >= 2", BinOpKind::GreaterThanOrEqual),
    ];

    for (input, op) in cases {
        let token_stream = parse_tokens_no_eof(input);
        let result = relational_expr(token_stream.source).parse(&token_stream.tokens);
        assert!(!result.has_errors(), "input: {input}");
        let expr = result.into_output().expect("expected relational expr");
        assert_eq!(
            expr,
            ExprKind::BinOp {
                left: Box::new(ExprKind::Literal(LitExprKind::Long(1))),
                op,
                right: Box::new(ExprKind::Literal(LitExprKind::Long(2))),
            },
            "input: {input}"
        );
    }
}

#[test]
fn test_equality_ops() {
    let cases = [
        ("1 == 2", BinOpKind::Equal),
        ("1 != 2", BinOpKind::NotEqual),
        ("1 <> 2", BinOpKind::NotEqual),
    ];

    for (input, op) in cases {
        let token_stream = parse_tokens_no_eof(input);
        let result = equality_expr(token_stream.source).parse(&token_stream.tokens);
        assert!(!result.has_errors(), "input: {input}");
        let expr = result.into_output().expect("expected equality expr");
        assert_eq!(
            expr,
            ExprKind::BinOp {
                left: Box::new(ExprKind::Literal(LitExprKind::Long(1))),
                op,
                right: Box::new(ExprKind::Literal(LitExprKind::Long(2))),
            },
            "input: {input}"
        );
    }
}

#[test]
fn test_logical_and_expr() {
    let token_stream = parse_tokens_no_eof("true and false");
    let result = logical_and_expr(token_stream.source).parse(&token_stream.tokens);
    assert!(!result.has_errors());
    let expr = result.into_output().expect("expected logical and expr");

    assert_eq!(
        expr,
        ExprKind::BinOp {
            left: Box::new(ExprKind::Literal(LitExprKind::Boolean(true))),
            op: BinOpKind::And,
            right: Box::new(ExprKind::Literal(LitExprKind::Boolean(false))),
        }
    );
}

#[test]
fn test_logical_or_expr() {
    let token_stream = parse_tokens_no_eof("true or false");
    let result = logical_or_expr(token_stream.source).parse(&token_stream.tokens);
    assert!(!result.has_errors());
    let expr = result.into_output().expect("expected logical or expr");

    assert_eq!(
        expr,
        ExprKind::BinOp {
            left: Box::new(ExprKind::Literal(LitExprKind::Boolean(true))),
            op: BinOpKind::Or,
            right: Box::new(ExprKind::Literal(LitExprKind::Boolean(false))),
        }
    );
}

#[test]
fn test_unnamed_expr_entry() {
    let token_stream = parse_tokens_no_eof("1 + 2");
    let result = unnamed_expr(token_stream.source).parse(&token_stream.tokens);
    assert!(!result.has_errors());
    let expr = result.into_output().expect("expected unnamed expr");

    assert_eq!(
        expr,
        ExprKind::BinOp {
            left: Box::new(ExprKind::Literal(LitExprKind::Long(1))),
            op: BinOpKind::Add,
            right: Box::new(ExprKind::Literal(LitExprKind::Long(2))),
        }
    );
}

#[test]
fn test_iden_name_decl_expr() {
    let token_stream = parse_tokens_no_eof("Column");
    let result = iden_name_decl_expr(token_stream.source).parse(&token_stream.tokens);
    assert!(!result.has_errors());
    let expr = result.into_output().expect("expected identifier name decl");
    assert_eq!(expr, ExprKind::NameDecl("Column".to_string()));
}

#[test]
fn test_bracketed_name_decl_expr() {
    let token_stream = parse_tokens_no_eof("['col']");
    let result = bracketed_name_decl_expr(token_stream.source).parse(&token_stream.tokens);
    assert!(!result.has_errors());
    let expr = result.into_output().expect("expected bracketed name decl");
    assert_eq!(expr, ExprKind::NameDecl("'col'".to_string()));
}

#[test]
fn test_ext_kw_as_iden_name_decl_expr() {
    let token_stream = parse_tokens_no_eof("where");
    let result = ext_kw_as_iden_name_decl_expr().parse(&token_stream.tokens);
    assert!(!result.has_errors());
    let expr = result
        .into_output()
        .expect("expected extended keyword name decl");
    assert_eq!(expr, ExprKind::NameDecl("where".to_string()));
}

#[test]
fn test_named_expr() {
    let token_stream = parse_tokens_no_eof("where = 1");
    let result = named_expr(token_stream.source).parse(&token_stream.tokens);
    assert!(!result.has_errors());
    let expr = result.into_output().expect("expected named expr");

    assert_eq!(
        expr,
        ExprKind::SimpleNamed {
            name: Box::new(ExprKind::NameDecl("where".to_string())),
            expr: Box::new(ExprKind::Literal(LitExprKind::Long(1))),
        }
    );
}
