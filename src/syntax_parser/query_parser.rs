use crate::{
    parser_return,
    syntax_parser::{SyntaxKind, expr_parsers::*},
    token_parser::TokenKind,
};
use chumsky::prelude::*;

pub fn query<'a>(source: &'a str) -> parser_return!(SyntaxKind) {
    named_expr(source)
        .or(unnamed_expr(source))
        .then_ignore(just(TokenKind::EndOfFile).or_not())
        .map(SyntaxKind::Expr)
}
