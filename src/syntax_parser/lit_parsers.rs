use crate::{
    parser_return,
    syntax_parser::LitExprKind,
    token_parser::{LiteralKind, TokenKind},
};
use chumsky::{prelude::*, primitive::select};

// TODO: DateTime, Decimal, Guid, Int, TimeSpan to be added later
// TODO: Currently we only support literals not goo literals like int(123)

pub(crate) fn boolean_lit<'a>(source: &'a str) -> parser_return!(LitExprKind) {
    select(|token, _| match token {
        TokenKind::Literal(LiteralKind::Boolean(value)) => Some(value),
        _ => None,
    })
    .validate(|span, e, emitter| {
        let value = &source[span.start..span.end];

        match value.parse::<bool>() {
            Ok(val) => LitExprKind::Boolean(val),
            Err(err) => {
                emitter.emit(Rich::custom(
                    e.span(),
                    format!("Failed to parse boolean literal: {}", err),
                ));
                LitExprKind::Boolean(false) // Return a default value
            }
        }
    })
}

pub(crate) fn long_lit<'a>(source: &'a str) -> parser_return!(LitExprKind) {
    select(|token, _| match token {
        TokenKind::Literal(LiteralKind::Long(value)) => Some(value),
        _ => None,
    })
    .validate(|span, e, emitter| {
        let value = &source[span.start..span.end];

        match value.parse::<i64>() {
            Ok(val) => LitExprKind::Long(val),
            Err(err) => {
                emitter.emit(Rich::custom(
                    e.span(),
                    format!("Failed to parse long literal: {}", err),
                ));
                LitExprKind::Long(0) // Return a default value
            }
        }
    })
}

pub(crate) fn real_lit<'a>(source: &'a str) -> parser_return!(LitExprKind) {
    select(|token, _| match token {
        TokenKind::Literal(LiteralKind::Real(value)) => Some(value),
        _ => None,
    })
    .validate(|span, e, emitter| {
        let value = &source[span.start..span.end];

        match value.parse::<f64>() {
            Ok(val) => LitExprKind::Real(val),
            Err(err) => {
                emitter.emit(Rich::custom(
                    e.span(),
                    format!("Failed to parse real literal: {}", err),
                ));
                LitExprKind::Real(0.0) // Return a default value
            }
        }
    })
}

// TODO: Handle escape sequences in string literals
pub(crate) fn string_lit<'a>(source: &'a str) -> parser_return!(LitExprKind) {
    select(|token, _| match token {
        TokenKind::Literal(LiteralKind::String(value)) => Some(source[value.start..value.end].to_string()),
        _ => None,
    })
    .repeated()
    .at_least(1)
    .collect()
    .map(|tokens: Vec<String>| LitExprKind::String(tokens.concat()))
}
