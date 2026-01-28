use crate::{
    parser_return,
    token_parser::{PunctuationKind, TokenKind},
};
use chumsky::prelude::*;

pub(crate) fn punct_token<'a>(kind: PunctuationKind) -> parser_return!(PunctuationKind) {
    select! {
        TokenKind::Punctuation(p) if p == kind => p,
    }
}
