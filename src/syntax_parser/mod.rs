use crate::token_parser::{LexicalToken, SyntaxKind};
use chumsky::prelude::*;
use chumsky::primitive::select;

#[derive(Debug, Clone)]
pub enum SyntaxNode {
    NameDeclaration(LexicalToken),
    BracketedNameDeclaration(LexicalToken, Box<SyntaxNode>, LexicalToken),
    NameAndTypeDeclaration(Box<SyntaxNode>, LexicalToken, Box<SyntaxNode>),
    StringLiteralExpression(LexicalToken),
    CompoundStringLiteralExpression(Vec<LexicalToken>),
    PrimitiveTypeExpression(LexicalToken),
    SchemaTypeExpression(LexicalToken, Box<SyntaxNode>, LexicalToken),
    SeparatedElement(Box<SyntaxNode>, Option<LexicalToken>),
    SyntaxList(Vec<SyntaxNode>),
    StarExpression(LexicalToken),
}

pub fn query<'a>() -> impl Parser<'a, &'a [LexicalToken], SyntaxNode> {
    name_and_type_declaration()
}

fn token<'a>(kind: SyntaxKind) -> impl Parser<'a, &'a [LexicalToken], LexicalToken> {
    select(move |token: LexicalToken, _| {
        if token.kind() == kind {
            Some(token)
        } else {
            None
        }
    })
}

fn name_and_type_declaration<'a>() -> impl Parser<'a, &'a [LexicalToken], SyntaxNode> {
    let case1 = extended_name_declaration()
        .then(token(SyntaxKind::ColonToken))
        .then(schema_type());

    let case2 = extended_name_declaration()
        .then(token(SyntaxKind::ColonToken))
        .then(param_type());

    case1.or(case2).map(|((name, colon), schema_type)| {
        SyntaxNode::NameAndTypeDeclaration(Box::new(name), colon, Box::new(schema_type))
    })
}

fn schema_type<'a>() -> impl Parser<'a, &'a [LexicalToken], SyntaxNode> {
    schema_asterisk_type().or(schema_multipart_type())
}

fn schema_asterisk_type<'a>() -> impl Parser<'a, &'a [LexicalToken], SyntaxNode> {
    token(SyntaxKind::OpenParenToken)
        .then(star_expression())
        .then(token(SyntaxKind::CloseParenToken))
        .map(|((open_paren, star), close_paren)| {
            SyntaxNode::SchemaTypeExpression(
                open_paren,
                Box::new(SyntaxNode::SyntaxList(vec![star])),
                close_paren,
            )
        })
}

fn schema_multipart_type<'a>() -> impl Parser<'a, &'a [LexicalToken], SyntaxNode> {
    token(SyntaxKind::OpenParenToken)
        .then(
            name_and_type_declaration()
                .separated_by(token(SyntaxKind::CommaToken))
                .allow_trailing()
                .collect::<Vec<_>>()
                .map(SyntaxNode::SyntaxList),
        )
        .then(token(SyntaxKind::CloseParenToken))
        .map(|((open_paren, columns), close_paren)| {
            SyntaxNode::SchemaTypeExpression(open_paren, Box::new(columns), close_paren)
        })
        .boxed()
}

fn extended_name_declaration<'a>() -> impl Parser<'a, &'a [LexicalToken], SyntaxNode> {
    identifier_name_declaration().or(bracketed_name_declaration())
    // TODO: ExtendedKeywordAsIdentifierNameDeclaration
}

fn bracketed_name_declaration<'a>() -> impl Parser<'a, &'a [LexicalToken], SyntaxNode> {
    token(SyntaxKind::OpenBracketToken)
        .then(string_or_compound_string_literal())
        .then(token(SyntaxKind::CloseBracketToken))
        .map(|((open_bracket, name), close_bracket)| {
            SyntaxNode::BracketedNameDeclaration(open_bracket, Box::new(name), close_bracket)
        })
}

fn identifier_name_declaration<'a>() -> impl Parser<'a, &'a [LexicalToken], SyntaxNode> {
    token(SyntaxKind::IdentifierToken).map(SyntaxNode::NameDeclaration)
}

fn string_or_compound_string_literal<'a>() -> impl Parser<'a, &'a [LexicalToken], SyntaxNode> {
    token(SyntaxKind::StringLiteralToken)
        .repeated()
        .at_least(1)
        .collect()
        .map(|tokens: Vec<LexicalToken>| {
            if tokens.len() == 1 {
                SyntaxNode::StringLiteralExpression(tokens[0].clone())
            } else {
                SyntaxNode::CompoundStringLiteralExpression(tokens)
            }
        })
}

fn param_type<'a>() -> impl Parser<'a, &'a [LexicalToken], SyntaxNode> {
    token(SyntaxKind::BoolKeyword)
        .or(token(SyntaxKind::DateTimeKeyword))
        .or(token(SyntaxKind::DecimalKeyword))
        .or(token(SyntaxKind::DynamicKeyword))
        .or(token(SyntaxKind::GuidKeyword))
        .or(token(SyntaxKind::IntKeyword))
        .or(token(SyntaxKind::LongKeyword))
        .or(token(SyntaxKind::RealKeyword))
        .or(token(SyntaxKind::StringKeyword))
        .or(token(SyntaxKind::TimespanKeyword))
        .map(SyntaxNode::PrimitiveTypeExpression)
}

fn star_expression<'a>() -> impl Parser<'a, &'a [LexicalToken], SyntaxNode> {
    token(SyntaxKind::AsteriskToken).map(|token| SyntaxNode::StarExpression(token))
}
