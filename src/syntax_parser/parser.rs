use crate::{
    syntax_parser::SyntaxKind,
    token_parser::{KeywordKind, LiteralKind, PunctuationKind, TokenKind},
};
use chumsky::prelude::*;

macro_rules! parser_return {
    ($output:ty) => {
        impl Parser<'a, &'a [TokenKind], $output, extra::Err<Rich<'a, TokenKind>>> + Clone
    };
}

pub fn query<'a>() -> parser_return!(SyntaxKind) {
    directive_token()
        .repeated()
        .collect::<Vec<_>>()
        .map(|directives| {
            directives
                .iter()
                .map(|directive| SyntaxKind::Directive(directive.clone()))
                .collect()
        })
        .then(let_statement().repeated().collect::<Vec<_>>())
        .then(eof_token().or_not())
        .map(|((directives, let_statements), _)| SyntaxKind::QueryBlock(directives, let_statements))
}

fn let_statement<'a>() -> parser_return!(SyntaxKind) {
    keyword_token(KeywordKind::Let)
        .then(name_declaration())
        .then(punctuation_token(PunctuationKind::Equal))
        .then(piped_query_operator())
        .map(|(((_, name), _), expr)| SyntaxKind::LetStatement(Box::new(name), Box::new(expr)))
}

fn piped_query_operator<'a>() -> parser_return!(SyntaxKind) {
    punctuation_token(PunctuationKind::Bar)
        .then(count_operator())
        .map(|(_, expr)| expr)
}

fn count_operator<'a>() -> parser_return!(SyntaxKind) {
    keyword_token(KeywordKind::Count)
        .then(punctuation_token(PunctuationKind::OpenParen).not())
        .then(count_as_identifier_clause().or_not())
        .map(|((_, _), count_as_clause)| SyntaxKind::CountOperator(Box::new(count_as_clause)))
}

fn count_as_identifier_clause<'a>() -> parser_return!(SyntaxKind) {
    keyword_token(KeywordKind::As)
        .then(identifier())
        .map(|(_, identifier)| SyntaxKind::CountAsIdentifierClause(Box::new(identifier)))
}

fn name_declaration<'a>() -> parser_return!(SyntaxKind) {
    // TODO: BracedNames
    bracketed_name()
        .or(identifier())
        .map(|expr| SyntaxKind::NameDeclaration(Box::new(expr)))
}

fn bracketed_name<'a>() -> parser_return!(SyntaxKind) {
    punctuation_token(PunctuationKind::OpenBracket)
        .then(string_or_compound_string_literal())
        .then(punctuation_token(PunctuationKind::CloseBracket))
        .map(|((_, expr), _)| SyntaxKind::BracketedName(Box::new(expr)))
}

fn identifier<'a>() -> parser_return!(SyntaxKind) {
    identifier_token().map(SyntaxKind::Identifier)
}

fn string_or_compound_string_literal<'a>() -> parser_return!(SyntaxKind) {
    string_token()
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        .map(|tokens: Vec<String>| {
            if tokens.len() == 1 {
                SyntaxKind::StringLiteral(tokens[0].clone())
            } else {
                SyntaxKind::CompoundStringLiteral(tokens)
            }
        })
}

fn keyword_token<'a>(kind: KeywordKind) -> parser_return!(TokenKind) {
    select! {
        tok @ TokenKind::Keyword(k) if k == kind => tok,
    }
}

fn punctuation_token<'a>(kind: PunctuationKind) -> parser_return!(TokenKind) {
    select! {
        tok @ TokenKind::Punctuation(p) if p == kind => tok,
    }
}

fn string_token<'a>() -> parser_return!(String) {
    select! {
        TokenKind::Literal(LiteralKind::String(value)) => value,
    }
}

fn identifier_token<'a>() -> parser_return!(String) {
    select! {
        TokenKind::Identifier(name) => name,
    }
}

fn directive_token<'a>() -> parser_return!(String) {
    select! {
        TokenKind::Directive(value) => value,
    }
}

fn eof_token<'a>() -> parser_return!(TokenKind) {
    select! {
        tok @ TokenKind::EndOfFile => tok,
    }
}
