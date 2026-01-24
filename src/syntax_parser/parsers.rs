use crate::token_parser::{LexicalToken, SyntaxKind};
use chumsky::prelude::*;
use chumsky::primitive::select;

use super::types::*;

macro_rules! parser_return {
    ($output:ty) => {
        impl Parser<'a, &'a [LexicalToken], $output, extra::Err<Rich<'a, LexicalToken>>> + Clone
    };
}

pub fn query<'a>(input: &'a str) -> parser_return!(SyntaxNode) {
    json(input)
}

fn json<'a>(input: &'a str) -> parser_return!(SyntaxNode) {
    recursive(|recursion| {
        let json_value = boolean_literal(input)
            .or(json_number(input))
            .or(string_literal(input));

        let json_array = json_value
            .clone()
            .separated_by(token(SyntaxKind::CommaToken))
            .collect()
            .delimited_by(
                token(SyntaxKind::OpenBracketToken),
                token(SyntaxKind::CloseBracketToken),
            )
            .map(|elements| SyntaxNode::JsonArrayExpression(elements));

        let json_pair = string_literal(input)
            .then(token(SyntaxKind::ColonToken).ignore_then(recursion))
            .map(|(key, value)| SyntaxNode::JsonPair(Box::new(key), Box::new(value)));

        let json_object = json_pair
            .separated_by(token(SyntaxKind::CommaToken))
            .collect::<Vec<_>>()
            .delimited_by(
                token(SyntaxKind::OpenBraceToken),
                token(SyntaxKind::CloseBraceToken),
            )
            .map(|pairs| SyntaxNode::JsonObjectExpression(pairs));

        choice((json_value, json_array, json_object))
    })
}

fn json_number<'a>(input: &'a str) -> parser_return!(SyntaxNode) {
    token(SyntaxKind::MinusToken)
        .then(long_literal(input).or(real_literal(input)))
        .map(|(_, value)| SyntaxNode::PrefixUnaryExpression {
            operator: Some(UnaryOperatorKind::Minus),
            expression: Box::new(value),
        })
        .or(long_literal(input).or(real_literal(input)))
}

fn boolean_literal<'a>(input: &'a str) -> parser_return!(SyntaxNode) {
    token(SyntaxKind::BooleanLiteralToken).map(|token| {
        let text_span = token.text_span();
        let parse_result = input[text_span.start..text_span.end].parse::<bool>();

        match parse_result {
            Ok(value) => SyntaxNode::BoolLiteral(value),
            Err(_) => SyntaxNode::BoolLiteral(false), // TODO: Handle error properly
        }
    })
}

fn long_literal<'a>(input: &'a str) -> parser_return!(SyntaxNode) {
    token(SyntaxKind::LongLiteralToken).map(|token| {
        let text_span = token.text_span();
        let parse_result = input[text_span.start..text_span.end].parse::<i64>();

        match parse_result {
            Ok(value) => SyntaxNode::LongLiteral(value),
            Err(_) => SyntaxNode::LongLiteral(0), // TODO: Handle error properly
        }
    })
}

fn real_literal<'a>(input: &'a str) -> parser_return!(SyntaxNode) {
    token(SyntaxKind::RealLiteralToken).map(|token| {
        let text_span = token.text_span();
        let parse_result = input[text_span.start..text_span.end].parse::<f64>();

        match parse_result {
            Ok(value) => SyntaxNode::RealLiteral(value),
            Err(_) => SyntaxNode::RealLiteral(0.0), // TODO: Handle error properly
        }
    })
}

fn string_literal<'a>(input: &'a str) -> parser_return!(SyntaxNode) {
    token(SyntaxKind::StringLiteralToken).map(|token| {
        let text_span = token.text_span();
        let raw_string = &input[text_span.start..text_span.end];
        let unquoted_string = &raw_string[1..raw_string.len() - 1]; // TODO: Handle all quoting cases

        SyntaxNode::StringLiteral(unquoted_string.to_string())
    })
}

fn token<'a>(kind: SyntaxKind) -> parser_return!(LexicalToken) {
    select(move |token: LexicalToken, _| {
        if token.kind() == kind {
            Some(token)
        } else {
            None
        }
    })
}
