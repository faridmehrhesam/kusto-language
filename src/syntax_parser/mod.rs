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
    JsonArrayExpression(JsonArrayExpression),
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
            SyntaxNode::JsonArrayExpression(node) => write!(f, "{:#?}", node),
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
pub struct JsonArrayExpression {
    open_bracket: Box<LexicalToken>,
    elements: SyntaxList,
    close_bracket: Box<LexicalToken>,
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
-> impl Parser<'a, &'a [LexicalToken], SyntaxNode, extra::Err<Rich<'a, LexicalToken>>> + Clone {
    json_value()
}

fn token<'a>(
    kind: SyntaxKind,
) -> impl Parser<'a, &'a [LexicalToken], LexicalToken, extra::Err<Rich<'a, LexicalToken>>> + Clone {
    select(move |token: LexicalToken, _| {
        if token.kind() == kind {
            Some(token)
        } else {
            None
        }
    })
}

fn json_array<'a>(
    value: impl Parser<'a, &'a [LexicalToken], SyntaxNode, extra::Err<Rich<'a, LexicalToken>>>
    + Clone
    + 'a,
) -> impl Parser<'a, &'a [LexicalToken], SyntaxNode, extra::Err<Rich<'a, LexicalToken>>> + Clone + 'a
{
    token(SyntaxKind::OpenBracketToken)
        .then(
            value
                .clone()
                .then(token(SyntaxKind::CommaToken))
                .repeated()
                .collect::<Vec<_>>()
                .then(value)
                .map(|(with_commas, last)| {
                    let mut elements = Vec::new();
                    for (val, comma) in with_commas {
                        elements.push(SyntaxNode::SeparatedElement(SeparatedElement {
                            element: Box::new(val),
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
        .then(token(SyntaxKind::CloseBracketToken))
        .map(|((open_bracket, elements), close_bracket)| {
            let elements = match elements {
                SyntaxNode::SyntaxList(list) => list,
                _ => unreachable!(),
            };

            SyntaxNode::JsonArrayExpression(JsonArrayExpression {
                open_bracket: Box::new(open_bracket),
                elements,
                close_bracket: Box::new(close_bracket),
            })
        })
}

fn json_object<'a>(
    value: impl Parser<'a, &'a [LexicalToken], SyntaxNode, extra::Err<Rich<'a, LexicalToken>>>
    + Clone
    + 'a,
) -> impl Parser<'a, &'a [LexicalToken], SyntaxNode, extra::Err<Rich<'a, LexicalToken>>> + Clone + 'a
{
    let json_pair = json_pair(value);
    token(SyntaxKind::OpenBraceToken)
        .then(
            json_pair
                .clone()
                .then(token(SyntaxKind::CommaToken))
                .repeated()
                .collect::<Vec<_>>()
                .then(json_pair)
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

fn json_pair<'a>(
    value: impl Parser<'a, &'a [LexicalToken], SyntaxNode, extra::Err<Rich<'a, LexicalToken>>>
    + Clone
    + 'a,
) -> impl Parser<'a, &'a [LexicalToken], SyntaxNode, extra::Err<Rich<'a, LexicalToken>>> + Clone + 'a
{
    string_literal()
        .then(token(SyntaxKind::ColonToken))
        .then(value)
        .map(|((key, colon), val)| {
            SyntaxNode::JsonPair(JsonPair {
                key: Box::new(key),
                colon: Box::new(colon),
                value: Box::new(val),
            })
        })
}

fn json_value<'a>()
-> impl Parser<'a, &'a [LexicalToken], SyntaxNode, extra::Err<Rich<'a, LexicalToken>>> + Clone {
    recursive(|value| {
        let number = json_number();
        let boolean = boolean_literal();
        let object = json_object(value.clone());
        let array = json_array(value);

        choice((number, boolean, object, array))
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
