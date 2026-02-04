use crate::{
    parser_return,
    syntax_parser::{
        constants::EXTENDED_KEYWORDS_AS_IDENTIFIERS, lit_parsers::*, types::*, utilities::*,
    },
    token_parser::{KeywordKind, PunctuationKind, TokenKind},
};
use chumsky::{prelude::*, primitive::select};

// TODO: In, InCs, NotIn, NotInCs, HasAny, HasAll, Between and NotBetween operators to be added later
// TODO: Start expression ( * == value) to be added later

pub(crate) fn lit_expr<'a>() -> parser_return!(ExprKind) {
    boolean_lit()
        .or(long_lit())
        .or(real_lit())
        .or(string_lit())
        .map(ExprKind::Literal)
}

pub(crate) fn multiplicative_expr<'a>() -> parser_return!(ExprKind) {
    lit_expr()
        .then(
            punct_token(PunctuationKind::Asterisk)
                .to(BinOpKind::Multiply)
                .or(punct_token(PunctuationKind::Slash).to(BinOpKind::Divide))
                .or(punct_token(PunctuationKind::Percent).to(BinOpKind::Modulo))
                .then(lit_expr())
                .repeated()
                .collect::<Vec<_>>(),
        )
        .map(|(first, rest)| {
            rest.into_iter()
                .fold(first, |left, (op, right)| ExprKind::BinOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                })
        })
}

pub(crate) fn additive_expr<'a>() -> parser_return!(ExprKind) {
    multiplicative_expr()
        .then(
            punct_token(PunctuationKind::Plus)
                .to(BinOpKind::Add)
                .or(punct_token(PunctuationKind::Minus).to(BinOpKind::Subtract))
                .then(multiplicative_expr())
                .repeated()
                .collect::<Vec<_>>(),
        )
        .map(|(first, rest)| {
            rest.into_iter()
                .fold(first, |left, (op, right)| ExprKind::BinOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                })
        })
}

pub(crate) fn relational_expr<'a>() -> parser_return!(ExprKind) {
    additive_expr()
        .then(
            punct_token(PunctuationKind::LessThan)
                .to(BinOpKind::LessThan)
                .or(punct_token(PunctuationKind::LessThanOrEqual).to(BinOpKind::LessThanOrEqual))
                .or(punct_token(PunctuationKind::GreaterThan).to(BinOpKind::GreaterThan))
                .or(punct_token(PunctuationKind::GreaterThanOrEqual)
                    .to(BinOpKind::GreaterThanOrEqual))
                .then(additive_expr())
                .repeated()
                .collect::<Vec<_>>(),
        )
        .map(|(first, rest)| {
            rest.into_iter()
                .fold(first, |left, (op, right)| ExprKind::BinOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                })
        })
}

pub(crate) fn equality_expr<'a>() -> parser_return!(ExprKind) {
    relational_expr()
        .then(
            punct_token(PunctuationKind::EqualEqual)
                .to(BinOpKind::Equal)
                .or(punct_token(PunctuationKind::BangEqual).to(BinOpKind::NotEqual))
                .or(punct_token(PunctuationKind::LessThanGreaterThan).to(BinOpKind::NotEqual))
                .then(relational_expr())
                .repeated()
                .collect::<Vec<_>>(),
        )
        .map(|(first, rest)| {
            rest.into_iter()
                .fold(first, |left, (op, right)| ExprKind::BinOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                })
        })
}

pub(crate) fn logical_and_expr<'a>() -> parser_return!(ExprKind) {
    equality_expr()
        .then(
            just(TokenKind::Keyword(KeywordKind::And))
                .ignore_then(equality_expr())
                .repeated()
                .collect::<Vec<_>>(),
        )
        .map(|(first, rest)| {
            rest.into_iter().fold(first, |left, right| ExprKind::BinOp {
                left: Box::new(left),
                op: BinOpKind::And,
                right: Box::new(right),
            })
        })
}

pub(crate) fn logical_or_expr<'a>() -> parser_return!(ExprKind) {
    logical_and_expr()
        .then(
            just(TokenKind::Keyword(KeywordKind::Or))
                .ignore_then(logical_and_expr())
                .repeated()
                .collect::<Vec<_>>(),
        )
        .map(|(first, rest)| {
            rest.into_iter().fold(first, |left, right| ExprKind::BinOp {
                left: Box::new(left),
                op: BinOpKind::Or,
                right: Box::new(right),
            })
        })
}

pub(crate) fn unnamed_expr<'a>() -> parser_return!(ExprKind) {
    logical_or_expr()
}

pub(crate) fn iden_name_decl_expr<'a>() -> parser_return!(ExprKind) {
    select(|token, _| match token {
        TokenKind::Identifier(value) => Some(ExprKind::NameDecl(value)),
        _ => None,
    })
}

//TODO: ClientParameterReferenceExpr to be added later

pub(crate) fn bracketed_name_decl_expr<'a>() -> parser_return!(ExprKind) {
    punct_token(PunctuationKind::OpenBracket)
        .ignore_then(string_lit())
        .then_ignore(punct_token(PunctuationKind::CloseBracket))
        .map(|lit| match lit {
            LitExprKind::String(value) => ExprKind::NameDecl(value),
            _ => panic!("Bracketed name must be a string literal"),
        })
}

pub(crate) fn ext_kw_as_iden_name_decl_expr<'a>() -> parser_return!(ExprKind) {
    select(|token, _| match token {
        TokenKind::Keyword(value) => EXTENDED_KEYWORDS_AS_IDENTIFIERS
            .iter()
            .find(|kw| value == kw.0)
            .map(|kw| ExprKind::NameDecl(kw.1.to_string())),
        _ => None,
    })
}

pub(crate) fn ext_name_decl_expr<'a>() -> parser_return!(ExprKind) {
    iden_name_decl_expr()
        .or(bracketed_name_decl_expr())
        .or(ext_kw_as_iden_name_decl_expr())
}

// TODO: Dashed name decl to be added later

// TODO: Tuple name decl to be added later
pub(crate) fn named_expr<'a>() -> parser_return!(ExprKind) {
    ext_name_decl_expr()
        .then(punct_token(PunctuationKind::Equal).ignore_then(unnamed_expr()))
        .map(|(name, expr)| ExprKind::SimpleNamed {
            name: Box::new(name),
            expr: Box::new(expr),
        })
}
