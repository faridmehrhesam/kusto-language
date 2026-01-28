use super::{expr_parsers::*, lit_parsers::*, types::*};
use crate::token_parser::{ParseOptions, TokenKind, parse_tokens};
use chumsky::prelude::*;

fn parse_tokens_no_eof(input: &str) -> Vec<TokenKind> {
    let options = ParseOptions::default().with_always_produce_end_tokens(false);
    parse_tokens(input, &options)
}

#[test]
fn test_additive_multiplicative_precedence() {
    let tokens = parse_tokens_no_eof("1 + 2 * 3");
    let result = additive_expr().parse(&tokens);
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
    let tokens = parse_tokens_no_eof("1 < 2 == 3");
    let result = equality_expr().parse(&tokens);
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
    let tokens = parse_tokens_no_eof("true and false or true");
    let result = logical_or_expr().parse(&tokens);
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
        let tokens = parse_tokens_no_eof(input);
        let result = lit_expr().parse(&tokens);
        assert!(!result.has_errors(), "input: {input}");
        let expr = result.into_output().expect("expected literal expr");
        assert_eq!(expr, expected, "input: {input}");
    }
}

#[test]
fn test_boolean_lit_parser() {
    let tokens = parse_tokens_no_eof("false");
    let result = boolean_lit().parse(&tokens);
    assert!(!result.has_errors());
    let lit = result.into_output().expect("expected boolean literal");
    assert_eq!(lit, LitExprKind::Boolean(false));
}

#[test]
fn test_long_lit_parser() {
    let tokens = parse_tokens_no_eof("42");
    let result = long_lit().parse(&tokens);
    assert!(!result.has_errors());
    let lit = result.into_output().expect("expected long literal");
    assert_eq!(lit, LitExprKind::Long(42));
}

#[test]
fn test_real_lit_parser() {
    let tokens = parse_tokens_no_eof("3.25");
    let result = real_lit().parse(&tokens);
    assert!(!result.has_errors());
    let lit = result.into_output().expect("expected real literal");
    assert_eq!(lit, LitExprKind::Real(3.25));
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
        let tokens = parse_tokens_no_eof(input);
        let result = multiplicative_expr().parse(&tokens);
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
        let tokens = parse_tokens_no_eof(input);
        let result = additive_expr().parse(&tokens);
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
        let tokens = parse_tokens_no_eof(input);
        let result = relational_expr().parse(&tokens);
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
        let tokens = parse_tokens_no_eof(input);
        let result = equality_expr().parse(&tokens);
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
    let tokens = parse_tokens_no_eof("true and false");
    let result = logical_and_expr().parse(&tokens);
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
    let tokens = parse_tokens_no_eof("true or false");
    let result = logical_or_expr().parse(&tokens);
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
    let tokens = parse_tokens_no_eof("1 + 2");
    let result = unnamed_expr().parse(&tokens);
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
