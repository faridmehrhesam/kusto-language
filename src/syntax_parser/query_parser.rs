use crate::{
    parser_return,
    syntax_parser::{SyntaxKind, expr_parsers::*},
    token_parser::TokenKind,
};
use chumsky::prelude::*;

pub fn query<'a>() -> parser_return!(SyntaxKind) {
    unnamed_expr()
        .then_ignore(just(TokenKind::EndOfFile).or_not())
        .map(SyntaxKind::Expr)
}
