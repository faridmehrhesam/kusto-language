use std::fmt::Debug;

use crate::token_parser::{LexicalToken, SyntaxKind};
use chumsky::prelude::*;
use chumsky::primitive::select;

#[derive(Clone)]
pub enum SyntaxNode {
    LiteralExpression(LiteralExpression),
    PrefixUnaryExpression(PrefixUnaryExpression),
    JsonPair(JsonPair),
    JsonObjectExpression(JsonObjectExpression),
    SeparatedElement(SeparatedElement),
    SyntaxList(SyntaxList),
}

impl Debug for SyntaxNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxNode::LiteralExpression(node) => write!(f, "{:#?}", node),
            SyntaxNode::PrefixUnaryExpression(node) => write!(f, "{:#?}", node),
            SyntaxNode::JsonPair(node) => write!(f, "{:#?}", node),
            SyntaxNode::JsonObjectExpression(node) => write!(f, "{:#?}", node),
            SyntaxNode::SeparatedElement(node) => write!(f, "{:#?}", node),
            SyntaxNode::SyntaxList(node) => write!(f, "{:#?}", node),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrefixUnaryExpression {
    kind: SyntaxKind,
    operator: Option<Box<LexicalToken>>,
    expression: Box<SyntaxNode>,
}

#[derive(Debug, Clone)]
pub struct LiteralExpression {
    kind: SyntaxKind,
    token: Box<LexicalToken>,
}

#[derive(Debug, Clone)]
pub struct JsonPair {
    key: Box<SyntaxNode>,
    colon: Box<LexicalToken>,
    value: Box<SyntaxNode>,
}

#[derive(Debug, Clone)]
pub struct JsonObjectExpression {
    open_brace: Box<LexicalToken>,
    pairs: SyntaxList,
    close_brace: Box<LexicalToken>,
}

#[derive(Debug, Clone)]
pub struct SeparatedElement {
    element: Box<SyntaxNode>,
    separator: Option<Box<LexicalToken>>,
}

#[derive(Debug, Clone)]
pub struct SyntaxList {
    elements: Vec<SyntaxNode>,
}

pub fn query<'a>()
-> impl Parser<'a, &'a [LexicalToken], SyntaxNode, extra::Err<Rich<'a, LexicalToken>>>  + Clone{
    json_object()
}

fn token<'a>(
    kind: SyntaxKind,
) -> impl Parser<'a, &'a [LexicalToken], LexicalToken, extra::Err<Rich<'a, LexicalToken>>>  + Clone{
    select(move |token: LexicalToken, _| {
        if token.kind() == kind {
            Some(token)
        } else {
            None
        }
    })
}

fn json_object<'a>()
-> impl Parser<'a, &'a [LexicalToken], SyntaxNode, extra::Err<Rich<'a, LexicalToken>>> + Clone {
    token(SyntaxKind::OpenBraceToken)
        .then(
            json_pair()
                .then(token(SyntaxKind::CommaToken))
                .repeated()
                .collect::<Vec<_>>()
                .then(json_pair())
                .map(|(with_commas, last)| {
                    let mut elements = Vec::new();
                    for (pair, comma) in with_commas {
                        elements.push(SyntaxNode::SeparatedElement(SeparatedElement {
                            element: Box::new(pair),
                            separator: Some(Box::new(comma)),
                        }));
                    }

                    elements.push(SyntaxNode::SeparatedElement(SeparatedElement {
                        element: Box::new(last),
                        separator: None,
                    }));

                    SyntaxNode::SyntaxList(SyntaxList { elements })
                }),
        )
        .then(token(SyntaxKind::CloseBraceToken))
        .map(|((open_brace, pairs), close_brace)| {
            let pairs = match pairs {
                SyntaxNode::SyntaxList(list) => list,
                _ => unreachable!(),
            };

            SyntaxNode::JsonObjectExpression(JsonObjectExpression {
                open_brace: Box::new(open_brace),
                pairs,
                close_brace: Box::new(close_brace),
            })
        })
}

fn json_pair<'a>()
-> impl Parser<'a, &'a [LexicalToken], SyntaxNode, extra::Err<Rich<'a, LexicalToken>>> + Clone {
    string_literal()
        .then(token(SyntaxKind::ColonToken))
        .then(json_value())
        .map(|((key, colon), value)| {
            SyntaxNode::JsonPair(JsonPair {
                key: Box::new(key),
                colon: Box::new(colon),
                value: Box::new(value),
            })
        })
}

fn json_value<'a>()
-> impl Parser<'a, &'a [LexicalToken], SyntaxNode, extra::Err<Rich<'a, LexicalToken>>> + Clone {
    recursive(|value| {        
        let number = json_number();
        let boolean = boolean_literal();
        let object = json_object();

        choice((number, boolean, object))
    })
}

fn json_number<'a>()
-> impl Parser<'a, &'a [LexicalToken], SyntaxNode, extra::Err<Rich<'a, LexicalToken>>> + Clone {
    token(SyntaxKind::MinusToken)
        .or_not()
        .then(long_literal().or(real_literal()))
        .map(|(unary_op, value)| {
            SyntaxNode::PrefixUnaryExpression(PrefixUnaryExpression {
                kind: SyntaxKind::UnaryMinusExpression,
                operator: unary_op.map(Box::new),
                expression: Box::new(value),
            })
        })
}

fn boolean_literal<'a>()
-> impl Parser<'a, &'a [LexicalToken], SyntaxNode, extra::Err<Rich<'a, LexicalToken>>> + Clone {
    token(SyntaxKind::BooleanLiteralToken).map(|token| {
        SyntaxNode::LiteralExpression(LiteralExpression {
            kind: SyntaxKind::BooleanLiteralExpression,
            token: Box::new(token),
        })
    })
}

fn long_literal<'a>()
-> impl Parser<'a, &'a [LexicalToken], SyntaxNode, extra::Err<Rich<'a, LexicalToken>>> + Clone {
    token(SyntaxKind::LongLiteralToken).map(|token| {
        SyntaxNode::LiteralExpression(LiteralExpression {
            kind: SyntaxKind::LongLiteralExpression,
            token: Box::new(token),
        })
    })
}

fn real_literal<'a>()
-> impl Parser<'a, &'a [LexicalToken], SyntaxNode, extra::Err<Rich<'a, LexicalToken>>> + Clone {
    token(SyntaxKind::RealLiteralToken).map(|token| {
        SyntaxNode::LiteralExpression(LiteralExpression {
            kind: SyntaxKind::RealLiteralExpression,
            token: Box::new(token),
        })
    })
}

fn string_literal<'a>()
-> impl Parser<'a, &'a [LexicalToken], SyntaxNode, extra::Err<Rich<'a, LexicalToken>>> + Clone {
    token(SyntaxKind::StringLiteralToken).map(|token| {
        SyntaxNode::LiteralExpression(LiteralExpression {
            kind: SyntaxKind::StringLiteralExpression,
            token: Box::new(token),
        })
    })
}
